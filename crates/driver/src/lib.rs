//! The compiler pipeline as a library, so that tests can run each stage; `main.rs` is the command line.

pub mod build;

use qb64rust_base::{Diagnostics, FileId, SourceMap};
use qb64rust_sema::Program;
use qb64rust_syntax::{NoLoader, ParsedProgram, parse};

/// The result of the front end for one program: its trees, and the typed program if parsing and checking gave no
/// errors.
pub struct Frontend {
    pub map: SourceMap,
    /// The main file.
    pub file: FileId,
    pub parsed: ParsedProgram,
    pub program: Program,
    pub diagnostics: Diagnostics,
}

impl Frontend {
    pub fn has_errors(&self) -> bool {
        self.diagnostics.has_errors()
    }

    /// Each diagnostic rendered as `<file>:<line>:<col>: error: <message>`, one per line, in source order.
    pub fn render_diagnostics(&self) -> String {
        let mut list: Vec<_> = self.diagnostics.list().to_vec();
        list.sort_by_key(|d| d.span.start);
        let mut out = String::new();
        for d in list {
            out.push_str(&d.render(&self.map));
            out.push('\n');
        }
        out
    }
}

/// Lexes, parses and checks a file. `name` is how the file is shown in diagnostics and `#line` directives.
pub fn frontend(name: &str, bytes: Vec<u8>) -> Frontend {
    let mut map = SourceMap::new();
    let file = map.add(name, bytes);
    // Included files are loaded from task 8.1 of `m2-parser-breadth` on.
    let parsed = parse(&mut map, file, &mut NoLoader);
    // QB64RUST_NO_FOLD=1 turns integer constant folding off (test use only, design D5).
    let fold = std::env::var_os("QB64RUST_NO_FOLD").is_none_or(|v| v != "1");
    let (program, sema_diags) = qb64rust_sema::check_with(&map, &parsed, fold);
    let mut diagnostics = parsed.diagnostics();
    diagnostics.extend(sema_diags);
    Frontend {
        map,
        file,
        parsed,
        program,
        diagnostics,
    }
}

/// The IR of a checked program.
pub fn lower(fe: &Frontend) -> qb64rust_ir::Program {
    qb64rust_ir::lower(&fe.program)
}

/// `--dump ir`.
pub fn dump_ir(fe: &Frontend) -> String {
    qb64rust_ir::dump(&lower(fe))
}

/// The C++ fragments of a checked program; `#line` directives name the file as the front end was given it.
pub fn emit(fe: &Frontend) -> qb64rust_codegen_cpp::Fragments {
    qb64rust_codegen_cpp::emit(&lower(fe), &fe.map.file(fe.file).name)
}

/// `--dump cpp`.
pub fn dump_cpp(fe: &Frontend) -> String {
    qb64rust_codegen_cpp::dump(&emit(fe))
}
