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
//! Error handling (D7): each label used by [`Op::SetHandler`] gets a handler number from 1, in the order the
//! emitter meets them; `error_goto_line` holds the active one, and `mainerr.txt`, which `qbx.cpp` places at the top
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

// A new type or operator must be handled everywhere, not fall into a `_ =>` arm (study\21).
#![warn(clippy::wildcard_enum_match_arm)]

use qb64rust_base::to_u32;
use qb64rust_builtins::BuiltinId;
use qb64rust_ir::{
    Arg, BinOp, Body, Conv, Expr, ExprKind, Facts, LabelId, MemberId, OnError, Op, Place, PrintItem, Proc, ProcId,
    ProcKind, Program, Resume, Storage, Ty, UnOp, Var, VarId, When,
};
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

/// Emits the program. `source_name` goes into the `#line` directives.
pub fn emit(p: &Program, source_name: &str) -> Fragments {
    let mut e = Emitter {
        p,
        type_sizes: type_sizes(p),
        line_file: line_name(source_name),
        skip: 0,
        pass: 0,
        pass_decls: Vec::new(),
        handlers: Vec::new(),
        body: &p.main,
        k: 0,
        gosubs: 0,
        ret_cases: String::new(),
        returns: false,
    };
    let mut files: Vec<(String, String)> = FRAGMENTS.iter().map(|n| (n.to_string(), String::new())).collect();
    let mut set = |name: &str, text: String| files.iter_mut().find(|(n, _)| n == name).unwrap().1 = text;
    set("global.txt", e.global());
    set("regsf.txt", e.prototypes());
    set("clear.txt", e.per_program_var(|v, n| e.clear(v, n)));
    set(
        "mainfree.txt",
        e.per_program_var(|v, n| {
            if is_qbs(v.ty) && !v.is_array() {
                format!("qbs_free({n});\n")
            } else {
                String::new()
            }
        }),
    );
    let mut main = String::from("#include \"main0.txt\"\n");
    for k in 1..=p.procs.len() {
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
    let mut maindata = e.per_program_var(|v, n| e.alloc(v, n));
    maindata.push_str(&e.temps(None));
    for d in std::mem::take(&mut e.pass_decls) {
        maindata.push_str(&d);
    }
    set("maindata.txt", maindata);
    if let Some(ret) = e.take_ret(0) {
        files.push(("ret0.txt".into(), ret));
    }
    for (i, proc) in p.procs.iter().enumerate() {
        let k = i + 1;
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

pub fn c_type(t: Ty) -> &'static str {
    match t {
        Ty::I16 => "int16",
        Ty::I32 => "int32",
        Ty::I64 => "int64",
        Ty::F32 => "float",
        Ty::F64 => "double",
        Ty::F80 => "long double",
        Ty::Str => "qbs*",
        Ty::User(_) => unreachable!("no value has a user type"),
    }
}

/// Whether a variable of this type is a `qbs *` (freed, assigned with `qbs_set`, used without `*`); otherwise it
/// is a pointer to a C scalar or (a user type) to its bytes.
fn is_qbs(t: Ty) -> bool {
    match t {
        Ty::Str => true,
        Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 | Ty::User(_) => false,
    }
}

/// Storage size of a numeric type; a `_FLOAT` takes 32 bytes as in the old compiler (`study\02` §1.7, measured in
/// a `TYPE` with `LEN`, `verification\v18_h_type_members`).
fn size(t: Ty) -> u32 {
    match t {
        Ty::I16 => 2,
        Ty::I32 | Ty::F32 => 4,
        Ty::I64 | Ty::F64 => 8,
        Ty::F80 => 32,
        Ty::Str | Ty::User(_) => unreachable!("size of {t:?}: see `Emitter::ty_size`"),
    }
}

/// The size of each user type: its members' sizes added up, in order, without padding (measured, `LEN` of a type
/// with one member of each numeric type and a 6-byte type is 64). A member's type is defined earlier.
fn type_sizes(p: &Program) -> Vec<u32> {
    let mut sizes: Vec<u32> = Vec::with_capacity(p.types.len());
    for t in &p.types {
        let total = t
            .members
            .iter()
            .map(|m| match m.ty {
                Ty::User(id) => sizes[id.0 as usize],
                other @ (Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str) => size(other),
            })
            .sum();
        sizes.push(total);
    }
    sizes
}

/// The declaration of a variable's pointer: an array's descriptor, a user type's bytes, a scalar.
fn declare(v: &Var, n: &str) -> String {
    if v.is_array() {
        return format!("ptrszint *{n}=NULL;\n");
    }
    match v.ty {
        Ty::Str => format!("qbs *{n}=NULL;\n"),
        Ty::User(_) => format!("void *{n}=NULL;\n"),
        t @ (Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80) => format!("{} *{n}=NULL;\n", c_type(t)),
    }
}

/// The type part of a C variable name.
fn type_word(t: Ty) -> &'static str {
    match t {
        Ty::I16 => "INTEGER",
        Ty::I32 => "LONG",
        Ty::I64 => "INTEGER64",
        Ty::F32 => "SINGLE",
        Ty::F64 => "DOUBLE",
        Ty::F80 => "FLOAT",
        Ty::Str => "STRING",
        Ty::User(_) => "UDT",
    }
}

/// The descriptor slot of dimension `k` (0 for the first) of an array of `n` dimensions: libqb keeps the
/// dimensions in reverse order, four slots each from slot 4 (lower bound, count, multiplier, unused;
/// `study\02` §3.4).
fn dim_slot(n: usize, k: usize) -> usize {
    4 * (n - 1 - k) + 4
}

/// A BASIC name as part of a C identifier: a `.` becomes `__046__`.
fn c_ident(name: &str) -> String {
    name.replace('.', "__046__")
}

/// `SUB_<NAME>` or `FUNC_<NAME>`.
pub fn proc_name(proc: &Proc) -> String {
    let prefix = match proc.kind {
        ProcKind::Sub => "SUB",
        ProcKind::Function(_) => "FUNC",
    };
    format!("{prefix}_{}", c_ident(&proc.name))
}

/// The C++ label of a body's label: `LABEL_<NAME>` for a user label, `L_<n>` (its index) for one the lowering made.
fn label_name(b: &Body, l: LabelId) -> String {
    match &b.labels[l.0 as usize].name {
        Some(name) => format!("LABEL_{}", c_ident(name)),
        None => format!("L_{}", l.0),
    }
}

/// `__<TYPE>_<NAME>` for a global variable; `_<SUB_P>_<TYPE>_<NAME>` for any variable of procedure P;
/// `temp_<name>` for a temporary of the lowering (`for1.value` is `temp_for1_value`).
pub fn var_name(p: &Program, v: &Var) -> String {
    let scope = match v.storage {
        Storage::Global => String::new(),
        Storage::Static(q) | Storage::Local(q) | Storage::Param(q) | Storage::Result(q) => proc_name(p.proc(q)),
        Storage::Temp(_) => return format!("temp_{}", v.name.replace('.', "_")),
    };
    let array = if v.is_array() { "ARRAY_" } else { "" };
    format!("_{scope}_{array}{}_{}", type_word(v.ty), c_ident(&v.name))
}

/// Whether a variable is a plain C variable (a temporary), not a pointer.
fn is_plain(v: &Var) -> bool {
    match v.storage {
        Storage::Temp(_) => true,
        Storage::Global | Storage::Static(_) | Storage::Local(_) | Storage::Param(_) | Storage::Result(_) => false,
    }
}

/// The declaration of a temporary, zero; `static` in the main module.
fn declare_temp(v: &Var, n: &str) -> String {
    let static_ = match v.storage {
        Storage::Temp(None) => "static ",
        Storage::Temp(Some(_)) => "",
        Storage::Global | Storage::Static(_) | Storage::Local(_) | Storage::Param(_) | Storage::Result(_) => {
            unreachable!("not a temporary")
        }
    };
    assert!(!is_qbs(v.ty), "the lowering makes numeric temporaries only");
    format!("{static_}{} {n}=0;\n", c_type(v.ty))
}

/// A C string literal for arbitrary bytes: printable ASCII as is, everything else (and `"`, `\`, `?`) as a
/// three-digit octal escape, so no byte value can end an escape early.
fn c_string(bytes: &[u8]) -> String {
    let mut s = String::from("\"");
    for &b in bytes {
        if (0x20..0x7f).contains(&b) && !matches!(b, b'"' | b'\\' | b'?') {
            s.push(b as char);
        } else {
            write!(s, "\\{b:03o}").unwrap();
        }
    }
    s.push('"');
    s
}

/// The file name of a `#line` directive. Its string is unevaluated, where clang rejects numeric escapes (`\134`),
/// so only `\\` and `\"` are escaped; other bytes stay as they are (the name is UTF-8), control bytes become `_`.
fn line_name(name: &str) -> String {
    let mut s = String::from("\"");
    for c in name.chars() {
        match c {
            '\\' | '"' => {
                s.push('\\');
                s.push(c);
            }
            c if c.is_control() => s.push('_'),
            c => s.push(c),
        }
    }
    s.push('"');
    s
}

/// The prologue of a procedure, after its signature line (`study\02` §8, verbatim from the old compiler).
const PROLOGUE: &[&str] = &[
    "qbs *tqbs;",
    "ptrszint tmp_long;",
    "int32 tmp_fileno;",
    "uint32 qbs_tmp_base=qbs_tmp_list_nexti;",
    "uint8 *tmp_mem_static_pointer=mem_static_pointer;",
    "uint32 tmp_cmem_sp=cmem_sp;",
];

const PROLOGUE_AFTER_DATA: &[&str] = &[
    "mem_lock *sf_mem_lock;",
    "new_mem_lock();",
    "sf_mem_lock=mem_lock_tmp;",
    "sf_mem_lock->type=3;",
    "libqb_check_stack();",
    "if (is_error_pending()) goto exit_subfunc;",
];

const EPILOGUE_AFTER_FREE: &[&str] = &[
    "if ((tmp_mem_static_pointer>=mem_static)&&(tmp_mem_static_pointer<=mem_static_limit)) mem_static_pointer=tmp_mem_static_pointer; else mem_static_pointer=mem_static;",
    "cmem_sp=tmp_cmem_sp;",
];

struct Emitter<'a> {
    p: &'a Program,
    /// The size of each user type ([`type_sizes`]).
    type_sizes: Vec<u32>,
    line_file: String,
    /// Counter for the `skipN` labels of PRINT statements.
    skip: u32,
    /// Counter for the `passN` by-value temporaries.
    pass: u32,
    /// Declarations of the `passN` of the body being emitted.
    pass_decls: Vec<String>,
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
}

