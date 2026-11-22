//! A call-trace build (OpenSpec change `call-trace`, spec `compiler/call-trace`): a call stack in the message of an
//! untrapped runtime error.
//!
//! A procedure pushes a frame at its start and pops it at `exit_subfunc` (design D1). A statement that calls a
//! procedure of the program first stores its site, the file and line the frame below records (design D2): the
//! emitter notes such a call where it writes one ([`Sites::called`]), so the store goes only before the statements
//! that need it. A pending `GOSUB` is read from libqb's own return stack, through a table from each `GOSUB`'s return
//! id to its site (design D4). An event's frame and an error handler's are pushed by the code that runs them
//! (`events.rs`, `mainerr.txt`; design D5).
//!
//! The run-time part is `call_trace_rt.h`, C++ that goes into the program after the tables [`Sites::table`] writes.
//! Its hook is the one libqb's report of an error calls (`tools\libqb\edits\call-trace-*.edit`, design D6).

use crate::names::c_string;
use qb64rust_base::FileId;
use qb64rust_ir::{ProcKind, Program};
use std::collections::HashMap;
use std::fmt::Write as _;

/// What a call-trace build needs to know about the program's files.
pub struct CallTrace<'m> {
    /// Each file of the program by its index (`FileId`): its path from the main file's folder, with `/` between
    /// folders, as a coverage record names it.
    pub files: Vec<String>,
    /// The main file.
    pub main: FileId,
    /// The text of a 1-based line of a file, for a procedure's name as its header line declares it.
    pub line: &'m dyn Fn(FileId, u32) -> Option<String>,
}

/// The run-time part, written after the tables.
const RUNTIME: &str = include_str!("call_trace_rt.h");

/// The first site number of a file and line: 0 is "not known", 1 "an event".
const FIRST_SITE: u32 = 2;

/// The sites numbered while the bodies are emitted.
pub(crate) struct Sites<'a> {
    call_trace: &'a CallTrace<'a>,
    /// Each site from [`FIRST_SITE`] on: its file's index and its line.
    sites: Vec<(u32, u32)>,
    index: HashMap<(u32, u32), u32>,
    /// The site of each `GOSUB`, by its return id less 1.
    gosubs: Vec<u32>,
    /// A call of one of the program's procedures was written since the statement began.
    pub(crate) called: bool,
}

impl<'a> Sites<'a> {
    pub(crate) fn new(call_trace: &'a CallTrace<'a>) -> Sites<'a> {
        Sites {
            call_trace,
            sites: Vec::new(),
            index: HashMap::new(),
            gosubs: Vec::new(),
            called: false,
        }
    }

    /// The program's main file.
    pub(crate) fn main(&self) -> FileId {
        self.call_trace.main
    }

    /// The number of the site at `line` of `file`.
    pub(crate) fn site(&mut self, file: FileId, line: u32) -> u32 {
        let at = (file.0, line);
        let next = FIRST_SITE + qb64rust_base::to_u32(self.sites.len());
        let n = *self.index.entry(at).or_insert(next);
        if n == next {
            self.sites.push(at);
        }
        n
    }

    /// The line that stores the site of a statement at `line` of `file` that calls a procedure.
    pub(crate) fn store(&mut self, file: FileId, line: u32) -> String {
        format!("qbr_ct.site={};", self.site(file, line))
    }

    /// Notes the `GOSUB` with return id `id` (from 1, in the order they are emitted) at `line` of `file`.
    pub(crate) fn gosub(&mut self, id: u32, file: FileId, line: u32) {
        let site = self.site(file, line);
        debug_assert_eq!(self.gosubs.len() + 1, id as usize, "GOSUB ids in order");
        self.gosubs.push(site);
    }

    /// The tables of `p`'s call-trace build, then the run-time part: the name of each frame's procedure, each
    /// file's path and base name, each site's file and line, and each `GOSUB`'s site.
    pub(crate) fn table(&self, p: &Program) -> String {
        let mut names = vec![c_string(b"Main module"), c_string(b"Error handler")];
        for q in p.procs.iter().filter(|q| q.external.is_none()) {
            let declared = declared_name(q.kind, &q.name, (self.call_trace.line)(q.file, q.line).as_deref());
            names.push(c_string(declared.as_bytes()));
        }
        let files = &self.call_trace.files;
        let paths: Vec<String> = files.iter().map(|f| c_string(f.as_bytes())).collect();
        let bases: Vec<String> = files
            .iter()
            .map(|f| c_string(f.rsplit('/').next().unwrap_or(f).as_bytes()))
            .collect();
        let mut at = vec!["0".to_string(); 2 * FIRST_SITE as usize];
        for &(file, line) in &self.sites {
            at.push(file.to_string());
            at.push(line.to_string());
        }
        let mut gosubs = vec!["0".to_string()];
        gosubs.extend(self.gosubs.iter().map(u32::to_string));
        let mut out = String::new();
        let list = |v: &[String]| if v.is_empty() { "\"\"".to_string() } else { v.join(",") };
        write!(
            out,
            "static const uint32 qbr_ct_procs={};\nstatic const char *const qbr_ct_names[]={{{}}};\n\
             static const uint32 qbr_ct_file_count={};\nstatic const char *const qbr_ct_files[]={{{}}};\n\
             static const char *const qbr_ct_bases[]={{{}}};\nstatic const uint32 qbr_ct_main_file={};\n\
             static const uint32 qbr_ct_sites={};\nstatic const uint32 qbr_ct_at[]={{{}}};\n\
             static const uint32 qbr_ct_gosub_count={};\nstatic const uint32 qbr_ct_gosub_at[]={{{}}};\n",
            names.len(),
            names.join(","),
            files.len(),
            list(&paths),
            list(&bases),
            self.call_trace.main.0,
            at.len() / 2,
            at.join(","),
            gosubs.len(),
            gosubs.join(","),
        )
        .unwrap();
        out.push_str(RUNTIME);
        out
    }
}

