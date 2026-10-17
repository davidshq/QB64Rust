//! The old compiler's wording of an error (`m3-old-wording-notes`): [`old_wording`] finds, for a message of this
//! compiler, the message `qb64pe.exe` gives for the same error, in the table `old_wording.tsv` that
//! `tools\diagnostics\old_wording.py` writes from recorded output. A message the table does not hold has none.
//!
//! A message is looked up by its key, made as the script makes it: a backtick span whose words are all reserved
//! words or built-in names (`old_wording_words.txt`), metacommands, strings or signs stays as written; any other
//! span (a name as the program writes it) becomes `` `{}` ``, and a number outside the spans `{#}`. The table's
//! wording names them `{n}` (the n-th, in order), `{n:U}` in capitals, `{n:P}` without its `()`, `{n:S}` also
//! without its type suffix.

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

/// Whether a byte joins a digit run to a word (`v16`); a dot does too (`1.5`), on either side.
fn joins(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.'
}

/// The words a span may hold and stay literal.
fn words() -> &'static HashSet<&'static str> {
    static WORDS: OnceLock<HashSet<&'static str>> = OnceLock::new();
    WORDS.get_or_init(|| {
        include_str!("old_wording_words.txt")
            .lines()
            .filter(|l| !l.starts_with('#') && !l.is_empty())
            .collect()
    })
}

/// The table: key to wording.
fn table() -> &'static HashMap<&'static str, &'static str> {
    static TABLE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    TABLE.get_or_init(|| {
        include_str!("old_wording.tsv")
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| l.split_once('\t'))
            .collect()
    })
}

/// Whether a span stays in a key as written.
fn literal(span: &str) -> bool {
    span.split_whitespace().all(|word| {
        let word = word.strip_suffix("()").unwrap_or(word);
        word.starts_with(['$', '"'])
            || !word.bytes().any(|b| b.is_ascii_alphabetic())
            || (!word.bytes().any(|b| b.is_ascii_lowercase()) && words().contains(word))
    })
}

/// The key of a message and the texts of its placeholders, in order.
fn key(message: &str) -> (String, Vec<&str>) {
    let (mut out, mut values) = (String::new(), Vec::new());
    for (i, piece) in message.split('`').enumerate() {
        if i % 2 == 1 {
            if literal(piece) {
                out.push('`');
                out.push_str(piece);
                out.push('`');
            } else {
                values.push(piece);
                out.push_str("`{}`");
            }
            continue;
        }
        let b = piece.as_bytes();
        let (mut start, mut k) = (0, 0);
        while k < b.len() {
            if b[k].is_ascii_digit() && (k == 0 || !joins(b[k - 1])) {
                let end = (k..b.len()).find(|&j| !b[j].is_ascii_digit()).unwrap_or(b.len());
                if end == b.len() || !joins(b[end]) {
                    out.push_str(&piece[start..k]);
                    out.push_str("{#}");
                    values.push(&piece[k..end]);
                    start = end;
                }
                k = end;
            } else {
                k += 1;
            }
        }
        out.push_str(&piece[start..]);
    }
    (out, values)
}

/// The old compiler's wording of the error this compiler reports as `message`, with the names and numbers of
/// this message filled in; `None` where the table holds nothing for it.
pub fn old_wording(message: &str) -> Option<String> {
    let (key, values) = key(message);
    fill(table().get(key.as_str())?, &values)
}

/// A wording of the table with `{n}` or `{n:code}` replaced by the n-th value, transformed: `U` capitals, `P`
/// without its `()`, `S` also without its type suffix.
/// `{n:=|S}` lists the forms the recorded pairs allow; where they give this value different texts (a name with a
/// suffix where every recording had none), the old wording is not known and there is no note.
fn fill(template: &str, values: &[&str]) -> Option<String> {
    let mut out = String::new();
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let close = after.find('}')?;
        let (n, codes) = after[..close].split_once(':').unwrap_or((&after[..close], "="));
        let v = values.get(n.parse::<usize>().ok()?.checked_sub(1)?)?;
        let mut texts = codes.split('|').map(|code| transform(v, code));
        let first = texts.next()?;
        if texts.any(|t| t != first) {
            return None;
        }
        out.push_str(&first);
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    Some(out)
}

/// A value in one form: `=` as written, `U` in capitals, `P` without its `()`, `S` also without its type suffix.
fn transform(v: &str, code: &str) -> String {
    let mut v = v;
    if code.contains('P') || code.contains('S') {
        v = v.strip_suffix("()").unwrap_or(v);
    }
    if code.contains('S') {
        v = v.trim_end_matches(['%', '&', '!', '#', '$', '~', '`']);
    }
    if code.contains('U') {
        v.to_ascii_uppercase()
    } else {
        v.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_message_with_a_keyword_span() {
        assert_eq!(
            old_wording("`_MEM` with empty parentheses").as_deref(),
            Some("Expected (...)")
        );
    }

    #[test]
    fn names_and_numbers_are_filled_in() {
        assert_eq!(
            old_wording("variable `count` (LONG) is not declared (`OPTION _EXPLICIT`)").as_deref(),
            Some("Variable 'count' (LONG) not defined")
        );
        assert_eq!(
            old_wording("`ASC` takes 1 or 2 arguments").as_deref(),
            Some("Incorrect number of arguments - Reference: ASC(text$[, position%])")
        );
    }

    #[test]
    fn a_message_the_table_does_not_hold() {
        assert_eq!(old_wording("no such message here"), None);
    }

    #[test]
    fn keys() {
        assert_eq!(
            key("`ab` and `LBOUND` in v16 line 4 of 1.5"),
            ("`{}` and `LBOUND` in v16 line {#} of 1.5".to_string(), vec!["ab", "4"])
        );
        assert_eq!(
            key("`A`, `TYPE T`, `$CONSOLE:ONLY`, `\"\"`").0,
            "`{}`, `{}`, `$CONSOLE:ONLY`, `\"\"`"
        );
    }

    #[test]
    fn fill_transforms() {
        assert_eq!(
            fill("{1:U} and {2:SU} {3}", &["ab", "c$()", "7"]).as_deref(),
            Some("AB and C 7")
        );
        // Forms that agree for this value give it; forms that do not give no wording.
        assert_eq!(fill("'{1:=|S}'", &["count"]).as_deref(), Some("'count'"));
        assert_eq!(fill("'{1:=|S}'", &["count&"]), None);
    }

    #[test]
    fn a_suffix_no_recording_had_gives_no_note() {
        // Measured with `qb64pe.exe` (`count&` gives "Variable 'count' (LONG) not defined"); the recordings have
        // names without a suffix only, so the form is not known.
        let message = "variable `count&` (LONG) is not declared (`OPTION _EXPLICIT`)";
        assert_ne!(
            old_wording(message).as_deref(),
            Some("Variable 'count&' (LONG) not defined")
        );
    }
}