impl<'a> Emitter<'a> {
    /// `LABEL_<NAME>` of a main-module label.
    fn label_name(&self, l: LabelId) -> String {
        label_name(&self.p.main, l)
    }

    /// The C++ label of a label of the body being emitted.
    fn local_label(&self, l: LabelId) -> String {
        label_name(self.body, l)
    }

    /// The handler number of a label, assigned on first use.
    fn handler(&mut self, l: LabelId) -> usize {
        match self.handlers.iter().position(|&h| h == l) {
            Some(i) => i + 1,
            None => {
                self.handlers.push(l);
                self.handlers.len()
            }
        }
    }

    fn name(&self, id: VarId) -> String {
        var_name(self.p, self.p.var(id))
    }

    /// `f(var, name)` for every variable that lives as long as the program: global and `STATIC` ones.
    fn per_program_var(&self, f: impl Fn(&Var, &str) -> String) -> String {
        self.p
            .vars
            .iter()
            .filter(|v| match v.storage {
                Storage::Global | Storage::Static(_) => true,
                Storage::Local(_) | Storage::Param(_) | Storage::Result(_) | Storage::Temp(_) => false,
            })
            .map(|v| f(v, &var_name(self.p, v)))
            .collect()
    }

    fn global(&self) -> String {
        let mut out = String::from(
            "template <typename QBL, typename QBR> static inline auto qb_safe_idiv(QBL qb_l,QBR qb_r)->decltype(qb_l/qb_r){if (!qb_r){error(11);return (decltype(qb_l/qb_r))0;}return qb_l/qb_r;}\n\
             template <typename QBL, typename QBR> static inline auto qb_safe_mod(QBL qb_l,QBR qb_r)->decltype(qb_l%qb_r){if (!qb_r){error(11);return (decltype(qb_l%qb_r))0;}return qb_l%qb_r;}\n",
        );
        out.push_str(&self.per_program_var(declare));
        // Globals that qbx.cpp and libqb refer to.
        out.push_str(
            "int32 console=1;\nint32 screen_hide_startup=0;\nint32 asserts=0;\nint32 vwatch=0;\n\
             ptrszint data_size=0;\nuint8 *data=(uint8*)calloc(1,1);\n",
        );
        out
    }

