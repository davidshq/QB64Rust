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

// A new type or operator must be handled everywhere, not fall into a `_ =>` arm (study\21).
#![warn(clippy::wildcard_enum_match_arm)]

use qb64rust_builtins::BuiltinId;
use qb64rust_ir::{
    Arg, BinOp, Body, Const, Conv, LabelId, Op, PrintItem, Proc, ProcKind, Program, Resume, Storage, Ty, Value,
    ValueKind, Var, VarId,
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

/// The emitted fragments: (file name, contents), one per entry of [`FRAGMENTS`], then three per procedure.
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
        line_file: c_string(source_name.as_bytes()),
        skip: 0,
        pass: 0,
        pass_decls: Vec::new(),
        handlers: Vec::new(),
    };
    let mut files: Vec<(String, String)> = FRAGMENTS.iter().map(|n| (n.to_string(), String::new())).collect();
    let mut set = |name: &str, text: String| files.iter_mut().find(|(n, _)| n == name).unwrap().1 = text;
    set("global.txt", e.global());
    set("regsf.txt", e.prototypes());
    set(
        "clear.txt",
        e.per_program_var(|v, n| match v.ty {
            Ty::Str => format!("{n}->len=0;\n"),
            Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 => format!("*{n}=0;\n"),
        }),
    );
    set(
        "mainfree.txt",
        e.per_program_var(|v, n| {
            if is_qbs(v.ty) {
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
    let mut maindata = e.per_program_var(alloc);
    for d in std::mem::take(&mut e.pass_decls) {
        maindata.push_str(&d);
    }
    set("maindata.txt", maindata);
    for (i, proc) in p.procs.iter().enumerate() {
        let k = i + 1;
        let (main_k, data_k, free_k) = e.procedure(i, proc, k);
        files.push((format!("main{k}.txt"), main_k));
        files.push((format!("data{k}.txt"), data_k));
        files.push((format!("free{k}.txt"), free_k));
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
    }
}

/// Whether a variable of this type is a `qbs *` (freed, assigned with `qbs_set`, used without `*`); otherwise it
/// is a pointer to a C scalar.
fn is_qbs(t: Ty) -> bool {
    match t {
        Ty::Str => true,
        Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 => false,
    }
}

/// Storage size; a `_FLOAT` takes 32 bytes as in the old compiler (`study\02` §1.7).
fn size(t: Ty) -> u32 {
    match t {
        Ty::I16 => 2,
        Ty::I32 | Ty::F32 => 4,
        Ty::I64 | Ty::F64 => 8,
        Ty::F80 => 32,
        Ty::Str => unreachable!(),
    }
}

/// The declaration of a variable's pointer.
fn declare(v: &Var, n: &str) -> String {
    match v.ty {
        Ty::Str => format!("qbs *{n}=NULL;\n"),
        t @ (Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80) => format!("{} *{n}=NULL;\n", c_type(t)),
    }
}

/// The allocation of a variable, zero or empty; it runs once per lifetime of the pointer.
fn alloc(v: &Var, n: &str) -> String {
    match v.ty {
        Ty::Str => format!("if (!{n}){n}=qbs_new(0,0);\n"),
        t @ (Ty::I16 | Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80) => format!(
            "if({n}==NULL){{\n{n}=({c}*)mem_static_malloc({size});\n*{n}=0;\n}}\n",
            c = c_type(t),
            size = size(t)
        ),
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
    }
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

/// `__<TYPE>_<NAME>` for a global variable; `_<SUB_P>_<TYPE>_<NAME>` for any variable of procedure P.
pub fn var_name(p: &Program, v: &Var) -> String {
    let scope = match v.storage {
        Storage::Global => String::new(),
        Storage::Static(q) | Storage::Local(q) | Storage::Param(q) | Storage::Result(q) => proc_name(p.proc(q)),
    };
    format!("_{scope}_{}_{}", type_word(v.ty), c_ident(&v.name))
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
    line_file: String,
    /// Counter for the `skipN` labels of PRINT statements.
    skip: u32,
    /// Counter for the `passN` by-value temporaries.
    pass: u32,
    /// Declarations of the `passN` of the body being emitted.
    pass_decls: Vec<String>,
    /// Handler labels; handler number N is entry N - 1.
    handlers: Vec<LabelId>,
}

impl Emitter<'_> {
    /// `LABEL_<NAME>` of a main-module label.
    fn label_name(&self, l: LabelId) -> String {
        format!("LABEL_{}", c_ident(&self.p.main.labels[l.0 as usize].name))
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
                Storage::Local(_) | Storage::Param(_) | Storage::Result(_) => false,
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
        self.body(&p.main, &mut out);
        out.push_str("sub_end();\nreturn;\n}\n");
        out
    }

    /// `mainK.txt`, `dataK.txt` and `freeK.txt` of procedure `i` (K = i + 1).
    fn procedure(&mut self, i: usize, proc: &Proc, k: usize) -> (String, String, String) {
        let (mut data, mut free) = (String::new(), String::new());
        let mine = |s: Storage| match s {
            Storage::Local(q) | Storage::Result(q) => q.0 as usize == i,
            Storage::Global | Storage::Static(_) | Storage::Param(_) => false,
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
            data.push_str(&alloc(v, &n));
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

        let mut main = String::new();
        let mut header = vec![format!("{}{{", self.signature(proc))];
        header.extend(PROLOGUE.iter().map(|s| s.to_string()));
        header.push(format!("#include \"data{k}.txt\""));
        header.extend(PROLOGUE_AFTER_DATA.iter().map(|s| s.to_string()));
        self.lines(proc.line, &header, &mut main);
        self.body(&proc.body, &mut main);
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

    fn body(&mut self, b: &Body, out: &mut String) {
        for (i, s) in b.stmts.iter().enumerate() {
            self.labels(b, i, out);
            let mut body = Vec::new();
            for op in &s.ops {
                self.op(op, &mut body);
            }
            let mut lines = vec!["do{".to_string()];
            lines.extend(body);
            lines.push(format!("if(!qbevent)break;evnt({});}}while(r);", s.line));
            self.lines(s.line, &lines, out);
        }
        self.labels(b, b.stmts.len(), out);
    }

    /// The labels that stand before statement `at`, each with the old compiler's event check.
    fn labels(&self, b: &Body, at: usize, out: &mut String) {
        for l in b.labels.iter().filter(|l| l.at == at) {
            let lines = [
                format!("LABEL_{}:;", c_ident(&l.name)),
                format!("if(qbevent){{evnt({});r=0;}}", l.line),
            ];
            self.lines(l.line, &lines, out);
        }
    }

    fn op(&mut self, op: &Op, out: &mut Vec<String>) {
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
            Op::Assign { place, value } => {
                let name = self.name(*place);
                let v = self.value(value);
                if is_qbs(value.ty) {
                    out.push(format!("qbs_set({name},{v});"));
                } else {
                    out.push(format!("*{name}={v};"));
                }
                if value.uses_strings() {
                    out.push("qbs_cleanup(qbs_tmp_base,0);".into());
                }
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

    /// Arguments of a procedure call: a variable's own pointer, a string temporary as it is, or a numeric copy in
    /// a new `passN`.
    fn args(&mut self, args: &[Arg]) -> String {
        let parts: Vec<String> = args
            .iter()
            .map(|a| match a {
                Arg::Ref(v) => self.name(*v),
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

    fn value(&mut self, v: &Value) -> String {
        match &v.kind {
            ValueKind::Const(Const::Int(i)) => int_const(*i, v.ty),
            ValueKind::Const(Const::Float(t)) => {
                let suffix = if v.ty == Ty::F80 { "L" } else { "" };
                if t.starts_with('-') {
                    format!("({t}{suffix})")
                } else {
                    format!("{t}{suffix}")
                }
            }
            ValueKind::Const(Const::Str(s)) => format!("qbs_new_txt_len({},{})", c_string(s), s.len()),
            ValueKind::Var(id) => {
                let name = self.name(*id);
                if is_qbs(self.p.var(*id).ty) {
                    name
                } else {
                    format!("*{name}")
                }
            }
            ValueKind::Convert { how, from } => {
                let x = self.value(from);
                match (how, v.ty) {
                    (Conv::RoundEven, Ty::I32) => format!("qbr_float_to_long({x})"),
                    (Conv::RoundEven, _) => format!("qbr({x})"),
                    _ => format!("(({})({x}))", c_type(v.ty)),
                }
            }
            ValueKind::Binary { op, lhs, rhs } => {
                let o = match op {
                    BinOp::Add => "+",
                    BinOp::Sub => "-",
                    BinOp::Mul => "*",
                    BinOp::Div => "/",
                };
                format!("({}{o}{})", self.value(lhs), self.value(rhs))
            }
            ValueKind::Neg(x) => format!("(-({}))", self.value(x)),
            ValueKind::Concat(a, b) => format!("qbs_add({},{})", self.value(a), self.value(b)),
            ValueKind::CallBuiltin { id, args } => self.call(*id, args),
            ValueKind::CallProc { proc, args } => {
                let a = self.args(args);
                format!("{}({a})", proc_name(self.p.proc(*proc)))
            }
        }
    }

    /// A built-in with optional slots gets a placeholder for each absent argument and, last, the `passed` mask:
    /// bit n set when the n-th optional slot is present (`study\02` §4.3).
    fn call(&mut self, id: BuiltinId, args: &[Option<Value>]) -> String {
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
