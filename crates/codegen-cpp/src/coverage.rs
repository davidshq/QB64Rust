//! A coverage build (design D1 of `cws-whole-game`, spec `compiler/cli`): a counter per source statement, and at exit
//! one record of the counts appended to the file `QB64RUST_COVERAGE` names.
//!
//! An entry is a position: file, line and column. A statement's column is where its span starts when that is on its
//! line, else 0 (a statement the lowering makes for a block on another line, as a `CASE` test, which carries the
//! `SELECT`'s span; the bottom of a loop carries its `NEXT`, `LOOP` or `WEND`, so a one-line loop counts its top and
//! bottom apart); statements at one position share an entry (`ON … GOSUB` is several statements of the `ON` line,
//! and `NEXT j, i` the steps of both loops). The lowering's own statements
//! ([`Stmt::own`]) get none. The record:
//!
//! ```text
//! coverage 1
//! file <index> <hash> <path>                 one per file of the program, by file index, tab-separated
//! <file> <line> <column> <count>             one per entry, tab-separated, unrun ones included
//! end
//! ```

use crate::names::c_string;
use qb64rust_base::Span;
use qb64rust_ir::Stmt;
use std::collections::HashMap;
use std::fmt::Write as _;

/// What a coverage build needs to know about the program's files.
pub struct Coverage<'m> {
    /// Each file of the program by its index (`FileId`): its path relative to the main file's folder and the
    /// FNV-1a 64 hash of its bytes, as 16 hex digits.
    pub files: Vec<(String, String)>,
    /// The 1-based line and column at which a span starts.
    pub position: &'m dyn Fn(Span) -> (u32, u32),
}

/// The entries numbered while the bodies are emitted.
pub(crate) struct Counters<'a> {
    coverage: &'a Coverage<'a>,
    entries: Vec<(u32, u32, u32)>,
    index: HashMap<(u32, u32, u32), usize>,
}

impl<'a> Counters<'a> {
    pub(crate) fn new(coverage: &'a Coverage<'a>) -> Counters<'a> {
        Counters {
            coverage,
            entries: Vec::new(),
            index: HashMap::new(),
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
        Some(format!("qbr_cov[{k}]++;"))
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
        write!(
            out,
            "}};\nstatic void qbr_cov_write(){{\nconst char *qbr_path=getenv(\"QB64RUST_COVERAGE\");\n\
             if (!qbr_path||!*qbr_path) return;\nFILE *qbr_f=fopen(qbr_path,\"ab\");\nif (!qbr_f) return;\n\
             fprintf(qbr_f,\"coverage 1\\n\");\n\
             for (int32 qbr_i=0;qbr_i<{files};qbr_i++) fprintf(qbr_f,\"file\\t%d\\t%s\\n\",(int)qbr_i,qbr_cov_files[qbr_i]);\n\
             for (int32 qbr_i=0;qbr_i<{entries};qbr_i++) fprintf(qbr_f,\"%u\\t%u\\t%u\\t%u\\n\",\
             (unsigned)qbr_cov_at[3*qbr_i],(unsigned)qbr_cov_at[3*qbr_i+1],(unsigned)qbr_cov_at[3*qbr_i+2],(unsigned)qbr_cov[qbr_i]);\n\
             fprintf(qbr_f,\"end\\n\");\nfclose(qbr_f);\n}}\n\
             static const int32 qbr_cov_registered=atexit(qbr_cov_write)|at_quick_exit(qbr_cov_write);\n",
            files = self.coverage.files.len(),
            entries = self.entries.len(),
        )
        .unwrap();
        out
    }
}