    /// `int32 FUNC_F(int32*_FUNC_F_LONG_A,qbs*_FUNC_F_STRING_S)`
    fn signature(&self, proc: &Proc) -> String {
        let ret = match proc.kind {
            ProcKind::Sub => "void",
            ProcKind::Function(t) => c_type(t),
        };
        let params: Vec<String> = proc
            .params
            .iter()
            .map(|&id| {
                let v = self.p.var(id);
                let t = if is_qbs(v.ty) {
                    "qbs*"
                } else {
                    &format!("{}*", c_type(v.ty))
                };
                format!("{t}{}", self.name(id))
            })
            .collect();
        format!("{ret} {}({})", proc_name(proc), params.join(","))
    }

    fn prototypes(&self) -> String {
        self.p
            .procs
            .iter()
            .map(|q| format!("{};\n", self.signature(q)))
            .collect()
    }

    fn main0(&mut self) -> String {
        // Tells the runtime that this program tracks error locations itself (it does not: no `$ERRORLOCATION`),
        // so a critical error says "Enable $ErrorLocation:ON" as with the old compiler, instead of a line number
        // from the runtime's fallback for old executables (measured with `s11_on_error`, error 11). Runs again
        // each time a handler re-enters `QBMAIN`, as in the old compiler.
        let mut out = String::from("error_track_line(0,0,NULL);\nS_0:;\n");
        let p = self.p;
        self.body(&p.main, 0, &mut out);
        out.push_str("sub_end();\nreturn;\n}\n");
        out
    }

    /// The declarations of the temporaries of a body (`None`: the main module), in order of creation.
    fn temps(&self, owner: Option<ProcId>) -> String {
        self.p
            .vars
            .iter()
            .filter(|v| v.storage == Storage::Temp(owner))
            .map(|v| declare_temp(v, &var_name(self.p, v)))
            .collect()
    }

    /// `retK.txt` of the body just emitted, if it has a `RETURN` without label: pop the last return id and go back
    /// to its `GOSUB`, if that was made in this body; otherwise, or with no `GOSUB` pending, error 3. Id 0 (pushed by
    /// event dispatch, which this compiler does not have yet) returns from `QBMAIN` in the main module and is error 3
    /// in a procedure, as in the old compiler (`qb64pe.bas` 3311, 5433, 17588).
    fn take_ret(&mut self, k: usize) -> Option<String> {
        let cases = std::mem::take(&mut self.ret_cases);
        if !std::mem::take(&mut self.returns) {
            return None;
        }
        let zero = if k == 0 { "return;" } else { "error(3);" };
        Some(format!(
            "if (next_return_point){{\nnext_return_point--;\nswitch(return_point[next_return_point]){{\n\
             case 0:\n{zero}\nbreak;\n{cases}}}\n}}\nerror(3);\n"
        ))
    }

    /// `mainK.txt`, `dataK.txt` and `freeK.txt` of procedure `i` (K = i + 1).
    fn procedure(&mut self, i: usize, proc: &'a Proc, k: usize) -> (String, String, String) {
        let (mut data, mut free) = (String::new(), String::new());
        let mine = |s: Storage| match s {
            Storage::Local(q) | Storage::Result(q) => q.0 as usize == i,
            Storage::Global | Storage::Static(_) | Storage::Param(_) | Storage::Temp(_) => false,
        };
        // The result first, then locals in order of creation.
        let mut per_call: Vec<&Var> = proc.result.iter().map(|&r| self.p.var(r)).collect();
        per_call.extend(
            self.p
                .vars
                .iter()
                .filter(|v| matches!(v.storage, Storage::Local(_)) && mine(v.storage)),
        );
        for v in per_call {
            let n = var_name(self.p, v);
            data.push_str(&declare(v, &n));
            data.push_str(&self.alloc(v, &n));
            // The result is returned with `qbs_maketmp`, so the caller's cleanup frees it.
            if is_qbs(v.ty) && !matches!(v.storage, Storage::Result(_)) {
                writeln!(free, "qbs_free({n});").unwrap();
            }
        }
        // A string argument that is a temporary, fixed-length or read-only is replaced by a copy for the call;
        // a fixed-length one gets the copy's final value back.
        for (j, &id) in proc.params.iter().enumerate() {
            if !is_qbs(self.p.var(id).ty) {
                continue;
            }
            let (n, old) = (self.name(id), format!("oldstr{j}"));
            write!(
                data,
                "qbs*{old}=NULL;\n\
                 if({n}->tmp||{n}->fixed||{n}->readonly){{\n\
                 {old}={n};\n\
                 if ({old}->cmem_descriptor){{\n\
                 {n}=qbs_new_cmem({old}->len,0);\n\
                 }}else{{\n\
                 {n}=qbs_new({old}->len,0);\n\
                 }}\n\
                 memcpy({n}->chr,{old}->chr,{old}->len);\n\
                 }}\n"
            )
            .unwrap();
            write!(
                free,
                "if({old}){{\nif({old}->fixed)qbs_set({old},{n});\nqbs_free({n});\n}}\n"
            )
            .unwrap();
        }
        data.push_str(&self.temps(Some(ProcId(to_u32(i)))));

        let mut main = String::new();
        let mut header = vec![format!("{}{{", self.signature(proc))];
        header.extend(PROLOGUE.iter().map(|s| s.to_string()));
        header.push(format!("#include \"data{k}.txt\""));
        header.extend(PROLOGUE_AFTER_DATA.iter().map(|s| s.to_string()));
        self.lines(proc.line, &header, &mut main);
        self.body(&proc.body, k, &mut main);
        let mut footer = vec![
            "exit_subfunc:;".to_string(),
            "free_mem_lock(sf_mem_lock);".to_string(),
            format!("#include \"free{k}.txt\""),
        ];
        footer.extend(EPILOGUE_AFTER_FREE.iter().map(|s| s.to_string()));
        if let Some(r) = proc.result {
            let n = self.name(r);
            footer.push(if is_qbs(self.p.var(r).ty) {
                format!("qbs_maketmp({n});return {n};")
            } else {
                format!("return *{n};")
            });
        }
        footer.push("}".to_string());
        self.lines(proc.end_line, &footer, &mut main);
        for d in std::mem::take(&mut self.pass_decls) {
            data.push_str(&d);
        }
        (main, data, free)
    }

