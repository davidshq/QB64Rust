//! IR to the C++ fragments that the old runtime's `qbx.cpp` includes (design D7).
//!
//! Only the ABI must match the old compiler: the names `qbx.cpp` and libqb refer to, calling conventions, the
//! fragment set, the statement wrapper and the per-item error check of PRINT. Spelling and layout are free;
//! variable names follow the old scheme (`__LONG_A`) for readers of generated code and the M5 debugger.

use qb64rust_builtins::BuiltinId;
use qb64rust_ir::{BinOp, Const, Conv, Op, PrintItem, Program, Ty, Value, ValueKind, Var};
use std::fmt::Write as _;

/// The version string `func__compvers` returns (measured from `qb64pe.exe` 4.7.0).
pub const COMPILER_VERSION: &str = "QB64-PE v4.7.0-GLFW-UNKNOWN";

/// Every fragment `qbx.cpp` includes, in a fixed order. Those the slice has nothing for are written empty.
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

/// The emitted fragments: (file name, contents), one per entry of [`FRAGMENTS`].
pub struct Fragments {
    pub files: Vec<(&'static str, String)>,
}

impl Fragments {
    pub fn get(&self, name: &str) -> &str {
        &self.files.iter().find(|(n, _)| *n == name).unwrap().1
    }
}

/// Emits the program. `source_name` goes into the `#line` directives.
pub fn emit(p: &Program, source_name: &str) -> Fragments {
    let mut e = Emitter {
        p,
        line_file: c_string(source_name.as_bytes()),
        skip: 0,
    };
    let mut files: Vec<(&'static str, String)> = FRAGMENTS.iter().map(|n| (*n, String::new())).collect();
    let mut set = |name: &str, text: String| files.iter_mut().find(|(n, _)| *n == name).unwrap().1 = text;
    set("global.txt", e.global());
    set(
        "maindata.txt",
        e.per_var(|v, n| match v.ty {
            Ty::Str => format!("if (!{n}){n}=qbs_new(0,0);\n"),
            t => format!(
                "if({n}==NULL){{\n{n}=({c}*)mem_static_malloc({size});\n*{n}=0;\n}}\n",
                c = c_type(t),
                size = size(t)
            ),
        }),
    );
    set(
        "clear.txt",
        e.per_var(|v, n| match v.ty {
            Ty::Str => format!("{n}->len=0;\n"),
            _ => format!("*{n}=0;\n"),
        }),
    );
    set(
        "mainfree.txt",
        e.per_var(|v, n| {
            if v.ty == Ty::Str {
                format!("qbs_free({n});\n")
            } else {
                String::new()
            }
        }),
    );
    set(
        "mainerr.txt",
        "if (!error_handler_history) error_handler_history = qbs_new(0, 0);\n\
         if (error_occurred){ error_occurred=0;\nexit(99);\n}\n"
            .to_string(),
    );
    set(
        "main.txt",
        format!(
            "#include \"main0.txt\"\n\
             qbs *func__compdate() {{\nreturn qbs_new_txt(__DATE__);\n}}\n\
             qbs *func__comptime() {{\nreturn qbs_new_txt(__TIME__);\n}}\n\
             qbs *func__compvers() {{\nreturn qbs_new_txt(\"{COMPILER_VERSION}\");\n}}\n"
        ),
    );
    set("main0.txt", e.main0());
    Fragments { files }
}

/// Text form for snapshots: each non-empty fragment under a header, then the list of empty ones.
pub fn dump(f: &Fragments) -> String {
    let mut out = String::new();
    let mut empty = Vec::new();
    for (name, text) in &f.files {
        if text.is_empty() {
            empty.push(*name);
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

/// `__<TYPE>_<NAME>`; a `.` in a name becomes `__046__`.
pub fn var_name(v: &Var) -> String {
    let t = match v.ty {
        Ty::I16 => "INTEGER",
        Ty::I32 => "LONG",
        Ty::I64 => "INTEGER64",
        Ty::F32 => "SINGLE",
        Ty::F64 => "DOUBLE",
        Ty::F80 => "FLOAT",
        Ty::Str => "STRING",
    };
    format!("__{t}_{}", v.name.replace('.', "__046__"))
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

struct Emitter<'a> {
    p: &'a Program,
    line_file: String,
    /// Counter for the `skipN` labels of PRINT statements.
    skip: u32,
}

impl Emitter<'_> {
    fn per_var(&self, f: impl Fn(&Var, &str) -> String) -> String {
        self.p.vars.iter().map(|v| f(v, &var_name(v))).collect()
    }

    fn global(&self) -> String {
        let mut out = String::from(
            "template <typename QBL, typename QBR> static inline auto qb_safe_idiv(QBL qb_l,QBR qb_r)->decltype(qb_l/qb_r){if (!qb_r){error(11);return (decltype(qb_l/qb_r))0;}return qb_l/qb_r;}\n\
             template <typename QBL, typename QBR> static inline auto qb_safe_mod(QBL qb_l,QBR qb_r)->decltype(qb_l%qb_r){if (!qb_r){error(11);return (decltype(qb_l%qb_r))0;}return qb_l%qb_r;}\n",
        );
        for v in &self.p.vars {
            match v.ty {
                Ty::Str => writeln!(out, "qbs *{}=NULL;", var_name(v)).unwrap(),
                t => writeln!(out, "{} *{}=NULL;", c_type(t), var_name(v)).unwrap(),
            }
        }
        // Globals that qbx.cpp and libqb refer to.
        out.push_str(
            "int32 console=1;\nint32 screen_hide_startup=0;\nint32 asserts=0;\nint32 vwatch=0;\n\
             ptrszint data_size=0;\nuint8 *data=(uint8*)calloc(1,1);\n",
        );
        out
    }

    fn main0(&mut self) -> String {
        let mut out = String::from("S_0:;\n");
        for s in &self.p.main {
            let mut body = Vec::new();
            for op in &s.ops {
                self.op(op, &mut body);
            }
            let mut lines = vec!["do{".to_string()];
            lines.extend(body);
            lines.push(format!("if(!qbevent)break;evnt({});}}while(r);", s.line));
            for l in lines {
                writeln!(out, "#line {} {}\n{l}", s.line, self.line_file).unwrap();
            }
        }
        out.push_str("sub_end();\nreturn;\n}\n");
        out
    }

    fn op(&mut self, op: &Op, out: &mut Vec<String>) {
        match op {
            Op::SelectConsole => {
                out.push("sub__dest(func__console());".into());
                out.push("sub__source(func__console());".into());
            }
            Op::End => out.push("sub_end();".into()),
            Op::Assign { place, value } => {
                let name = var_name(self.p.var(*place));
                let v = self.value(value);
                if value.ty == Ty::Str {
                    out.push(format!("qbs_set({name},{v});"));
                } else {
                    out.push(format!("*{name}={v};"));
                }
                if value.uses_strings() {
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

    fn value(&self, v: &Value) -> String {
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
                let var = self.p.var(*id);
                if var.ty == Ty::Str {
                    var_name(var)
                } else {
                    format!("*{}", var_name(var))
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
        }
    }

    /// A built-in with optional slots gets a placeholder for each absent argument and, last, the `passed` mask:
    /// bit n set when the n-th optional slot is present (`study\02` §4.3).
    fn call(&self, id: BuiltinId, args: &[Option<Value>]) -> String {
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
        assert_eq!(
            var_name(&Var {
                name: "A".into(),
                ty: Ty::I32
            }),
            "__LONG_A"
        );
        assert_eq!(
            var_name(&Var {
                name: "S".into(),
                ty: Ty::Str
            }),
            "__STRING_S"
        );
    }
}
