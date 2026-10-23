//! IR to the C++ fragments that the old runtime's `qbx.cpp` includes (design D7).
//!
//! Only the ABI must match the old compiler: the names `qbx.cpp` and libqb refer to, calling conventions, the
//! fragment set, the statement wrapper and the per-item error check of PRINT. Spelling and layout are free;
//! variable names follow the old scheme (`__LONG_A`, `_SUB_BUMP_LONG_X`) for readers of generated code and the M5
//! debugger.
//!
//! Storage classes (design D7 of `m2-procedures-and-errors`): global and `STATIC` variables are pointers declared
//! in `global.txt` and allocated once in `maindata.txt`; locals and FUNCTION results are pointers declared and
//! allocated in the procedure's `dataK.txt` on every call, from the `mem_static` arena that the epilogue rewinds;
//! a parameter is the C parameter itself. An [`Arg::Temp`] becomes `&(passN=<value>)` with `passN` declared in the
//! data fragment of the body that makes the call; a string copy passes the temporary `qbs*` directly.
//!
//! Error handling (D7): each label used by [`Op::SetHandler`] or [`Op::NewHandler`] gets a handler number from 1, in
//! the order the emitter meets them; `error_goto_line` holds the active one, `error_handler_history` the kept ones, and `mainerr.txt`, which `qbx.cpp` places at the top
//! of `QBMAIN`, jumps to it. The runtime runs a handler by calling `QBMAIN` again from the statement boundary
//! (`evnt`); the handler's resume returns from that call (retry and next) or jumps to a label in it.
//!
//! Control flow (design D9 of `m2-control-flow-slice`): a body's labels are C++ labels of its function, user labels
//! `LABEL_<NAME>` followed by the old compiler's event check, the lowering's `L_<n>` without one. A jump or branch
//! is a `goto` out of its statement's `do{…}while(r);`, which skips the statement's boundary, as in the old
//! compiler. `GOSUB` pushes a program-wide return id G on libqb's `return_point` stack and jumps; `RETURN_G:;`
//! inside the same statement is where `RETURN` comes back, through the switch in `retK.txt` of its body (K = 0 for
//! the main module), which the `RETURN` statement includes. The lowering's temporaries are plain C variables:
//! `static` in `maindata.txt` for the main module, per call in `dataK.txt`; none is declared inside a statement,
//! so no `goto` crosses an initialisation.

// A new type or operator must be handled everywhere, not fall into a `_ =>` arm (study\archive\21).
#![warn(clippy::wildcard_enum_match_arm)]

// The emitter by concern (design D6 of `m2-core-builtins`): each module adds the `Emitter` methods of its part.
mod arrays;
mod builtins;
mod coverage;
mod decl;
mod events;
mod io;
mod library;
mod mem;
mod names;
mod owner;
mod place;
mod procs;
mod stmt;
mod value;

pub use coverage::Coverage;
pub use library::{Linkage, Resolved, ResolvedLibrary, dll_handle, strip_version};
pub use names::{c_type, proc_name, var_name};

use decl::is_qbs;
use names::{c_string, line_name};
use qb64rust_base::FileId;
use qb64rust_ir::{Body, LabelId, Program, Shape};
use std::collections::HashMap;
use std::fmt::Write as _;

/// The version string `func__compvers` returns (measured from `qb64pe.exe` 4.7.0).
pub const COMPILER_VERSION: &str = "QB64-PE v4.7.0-GLFW-UNKNOWN";

/// Every fragment `qbx.cpp` includes, in a fixed order. Those the program has nothing for are written empty.
/// Each procedure K (from 1) adds `mainK.txt`, `dataK.txt` and `freeK.txt` after them.
pub const FRAGMENTS: &[&str] = &[
    "global.txt",
    "regsf.txt",
    "dyninfo.txt",
    "clear.txt",
    "inpchain.txt",
    "chain.txt",
    "onstrig.txt",
    "onkey.txt",
    "ontimer.txt",
    "maindata.txt",
    "mainerr.txt",
    "runline.txt",
    "ontimerj.txt",
    "onkeyj.txt",
    "onstrigj.txt",
    "main.txt",
    "main0.txt",
    "mainfree.txt",
];