    /// Each line preceded by a `#line` directive for source line `line`.
    fn lines(&self, line: u32, lines: &[String], out: &mut String) {
        for l in lines {
            writeln!(out, "#line {line} {}\n{l}", self.line_file).unwrap();
        }
    }

    /// Body number `k` (0 for the main module, K for procedure K).
    fn body(&mut self, b: &'a Body, k: usize, out: &mut String) {
        self.body = b;
        self.k = k;
        for (i, s) in b.stmts.iter().enumerate() {
            self.labels(i, out);
            let mut body = Vec::new();
            // Whether an earlier operation of the statement may have left an error pending.
            let mut raised = false;
            for op in &s.ops {
                self.op(op, raised, &mut body);
                raised |= op.may_raise();
            }
            let mut lines = vec!["do{".to_string()];
            lines.extend(body);
            lines.push(format!("if(!qbevent)break;evnt({});}}while(r);", s.line));
            self.lines(s.line, &lines, out);
        }
        self.labels(b.stmts.len(), out);
    }

    /// The labels that stand before statement `at`; a user label with the old compiler's event check.
    fn labels(&self, at: usize, out: &mut String) {
        for (i, l) in self.body.labels.iter().enumerate().filter(|(_, l)| l.at == at) {
            let mut lines = vec![format!("{}:;", self.local_label(LabelId(to_u32(i))))];
            if l.name.is_some() {
                lines.push(format!("if(qbevent){{evnt({});r=0;}}", l.line));
            }
            self.lines(l.line, &lines, out);
        }
    }

