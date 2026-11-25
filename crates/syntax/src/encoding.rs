//! A file's `'$ENCODING` line and its byte order mark (OpenSpec change `utf8-source`, design D2, D3 and D6; spec
//! `compiler/source-encoding`).
//!
//! The line is a comment that starts with `'$ENCODING:`, alone on its line but for blanks, and names `UTF8` or
//! `CP437` in any case. QB64pe reads it as a comment (`verification\v74_a1`–`a4`), so a line with another value,
//! or a second line that names another encoding than the first, is a warning, and the file then reads as if it
//! were not there (user, 2026-11-24).

use crate::SyntaxKind;
use crate::lexer::Token;
use qb64rust_base::{Diagnostic, FileId, SourceEncoding, SourceMap, Span};

/// The UTF-8 byte order mark.
pub const BOM: &[u8] = b"\xEF\xBB\xBF";

/// The `'$ENCODING` line that decides a file's encoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncodingLine {
    pub encoding: SourceEncoding,
    /// The comment token.
    pub span: Span,
}

/// How a file's encoding was found (spec `compiler/source-encoding`, "A file's encoding").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclaredBy {
    /// The file's own `'$ENCODING` line.
    Line(Span),
    /// The `encoding` key of the settings file.
    Settings,
    /// Neither: the file is code page 437.
    Nothing,
}

/// A file's encoding and how it was found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileEncoding {
    pub encoding: SourceEncoding,
    pub by: DeclaredBy,
}

impl FileEncoding {
    /// The encoding of a file whose line is `line` and whose settings file gives `project`.
    pub fn of(line: Option<EncodingLine>, project: Option<SourceEncoding>) -> FileEncoding {
        match (line, project) {
            (Some(l), _) => FileEncoding {
                encoding: l.encoding,
                by: DeclaredBy::Line(l.span),
            },
            (None, Some(encoding)) => FileEncoding {
                encoding,
                by: DeclaredBy::Settings,
            },
            (None, None) => FileEncoding {
                encoding: SourceEncoding::Cp437,
                by: DeclaredBy::Nothing,
            },
        }
    }

    pub fn is_utf8(self) -> bool {
        self.encoding == SourceEncoding::Utf8
    }
}

/// The value of a comment that starts with `'$ENCODING:` (any case), or `None` for any other comment.
fn value(comment: &[u8]) -> Option<&[u8]> {
    const LEAD: &[u8] = b"'$ENCODING:";
    let head = comment.get(..LEAD.len())?;
    head.eq_ignore_ascii_case(LEAD).then(|| {
        let rest = &comment[LEAD.len()..];
        let end = rest
            .iter()
            .rposition(|&b| !matches!(b, b' ' | b'\t' | b'\r'))
            .map_or(0, |i| i + 1);
        &rest[..end]
    })
}

/// The file's `'$ENCODING` line from its tokens, with a warning for each line that names no encoding or another
/// encoding than the first, and an error for a byte order mark in a file that is not UTF-8. `project` is the
/// encoding the settings file gives the file.
pub(crate) fn scan(
    file: FileId,
    bytes: &[u8],
    tokens: &[Token],
    project: Option<SourceEncoding>,
) -> (Option<EncodingLine>, Vec<Diagnostic>) {
    let mut found: Option<EncodingLine> = None;
    let mut diags = Vec::new();
    let mut at = 0u32;
    let mut line_start = true;
    for t in tokens {
        let start = at;
        at += t.len;
        match t.kind {
            SyntaxKind::Newline => line_start = true,
            SyntaxKind::Whitespace | SyntaxKind::ByteOrderMark => {}
            SyntaxKind::MetaComment if line_start => {
                line_start = false;
                let span = Span::new(file, start, at);
                let Some(v) = value(&bytes[start as usize..at as usize]) else {
                    continue;
                };
                let encoding = if v.eq_ignore_ascii_case(b"UTF8") {
                    SourceEncoding::Utf8
                } else if v.eq_ignore_ascii_case(b"CP437") {
                    SourceEncoding::Cp437
                } else {
                    let shown = qb64rust_base::show_bytes(v);
                    diags.push(Diagnostic::warning(
                        span,
                        format!("`'$ENCODING:` takes `UTF8` or `CP437`, not `{shown}`; this line declares nothing"),
                    ));
                    continue;
                };
                match found {
                    None => found = Some(EncodingLine { encoding, span }),
                    Some(first) if first.encoding != encoding => {
                        let line = 1 + bytes[..first.span.start as usize]
                            .iter()
                            .filter(|&&b| b == b'\n')
                            .count();
                        diags.push(Diagnostic::warning(
                            span,
                            format!(
                                "this `'$ENCODING` line names another encoding than the one on line {line}, which \
                                 decides; this line declares nothing"
                            ),
                        ));
                    }
                    Some(_) => {}
                }
            }
            _ => line_start = false,
        }
    }
    let utf8 = FileEncoding::of(found, project).is_utf8();
    if !utf8 && tokens.first().is_some_and(|t| t.kind == SyntaxKind::ByteOrderMark) {
        diags.push(Diagnostic::error(
            Span::new(file, 0, 3),
            "the file starts with a byte order mark (the bytes 0xEF 0xBB 0xBF), which only a file declared UTF-8 \
             may hold",
        ));
    }
    (found, diags)
}

/// The encoding of `file`, whose tree holds `line` ([`crate::Tree::encoding`]).
pub fn file_encoding(map: &SourceMap, file: FileId, line: Option<EncodingLine>) -> FileEncoding {
    FileEncoding::of(line, map.project_encoding(file))
}
