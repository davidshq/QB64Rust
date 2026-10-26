//! The layout of a parsed and checked program: `qb64rust -y` (spec `compiler/format`), which writes a file as
//! `qb64pe -y` lays it out.
//!
//! QB64pe has no separate formatter: each statement handler of its main pass appends the statement's text to
//! `layout$`, and `apply_layout_indent$` turns that into the line it writes (`openspec\changes\m5b-formatter\
//! design.md`, Context). This crate builds the same intermediate form from the tree, statement by statement
//! (design D2): [`walk`] the lines and their indent, `stmt` the statements, `expr` the expressions, [`fmt`] the
//! spelling of keywords and names; and [`apply`] ports the last step.

pub mod apply;
mod expr;
mod fmt;
mod lay;
mod stmt;
mod walk;
mod words;

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

/// Lays out the file `file` of a program that parsed and checked without errors, with `settings`. The result
/// holds every line of the file, each followed by CRLF, as `qb64pe -y` writes its buffer (design D4).
pub fn format_file(
    map: &SourceMap,
    parsed: &ParsedProgram,
    program: &Program,
    file: FileId,
    settings: &Settings,
) -> Vec<u8> {
    let bytes = &map.file(file).bytes;
    let ranges = line_ranges(bytes);
    let f = fmt::Fmt {
        map,
        program,
        file,
        settings,
    };
    let plans = match parsed.trees.iter().find(|t| t.file == file) {
        Some(tree) => f.plan(tree.root(), &ranges),
        None => Vec::new(),
    };
    let mut out = Vec::with_capacity(bytes.len() + bytes.len() / 8);
    for (i, &(a, b)) in ranges.iter().enumerate() {
        let line = &bytes[a as usize..b as usize];
        match plans.get(i) {
            Some(walk::Plan::Layout(layout, apply)) => match apply::apply_layout_indent(layout, line, *apply) {
                Some(text) => out.extend_from_slice(&text),
                None => out.extend_from_slice(line),
            },
            Some(walk::Plan::AsRead) | None => out.extend_from_slice(line),
        }
        out.extend_from_slice(b"\r\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_end_as_qb64pe_reads_them() {
        let lines = source_lines(b"a\r\nb\nc\rd\n\re\r\n\r\nf");
        assert_eq!(lines, [&b"a"[..], b"b", b"c", b"d", b"e", b"", b"f"]);
        assert_eq!(source_lines(b"a\r\n"), [&b"a"[..]]);
        assert!(source_lines(b"").is_empty());
    }
}
