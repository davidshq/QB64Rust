//! The text of a `DATA` statement, scanned by the old compiler's rule (`qb64pe.bas` 25210–25310, `lineformat`;
//! measured M2): items are separated by `,`; a `"` opens a quoted item only as the item's first non-blank
//! character, and the next `"` closes it; inside quotes `,` and `:` are data; outside, `:` ends the statement. An
//! unclosed quote runs to the line end (the old compiler assumes the closing quote). After a closing quote only
//! blanks may follow before the next `,` or the end ("Expected , after quoted string in DATA statement").

/// One item: the byte range without the blanks around it (empty for an empty item), and whether it is quoted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RawItem {
    pub start: usize,
    pub end: usize,
    pub quoted: bool,
}

pub(crate) struct Scan {
    /// Where the `DATA` text ends: the line end, a `:` outside quotes, or the end of `bytes`.
    pub end: usize,
    pub items: Vec<RawItem>,
    /// The first non-blank byte after a closing quote, if any (an error in the old compiler).
    pub after_quote: Option<usize>,
}

/// Scans the `DATA` text that starts at `start`.
pub(crate) fn scan(bytes: &[u8], start: usize) -> Scan {
    let mut items = Vec::new();
    let mut after_quote = None;
    // The item being read: first and last non-blank byte, whether it is inside quotes, whether it was quoted and
    // closed.
    let mut first: Option<usize> = None;
    let mut last = start;
    let mut in_quotes = false;
    let mut closed = false;
    let mut i = start;
    let push = |items: &mut Vec<RawItem>, first: Option<usize>, last: usize, at: usize| {
        let (s, e) = first.map_or((at, at), |f| (f, last + 1));
        items.push(RawItem {
            start: s,
            end: e,
            quoted: first.is_some_and(|f| bytes[f] == b'"'),
        });
    };
    while let Some(&c) = bytes.get(i) {
        match c {
            b'\r' | b'\n' => break,
            b':' | b',' if !in_quotes => {
                push(&mut items, first, last, i);
                if c == b':' {
                    return Scan {
                        end: i,
                        items,
                        after_quote,
                    };
                }
                first = None;
                closed = false;
            }
            b' ' | b'\t' => {}
            _ => {
                if closed && after_quote.is_none() {
                    after_quote = Some(i);
                }
                if c == b'"' {
                    if in_quotes {
                        in_quotes = false;
                        closed = true;
                    } else if first.is_none() {
                        in_quotes = true;
                    }
                }
                first.get_or_insert(i);
                last = i;
            }
        }
        // Blanks inside quotes belong to the item.
        if in_quotes && matches!(c, b' ' | b'\t') {
            last = i;
        }
        i += 1;
    }
    push(&mut items, first, last, i);
    Scan {
        end: i,
        items,
        after_quote,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(src: &[u8]) -> Vec<&[u8]> {
        scan(src, 0).items.iter().map(|it| &src[it.start..it.end]).collect()
    }

    #[test]
    fn items_as_measured() {
        // `verification\v16_m2_data_blanks`, `_case`, `_quote`, `_rem`.
        let want: [&[u8]; 5] = [b"x", b"y y", b"", b"\"  q  \"", b""];
        assert_eq!(texts(b"  x  ,  y y  ,,\"  q  \","), want);
        let want: [&[u8]; 2] = [b"", b""];
        assert_eq!(texts(b","), want);
        let want: [&[u8]; 3] = [b"MixedCase", b"a\"b", b"x"];
        assert_eq!(texts(b"MixedCase, a\"b, x"), want);
        let want: [&[u8]; 2] = [b"a'b", b"c REM d"];
        assert_eq!(texts(b"a'b, c REM d"), want);
    }

    #[test]
    fn colon_ends_outside_quotes() {
        let s = scan(b"\"a:b\", c: PRINT 1", 0);
        assert_eq!(s.end, 8);
        assert_eq!(s.items.len(), 2);
        assert!(s.items[0].quoted);
        assert!(!s.items[1].quoted);
        // An inner quote does not protect a colon.
        assert_eq!(scan(b"a\"b: c", 0).end, 3);
    }

    #[test]
    fn unclosed_quote_and_text_after_a_quote() {
        let s = scan(b"\"a, b\r\nx", 0);
        assert_eq!(s.end, 5);
        assert_eq!(s.items.len(), 1);
        assert_eq!(scan(b"\"a\"  , b", 0).after_quote, None);
        assert_eq!(scan(b"\"a\" x, b", 0).after_quote, Some(4));
    }
}
