//! Files, byte positions and diagnostics shared by every compiler stage.
//!
//! Source text is bytes (design D2): nothing here assumes UTF-8. A position is a file and a byte offset; columns
//! are 1-based byte columns, which equal character columns under CP437.

use std::fmt;

/// Index of a file in a [`SourceMap`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FileId(pub u32);

/// A byte range `start..end` in one file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub file: FileId,
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(file: FileId, start: u32, end: u32) -> Span {
        debug_assert!(start <= end);
        Span { file, start, end }
    }

    pub fn len(&self) -> u32 {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// The smallest span covering both (same file).
    pub fn cover(self, other: Span) -> Span {
        debug_assert_eq!(self.file, other.file);
        Span::new(self.file, self.start.min(other.start), self.end.max(other.end))
    }
}

/// One source file: its name as given (used in diagnostics and `#line`), its bytes and its line starts.
pub struct SourceFile {
    pub name: String,
    pub bytes: Vec<u8>,
    line_starts: Vec<u32>,
}

impl SourceFile {
    pub fn new(name: impl Into<String>, bytes: Vec<u8>) -> SourceFile {
        let line_starts = line_starts(&bytes);
        SourceFile {
            name: name.into(),
            bytes,
            line_starts,
        }
    }

    /// 1-based line and 1-based byte column of an offset. An offset at the end of the file is on the last line
    /// (or on a new, empty line after a final line end).
    pub fn line_col(&self, offset: u32) -> (u32, u32) {
        let line = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        (line as u32 + 1, offset - self.line_starts[line] + 1)
    }

    pub fn line_count(&self) -> u32 {
        self.line_starts.len() as u32
    }
}

/// Offsets of line starts. CR LF, LF and a lone CR each end a line (as `lineformat$` in the old compiler).
fn line_starts(bytes: &[u8]) -> Vec<u32> {
    let mut starts = vec![0];
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\r' if bytes.get(i + 1) == Some(&b'\n') => {
                i += 2;
                starts.push(i as u32);
            }
            b'\r' | b'\n' => {
                i += 1;
                starts.push(i as u32);
            }
            _ => i += 1,
        }
    }
    starts
}

/// All files of one compilation.
#[derive(Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    pub fn new() -> SourceMap {
        SourceMap::default()
    }

    pub fn add(&mut self, name: impl Into<String>, bytes: Vec<u8>) -> FileId {
        self.files.push(SourceFile::new(name, bytes));
        FileId(self.files.len() as u32 - 1)
    }

    pub fn file(&self, id: FileId) -> &SourceFile {
        &self.files[id.0 as usize]
    }

    /// The bytes a span covers.
    pub fn text(&self, span: Span) -> &[u8] {
        &self.file(span.file).bytes[span.start as usize..span.end as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
        })
    }
}

/// A message about a span of source. Messages are plain ASCII; any source bytes quoted in them are shown as
/// written when printable ASCII and as `\xNN` otherwise (see [`show_bytes`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub span: Span,
    pub message: String,
}

impl Diagnostic {
    pub fn error(span: Span, message: impl Into<String>) -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            span,
            message: message.into(),
        }
    }

    /// `<file>:<line>:<column>: error: <message>` (spec `compiler/cli`).
    pub fn render(&self, map: &SourceMap) -> String {
        let file = map.file(self.span.file);
        let (line, col) = file.line_col(self.span.start);
        format!("{}:{}:{}: {}: {}", file.name, line, col, self.severity, self.message)
    }
}

/// Maximum number of errors reported per run (design D4).
pub const MAX_ERRORS: usize = 100;

