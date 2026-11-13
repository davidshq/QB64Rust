//! The layout of a parsed and checked program: `qb64rust -y` (spec `compiler/format`), which writes a file as
//! `qb64pe -y` lays it out.
//!
//! QB64pe has no separate formatter: each statement handler of its main pass appends the statement's text to
//! `layout$`, and `apply_layout_indent$` turns that into the line it writes
//! (`openspec\changes\archive\2026-10-25-m5b-formatter\design.md`, Context). This crate builds the same
//! intermediate form from the tree, statement by statement (design D2): [`walk`] the lines and their indent,
//! `stmt` the statements, `expr` the expressions, [`fmt`] the spelling of keywords and names; and [`apply`]
//! ports the last step.

pub mod apply;
mod expr;
mod fmt;
mod lay;
mod stmt;
mod walk;
/// Public for the fuzz dictionary (`driver`'s `fuzz_regressions` test).
pub mod words;

use qb64rust_base::layout::Settings;
use qb64rust_base::{FileId, SourceMap};
use qb64rust_sema::Program;
use qb64rust_syntax::ParsedProgram;

/// The lines of `bytes` as QB64pe reads them (`lineinput3$`): a line ends at CR, LF, CRLF or LFCR; a line ending at
/// the end of the file starts no line after it. Each line as its byte range, without the line end.
pub fn line_ranges(bytes: &[u8]) -> Vec<(u32, u32)> {
    let mut lines = Vec::new();
    let mut i = 0;
    let to = |n: usize| u32::try_from(n).expect("a source file is at most MAX_SOURCE_LEN bytes");
    while i < bytes.len() {
        let Some(end) = bytes[i..].iter().position(|&c| c == b'\r' || c == b'\n').map(|p| i + p) else {
            lines.push((to(i), to(bytes.len())));
            break;
        };
        lines.push((to(i), to(end)));
        let pair = if bytes[end] == b'\r' { b'\n' } else { b'\r' };
        i = end + 1;
        if bytes.get(i) == Some(&pair) {
            i += 1;
        }
    }
    lines
}

/// The lines of `bytes` as QB64pe reads them ([`line_ranges`]).
pub fn source_lines(bytes: &[u8]) -> Vec<&[u8]> {
    line_ranges(bytes)
        .into_iter()
        .map(|(a, b)| &bytes[a as usize..b as usize])
        .collect()
}

/// Lays out the file `file` of a program that parsed without errors, with `settings`. The result holds every line
/// of the file, each followed by CRLF, as `qb64pe -y` writes its buffer (design D4).
///
/// `complete`: the program checked without errors and its main file is a `.bas` file. When it is not, a name takes
/// the spelling of its definition only when a statement declares it (spec `compiler/format`, "Names in an
/// incomplete program"; design D3 of `include-file-program`).
///
/// In an incomplete program, a line whose layout would read otherwise than the line (other than by blanks and
/// letter case outside strings) is laid out as with `AutoLayout` off: indented, its text as typed but for letter
/// case. The layout of a statement the check rejected, or one beside it, may misread it (`KILL ("a", "b")` laid out
/// `KILL ("a"), "b"`; task 2.6 of `include-file-program`).
pub fn format_file(
    map: &SourceMap,
    parsed: &ParsedProgram,
    program: &Program,
    file: FileId,
    settings: &Settings,
    complete: bool,
) -> Vec<u8> {
    let bytes = &map.file(file).bytes;
    let ranges = line_ranges(bytes);
    let f = fmt::Fmt {
        map,
        program,
        file,
        settings,
        complete,
    };
    let plans = match parsed.trees.iter().find(|t| t.file == file) {
        Some(tree) => f.plan(tree.root(), &ranges),
        None => Vec::new(),
    };
    let mut out = Vec::with_capacity(bytes.len() + bytes.len() / 8);
    for (i, &(a, b)) in ranges.iter().enumerate() {
        let line = &bytes[a as usize..b as usize];
        match plans.get(i) {
            Some(walk::Plan::Layout(layout, apply)) => {
                let mut text = apply::apply_layout_indent(layout, line, *apply);
                let misreads = |t: &Option<Vec<u8>>| t.as_ref().is_some_and(|t| reading(t) != reading(line));
                if !complete && misreads(&text) {
                    let off = apply::Apply {
                        auto_layout: false,
                        ..*apply
                    };
                    text = apply::apply_layout_indent(layout, line, off);
                    if misreads(&text) {
                        text = None;
                    }
                }
                match text {
                    Some(text) => out.extend_from_slice(&text),
                    None => out.extend_from_slice(line),
                }
            }
            Some(walk::Plan::AsRead) | None => out.extend_from_slice(line),
        }
        out.extend_from_slice(b"\r\n");
    }
    out
}

/// A line as the lexer reads it, for the check of an incomplete program's layout: letters outside strings in
/// capitals, and blanks outside strings left out except one between two characters of a name or number (`k AS`
/// is not `kAS`, `a . b` is not `a.b`); a string's text as it is.
fn reading(text: &[u8]) -> Vec<u8> {
    let word = |c: u8| c.is_ascii_alphanumeric() || b"_.$%&!#~`".contains(&c);
    let mut key = Vec::with_capacity(text.len());
    let (mut in_string, mut blank) = (false, false);
    for &c in text {
        if c == b'"' {
            in_string = !in_string;
        }
        if !in_string && (c == b' ' || c == b'\t') {
            blank = true;
            continue;
        }
        if blank && !in_string && key.last().is_some_and(|&p| word(p)) && word(c) {
            key.push(b' ');
        }
        blank = false;
        key.push(if in_string { c } else { c.to_ascii_uppercase() });
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_keeps_the_blanks_that_part_names() {
        assert_eq!(reading(b"  call   missing"), reading(b"Call missing"));
        assert_eq!(reading(b"x=1+ 2"), reading(b"x = 1 + 2"));
        assert_ne!(reading(b"k AS LONG"), reading(b"kAS LONG"));
        assert_ne!(reading(b"a . b"), reading(b"a.b"));
        assert_ne!(reading(b"PRINT \"a  b\""), reading(b"PRINT \"a b\""));
    }

    #[test]
    fn lines_end_as_qb64pe_reads_them() {
        let lines = source_lines(b"a\r\nb\nc\rd\n\re\r\n\r\nf");
        assert_eq!(lines, [&b"a"[..], b"b", b"c", b"d", b"e", b"", b"f"]);
        assert_eq!(source_lines(b"a\r\n"), [&b"a"[..]]);
        assert!(source_lines(b"").is_empty());
    }
}