    /// One operation. `raised`: an earlier operation of the statement may have raised, so a jump checks for a
    /// pending error (design D9).
    fn op(&mut self, op: &Op, raised: bool, out: &mut Vec<String>) {
        let unless_error = if raised { "if (!is_error_pending()) " } else { "" };
        match op {
            Op::SelectConsole => {
                out.push("sub__dest(func__console());".into());
                out.push("sub__source(func__console());".into());
            }
            Op::End => out.push("sub_end();".into()),
            Op::System => {
                out.push("if (sub_gl_called) error(271);".into());
                out.push("close_program=1;".into());
                out.push("end();".into());
            }
            Op::Exit => out.push("goto exit_subfunc;".into()),
            Op::SetHandler(Some(l)) => {
                let n = self.handler(*l);
                out.push(format!("error_goto_line={n};"));
            }
            Op::SetHandler(None) => {
                out.push("error_goto_line=0;".into());
                out.push("qbs_set(error_handler_history, qbs_new_txt_len(\"\", 0));".into());
            }
            Op::Raise(v) => {
                out.push(format!("error({});", self.value(v)));
                if v.uses_strings() {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
            Op::Resume(r) => {
                let then = match r {
                    Resume::Retry => "error_retry=1; qbevent=1; error_handling=0; error_err=0; return;".to_string(),
                    Resume::Next => "error_handling=0; error_err=0; return;".to_string(),
                    Resume::To(l) => format!("error_handling=0; error_err=0; goto {};", self.label_name(*l)),
                };
                out.push(format!("if (!error_handling){{error(20);}}else{{{then}}}"));
            }
            Op::Assign { place, value } => self.assign(place, value, out),
            Op::AssignAll(stores) => {
                for (place, value) in stores {
                    self.assign(&Place::Var(*place), value, out);
                }
            }
            Op::Jump(l) => out.push(format!("{unless_error}goto {};", self.local_label(*l))),
            Op::Branch {
                cond,
                when,
                to,
                on_error,
            } => {
                let mut c = self.value(cond);
                if cond.uses_strings() {
                    c = format!("qbs_cleanup(qbs_tmp_base,{c})");
                }
                let test = match when {
                    When::Zero => format!("!({c})"),
                    When::NonZero => format!("({c})"),
                };
                // The condition is evaluated first, so a raising condition never jumps.
                let test = match on_error {
                    OnError::Skip => format!("({test})&&(!is_error_pending())"),
                    OnError::UseValue => test,
                };
                out.push(format!("if ({test}) goto {};", self.local_label(*to)));
            }
            Op::Gosub(l) => {
                self.gosubs += 1;
                let g = self.gosubs;
                // With an error pending the jump is not taken and nothing is pushed.
                if raised {
                    out.push("if (!is_error_pending()){".into());
                }
                out.push(format!("return_point[next_return_point++]={g};"));
                out.push("if (next_return_point>=return_points) more_return_points();".into());
                out.push(format!("goto {};", self.local_label(*l)));
                if raised {
                    out.push("}".into());
                }
                out.push(format!("RETURN_{g}:;"));
                writeln!(self.ret_cases, "case {g}:\ngoto RETURN_{g};\nbreak;").unwrap();
            }
            Op::Return(None) => {
                self.returns = true;
                out.push(format!("#include \"ret{}.txt\"", self.k));
            }
            // The decrement is guarded, unlike the old compiler's, whose counter wrapped below zero (DIVERGENCES.md
            // D-003).
            Op::Return(Some(l)) => {
                out.push("if (!next_return_point) error(3); else next_return_point--;".into());
                out.push(format!("goto {};", self.label_name(*l)));
            }
            Op::Call { proc, args } => {
                let q = self.p.proc(*proc);
                let a = self.args(args);
                out.push(format!("{}({a});", proc_name(q)));
                let strings = args.iter().any(Arg::uses_strings) || q.params.iter().any(|&v| is_qbs(self.p.var(v).ty));
                if strings {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
            }
            Op::Print { items, newline } => {
                // The IR's raise rule for PRINT: a raising item skips the rest of the statement.
                self.skip += 1;
                let skip = format!("skip{}", self.skip);
                out.push("tqbs=qbs_new(0,0);".into());
                for item in items {
                    let text = match item {
                        PrintItem::Zone => {
                            out.push("tab();".into());
                            continue;
                        }
                        PrintItem::Str(v) => self.value(v),
                        PrintItem::Num(v) => {
                            format!(
                                "qbs_add(qbs_str(({})({})),qbs_new_txt(\" \"))",
                                c_type(v.ty),
                                self.value(v)
                            )
                        }
                    };
                    out.push(format!("qbs_set(tqbs,{text});"));
                    out.push(format!("if (is_error_pending()) goto {skip};"));
                    out.push("makefit(tqbs);".into());
                    out.push("qbs_print(tqbs,0);".into());
                }
                if *newline {
                    out.push("qbs_print(nothingstring,1);".into());
                }
                out.push(format!("{skip}:"));
                out.push("qbs_free(tqbs);".into());
                out.push("qbs_cleanup(qbs_tmp_base,0);".into());
            }
        }
    }

    /// A variable as a C expression: a `qbs*` or a temporary as it is, any other through its pointer.
    fn scalar(&self, id: VarId) -> String {
        let (v, name) = (self.p.var(id), self.name(id));
        if is_qbs(v.ty) || is_plain(v) {
            name
        } else {
            format!("*{name}")
        }
    }

    /// A store, by the rule of its place (the IR's error rule; `study\02` §2.3 for the old compiler's forms).
    fn assign(&mut self, place: &Place, value: &Expr, out: &mut Vec<String>) {
        let cleanup = value.uses_strings() || place.indexes().iter().any(|i| i.uses_strings());
        match place {
            Place::Var(id) => {
                let v = self.value(value);
                if is_qbs(value.ty) {
                    out.push(format!("qbs_set({},{v});", self.name(*id)));
                } else {
                    out.push(format!("{}={v};", self.scalar(*id)));
                }
            }
            // The index first; with an error pending after it, neither the value nor the store.
            Place::Element { array, index } => {
                let flat = self.flat_index(*array, index);
                out.push(format!("tmp_long={flat};"));
                let element = self.element_at(*array, "tmp_long");
                let v = self.value(value);
                if is_qbs(value.ty) {
                    out.push(format!("if (!is_error_pending()) qbs_set({element},{v});"));
                } else {
                    out.push(format!("if (!is_error_pending()) {element}={v};"));
                }
            }
            Place::Member { .. } if place.has_element() => self.store_member_of_element(place, value, out),
            Place::Member { .. } => {
                let v = self.value(value);
                out.push(format!("{}={v};", self.load_place(place)));
            }
        }
        if cleanup {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// A store into a member of an element (`a(i).m = v`): the value first, then the indexes, as the old compiler
    /// (measured, `verification\v18_h_member_index_order`); each index checked against its dimension. The store is
    /// skipped when an index is out of range or the indexes raised an error while none was pending before them;
    /// the old compiler writes the first element then (`DIVERGENCES.md` D-004). The block's own variables are in
    /// braces, so no `goto` crosses their initialisation.
    fn store_member_of_element(&mut self, place: &Place, value: &Expr, out: &mut Vec<String>) {
        let ty = self.p.place_ty(place);
        let (array, index, offset) = self.element_root(place);
        let n = self.p.var(array).dims.len();
        let name = self.name(array);
        let size = self.ty_size(self.p.var(array).ty);
        out.push("{".into());
        out.push(format!("{} mbr_value={};", c_type(ty), self.value(value)));
        out.push("int32 mbr_pending=is_error_pending();".into());
        out.push("int32 mbr_bad=0;".into());
        out.push("int64 mbr_k;".into());
        out.push("ptrszint mbr_flat=0;".into());
        for (k, i) in index.iter().enumerate() {
            let s = dim_slot(n, k);
            out.push(format!("mbr_k=({})-{name}[{s}];", self.value(i)));
            out.push(format!(
                "if ((uptrszint)mbr_k>=(uptrszint){name}[{}]){{error(9);mbr_bad=1;}}",
                s + 1
            ));
            if k == 0 {
                out.push("mbr_flat=mbr_k;".into());
            } else {
                out.push(format!("mbr_flat+=mbr_k*{name}[{}];", s + 2));
            }
        }
        out.push(format!(
            "if (!mbr_bad&&(mbr_pending||!is_error_pending())) *({}*)(((char*){name}[0])+(mbr_flat*{size}+{offset}))=mbr_value;",
            c_type(ty)
        ));
        out.push("}".into());
    }

    /// The element a member place is found in, its indexes, and the member's byte offset in the element.
    fn element_root<'v>(&self, place: &'v Place) -> (VarId, &'v [Expr], u32) {
        match place {
            Place::Element { array, index } => (*array, index, 0),
            Place::Member { base, member } => {
                let (array, index, offset) = self.element_root(base);
                (
                    array,
                    index,
                    offset + self.member_offset(self.p.place_ty(base), *member),
                )
            }
            Place::Var(_) => unreachable!("a member place without an element"),
        }
    }

    /// The size of a value of a type in memory: a number's, a user type's, a string's descriptor pointer.
    fn ty_size(&self, t: Ty) -> u32 {
        match t {
            Ty::User(id) => self.type_sizes[id.0 as usize],
            Ty::Str => 8,
            Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 => size(t),
        }
    }

    /// The byte offset of a member in a value of user type `base`: the sizes of the members before it.
    fn member_offset(&self, base: Ty, m: MemberId) -> u32 {
        let Ty::User(t) = base else {
            unreachable!("a member of {base:?}");
        };
        self.p.user_type(t).members[..m.0 as usize]
            .iter()
            .map(|x| self.ty_size(x.ty))
            .sum()
    }

    /// The flat position of an element, with libqb's `array_check` per dimension (error 9 outside the bounds; it
    /// then gives 0, the first position, as the old compiler reads), first dimension fastest.
    fn flat_index(&mut self, array: VarId, index: &[Expr]) -> String {
        let n = index.len();
        let name = self.name(array);
        let parts: Vec<String> = index
            .iter()
            .enumerate()
            .map(|(k, i)| {
                let s = dim_slot(n, k);
                let check = format!("array_check(({})-{name}[{s}],{name}[{}])", self.value(i), s + 1);
                if k == 0 {
                    check
                } else {
                    format!("{check}*{name}[{}]", s + 2)
                }
            })
            .collect();
        parts.join("+")
    }

    /// The element at flat position `flat` of a numeric or string array, as an lvalue.
    fn element_at(&self, array: VarId, flat: &str) -> String {
        let (v, name) = (self.p.var(array), self.name(array));
        if is_qbs(v.ty) {
            format!("(((qbs**)({name}[0]))[{flat}])")
        } else {
            format!("(({}*)({name}[0]))[{flat}]", c_type(v.ty))
        }
    }

    /// The address of a place of a user type, or of a member, as a `char*` expression.
    fn bytes_of(&mut self, place: &Place) -> String {
        match place {
            Place::Var(v) => format!("((char*){})", self.name(*v)),
            Place::Element { array, index } => {
                let flat = self.flat_index(*array, index);
                let size = self.ty_size(self.p.var(*array).ty);
                format!("(((char*){}[0])+(({flat})*{size}))", self.name(*array))
            }
            Place::Member { base, member } => {
                let offset = self.member_offset(self.p.place_ty(base), *member);
                format!("({}+{offset})", self.bytes_of(base))
            }
        }
    }

    /// A place as a C lvalue: a scalar as [`Self::scalar`], an element, a member through a typed pointer.
    fn load_place(&mut self, place: &Place) -> String {
        match place {
            Place::Var(id) => self.scalar(*id),
            Place::Element { array, index } => {
                let flat = self.flat_index(*array, index);
                self.element_at(*array, &flat)
            }
            Place::Member { .. } => {
                let c = c_type(self.p.place_ty(place));
                format!("*({c}*)({})", self.bytes_of(place))
            }
        }
    }

    /// A place passed by reference: a pointer to it, or a string's own `qbs*`; found in argument order, as the old
    /// compiler passes them (`study\02` §4.1).
    fn place_ref(&mut self, place: &Place) -> String {
        match place {
            Place::Var(id) => self.name(*id),
            Place::Element { array, index } => {
                let flat = self.flat_index(*array, index);
                let element = self.element_at(*array, &flat);
                if is_qbs(self.p.var(*array).ty) {
                    element
                } else {
                    format!("(&{element})")
                }
            }
            Place::Member { .. } => {
                let c = c_type(self.p.place_ty(place));
                format!("({c}*)(void*)({})", self.bytes_of(place))
            }
        }
    }

    /// The allocation of a variable, zero or empty; it runs once per lifetime of the pointer. A static array gets
    /// its descriptor and its zeroed elements (empty strings) as the old compiler builds them (`study\02` §3.4).
    fn alloc(&self, v: &Var, n: &str) -> String {
        if v.is_array() {
            return self.alloc_array(v, n);
        }
        match v.ty {
            Ty::Str => format!("if (!{n}){n}=qbs_new(0,0);\n"),
            Ty::User(_) => {
                let size = self.ty_size(v.ty);
                format!("if({n}==NULL){{\n{n}=(void*)mem_static_malloc({size});\nmemset({n},0,{size});\n}}\n")
            }
            t @ (Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80) => format!(
                "if({n}==NULL){{\n{n}=({c}*)mem_static_malloc({size});\n*{n}=0;\n}}\n",
                c = c_type(t),
                size = size(t)
            ),
        }
    }

    fn alloc_array(&self, v: &Var, n: &str) -> String {
        let dims = v.dims.len();
        let lock = 4 * dims + 4;
        let mut out = format!(
            "if (!{n}){{\n{n}=(ptrszint*)mem_static_malloc({}*ptrsz);\nnew_mem_lock();\nmem_lock_tmp->type=4;\n\
             ((ptrszint*){n})[{lock}]=(ptrszint)mem_lock_tmp;\n",
            lock + 1
        );
        for (k, &(lower, upper)) in v.dims.iter().enumerate() {
            let s = dim_slot(dims, k);
            writeln!(out, "{n}[{s}]={};", int_const(lower, Ty::I64)).unwrap();
            writeln!(out, "{n}[{}]=({})-{n}[{s}]+1;", s + 1, int_const(upper, Ty::I64)).unwrap();
            writeln!(out, "if ({n}[{}]<=0) error(5);", s + 1).unwrap();
            if k == 0 {
                writeln!(out, "{n}[{}]=1;", s + 2).unwrap();
            } else {
                let p = dim_slot(dims, k - 1);
                writeln!(out, "{n}[{}]={n}[{}]*{n}[{}];", s + 2, p + 2, p + 1).unwrap();
            }
        }
        let count = self.element_count(v, n);
        if is_qbs(v.ty) {
            write!(
                out,
                "{n}[0]=(ptrszint)mem_static_malloc({count}*ptrsz);\ntmp_long={count};\nwhile(tmp_long--){{\n\
                 ((qbs**)({n}[0]))[tmp_long]=qbs_new(0,0);\n}}\n"
            )
            .unwrap();
        } else {
            let bytes = format!("{count}*{}", self.ty_size(v.ty));
            write!(
                out,
                "{n}[0]=(ptrszint)mem_static_malloc({bytes});\nmemset((void*)({n}[0]),0,{bytes});\n"
            )
            .unwrap();
        }
        write!(out, "{n}[2]=1+2;\n}}\n").unwrap();
        out
    }

    /// The number of elements of an array as a C expression over its descriptor.
    fn element_count(&self, v: &Var, n: &str) -> String {
        let dims = v.dims.len();
        let counts: Vec<String> = (0..dims).map(|k| format!("{n}[{}]", dim_slot(dims, k) + 1)).collect();
        counts.join("*")
    }

    /// `clear.txt`: what `CLEAR` resets in a variable that lives as long as the program.
    fn clear(&self, v: &Var, n: &str) -> String {
        if v.is_array() {
            let count = self.element_count(v, n);
            return if is_qbs(v.ty) {
                format!("tmp_long={count};\nwhile(tmp_long--){{\n(((qbs**)({n}[0]))[tmp_long])->len=0;\n}}\n")
            } else {
                format!("memset((void*)({n}[0]),0,{count}*{});\n", self.ty_size(v.ty))
            };
        }
        match v.ty {
            Ty::Str => format!("{n}->len=0;\n"),
            Ty::User(_) => format!("memset((void*){n},0,{});\n", self.ty_size(v.ty)),
            Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 => format!("*{n}=0;\n"),
        }
    }

    /// Arguments of a procedure call: a place's own pointer, a string temporary as it is, or a numeric copy in a
    /// new `passN`.
    fn args(&mut self, args: &[Arg]) -> String {
        let parts: Vec<String> = args
            .iter()
            .map(|a| match a {
                Arg::Ref(place) => self.place_ref(place),
                Arg::Temp(v) if is_qbs(v.ty) => self.value(v),
                Arg::Temp(v) => {
                    self.pass += 1;
                    let n = format!("pass{}", self.pass);
                    self.pass_decls.push(format!("{} {n};\n", c_type(v.ty)));
                    format!("&({n}={})", self.value(v))
                }
            })
            .collect();
        parts.join(",")
    }

    fn value(&mut self, v: &Expr) -> String {
        match &v.kind {
            ExprKind::Int(i) => int_const(*i, v.ty),
            ExprKind::Float(t) => {
                let suffix = if v.ty == Ty::F80 { "L" } else { "" };
                if t.starts_with('-') {
                    format!("({t}{suffix})")
                } else {
                    format!("{t}{suffix}")
                }
            }
            ExprKind::Str(s) => format!("qbs_new_txt_len({},{})", c_string(s), s.len()),
            ExprKind::Load(place) => self.load_place(place),
            // libqb's functions take the descriptor, the dimension and the number of dimensions.
            ExprKind::Bound { upper, array, dim } => {
                let f = if *upper { "func_ubound" } else { "func_lbound" };
                let d = dim.as_deref().map_or("1".to_string(), |d| self.value(d));
                let n = self.p.var(*array).dims.len();
                format!("((int64){f}({},{d},{n}))", self.name(*array))
            }
            ExprKind::Convert { how, from } => {
                let x = self.value(from);
                match (how, v.ty) {
                    (Conv::RoundEven, Ty::I32) => format!("qbr_float_to_long({x})"),
                    (Conv::RoundEven, _) => format!("qbr({x})"),
                    _ => format!("(({})({x}))", c_type(v.ty)),
                }
            }
            ExprKind::Binary { op, lhs, rhs } => {
                let (a, b) = (self.value(lhs), self.value(rhs));
                // As the old compiler writes them (`study\02` §1.4); comparisons turn C's 1 into -1. `/` is followed
                // by a space: `*a/*b` would open a comment.
                match op {
                    BinOp::Add => format!("({a}+{b})"),
                    BinOp::Sub => format!("({a}-{b})"),
                    BinOp::Mul => format!("({a}*{b})"),
                    BinOp::Div => format!("({a}/ {b})"),
                    BinOp::Eq => format!("(-({a}=={b}))"),
                    BinOp::Ne => format!("(-({a}!={b}))"),
                    BinOp::Lt => format!("(-({a}<{b}))"),
                    BinOp::Gt => format!("(-({a}>{b}))"),
                    BinOp::Le => format!("(-({a}<={b}))"),
                    BinOp::Ge => format!("(-({a}>={b}))"),
                    BinOp::And => format!("({a}&{b})"),
                    BinOp::Or => format!("({a}|{b})"),
                    BinOp::Xor => format!("({a}^{b})"),
                    BinOp::Eqv => format!("(~({a}^{b}))"),
                    BinOp::Imp => format!("((~({a}))|{b})"),
                    BinOp::AndAlso => format!("(-({a}&&{b}))"),
                    BinOp::OrElse => format!("(-({a}||{b}))"),
                    BinOp::IDiv => format!("qb_safe_idiv({a},{b})"),
                    BinOp::Mod => format!("qb_safe_mod({a},{b})"),
                    BinOp::Pow => format!("pow2({a},{b})"),
                }
            }
            ExprKind::Unary { op, operand } => {
                let x = self.value(operand);
                match op {
                    UnOp::Neg => format!("(-({x}))"),
                    UnOp::Not => format!("(~({x}))"),
                    UnOp::Negate => format!("(-(!({x})))"),
                }
            }
            ExprKind::Concat(a, b) => format!("qbs_add({},{})", self.value(a), self.value(b)),
            ExprKind::StrCompare { op, lhs, rhs } => {
                let f = match op {
                    BinOp::Eq => "qbs_equal",
                    BinOp::Ne => "qbs_notequal",
                    BinOp::Lt => "qbs_lessthan",
                    BinOp::Gt => "qbs_greaterthan",
                    BinOp::Le => "qbs_lessorequal",
                    BinOp::Ge => "qbs_greaterorequal",
                    BinOp::Add
                    | BinOp::Sub
                    | BinOp::Mul
                    | BinOp::Div
                    | BinOp::And
                    | BinOp::Or
                    | BinOp::Xor
                    | BinOp::Eqv
                    | BinOp::Imp
                    | BinOp::AndAlso
                    | BinOp::OrElse
                    | BinOp::IDiv
                    | BinOp::Mod
                    | BinOp::Pow => unreachable!("a string comparison with {op:?}"),
                };
                format!("{f}({},{})", self.value(lhs), self.value(rhs))
            }
            ExprKind::Call { builtin: id, args } => self.call(*id, args),
            ExprKind::CallProc { proc, args } => {
                let a = self.args(args);
                format!("{}({a})", proc_name(self.p.proc(*proc)))
            }
        }
    }

    /// A built-in with optional slots gets a placeholder for each absent argument and, last, the `passed` mask:
    /// bit n set when the n-th optional slot is present (`study\02` §4.3).
    fn call(&mut self, id: BuiltinId, args: &[Option<Expr>]) -> String {
        let b = id.get();
        let optional = b.optional.unwrap_or(&[]);
        let mut parts = Vec::new();
        let mut mask = 0u32;
        let mut bit = 0;
        for (i, a) in args.iter().enumerate() {
            let is_optional = optional.get(i).copied().unwrap_or(false);
            match a {
                Some(a) => {
                    if is_optional {
                        mask |= 1 << bit;
                    }
                    parts.push(self.value(a));
                }
                None => parts.push("0".into()),
            }
            if is_optional {
                bit += 1;
            }
        }
        if optional.iter().any(|&o| o) {
            parts.push(mask.to_string());
        }
        format!("{}({})", b.callname, parts.join(","))
    }
}

/// An integer constant of a type; negative ones in parentheses, the minimum values spelled without overflow.
fn int_const(i: i64, ty: Ty) -> String {
    let text = match (ty, i) {
        (Ty::I64, i64::MIN) => "(-9223372036854775807ll-1)".to_string(),
        (Ty::I64, _) => format!("{i}ll"),
        (_, -2147483648) => "(-2147483647-1)".to_string(),
        _ => i.to_string(),
    };
    if i < 0 && !text.starts_with('(') {
        format!("({text})")
    } else {
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qb64rust_ir::ProcId;

    #[test]
    fn strings_escape_every_non_printable_byte() {
        assert_eq!(c_string(b"a\"\\?\x00\xC9"), "\"a\\042\\134\\077\\000\\311\"");
    }

    /// clang rejects numeric escapes in a `#line` file name; an absolute Windows path must still work.
    #[test]
    fn line_names_use_simple_escapes_only() {
        assert_eq!(
            line_name("C:\\does\\not\\exist\\p q.bas"),
            r#""C:\\does\\not\\exist\\p q.bas""#
        );
        assert_eq!(line_name("a\"b\u{1}\u{e9}.bas"), "\"a\\\"b_\u{e9}.bas\"");
    }

    #[test]
    fn int_constants() {
        assert_eq!(int_const(-5, Ty::I16), "(-5)");
        assert_eq!(int_const(i32::MIN as i64, Ty::I32), "(-2147483647-1)");
        assert_eq!(int_const(i64::MIN, Ty::I64), "(-9223372036854775807ll-1)");
        assert_eq!(int_const(7, Ty::I64), "7ll");
    }

    #[test]
    fn var_names() {
        let var = |name: &str, ty, storage| Var {
            name: name.into(),
            ty,
            storage,
            dims: Vec::new(),
        };
        let mut p = Program::default();
        for (name, kind) in [("BUMP", ProcKind::Sub), ("TWICE", ProcKind::Function(Ty::I32))] {
            p.procs.push(Proc {
                name: name.into(),
                kind,
                params: Vec::new(),
                result: None,
                body: Body::default(),
                line: 1,
                end_line: 1,
            });
        }
        assert_eq!(var_name(&p, &var("A", Ty::I32, Storage::Global)), "__LONG_A");
        assert_eq!(var_name(&p, &var("S", Ty::Str, Storage::Global)), "__STRING_S");
        assert_eq!(
            var_name(&p, &var("X", Ty::I32, Storage::Param(ProcId(0)))),
            "_SUB_BUMP_LONG_X"
        );
        assert_eq!(
            var_name(&p, &var("C", Ty::F32, Storage::Static(ProcId(0)))),
            "_SUB_BUMP_SINGLE_C"
        );
        assert_eq!(
            var_name(&p, &var("TWICE", Ty::I32, Storage::Result(ProcId(1)))),
            "_FUNC_TWICE_LONG_TWICE"
        );
        assert_eq!(
            var_name(&p, &var("A.B", Ty::I16, Storage::Local(ProcId(1)))),
            "_FUNC_TWICE_INTEGER_A__046__B"
        );
    }
}
