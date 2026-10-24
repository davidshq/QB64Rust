//! The descriptor of a `$USELIBRARY` library (design D7 of `m4-builtins-remaining`): the name a line gives, the
//! descriptor's three keys read by the rules of QB64pe's `ReadSetting$` (`source\utilities\ini-manager\ini.bm`,
//! read in full and followed here, nothing copied), and the paths of the files under the compiler root.
//!
//! The old compiler (`qb64pe.bas` 1915–1976) takes any line that starts with `$USELIBRARY` (eleven characters, so
//! `$USELIBRARYX:'a/b'` and `$USELIBRARY 'a/b'` count too), reads the name between the line's first two `'`,
//! trimmed, and reads `libraries/descriptors/<name>.ini` under its own folder: the keys `IncAtTop`,
//! `IncAfterMain` and `IncAtBottom` of the section `[LIBRARY INCLUDES]`, each a file under
//! `libraries/includes/<name>/`. A key that is missing, or whose value is empty, includes nothing; so does a
//! descriptor without the section or with no text (`ReadSetting$` gives "" for each, codes 14 and 17). Measured,
//! `verification\v58_u00`–`u26`.

use std::path::PathBuf;

/// The three files a descriptor names, each `None` when its key is missing or has no value.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Descriptor {
    pub top: Option<Vec<u8>>,
    pub after_main: Option<Vec<u8>>,
    pub bottom: Option<Vec<u8>>,
}

/// The section of a descriptor the old compiler reads.
pub const SECTION: &[u8] = b"LIBRARY INCLUDES";
/// Its keys, in the order the old compiler reads them (and names them in its error).
pub const KEYS: [&str; 3] = ["IncAtTop", "IncAfterMain", "IncAtBottom"];

impl Descriptor {
    /// Reads a descriptor's text.
    pub fn parse(text: &[u8]) -> Descriptor {
        let read = |key: &str| {
            let v = read_setting(text, SECTION, key.as_bytes());
            (!v.is_empty()).then_some(v)
        };
        Descriptor {
            top: read(KEYS[0]),
            after_main: read(KEYS[1]),
            bottom: read(KEYS[2]),
        }
    }

