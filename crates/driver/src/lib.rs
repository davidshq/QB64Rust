//! The compiler pipeline as a library, so that tests can run each stage; `main.rs` is the command line.

pub mod build;

use qb64rust_base::{Diagnostics, FileId, SourceMap};
use qb64rust_sema::Program;
use qb64rust_syntax::{NoLoader, ParsedProgram, parse};

/// Stack size of the thread the compiler runs on. Every stage walks the tree recursively; the parser's limits
/// (blocks nested 200 deep, expressions 1,000 levels) keep that bounded, and this leaves room for both at once
/// in a debug build (a debug build needs about 4 KiB per expression level; the main thread has 1 MiB on Windows).
pub const STACK_SIZE: usize = 64 * 1024 * 1024;

/// Runs `f` on a new thread with [`STACK_SIZE`] of stack and returns its result. A panic in `f` is passed on.
pub fn with_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let thread = std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(f)
        .expect("cannot start the compiler thread");
    match thread.join() {
        Ok(v) => v,
        Err(e) => std::panic::resume_unwind(e),
    }
}

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

/// Adds a "not supported yet" error for each statement the front end accepts but the back end cannot handle yet.
/// Called before [`lower`] and [`dump_ir`] (`to_cpp` false), or before [`emit`] and [`dump_cpp`] (`to_cpp` true),
/// and only then: `--dump typed` and the language server see the front end's result alone. Everything lowers to
/// the IR; the C++ emitter does not handle jumps yet ([`not_emitted`]).
pub fn check_backend(fe: &mut Frontend, to_cpp: bool) {
    if to_cpp && !fe.has_errors() {
        let gaps = not_emitted(&fe.program);
        fe.diagnostics.extend(gaps);
    }
}

/// The statements whose IR the C++ emitter cannot write yet: those that lower to jumps, branches, `GOSUB`,
/// `RETURN` or temporaries (task 7 of `m2-control-flow-slice`). Only the bodies' own statements are looked at: a
/// block is reported as a whole.
fn not_emitted(p: &Program) -> Diagnostics {
    use qb64rust_sema::StmtKind;
    let mut diags = Diagnostics::new();
    for s in p.stmts.iter().chain(p.procs.iter().flat_map(|q| &q.stmts)) {
        let what = match s.kind {
            StmtKind::Goto(_) => "`GOTO`",
            StmtKind::Gosub(_) => "`GOSUB`",
            StmtKind::Return(_) => "`RETURN`",
            StmtKind::If { .. } => "`IF`",
            StmtKind::For { .. } => "`FOR`",
            StmtKind::Do { .. } => "`DO`",
            StmtKind::While { .. } => "`WHILE`",
            // Only inside a loop, which is reported itself.
            StmtKind::ExitLoop(_) => "`EXIT`",
            StmtKind::ConsoleOnly
            | StmtKind::Assign { .. }
            | StmtKind::Print { .. }
            | StmtKind::End
            | StmtKind::System
            | StmtKind::Call { .. }
            | StmtKind::Exit
            | StmtKind::OnError(_)
            | StmtKind::Resume(_)
            | StmtKind::Error(_)
            | StmtKind::Label(_) => continue,
        };
        diags.unsupported(s.span, format!("{what} in code generation"));
    }
    diags
}

/// The IR of a checked program, after [`check_backend`] found nothing.
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
