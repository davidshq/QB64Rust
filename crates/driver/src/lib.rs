//! The compiler pipeline as a library, so that tests can run each stage; `main.rs` is the command line.

pub mod build;

use qb64rust_base::{Diagnostics, FileId, SourceMap};
use qb64rust_sema::Program;
use qb64rust_syntax::tree::Node;
use qb64rust_syntax::{Parse, parse};

/// The result of the front end for one file: the parse, and the typed program if parsing and checking gave no
/// errors.
pub struct Frontend {
    pub map: SourceMap,
    pub file: FileId,
    pub parse: Parse,
    pub program: Program,
    pub diagnostics: Diagnostics,
}

impl Frontend {
    pub fn root(&self) -> Node<'_> {
        Node::root(&self.parse.green, self.file)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.map.file(self.file).bytes
    }

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
    let parse = parse(file, &map.file(file).bytes);
    let root = Node::root(&parse.green, file);
    // QB64RUST_NO_FOLD=1 turns integer constant folding off (test use only, design D5).
    let fold = std::env::var_os("QB64RUST_NO_FOLD").is_none_or(|v| v != "1");
    let (program, sema_diags) = qb64rust_sema::check_with(root, map.file(file), &parse.diagnostics, fold);
    let mut diagnostics = Diagnostics::new();
    diagnostics.extend(parse.diagnostics.clone());
    diagnostics.extend(sema_diags);
    Frontend {
        map,
        file,
        parse,
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
