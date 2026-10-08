//! The encoding boundary (design D3): the editor's text becomes the bytes the file holds, and byte columns become
//! the editor's UTF-16 columns.

use crate::encoding_tables::TABLES;
use lsp_types::Position;
use qb64rust_base::{SourceFile, to_u32};
use std::collections::HashMap;

/// How a document's text is stored as bytes: a single-byte table (VS Code's encoding id, `cp437` the default), or
/// UTF-8. An encoding the server has no table for is read as UTF-8: the parse then sees other bytes than the file
/// holds, but every position is still right.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    Table(&'static str, &'static [u16; 256]),
    Utf8,
}

impl Encoding {
    pub const DEFAULT: Encoding = Encoding::Table(TABLES[0].0, &TABLES[0].1);

    /// VS Code's encoding id (`files.encoding`, `TextDocument.encoding`), in any case.
    pub fn from_name(name: &str) -> Encoding {
        let name = name.to_ascii_lowercase();
        TABLES
            .iter()
            .find(|(n, _)| *n == name)
            .map_or(Encoding::Utf8, |(n, t)| Encoding::Table(n, t))
    }

    pub fn name(self) -> &'static str {
        match self {
            Encoding::Table(n, _) => n,
            Encoding::Utf8 => "utf8",
        }
    }

    /// The bytes of `text`. In a table, each UTF-16 unit the table cannot hold becomes `?`, as `iconv-lite` writes
    /// it on save: one unit is always one byte.
    pub fn encode(self, text: &str) -> Vec<u8> {
        match self {
            Encoding::Utf8 => text.as_bytes().to_vec(),
            Encoding::Table(_, table) => {
                let mut reverse = HashMap::with_capacity(256);
                // The first byte of a character wins, as in iconv-lite's encoder.
                for (b, &u) in table.iter().enumerate().rev() {
                    reverse.insert(u, u8::try_from(b).expect("256 entries"));
                }
                text.encode_utf16()
                    .map(|u| reverse.get(&u).copied().unwrap_or(b'?'))
                    .collect()
            }
        }
    }

    /// Where the UTF-16 columns of `file`'s lines differ from their byte columns.
    pub fn columns(self, file: &SourceFile) -> Columns {
        match self {
            Encoding::Table(..) => Columns::default(),
            Encoding::Utf8 => Columns::utf8(file),
        }
    }
}

/// The UTF-16 columns of the lines where they differ from the byte columns: for each such 0-based line, the byte
/// column and UTF-16 column of every character start, and of the line's end. Empty for a single-byte encoding.
#[derive(Clone, Debug, Default)]
pub struct Columns {
    lines: HashMap<u32, Vec<(u32, u32)>>,
}

impl Columns {
    /// A UTF-8 file: lines with a byte above 127. An invalid byte is one unit (VS Code shows it as U+FFFD).
    fn utf8(file: &SourceFile) -> Columns {
        let mut lines = HashMap::new();
        for line in 0..file.line_count() {
            let (start, end) = file.line_content(line).expect("line in range");
            let bytes = &file.bytes[start as usize..end as usize];
            if bytes.is_ascii() {
                continue;
            }
            let mut starts = Vec::new();
            let (mut b, mut u) = (0, 0);
            while b < bytes.len() {
                starts.push((to_u32(b), u));
                let (len, units) = utf8_char(&bytes[b..]);
                b += len;
                u += units;
            }
            starts.push((to_u32(b), u));
            lines.insert(line, starts);
        }
        Columns { lines }
    }

    /// The UTF-16 column of a byte column; a byte inside a character gives the character's column.
    fn unit_col(&self, line: u32, byte_col: u32) -> u32 {
        match self.lines.get(&line) {
            None => byte_col,
            Some(starts) => {
                let i = starts.partition_point(|&(b, _)| b <= byte_col);
                let (b, u) = starts[i.saturating_sub(1)];
                // Past the line's content (the line end): one unit per byte.
                if i == starts.len() { u + (byte_col - b) } else { u }
            }
        }
    }

    /// The byte column of a UTF-16 column; a column inside a character (between two surrogates) gives the
    /// character's start.
    fn byte_col(&self, line: u32, unit_col: u32) -> u32 {
        match self.lines.get(&line) {
            None => unit_col,
            Some(starts) => {
                let i = starts.partition_point(|&(_, u)| u <= unit_col);
                let (b, u) = starts[i.saturating_sub(1)];
                if i == starts.len() { b + (unit_col - u) } else { b }
            }
        }
    }

    /// The editor position of a byte offset.
    pub fn position(&self, file: &SourceFile, offset: u32) -> Position {
        let (line, col) = file.line_col(offset);
        Position::new(line - 1, self.unit_col(line - 1, col - 1))
    }

    /// The byte offset of an editor position, clamped to the line's content (and to the last line).
    pub fn offset(&self, file: &SourceFile, pos: Position) -> u32 {
        let line = pos.line.min(file.line_count() - 1);
        let (start, end) = file.line_content(line).expect("line in range");
        if pos.line > line {
            return end;
        }
        (start + self.byte_col(line, pos.character)).min(end)
    }
}