/// A procedure's name as the debugger's call stack shows it (`crates\dap\src\stack.rs`, `procedure_names`):
/// `SUB show` or `FUNCTION twice&`, the name as its header line `line` declares it; the IR's name `name` (upper
/// case, no suffix) when the line does not start with the keyword.
pub(crate) fn declared_name(kind: ProcKind, name: &str, line: Option<&str>) -> String {
    let keyword = match kind {
        ProcKind::Sub => "SUB",
        ProcKind::Function(_) => "FUNCTION",
    };
    let declared = line.and_then(|text| {
        let rest = text.trim_start();
        let word = rest.get(..keyword.len())?;
        if !word.eq_ignore_ascii_case(keyword) {
            return None;
        }
        let name: String = rest[keyword.len()..]
            .trim_start()
            .chars()
            .take_while(|c| !c.is_whitespace() && *c != '(' && *c != '\'')
            .collect();
        (!name.is_empty()).then_some(name)
    });
    format!("{keyword} {}", declared.unwrap_or_else(|| name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_line(_: FileId, _: u32) -> Option<String> {
        None
    }

    #[test]
    fn sites_are_numbered_from_two_once_each() {
        let ct = CallTrace {
            files: vec!["main.bas".into(), "rules/army.bm".into()],
            main: FileId(0),
            line: &no_line,
        };
        let mut s = Sites::new(&ct);
        assert_eq!(s.site(FileId(0), 3), 2);
        assert_eq!(s.site(FileId(1), 3), 3, "another file is another site");
        assert_eq!(s.site(FileId(0), 3), 2, "the same place again");
        assert_eq!(s.store(FileId(0), 8), "qbr_ct.site=4;");
        s.gosub(1, FileId(0), 3);
        s.gosub(2, FileId(0), 9);
        assert_eq!(s.gosubs, vec![2, 5]);
    }

    #[test]
    fn the_tables() {
        let ct = CallTrace {
            files: vec!["main.bas".into(), "rules/army.bm".into()],
            main: FileId(0),
            line: &no_line,
        };
        let mut s = Sites::new(&ct);
        s.site(FileId(1), 212);
        s.gosub(1, FileId(0), 3);
        let text = s.table(&Program::default());
        assert!(
            text.contains("qbr_ct_names[]={\"Main module\",\"Error handler\"};"),
            "{text}"
        );
        assert!(
            text.contains("qbr_ct_files[]={\"main.bas\",\"rules/army.bm\"};"),
            "{text}"
        );
        assert!(text.contains("qbr_ct_bases[]={\"main.bas\",\"army.bm\"};"), "{text}");
        assert!(text.contains("qbr_ct_sites=4;"), "{text}");
        assert!(text.contains("qbr_ct_at[]={0,0,0,0,1,212,0,3};"), "{text}");
        assert!(text.contains("qbr_ct_gosub_at[]={0,3};"), "{text}");
        assert!(
            text.contains("static const int32 qbr_ct_registered"),
            "the run-time part follows"
        );
    }

    #[test]
    fn names_as_declared() {
        let sub = ProcKind::Sub;
        assert_eq!(
            declared_name(sub, "TWICE", Some("  sub Twice (n AS LONG)")),
            "SUB Twice"
        );
        assert_eq!(declared_name(sub, "FLAG_X", Some("SUB flag_x 'the flag")), "SUB flag_x");
        assert_eq!(declared_name(sub, "TWICE", None), "SUB TWICE");
        assert_eq!(declared_name(sub, "TWICE", Some("DECLARE SUB twice")), "SUB TWICE");
        let function = ProcKind::Function(qb64rust_ir::Ty::I32);
        assert_eq!(
            declared_name(function, "TWICE", Some("FUNCTION twice& (n&)")),
            "FUNCTION twice&"
        );
    }
}