/// Collects diagnostics, stopping at [`MAX_ERRORS`] errors with a final "too many errors".
#[derive(Clone, Default, Debug)]
pub struct Diagnostics {
    list: Vec<Diagnostic>,
    errors: usize,
    capped: bool,
}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        Diagnostics::default()
    }

    pub fn push(&mut self, d: Diagnostic) {
        if self.capped {
            return;
        }
        if d.severity == Severity::Error {
            if self.errors == MAX_ERRORS {
                self.capped = true;
                self.list.push(Diagnostic::error(d.span, "too many errors; stopping"));
                return;
            }
            self.errors += 1;
        }
        self.list.push(d);
    }

    pub fn error(&mut self, span: Span, message: impl Into<String>) {
        self.push(Diagnostic::error(span, message));
    }

    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }

    pub fn error_count(&self) -> usize {
        self.errors
    }

    pub fn is_capped(&self) -> bool {
        self.capped
    }

    pub fn list(&self) -> &[Diagnostic] {
        &self.list
    }

    pub fn extend(&mut self, other: Diagnostics) {
        for d in other.list {
            self.push(d);
        }
    }
}

/// Source bytes for a message: printable ASCII as is, anything else as `\xNN`.
pub fn show_bytes(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &b in bytes {
        if (0x20..0x7f).contains(&b) {
            s.push(b as char);
        } else {
            s.push_str(&format!("\\x{b:02X}"));
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lc(text: &[u8], offset: u32) -> (u32, u32) {
        SourceFile::new("t.bas", text.to_vec()).line_col(offset)
    }

    #[test]
    fn line_ends_lf() {
        let t = b"ab\ncd\n";
        assert_eq!(lc(t, 0), (1, 1));
        assert_eq!(lc(t, 2), (1, 3));
        assert_eq!(lc(t, 3), (2, 1));
        assert_eq!(lc(t, 4), (2, 2));
        assert_eq!(lc(t, 6), (3, 1)); // end of file after a final line end
    }

    #[test]
    fn line_ends_crlf() {
        let t = b"ab\r\ncd";
        assert_eq!(lc(t, 2), (1, 3)); // the CR
        assert_eq!(lc(t, 3), (1, 4)); // the LF belongs to line 1
        assert_eq!(lc(t, 4), (2, 1));
        assert_eq!(lc(t, 6), (2, 3)); // end of file without a final line end
    }

    #[test]
    fn line_ends_lone_cr() {
        let t = b"a\rb\r\rc";
        assert_eq!(lc(t, 2), (2, 1));
        assert_eq!(lc(t, 4), (3, 1));
        assert_eq!(lc(t, 5), (4, 1));
        assert_eq!(lc(t, 6), (4, 2));
    }

    #[test]
    fn mixed_line_ends_and_empty_file() {
        let f = SourceFile::new("t.bas", b"a\r\nb\nc\rd".to_vec());
        assert_eq!(f.line_count(), 4);
        assert_eq!(f.line_col(7), (4, 1));
        assert_eq!(f.line_col(8), (4, 2));
        assert_eq!(lc(b"", 0), (1, 1));
    }

    #[test]
    fn byte_columns_not_characters() {
        // 0xC9 0xCD are one CP437 character each and one byte each; a UTF-8 reading would differ.
        assert_eq!(lc(b"\xC9\xCD x", 3), (1, 4));
    }

    #[test]
    fn render_and_cap() {
        let mut map = SourceMap::new();
        let f = map.add("p.bas", b"x\nFOR i\n".to_vec());
        let d = Diagnostic::error(Span::new(f, 2, 5), "`FOR` is not supported yet");
        assert_eq!(d.render(&map), "p.bas:2:1: error: `FOR` is not supported yet");

        let mut ds = Diagnostics::new();
        for _ in 0..105 {
            ds.error(Span::new(f, 0, 1), "e");
        }
        assert_eq!(ds.error_count(), MAX_ERRORS);
        assert_eq!(ds.list().len(), MAX_ERRORS + 1);
        assert!(ds.list().last().unwrap().message.starts_with("too many errors"));
    }

    #[test]
    fn show_bytes_escapes() {
        assert_eq!(show_bytes(b"A\xC9"), "A\\xC9");
    }
}