/// The emitted fragments: (file name, contents), one per entry of [`FRAGMENTS`], then three per procedure, and a
/// `retK.txt` for each body that has a `RETURN` without label (`ret0.txt` for the main module).
pub struct Fragments {
    pub files: Vec<(String, String)>,
}

impl Fragments {
    pub fn get(&self, name: &str) -> &str {
        &self.files.iter().find(|(n, _)| n == name).unwrap().1
    }
}

/// Emits the program. `source_name` goes into the `#line` directives; `included` names the files the program
/// included, as shown in diagnostics (their statements get their own `#line` name, and their event checks report
/// the file, as the old compiler's do: `evnt(line, line in file, "file")`, measured `verification\
/// v19_include_runtime_error`: "Line: 3 (in raise.bi)"). `resolved` is what the driver found for the program's
/// `DECLARE LIBRARY` blocks.
pub fn emit(p: &Program, source_name: &str, included: &HashMap<FileId, String>, resolved: &Resolved) -> Fragments {
    emit_with(p, source_name, included, resolved, None, None)
}

/// [`emit`], with a counter at each source statement when `coverage` is given (design D1 of `cws-whole-game`,
/// `coverage.rs`). Without it the fragments are [`emit`]'s. `log_folder` (the working folder with its separator) is
/// put before each name of `source_name` and `included` that is relative, for the file a log statement names, as
/// the old compiler names it from its start folder (design D2 of `m4-handlers-logging-checking`); without it the
/// names are as given.
pub fn emit_with(
    p: &Program,
    source_name: &str,
    included: &HashMap<FileId, String>,
    resolved: &Resolved,
    coverage: Option<&Coverage>,
    log_folder: Option<&str>,
) -> Fragments {
    let log_name = |name: &str| {
        let full = match log_folder {
            Some(folder) if std::path::Path::new(name).is_relative() => format!("{folder}{name}"),
            _ => name.to_string(),
        };
        names::escape_string(full.as_bytes())
    };
    let log_files: HashMap<FileId, String> = included.iter().map(|(&f, name)| (f, log_name(name))).collect();
    let log_main = log_name(source_name);
    let included = included
        .iter()
        .map(|(&f, name)| {
            let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
            (f, (line_name(name), c_string(base.as_bytes())))
        })
        .collect();
    let mut e = Emitter {
        p,
        resolved,
        line_file: line_name(source_name),
        included,
        skip: 0,
        inside: Vec::new(),
        pass: 0,
        pass_decls: Vec::new(),
        byte_element: 0,
        handlers: Vec::new(),
        body: &p.main,
        k: 0,
        gosubs: 0,
        ret_cases: String::new(),
        returns: false,
        funcs: 0,
        func_decls: Vec::new(),
        file_types: Vec::new(),
        file_buffers: 0,
        stmt_at: (None, 0, None),
        checking: true,
        log_files,
        log_main,
        subfunc: None,
        on_timer: events::EventCode::default(),
        on_key: events::EventCode::default(),
        on_strig: events::EventCode::default(),
        coverage: coverage.map(coverage::Counters::new),
    };
    let mut files: Vec<(String, String)> = FRAGMENTS.iter().map(|n| (n.to_string(), String::new())).collect();
    let mut set = |name: &str, text: String| files.iter_mut().find(|(n, _)| n == name).unwrap().1 = text;
    set("global.txt", e.global() + &e.library_globals());
    // The window's resizing, which libqb reads from here (`qb64pe.bas` 13100–13106).
    let mut dyninfo = String::new();
    if p.settings.resize != qb64rust_ir::Resize::Off {
        dyninfo.push_str("ScreenResize=1;\n");
    }
    let scale = p.settings.resize.scale();
    if scale != 0 {
        writeln!(dyninfo, "ScreenResizeScale={scale};").unwrap();
    }
    set("dyninfo.txt", dyninfo);
    // The libraries' headers and prototypes first: a program procedure's prototype names no type of theirs.
    set("regsf.txt", e.library_prototypes() + &e.prototypes());
    set("clear.txt", e.per_program_var(|v, n| e.clear(v, n)));
    set(
        "mainfree.txt",
        e.per_program_var(|v, n| {
            if v.is_array() {
                String::new()
            } else if is_qbs(v.ty) {
                format!("qbs_free({n});\n")
            } else {
                // The strings and dynamic member arrays a variable of a user type holds.
                if let qb64rust_ir::Ty::User(_) = v.ty {
                    e.free_value(v.ty, &format!("(char*){n}"))
                } else {
                    String::new()
                }
            }
        }),
    );
    // Procedure K is the K-th program procedure; a library procedure has no fragments.
    let program_procs = p.procs.iter().filter(|q| q.external.is_none()).count();
    let mut main = String::from("#include \"main0.txt\"\n");
    for k in 1..=program_procs {
        writeln!(main, "#include \"main{k}.txt\"").unwrap();
    }
    write!(
        main,
        "qbs *func__compdate() {{\nreturn qbs_new_txt(__DATE__);\n}}\n\
         qbs *func__comptime() {{\nreturn qbs_new_txt(__TIME__);\n}}\n\
         qbs *func__compvers() {{\nreturn qbs_new_txt(\"{COMPILER_VERSION}\");\n}}\n"
    )
    .unwrap();
    set("main.txt", main);
    // main0 first: it collects the `passN` declarations that go into maindata.txt.
    let main0 = e.main0();
    set("main0.txt", main0);
    // The libraries' start first, as the old compiler's: a `DYNAMIC` library's errors come before anything else.
    let mut maindata = e.library_start();
    maindata += &e.per_program_var(|v, n| {
        let mut text = e.alloc(v, n);
        if let Shape::Dynamic {
            at_start: Some(bounds), ..
        } = &v.shape
        {
            text.push_str(&e.allocate_at_start(v, n, bounds));
        }
        text
    });
    // The allocations other declarations add (`STATIC a(n)` beside a shared array), after every array's own.
    for (id, bounds) in &p.start_allocations {
        let v = p.var(*id);
        maindata.push_str(&e.allocate_at_start(v, &names::var_name(p, v), bounds));
    }
    maindata.push_str(&e.temps(None));
    for d in std::mem::take(&mut e.pass_decls) {
        maindata.push_str(&d);
    }
    set("maindata.txt", maindata);
    if let Some(ret) = e.take_ret(0) {
        files.push(("ret0.txt".into(), ret));
    }
    let program_procs = p.procs.iter().enumerate().filter(|(_, q)| q.external.is_none());
    for (k, (i, proc)) in (1..).zip(program_procs) {
        let (main_k, data_k, free_k) = e.procedure(i, proc, k);
        files.push((format!("main{k}.txt"), main_k));
        files.push((format!("data{k}.txt"), data_k));
        files.push((format!("free{k}.txt"), free_k));
        if let Some(ret) = e.take_ret(k) {
            files.push((format!("ret{k}.txt"), ret));
        }
    }
    // Last: the handlers are numbered while the bodies are emitted.
    let mut mainerr = String::from(
        "if (!error_handler_history) error_handler_history = qbs_new(0, 0);\nif (error_occurred){ error_occurred=0;\n",
    );
    for (i, &l) in e.handlers.iter().enumerate() {
        writeln!(
            mainerr,
            "if (error_goto_line=={}){{error_handling=1; goto {};}}",
            i + 1,
            e.label_name(l)
        )
        .unwrap();
    }
    mainerr.push_str("exit(99);\n}\n");
    files.iter_mut().find(|(n, _)| n == "mainerr.txt").unwrap().1 = mainerr;
    // Also last: the handlers of events are numbered while the bodies are emitted.
    for (name, text) in [
        ("ontimer.txt", std::mem::take(&mut e.on_timer.cases)),
        ("ontimerj.txt", std::mem::take(&mut e.on_timer.jumpers)),
        ("onkey.txt", std::mem::take(&mut e.on_key.cases)),
        ("onkeyj.txt", std::mem::take(&mut e.on_key.jumpers)),
        ("onstrig.txt", std::mem::take(&mut e.on_strig.cases)),
        ("onstrigj.txt", std::mem::take(&mut e.on_strig.jumpers)),
    ] {
        files.iter_mut().find(|(n, _)| n == name).unwrap().1 = text;
    }
    // Also last: the temporaries of blocks are counted while the bodies are emitted.
    let global = &mut files.iter_mut().find(|(n, _)| n == "global.txt").unwrap().1;
    global.extend(e.func_decls.drain(..));
    // Also last: the statements are numbered while the bodies are emitted.
    if let Some(c) = &e.coverage {
        global.push_str(&c.table());
    }
    Fragments { files }
}

