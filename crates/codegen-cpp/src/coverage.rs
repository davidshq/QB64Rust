//! A coverage build (design D1 of `cws-whole-game`, spec `compiler/cli`): a counter per source statement, and at exit
//! one record of the counts appended to the file `QB64RUST_COVERAGE` names.
//!
//! An entry is a position: file, line and column. A statement's column is where its span starts when that is on its
//! line, else 0 (a statement the lowering makes for a block on another line, as a `CASE` test, which carries the
//! `SELECT`'s span; the bottom of a loop carries its `NEXT`, `LOOP` or `WEND`, so a one-line loop counts its top and
//! bottom apart); statements at one position share an entry (`ON … GOSUB` is several statements of the `ON` line,
//! and `NEXT j, i` the steps of both loops). The lowering's own statements
//! ([`Stmt::own`]) get none. The record (version 2, design D2 of `m4-coverage-review`; version 1 had no names):
//!
//! ```text
//! coverage 2
//! file <index> <hash> <path>                 one per file of the program, by file index, tab-separated
//! <file> <line> <column> <count> <names>     one per entry, tab-separated, unrun ones included; the built-ins its
//!                                            statements name, space-separated, empty for none
//! end
//! ```

use crate::names::c_string;
use qb64rust_base::Span;
use qb64rust_ir::Stmt;
use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;

/// What a coverage build needs to know about the program's files.
pub struct Coverage<'m> {
    /// Each file of the program by its index (`FileId`): its path relative to the main file's folder and the
    /// FNV-1a 64 hash of its bytes, as 16 hex digits.
    pub files: Vec<(String, String)>,
    /// The 1-based line and column at which a span starts.
    pub position: &'m dyn Fn(Span) -> (u32, u32),
    /// Each name of a built-in in the program's source, at its token (design D1 of `m4-coverage-review`): an entry
    /// names those its statements hold innermost ([`innermost`]).
    pub builtins: Vec<(Span, &'static str)>,
}

/// The entries numbered while the bodies are emitted.
pub(crate) struct Counters<'a> {
    coverage: &'a Coverage<'a>,
    entries: Vec<(u32, u32, u32)>,
    index: HashMap<(u32, u32, u32), usize>,
    /// Each counted statement's span, its line and its entry.
    spans: Vec<(Span, u32, usize)>,
}