/// Length in bytes and in UTF-16 units of the UTF-8 character at the start of `bytes` (not empty). An invalid
/// sequence is one byte, one unit.
fn utf8_char(bytes: &[u8]) -> (usize, u32) {
    let (len, units) = match bytes[0] {
        0x00..=0x7F => return (1, 1),
        0xC2..=0xDF => (2, 1),
        0xE0..=0xEF => (3, 1),
        0xF0..=0xF4 => (4, 2),
        _ => return (1, 1),
    };
    let cont = bytes.len() >= len && bytes[1..len].iter().all(|&b| (0x80..=0xBF).contains(&b));
    if cont { (len, units) } else { (1, 1) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(Encoding::DEFAULT.name(), "cp437");
        assert_eq!(Encoding::from_name("CP437"), Encoding::DEFAULT);
        assert_eq!(Encoding::from_name("windows1252").name(), "windows1252");
        assert_eq!(Encoding::from_name("utf8bom"), Encoding::Utf8);
        assert_eq!(Encoding::from_name("shiftjis"), Encoding::Utf8);
    }

    #[test]
    fn cp437_round_trip() {
        // Every byte, as iconv-lite reads a CP437 file holding them (tools/encodings/gen_tables.js).
        let text = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/cp437_all_bytes_utf8.bin"
        ))
        .unwrap();
        let text: String = utf8_text(&text);
        let all: Vec<u8> = (0..=255).collect();
        assert_eq!(Encoding::DEFAULT.encode(&text), all);
        assert_eq!(text.chars().count(), 256);
        assert_eq!(text.encode_utf16().count(), 256);
    }

    #[test]
    fn unencodable_as_iconv_writes_it() {
        let expected = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/cp437_unencodable.bin"
        ))
        .unwrap();
        assert_eq!(Encoding::DEFAULT.encode("A\u{e9}\u{20ac}\u{1F600}B"), expected);
        assert_eq!(expected, b"A\x82???B");
    }

    #[test]
    fn every_table_is_one_to_one_where_defined() {
        for (name, table) in TABLES {
            let e = Encoding::from_name(name);
            for (b, &u) in table.iter().enumerate() {
                if u == 0xFFFD {
                    continue;
                }
                let s = String::from_utf16(&[u]).unwrap();
                assert_eq!(e.encode(&s), [u8::try_from(b).unwrap()], "{name} byte {b}");
            }
        }
    }

    #[test]
    fn utf8_columns() {
        // `é` is two bytes, one unit; the emoji four bytes, two units.
        let text = "PRINT \"\u{e9}\" +\r\nx = \"\u{1F600}\": y\nplain";
        let file = SourceFile::new("t.bas", Encoding::Utf8.encode(text));
        let c = Encoding::Utf8.columns(&file);
        // The `+` after `"é" `: byte 11, unit 10.
        assert_eq!(c.position(&file, 11), Position::new(0, 10));
        assert_eq!(c.offset(&file, Position::new(0, 10)), 11);
        // Inside `é` maps to its start.
        assert_eq!(c.position(&file, 8), Position::new(0, 7));
        // The line end (CR) and past it.
        assert_eq!(c.position(&file, 12), Position::new(0, 11));
        assert_eq!(c.offset(&file, Position::new(0, 40)), 12);
        // Line 1: `: y` after the emoji; `:` is byte 10, unit 8.
        let l1 = 14;
        assert_eq!(c.position(&file, l1 + 10), Position::new(1, 8));
        assert_eq!(c.offset(&file, Position::new(1, 8)), l1 + 10);
        // Between the surrogates: the emoji's start.
        assert_eq!(c.offset(&file, Position::new(1, 6)), l1 + 5);
        // An ASCII line is the identity; a line past the end clamps to the end of the file.
        assert_eq!(c.position(&file, 30), Position::new(2, 2));
        assert_eq!(c.offset(&file, Position::new(9, 0)), to_u32(file.bytes.len()));
    }

    #[test]
    fn single_byte_columns_are_bytes() {
        let file = SourceFile::new(
            "t.bas",
            Encoding::DEFAULT.encode("PRINT \"\u{2551}\u{2550}\u{2557}\" +"),
        );
        assert_eq!(&file.bytes[7..10], b"\xBA\xCD\xBB");
        let c = Encoding::DEFAULT.columns(&file);
        assert_eq!(c.position(&file, 12), Position::new(0, 12));
        assert_eq!(c.offset(&file, Position::new(0, 12)), 12);
    }

    #[test]
    fn invalid_utf8_is_one_unit_per_byte() {
        let file = SourceFile::new("t.bas", b"a\xFF\xC3b".to_vec());
        let c = Encoding::Utf8.columns(&file);
        assert_eq!(c.position(&file, 3), Position::new(0, 3));
    }

    /// Test data that is UTF-8 by construction.
    fn utf8_text(bytes: &[u8]) -> String {
        let mut s = String::new();
        let mut i = 0;
        while i < bytes.len() {
            let (len, _) = utf8_char(&bytes[i..]);
            let c = match len {
                1 => u32::from(bytes[i]),
                2 => (u32::from(bytes[i] & 0x1F) << 6) | u32::from(bytes[i + 1] & 0x3F),
                3 => {
                    (u32::from(bytes[i] & 0x0F) << 12)
                        | (u32::from(bytes[i + 1] & 0x3F) << 6)
                        | u32::from(bytes[i + 2] & 0x3F)
                }
                _ => unreachable!("the fixture is in the Basic Multilingual Plane"),
            };
            s.push(char::from_u32(c).unwrap());
            i += len;
        }
        s
    }
}
