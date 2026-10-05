//! Metacommands in comments (design D2 of `m2-parser-breadth`), by the old compiler's rule (`qb64pe.bas`
//! `lineformat`, 25426–25496):
//!
//! - a comment (`'` or `REM`) whose first non-blank character after the leader is `$` is a metacommand comment;
//!   any other comment is plain;
//! - in it, every `$STATIC`, `$DYNAMIC` and `$INCLUDE` (not `$INCLUDEONCE`) is processed, wherever it is (also
//!   inside a word: `$STATICanychars$DYNAMIC` has both); the search goes from one `$` to the next;
//! - `$INCLUDE` must be followed by blanks, `:`, blanks and a file name in single quotes, else it is an error;
//!   only the last `$INCLUDE` of a comment counts;
//! - `$FORMAT:ON` and `$FORMAT:OFF` are for the old IDE only, as is any other text.

/// What a metacommand comment asks for.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommentDirectives {
    /// `$STATIC` or `$DYNAMIC`, whichever comes last.
    pub memory: Option<MemoryMode>,
    /// The file name of the last `$INCLUDE`, as written (case kept).
    pub include: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryMode {
    Static,
    Dynamic,
}

impl CommentDirectives {
    pub fn is_empty(&self) -> bool {
        self.memory.is_none() && self.include.is_none()
    }
}

/// The comment's text after its leader (`'` or `REM`), or `None` when `comment` is not a comment.
fn body(comment: &[u8]) -> Option<&[u8]> {
    if let Some(rest) = comment.strip_prefix(b"'") {
        Some(rest)
    } else if comment.len() >= 3 && comment[..3].eq_ignore_ascii_case(b"REM") {
        Some(&comment[3..])
    } else {
        None
    }
}

fn is_blank(b: u8) -> bool {
    b == b' ' || b == b'\t'
}

/// Whether the comment token text `comment` (with its `'` or `REM`) is a metacommand comment.
pub fn is_meta_comment(comment: &[u8]) -> bool {
    body(comment).is_some_and(|b| b.iter().find(|&&c| !is_blank(c)) == Some(&b'$'))
}

/// The directives of a metacommand comment; an empty value for a plain comment. `Err` is the old compiler's
/// error for a malformed `$INCLUDE`.
pub fn comment_directives(comment: &[u8]) -> Result<CommentDirectives, &'static str> {
    let mut out = CommentDirectives::default();
    if !is_meta_comment(comment) {
        return Ok(out);
    }
    let body = body(comment).unwrap_or_default();
    let start = body.iter().position(|&c| !is_blank(c)).unwrap_or(body.len());
    let c = &body[start..];
    let at = |x: usize, word: &[u8]| c.len() >= x + word.len() && c[x..x + word.len()].eq_ignore_ascii_case(word);
    const BAD_INCLUDE: &str = "expected `$INCLUDE:'filename'`";
    let mut x = 0;
    loop {
        if at(x, b"$FORMAT:OFF") || at(x, b"$FORMAT:ON") {
            // For the old IDE only.
        } else if at(x, b"$STATIC") {
            out.memory = Some(MemoryMode::Static);
        } else if at(x, b"$DYNAMIC") {
            out.memory = Some(MemoryMode::Dynamic);
        } else if at(x, b"$INCLUDE") && !at(x + 8, b"ONCE") {
            let mut i = x + 8;
            while i < c.len() && is_blank(c[i]) {
                i += 1;
            }
            if c.get(i) != Some(&b':') {
                return Err(BAD_INCLUDE);
            }
            i += 1;
            while i < c.len() && is_blank(c[i]) {
                i += 1;
            }
            if c.get(i) != Some(&b'\'') {
                return Err(BAD_INCLUDE);
            }
            let name_start = i + 1;
            let Some(len) = c[name_start..].iter().position(|&b| b == b'\'') else {
                return Err(BAD_INCLUDE);
            };
            if len == 0 {
                return Err(BAD_INCLUDE);
            }
            out.include = Some(c[name_start..name_start + len].to_vec());
            x = i;
        }
        match c[x + 1..].iter().position(|&b| b == b'$') {
            Some(n) => x += 1 + n,
            None => break,
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn include(name: &[u8]) -> CommentDirectives {
        CommentDirectives {
            memory: None,
            include: Some(name.to_vec()),
        }
    }

    fn memory(m: MemoryMode) -> CommentDirectives {
        CommentDirectives {
            memory: Some(m),
            include: None,
        }
    }

    #[test]
    fn plain_comments() {
        assert!(!is_meta_comment(b"' hello $INCLUDE:'a.bi'"));
        assert!(!is_meta_comment(b"'"));
        assert!(!is_meta_comment(b"REM"));
        assert!(comment_directives(b"' x $DYNAMIC").unwrap().is_empty());
    }

    #[test]
    fn include_forms() {
        assert_eq!(comment_directives(b"'$INCLUDE:'a.bi'"), Ok(include(b"a.bi")));
        assert_eq!(
            comment_directives(b"' $include : 'Dir\\Lib.BM' rest"),
            Ok(include(b"Dir\\Lib.BM"))
        );
        assert_eq!(comment_directives(b"REM $INCLUDE: 'x.bm'"), Ok(include(b"x.bm")));
        assert_eq!(comment_directives(b"rem\t$Include:'x.bm'"), Ok(include(b"x.bm")));
        // Only the last one counts.
        assert_eq!(
            comment_directives(b"'$INCLUDE:'a.bi' $INCLUDE:'b.bi'"),
            Ok(include(b"b.bi"))
        );
    }

    #[test]
    fn bad_include() {
        assert!(comment_directives(b"'$INCLUDE 'a.bi'").is_err());
        assert!(comment_directives(b"'$INCLUDE:a.bi").is_err());
        assert!(comment_directives(b"'$INCLUDE:'a.bi").is_err());
        assert!(comment_directives(b"'$INCLUDE:''").is_err());
    }

    #[test]
    fn memory_modes() {
        assert_eq!(comment_directives(b"'$DYNAMIC"), Ok(memory(MemoryMode::Dynamic)));
        assert_eq!(
            comment_directives(b"REM $FOO $DYNAMIC"),
            Ok(memory(MemoryMode::Dynamic))
        );
        assert_eq!(
            comment_directives(b"'$STATICanychars$DYNAMIC"),
            Ok(memory(MemoryMode::Dynamic))
        );
        assert_eq!(comment_directives(b"'$dynamic $static"), Ok(memory(MemoryMode::Static)));
    }

    #[test]
    fn ide_only_and_includeonce() {
        assert!(is_meta_comment(b"'$Format:Off"));
        assert!(comment_directives(b"'$Format:Off").unwrap().is_empty());
        assert!(comment_directives(b"'$FORMAT:ON").unwrap().is_empty());
        assert!(comment_directives(b"'$INCLUDEONCE").unwrap().is_empty());
    }
}