    /// The three keys in the old compiler's order ([`KEYS`]), each with its file if the descriptor names one.
    pub fn files(&self) -> [(&'static str, Option<&[u8]>); 3] {
        [
            (KEYS[0], self.top.as_deref()),
            (KEYS[1], self.after_main.as_deref()),
            (KEYS[2], self.bottom.as_deref()),
        ]
    }
}

/// The library name a `$USELIBRARY` line gives: the text between the line's first two `'`, trimmed of spaces (the
/// old compiler's `_TRIM$`); `None` when the line has no two quotes. An empty name is `Some("")`, the same error
/// to the old compiler.
pub fn library_name(line: &[u8]) -> Option<Vec<u8>> {
    let bra = line.iter().position(|&b| b == b'\'')?;
    let ket = bra + 1 + line[bra + 1..].iter().position(|&b| b == b'\'')?;
    Some(trim_spaces(&line[bra + 1..ket]).to_vec())
}

/// Whether a metacommand line (upper case, trimmed: `pp::meta_line`) is one the old compiler takes for
/// `$USELIBRARY`: its first eleven characters.
pub fn is_use_library(line: &[u8]) -> bool {
    line.starts_with(b"$USELIBRARY")
}

/// The descriptor's path under the compiler root, as the old compiler spells it (forward slashes, `.ini`).
pub fn descriptor_path(name: &[u8]) -> PathBuf {
    PathBuf::from(format!("libraries/descriptors/{}.ini", lossy(name)))
}

/// A library file's path under the compiler root.
pub fn file_path(name: &[u8], file: &[u8]) -> PathBuf {
    PathBuf::from(format!("libraries/includes/{}/{}", lossy(name), lossy(file)))
}

fn lossy(bytes: &[u8]) -> String {
    #[expect(clippy::disallowed_methods, reason = "a file name, not BASIC source")]
    String::from_utf8_lossy(bytes).into_owned()
}

fn trim_spaces(s: &[u8]) -> &[u8] {
    let start = s.iter().position(|&b| b != b' ').unwrap_or(s.len());
    let end = s.iter().rposition(|&b| b != b' ').map_or(start, |i| i + 1);
    &s[start..end.max(start)]
}

/// A key's value by `ReadSetting$`'s rules: the section found at a line start (spaces before it allowed, letter
/// case ignored) and running to the next `[` that is not inside quotes and has no `=` before it on its line; in it
/// the first `key` (letter case ignored) with only spaces before it on its line and only spaces between it and
/// an `=` on the same line; the value trimmed of spaces, a value that starts with `"` taken up to the next `"`,
/// else cut at the first `'`, or at the first `;` when it has none. Empty when anything is missing.
pub fn read_setting(text: &[u8], section: &[u8], key: &[u8]) -> Vec<u8> {
    let Some(data) = section_data(text, section) else {
        return Vec::new();
    };
    let lower: Vec<u8> = data.to_ascii_lowercase();
    let key_lower: Vec<u8> = key.to_ascii_lowercase();
    let mut from = 0;
    while let Some(off) = find(&lower[from..], &key_lower) {
        let at = from + off;
        from = at + 1;
        // Only spaces between the line start and the key.
        let line_start = data[..at].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
        if data[line_start..at].iter().any(|&b| b != b' ') {
            continue;
        }
        // Only spaces between the key and an `=`, on this line.
        let after = at + key.len();
        let line_end = data[after..]
            .iter()
            .position(|&b| b == b'\n')
            .map_or(data.len(), |i| after + i);
        let Some(eq) = data[after..line_end].iter().position(|&b| b == b'=').map(|i| after + i) else {
            continue;
        };
        if data[after..eq].iter().any(|&b| b != b' ') {
            continue;
        }
        let raw = &data[eq + 1..line_end];
        let raw = raw.strip_suffix(b"\r").unwrap_or(raw);
        let value = trim_spaces(raw);
        if value.is_empty() {
            return Vec::new();
        }
        if let Some(rest) = value.strip_prefix(b"\"") {
            let end = rest.iter().position(|&b| b == b'"').unwrap_or(rest.len());
            return rest[..end].to_vec();
        }
        // The compiler turns BASIC comments on at its start (`source\ide\config\cfg_global.bas`,
        // `IniSetAllowBasicComments _TRUE`): a `'` cuts the value, else a `;` (measured, `verification\v58_u23`).
        let end = value
            .iter()
            .position(|&b| b == b'\'')
            .or_else(|| value.iter().position(|&b| b == b';'))
            .unwrap_or(value.len());
        return trim_spaces(&value[..end]).to_vec();
    }
    Vec::new()
}

/// The text of the section `[section]` (`IniGetSection$`): from its header's line start to the next section
/// header. `None` when the file has no text or no such section.
fn section_data<'t>(text: &'t [u8], section: &[u8]) -> Option<&'t [u8]> {
    if text
        .iter()
        .all(|&b| b == b' ' || b == b'\r' || b == b'\n' || b == b'\t')
    {
        return None;
    }
    let header: Vec<u8> = [b"[", &section.to_ascii_lowercase()[..], b"]"].concat();
    let lower = text.to_ascii_lowercase();
    let mut from = 0;
    while let Some(off) = find(&lower[from..], &header) {
        let at = from + off;
        from = at + 1;
        let line_start = text[..at].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
        if text[line_start..at].iter().any(|&b| b != b' ') {
            continue;
        }
        // The end: the next `[` outside quotes that no `=` precedes on its line.
        let mut bracket = at + 1;
        while let Some(off) = text[bracket..].iter().position(|&b| b == b'[') {
            bracket += off;
            let in_quote = text[..bracket].iter().filter(|&&b| b == b'"').count() % 2 == 1;
            let its_line = text[..bracket].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
            let in_value = text[its_line.max(line_start)..bracket].contains(&b'=');
            if in_quote || in_value {
                bracket += 1;
                continue;
            }
            return Some(&text[line_start..its_line]);
        }
        return Some(&text[line_start..]);
    }
    None
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setting(text: &str, key: &str) -> String {
        #[expect(clippy::disallowed_methods, reason = "a test's comparison text, not BASIC source")]
        String::from_utf8(read_setting(text.as_bytes(), SECTION, key.as_bytes())).unwrap()
    }

    #[test]
    fn the_name_between_the_first_two_quotes() {
        assert_eq!(library_name(b"$USELIBRARY:'test/a'"), Some(b"test/a".to_vec()));
        assert_eq!(
            library_name(b"$USELIBRARY:'  test/a  ' ' comment"),
            Some(b"test/a".to_vec())
        );
        assert_eq!(library_name(b"$USELIBRARY 'x'"), Some(b"x".to_vec()));
        assert_eq!(library_name(b"$USELIBRARY:''"), Some(Vec::new()));
        assert_eq!(library_name(b"$USELIBRARY:test/a"), None);
        assert_eq!(library_name(b"$USELIBRARY:\"test/a\""), None);
        assert_eq!(library_name(b"$USELIBRARY:'test/a"), None);
        assert!(is_use_library(b"$USELIBRARYX:'A'"));
        assert!(!is_use_library(b"$USELIBRAR"));
    }

    #[test]
    fn the_three_keys() {
        let d = Descriptor::parse(b"[LIBRARY INCLUDES]\nIncAtTop=a.bi\nIncAfterMain=b.bm\nIncAtBottom=c.bm\n");
        assert_eq!(
            d.files(),
            [
                ("IncAtTop", Some(&b"a.bi"[..])),
                ("IncAfterMain", Some(&b"b.bm"[..])),
                ("IncAtBottom", Some(&b"c.bm"[..]))
            ]
        );
        let d = Descriptor::parse(b"[LIBRARY INCLUDES]\nIncAtBottom=c.bm\n");
        assert_eq!(
            d,
            Descriptor {
                bottom: Some(b"c.bm".to_vec()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn sections_keys_quotes_and_comments() {
        let text = "; a comment line\n[OTHER]\nIncAtTop=wrong.bi\n[library includes]\n  incattop = \"q top.bi\" ; after\nIncAtBottom=q_bottom.bm ' a BASIC comment ; too\nxIncAfterMain=no.bm\n[LAST]\nIncAfterMain=later.bm\n";
        assert_eq!(setting(text, "IncAtTop"), "q top.bi");
        assert_eq!(setting(text, "IncAtBottom"), "q_bottom.bm");
        assert_eq!(setting(text, "IncAfterMain"), "");
        assert_eq!(setting(text, "incatbottom"), "q_bottom.bm");
        // The apostrophe is looked for first, the semicolon only when there is none.
        assert_eq!(setting("[LIBRARY INCLUDES]\nIncAtTop=a.bi;x'y\n", "IncAtTop"), "a.bi;x");
        assert_eq!(setting("[LIBRARY INCLUDES]\nIncAtTop=a.bi ; x\n", "IncAtTop"), "a.bi");
    }

    #[test]
    fn missing_things_give_nothing() {
        assert_eq!(setting("", "IncAtTop"), "");
        assert_eq!(setting("   \n", "IncAtTop"), "");
        assert_eq!(setting("IncAtTop=a.bi\n", "IncAtTop"), "");
        assert_eq!(
            setting("[LIBRARY INCLUDES]\nIncAtTop=\nIncAtBottom=b\n", "IncAtTop"),
            ""
        );
        assert_eq!(setting("[LIBRARY INCLUDES]\nIncAtTop a.bi\n", "IncAtTop"), "");
        assert_eq!(setting("[LIBRARY INCLUDES]\n\tIncAtTop=a.bi\n", "IncAtTop"), "");
        assert_eq!(setting("[LIBRARY INCLUDES]\nIncAtTop=a.bi", "IncAtTop"), "a.bi");
        assert_eq!(
            setting(
                "[LIBRARY INCLUDES]\r\nIncAtTop=a.bi\r\nIncAtBottom=b.bm\r\n",
                "IncAtTop"
            ),
            "a.bi"
        );
        assert_eq!(setting("[LIBRARY INCLUDES]\r\nIncAtTop=a.bi\r\n", "IncAtTop"), "a.bi");
    }

    #[test]
    fn a_bracket_in_a_value_does_not_end_the_section() {
        let text =
            "[LIBRARY INCLUDES]\nIncAtTop=a[1].bi\nIncAtBottom=\"b[\".bm\nIncAfterMain=c.bm\n[X]\nIncAfterMain=d\n";
        assert_eq!(setting(text, "IncAtTop"), "a[1].bi");
        assert_eq!(setting(text, "IncAtBottom"), "b[");
        assert_eq!(setting(text, "IncAfterMain"), "c.bm");
    }

    /// Whether a `[` is in quotes goes by every `"` from the start of the file, as `IniGetSection$` counts them
    /// (`source\utilities\ini-manager\ini.bm`): an odd one in an earlier comment makes the next header "in
    /// quotes", so the section runs on into it. Kept QB64pe's way (`SOMEDAY.md`, "QB64pe behaviours to review").
    #[test]
    fn a_quote_before_the_section_hides_the_next_header() {
        let text = "; it's \"quoted\n[LIBRARY INCLUDES]\nIncAtTop=a.bi\n[OTHER]\nIncAfterMain=x.bm\n";
        assert_eq!(setting(text, "IncAtTop"), "a.bi");
        assert_eq!(setting(text, "IncAfterMain"), "x.bm");
        let even = "; \"quoted\"\n[LIBRARY INCLUDES]\nIncAtTop=a.bi\n[OTHER]\nIncAfterMain=x.bm\n";
        assert_eq!(setting(even, "IncAfterMain"), "");
    }

    #[test]
    fn paths_under_the_root() {
        assert_eq!(
            descriptor_path(b"test/a"),
            PathBuf::from("libraries/descriptors/test/a.ini")
        );
        assert_eq!(
            file_path(b"test/a", b"a_top.bi"),
            PathBuf::from("libraries/includes/test/a/a_top.bi")
        );
    }
}