/// Text form for snapshots: each non-empty fragment under a header, then the list of empty ones.
pub fn dump(f: &Fragments) -> String {
    let mut out = String::new();
    let mut empty = Vec::new();
    for (name, text) in &f.files {
        if text.is_empty() {
            empty.push(name.as_str());
        } else {
            writeln!(out, "==> {name} <==").unwrap();
            out.push_str(text);
        }
    }
    writeln!(out, "==> empty: {}", empty.join(" ")).unwrap();
    out
}

struct Emitter<'a> {
    p: &'a Program,
    /// The files of the program's libraries (`library.rs`).
    resolved: &'a Resolved,
    line_file: String,
    /// Per included file: its `#line` name and its base name as a C string (for `evnt`).
    included: HashMap<FileId, (String, String)>,
    /// Counter for the `skipN` labels of PRINT statements.
    skip: u32,
    /// The allocations of the op of items being written, each by the part it runs before (`io.rs`,
    /// [`Emitter::allocs_before`]).
    inside: Vec<(usize, qb64rust_ir::Allocation)>,
    /// Counter for the `passN` by-value temporaries.
    pass: u32,
    /// Declarations of the `passN` of the body being emitted.
    pass_decls: Vec<String>,
    /// Counter for the `byte_element_N` temporaries of graphics `GET` and `PUT`.
    byte_element: u32,
    /// Handler labels; handler number N is entry N - 1.
    handlers: Vec<LabelId>,
    /// The body being emitted, whose labels the jumps name, and its number K (0 for the main module).
    body: &'a Body,
    k: usize,
    /// Return ids of `GOSUB`s emitted so far in the program; the next one is this plus 1.
    gosubs: u32,
    /// The `case`s of `retK.txt` of the body being emitted, one per `GOSUB` in it.
    ret_cases: String,
    /// Whether the body being emitted has a `RETURN` without label (which includes its `retK.txt`).
    returns: bool,
    /// Counter for the `funcN` temporaries a block of a built-in is held in, and their declarations, which go into
    /// `global.txt` (design D2 of `m3-mem`).
    funcs: u32,
    func_decls: Vec<String>,
    /// The owner types whose file functions are in `func_decls` (`Emitter::owner_file`).
    file_types: Vec<qb64rust_ir::TypeId>,
    /// Counter for the buffers of those file statements, one per statement (`qbr_fbufN`).
    file_buffers: u32,
    /// The file, line and main-file line of the statement being emitted, for an event check inside it (`END n`).
    stmt_at: (Option<FileId>, u32, Option<u32>),
    /// The statement being emitted is checked ([`qb64rust_ir::Stmt::checked`]): false below `$CHECKING:OFF`, where an
    /// index has no `array_check` and the memory statements are written unchecked (design D4 of
    /// `m4-handlers-logging-checking`). True outside a body.
    checking: bool,
    /// The file each included file is to a log statement, as a C string ([`emit_with`]), and the main file.
    log_files: HashMap<FileId, String>,
    log_main: String,
    /// The old compiler's C name of the procedure whose body is being emitted ([`names::old_proc_name`]); `None` in
    /// the main module.
    subfunc: Option<String>,
    /// The handlers of `ON TIMER` and of `ON KEY` (`events.rs`).
    on_timer: events::EventCode,
    on_key: events::EventCode,
    on_strig: events::EventCode,
    /// The counters of a coverage build (`coverage.rs`), `None` in any other.
    coverage: Option<coverage::Counters<'a>>,
}
