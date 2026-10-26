//! A line's layout in QB64pe's own form (design D2): elements joined by [`SP`] ("one space") or [`SP2`] ("no
//! space"), as its handlers build `l$` and `layout$`; and the letter case of keywords (`SCase$`, `SCase2$`).

use crate::apply::{SP, SP2};
use crate::words::WORDS;
use qb64rust_base::layout::KeywordStyle;

/// A layout being built: the bytes of `l$`, separators included.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Lay(pub Vec<u8>);

impl Lay {
    pub fn new() -> Lay {
        Lay(Vec::new())
    }

    pub fn of(bytes: &[u8]) -> Lay {
        Lay(bytes.to_vec())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Appends text as it is.
    pub fn text(&mut self, bytes: &[u8]) -> &mut Lay {
        self.0.extend_from_slice(bytes);
        self
    }

    pub fn sp(&mut self) -> &mut Lay {
        self.0.push(SP);
        self
    }

    pub fn sp2(&mut self) -> &mut Lay {
        self.0.push(SP2);
        self
    }

    /// `l$ = l$ + sp + x$`, or `x$` alone when `l$` is empty (`IF LEN(layout$) THEN layout$ = layout$ + sp + l$ ELSE
    /// layout$ = l$`).
    pub fn then(&mut self, other: &Lay) -> &mut Lay {
        if !self.0.is_empty() && !other.0.is_empty() {
            self.0.push(SP);
        }
        self.0.extend_from_slice(&other.0);
        self
    }

    pub fn append(&mut self, other: &Lay) -> &mut Lay {
        self.0.extend_from_slice(&other.0);
        self
    }

    /// The last byte, if any.
    pub fn last(&self) -> Option<u8> {
        self.0.last().copied()
    }

    /// Replaces the last byte (a separator, as QB64pe's `MID$(l$, LEN(l$), 1) = sp2`).
    pub fn set_last(&mut self, b: u8) {
        if let Some(l) = self.0.last_mut() {
            *l = b;
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.0.pop()
    }
}

/// The spelling of a keyword word (one word of an `SCase$` literal) by its upper-case form; `None` for a word QB64pe's
/// sources do not spell.
pub fn word(upper: &[u8]) -> Option<&'static str> {
    WORDS
        .binary_search_by(|(k, _)| k.as_bytes().cmp(upper))
        .ok()
        .map(|i| WORDS[i].1)
}

/// `SCase$` of a spelling: as it is in CamelCase, else in capitals or lower case (ASCII letters only, as `UCASE$`
/// and `LCASE$`).
pub fn scase(spelling: &[u8], style: KeywordStyle) -> Vec<u8> {
    match style {
        KeywordStyle::Camel => spelling.to_vec(),
        KeywordStyle::Upper => spelling.to_ascii_uppercase(),
        KeywordStyle::Lower => spelling.to_ascii_lowercase(),
    }
}

/// `SCase2$`: in CamelCase each word's first letter in capitals and the rest in lower case, a word starting after
/// the start, `_` or `sp`, with QB64pe's four exceptions; else capitals or lower case.
pub fn scase2(text: &[u8], style: KeywordStyle) -> Vec<u8> {
    match style {
        KeywordStyle::Upper => text.to_ascii_uppercase(),
        KeywordStyle::Lower => text.to_ascii_lowercase(),
        KeywordStyle::Camel => {
            let upper = text.to_ascii_uppercase();
            for (u, s) in [
                (&b"_ANDALSO"[..], &b"_AndAlso"[..]),
                (b"_ORELSE", b"_OrElse"),
                (b"_DYNAMIC", b"_Dynamic"),
                (b"_STATIC", b"_Static"),
            ] {
                if upper == u {
                    return s.to_vec();
                }
            }
            let mut out = Vec::with_capacity(text.len());
            let mut new_word = true;
            for &c in text {
                if new_word {
                    if c == b'_' || c == SP {
                        out.push(c);
                    } else {
                        out.push(c.to_ascii_uppercase());
                        new_word = false;
                    }
                } else if c == SP {
                    out.push(c);
                    new_word = true;
                } else {
                    out.push(c.to_ascii_lowercase());
                }
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_come_from_the_table() {
        assert_eq!(word(b"GOSUB"), Some("GoSub"));
        assert_eq!(word(b"$INCLUDE"), Some("$Include"));
        assert_eq!(word(b"NOPE"), None);
    }

    #[test]
    fn scase_styles() {
        // `SCase$` (`qb64pe.bas` 28733).
        assert_eq!(scase(b"End If", KeywordStyle::Camel), b"End If");
        assert_eq!(scase(b"End If", KeywordStyle::Upper), b"END IF");
        assert_eq!(scase(b"End If", KeywordStyle::Lower), b"end if");
    }

    #[test]
    fn scase2_camel_rule_and_exceptions() {
        // `SCase2$` (`qb64pe.bas` 28741).
        let camel = |t: &[u8]| scase2(t, KeywordStyle::Camel);
        assert_eq!(camel(b"LONG"), b"Long");
        assert_eq!(camel(b"_UNSIGNED\r_INTEGER64"), b"_Unsigned\r_Integer64");
        assert_eq!(camel(b"MOD"), b"Mod");
        assert_eq!(camel(b"_ANDALSO"), b"_AndAlso");
        assert_eq!(camel(b"_dynamic"), b"_Dynamic");
        assert_eq!(scase2(b"Long", KeywordStyle::Upper), b"LONG");
    }
}