impl<'a> Counters<'a> {
    pub(crate) fn new(coverage: &'a Coverage<'a>) -> Counters<'a> {
        Counters {
            coverage,
            entries: Vec::new(),
            index: HashMap::new(),
            spans: Vec::new(),
        }
    }

    /// The line that counts statement `s`, or `None` for a statement of the lowering's own.
    pub(crate) fn count(&mut self, s: &Stmt) -> Option<String> {
        if s.own {
            return None;
        }
        let (line, col) = (self.coverage.position)(s.span);
        let at = (s.span.file.0, s.line, if line == s.line { col } else { 0 });
        let next = self.entries.len();
        let k = *self.index.entry(at).or_insert(next);
        if k == next {
            self.entries.push(at);
        }
        self.spans.push((s.span, s.line, k));
        Some(format!("qbr_cov[{k}]++;"))
    }

    /// The names of each entry, sorted and joined by a space; empty for an entry whose statements name none.
    fn names(&self) -> Vec<String> {
        let mut names = vec![BTreeSet::new(); self.entries.len()];
        for &(at, name) in &self.coverage.builtins {
            let line = (self.coverage.position)(at).0;
            if let Some(k) = innermost(&self.spans, at, line) {
                names[k].insert(name);
            }
        }
        names
            .into_iter()
            .map(|n| n.into_iter().collect::<Vec<_>>().join(" "))
            .collect()
    }

    /// The counters, the table of entries and files, and the writer, registered with `atexit` when the program
    /// starts: every end of a libqb program (`SYSTEM`, `END`, the window closed, a runtime error) reaches it. It is
    /// registered with `at_quick_exit` too, for the screen-state check's stop (`quick_exit`, which skips `atexit`).
    pub(crate) fn table(&self) -> String {
        let n = self.entries.len().max(1);
        let mut out = format!(
            "static uint32 qbr_cov[{n}];\nstatic const uint32 qbr_cov_at[{}]={{",
            3 * n
        );
        let at: Vec<String> = if self.entries.is_empty() {
            vec!["0".into(), "0".into(), "0".into()]
        } else {
            self.entries
                .iter()
                .flat_map(|&(f, l, c)| [f.to_string(), l.to_string(), c.to_string()])
                .collect()
        };
        out.push_str(&at.join(","));
        out.push_str("};\nstatic const char *const qbr_cov_files[]={");
        let files: Vec<String> = self
            .coverage
            .files
            .iter()
            .map(|(path, hash)| c_string(format!("{hash}\t{path}").as_bytes()))
            .collect();
        out.push_str(&files.join(","));
        out.push_str("};\nstatic const char *const qbr_cov_names[]={");
        let mut names: Vec<String> = self.names().iter().map(|n| c_string(n.as_bytes())).collect();
        if names.is_empty() {
            names.push(c_string(b""));
        }
        out.push_str(&names.join(","));
        write!(
            out,
            "}};\nstatic void qbr_cov_write(){{\nconst char *qbr_path=getenv(\"QB64RUST_COVERAGE\");\n\
             if (!qbr_path||!*qbr_path) return;\nFILE *qbr_f=fopen(qbr_path,\"ab\");\nif (!qbr_f) return;\n\
             fprintf(qbr_f,\"coverage 2\\n\");\n\
             for (int32 qbr_i=0;qbr_i<{files};qbr_i++) fprintf(qbr_f,\"file\\t%d\\t%s\\n\",(int)qbr_i,qbr_cov_files[qbr_i]);\n\
             for (int32 qbr_i=0;qbr_i<{entries};qbr_i++) fprintf(qbr_f,\"%u\\t%u\\t%u\\t%u\\t%s\\n\",\
             (unsigned)qbr_cov_at[3*qbr_i],(unsigned)qbr_cov_at[3*qbr_i+1],(unsigned)qbr_cov_at[3*qbr_i+2],(unsigned)qbr_cov[qbr_i],\
             qbr_cov_names[qbr_i]);\n\
             fprintf(qbr_f,\"end\\n\");\nfclose(qbr_f);\n}}\n\
             static const int32 qbr_cov_registered=atexit(qbr_cov_write)|at_quick_exit(qbr_cov_write);\n",
            files = self.coverage.files.len(),
            entries = self.entries.len(),
        )
        .unwrap();
        out
    }
}

/// The entry a name at `at` (on line `line`) belongs to: that of the smallest counted span that holds it, so a name
/// in the body of a block is its own statement's, not the block's. Of spans equally small, the one of a statement
/// on the name's line (a `CASE` test carries its `SELECT`'s span), then the first counted. `None` when no counted
/// statement holds it (a declaration, a `CONST`).
fn innermost(spans: &[(Span, u32, usize)], at: Span, line: u32) -> Option<usize> {
    spans
        .iter()
        .enumerate()
        .filter(|(_, (s, _, _))| s.file == at.file && s.start <= at.start && at.end <= s.end)
        .min_by_key(|&(i, &(s, l, _))| (s.end - s.start, l != line, i))
        .map(|(_, &(_, _, k))| k)
}

#[cfg(test)]
mod tests {
    use super::innermost;
    use qb64rust_base::{FileId, Span};

    fn span(start: u32, end: u32) -> Span {
        Span {
            file: FileId(0),
            start,
            end,
        }
    }

    #[test]
    fn a_name_in_a_block_body_is_the_body_statements() {
        // IF LEN(a$) THEN | PRINT LEFT$(b$, 1) | END IF: the block spans 0..40, its body statement 20..35.
        let spans = [(span(0, 40), 1, 0), (span(20, 35), 2, 1)];
        assert_eq!(innermost(&spans, span(3, 6), 1), Some(0));
        assert_eq!(innermost(&spans, span(26, 31), 2), Some(1));
    }

    #[test]
    fn of_equal_spans_the_one_on_the_names_line() {
        // A CASE test carries the SELECT's span; the name on the CASE's line is the test's.
        let spans = [(span(0, 50), 1, 0), (span(0, 50), 3, 1)];
        assert_eq!(innermost(&spans, span(30, 33), 3), Some(1));
        assert_eq!(innermost(&spans, span(10, 13), 1), Some(0));
    }

    #[test]
    fn a_name_outside_every_statement_has_no_entry() {
        let spans = [(span(10, 20), 1, 0)];
        assert_eq!(innermost(&spans, span(0, 3), 1), None);
        let other = Span {
            file: FileId(1),
            start: 12,
            end: 15,
        };
        assert_eq!(innermost(&spans, other, 1), None);
    }
}
