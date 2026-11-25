//! Code page 437 and UTF-8 (OpenSpec change `utf8-source`, design D1 and D4).
//!
//! A file declared UTF-8 keeps its bytes in the source map. The checker turns the text of each string literal and
//! `DATA` item into code page 437 bytes with [`text_of`]. `qb64rust convert` rewrites whole files with
//! [`char_of`] and [`byte_of`].
//!
//! The table of bytes 128 to 255 is iconv-lite's, which VS Code uses for `cp437`, so that a file saved by VS Code
//! as `cp437` and the same text saved as UTF-8 give the same bytes. `crates\lsp` checks that its generated table
//! agrees. Bytes 1 to 31 and 127 are control characters to iconv-lite, but the screen draws a symbol for each
//! (`☺`, `♥`, `⌂`). [`byte_of`] takes those symbols too. They are QB64pe's own table
//! (`codepage437_to_unicode16` in `internal\c\libqb.cpp`).

/// The character of each byte 128 to 255 (iconv-lite's `cp437`, the same as QB64pe's table).
const HIGH: [char; 128] = [
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', //
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', //
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', //
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', //
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', //
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', //
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', //
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{A0}',
];

/// The symbols the screen draws for the bytes 1 to 31 and 127, with their bytes.
pub const SYMBOLS: [(u8, char); 32] = [
    (1, '☺'),
    (2, '☻'),
    (3, '♥'),
    (4, '♦'),
    (5, '♣'),
    (6, '♠'),
    (7, '•'),
    (8, '◘'),
    (9, '○'),
    (10, '◙'),
    (11, '♂'),
    (12, '♀'),
    (13, '♪'),
    (14, '♫'),
    (15, '☼'),
    (16, '►'),
    (17, '◄'),
    (18, '↕'),
    (19, '‼'),
    (20, '¶'),
    (21, '§'),
    (22, '▬'),
    (23, '↨'),
    (24, '↑'),
    (25, '↓'),
    (26, '→'),
    (27, '←'),
    (28, '∟'),
    (29, '↔'),
    (30, '▲'),
    (31, '▼'),
    (127, '⌂'),
];

/// The character a byte stands for when a file is converted to UTF-8: a byte below 128 is itself, so a control
/// character stays one and a round trip through [`byte_of`] is exact.
pub fn char_of(b: u8) -> char {
    if b < 128 {
        char::from(b)
    } else {
        HIGH[usize::from(b - 128)]
    }
}

/// The code page 437 byte of a character: itself below 128, then the table of bytes 128 to 255, then the 32
/// symbols of the bytes below 32 and 127. `None` for a character code page 437 does not hold.
pub fn byte_of(c: char) -> Option<u8> {
    if c.is_ascii() {
        return u8::try_from(u32::from(c)).ok();
    }
    if let Some(i) = HIGH.iter().position(|&h| h == c) {
        return Some(128 + u8::try_from(i).expect("128 entries"));
    }
    SYMBOLS.iter().find(|&&(_, s)| s == c).map(|&(b, _)| b)
}

/// Why a text has no code page 437 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadChar {
    /// The character at this byte offset of the text has no code page 437 byte.
    Outside { at: usize, ch: char },
    /// The bytes at this offset are not UTF-8.
    NotUtf8 { at: usize, byte: u8 },
}

/// The bytes the program holds for the text of a string literal or a `DATA` item. In a code page 437 file
/// (`utf8` false) they are the text's own bytes. In a UTF-8 file each character becomes its code page 437 byte.
pub fn text_of(text: &[u8], utf8: bool) -> Result<Vec<u8>, BadChar> {
    if !utf8 || text.is_ascii() {
        return Ok(text.to_vec());
    }
    let mut out = Vec::with_capacity(text.len());
    let mut at = 0;
    while at < text.len() {
        let (ch, len) = next_char(&text[at..]).ok_or(BadChar::NotUtf8 { at, byte: text[at] })?;
        out.push(byte_of(ch).ok_or(BadChar::Outside { at, ch })?);
        at += len;
    }
    Ok(out)
}

/// The offset of the first byte of `bytes` that does not start a valid UTF-8 character, or `None` when all of
/// `bytes` is UTF-8.
pub fn first_bad_utf8(bytes: &[u8]) -> Option<usize> {
    #[expect(clippy::disallowed_methods, reason = "whether the bytes are UTF-8 is the question")]
    std::str::from_utf8(bytes).err().map(|e| e.valid_up_to())
}

/// The UTF-8 character at the start of `bytes` and its length in bytes, or `None` when they do not start one.
pub fn next_char(bytes: &[u8]) -> Option<(char, usize)> {
    let len = match *bytes.first()? {
        0x00..=0x7F => 1,
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => return None,
    };
    let head = bytes.get(..len)?;
    #[expect(clippy::disallowed_methods, reason = "one character is checked as UTF-8")]
    let s = std::str::from_utf8(head).ok()?;
    s.chars().next().map(|c| (c, len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_byte_there_and_back() {
        for b in 0..=255u8 {
            assert_eq!(byte_of(char_of(b)), Some(b), "byte {b}");
            let text = char_of(b).to_string();
            assert_eq!(text_of(text.as_bytes(), true), Ok(vec![b]), "byte {b}");
        }
    }

    #[test]
    fn the_symbols_of_the_low_bytes() {
        assert_eq!(byte_of('♥'), Some(3));
        assert_eq!(byte_of('⌂'), Some(127));
        assert_eq!(text_of("♥".as_bytes(), true), Ok(vec![3]));
        // A tab written as itself stays a tab.
        assert_eq!(text_of(b"a\tb", true), Ok(b"a\tb".to_vec()));
        for (b, c) in SYMBOLS {
            assert!(!HIGH.contains(&c), "{c} is also a byte above 127");
            assert_eq!(byte_of(c), Some(b));
        }
    }

    #[test]
    fn box_drawing_and_accents() {
        assert_eq!(text_of("─".as_bytes(), true), Ok(vec![0xC4]));
        assert_eq!(
            text_of("╔══╗ é √".as_bytes(), true),
            Ok(vec![0xC9, 0xCD, 0xCD, 0xBB, 32, 0x82, 32, 0xFB])
        );
    }

    #[test]
    fn a_character_outside_the_table() {
        assert_eq!(
            text_of("a日本".as_bytes(), true),
            Err(BadChar::Outside { at: 1, ch: '日' })
        );
        assert_eq!(byte_of('€'), None);
    }

    #[test]
    fn bytes_that_are_not_utf8() {
        assert_eq!(text_of(b"ab\xC4", true), Err(BadChar::NotUtf8 { at: 2, byte: 0xC4 }));
        assert_eq!(first_bad_utf8(b"ab\xC4cd"), Some(2));
        assert_eq!(first_bad_utf8("é".as_bytes()), None);
        assert_eq!(next_char(b"\xED\xA0\x80"), None, "a surrogate is not UTF-8");
    }

    #[test]
    fn a_code_page_437_file_keeps_its_bytes() {
        assert_eq!(text_of(b"\xC4\xE2\x94\x80", false), Ok(b"\xC4\xE2\x94\x80".to_vec()));
    }
}
