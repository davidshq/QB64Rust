//! Built-in functions: the call of a built-in in an expression, written as the old compiler writes it for the
//! function's rule (design D5 of `m2-core-builtins`; the entries and their choice by argument type are read from
//! `qb64pe.bas` `evaluatefunc` and the C++ it writes, `study\00` §5).

use crate::Emitter;
use crate::names::c_type;
use qb64rust_builtins::BuiltinId;
use qb64rust_builtins::passing::plan;
use qb64rust_ir::builtins::{Rule, Slot, StmtRule, find, int_entry, radix_width, slots, stmt_find};
use qb64rust_ir::{Expr, Facts, StmtArg, Ty};

impl Emitter<'_> {
    /// A built-in call. A built-in no rule covers (`ERR`, `ERL` are `Fixed`; every call `sema` makes has a rule) is
    /// written by the table.
    pub(crate) fn call(&mut self, id: BuiltinId, args: &[Option<Expr>], ty: Ty, qb: Ty) -> String {
        let Some(s) = find(id) else {
            return self.table_call(id, args, &[]);
        };
        let arg = |k: usize| args[k].as_ref().expect("a required argument");
        match s.rule {
            Rule::Plain | Rule::ResultOfArg | Rule::FloatByArg | Rule::Fixed(_) => {
                let slots: Vec<Slot> = slots(s).into_iter().map(|(slot, _)| slot).collect();
                self.table_call(id, args, &slots)
            }
            Rule::IntFix => {
                let a = arg(0);
                let x = self.value(a);
                // `INT`: `std::floor` of a float, the value of an integer; `FIX`: `func_fix_float` above 64 bits.
                let f = match (s.name, a.qb) {
                    (_, t) if t.is_int() => "",
                    ("INT", _) => "std::floor",
                    (_, Ty::F80) => "func_fix_float",
                    (_, Ty::F32 | Ty::F64) => "func_fix_double",
                    (_, t) => unreachable!("`{}` of a {t:?}", s.name),
                };
                format!("{f}({x})")
            }
            Rule::Exp => {
                let x = self.value(arg(0));
                let f = if qb == Ty::F32 {
                    "func_exp_single"
                } else {
                    "func_exp_float"
                };
                format!("{f}({x})")
            }
            Rule::Convert(to) => self.convert_fn(to, arg(0)),
            Rule::Len => format!("((int32)({})->len)", self.value(arg(0))),
            // The result type says which parse (`check\builtins.rs`).
            Rule::Val => {
                let x = self.value(arg(0));
                match ty {
                    Ty::F32 => format!("((float)qbs_val<long double>({x}))"),
                    Ty::F64 => format!("((double)qbs_val<long double>({x}))"),
                    Ty::F80 => format!("qbs_val<long double>({x})"),
                    Ty::I64 => format!("qbs_val<int64_t>({x})"),
                    Ty::U64 => format!("qbs_val<uint64_t>({x})"),
                    Ty::I8
                    | Ty::U8
                    | Ty::I16
                    | Ty::U16
                    | Ty::I32
                    | Ty::U32
                    | Ty::Off
                    | Ty::UOff
                    | Ty::Bit { .. }
                    | Ty::Str
                    | Ty::FixedStr(_)
                    | Ty::User(_) => unreachable!("VAL typed {ty:?}"),
                }
            }
            Rule::Radix(bits) => {
                let a = arg(0);
                let x = self.value(a);
                let f = match bits {
                    4 => "func_hex",
                    3 => "func_oct",
                    1 => "func__bin",
                    _ => unreachable!("{bits} bits per digit"),
                };
                match radix_width(bits, a) {
                    Some(w) => format!("{f}({x},{w})"),
                    None => format!("{f}_float({x})"),
                }
            }
            Rule::Asc => {
                let s = self.value(arg(0));
                match &args[1] {
                    Some(p) => format!("qbs_asc({s},{})", self.value(p)),
                    None => format!("qbs_asc({s})"),
                }
            }
            // `STRING$(n, s$)` takes the string's first byte.
            Rule::StringFill => {
                let n = self.value(arg(0));
                let c = arg(1);
                let code = if c.ty == Ty::Str {
                    format!("({})->chr[0]", self.value(c))
                } else {
                    self.value(c)
                };
                format!("func_string({n},{code})")
            }
        }
    }

    /// `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND` of an argument: the libqb entry the old compiler picks by the
    /// argument's believed type, or the value itself (`qb64pe.bas` 21116–21215).
    fn convert_fn(&mut self, to: Ty, a: &Expr) -> String {
        let x = self.value(a);
        let int = a.qb.is_int();
        let f = match (to, a.qb) {
            (Ty::I16 | Ty::I32, _) if int => int_entry(to, a.qb).unwrap_or(""),
            (Ty::I16, Ty::F80) => "func_cint_float",
            (Ty::I16, Ty::F32 | Ty::F64) => "func_cint_double",
            (Ty::I32, Ty::F80) => "func_clng_float",
            (Ty::I32, Ty::F32 | Ty::F64) => "func_clng_double",
            (Ty::F32, Ty::F64) => "func_csng_double",
            (Ty::F32, Ty::F80) => "func_csng_float",
            (Ty::F64, Ty::F80) => "func_cdbl_float",
            (Ty::F32 | Ty::F64, _) if int => return format!("((double)({x}))"),
            (Ty::I64, Ty::F80) => "func_round_float",
            (Ty::I64, Ty::F32 | Ty::F64) => "func_round_double",
            (Ty::I64, _) if int => "",
            (Ty::F32, Ty::F32) | (Ty::F64, Ty::F32 | Ty::F64) => "",
            (_, from) => unreachable!("a conversion to {to:?} of {from:?}"),
        };
        format!("{f}({x})")
    }

    /// A built-in statement, written by its rule (design D5 of `m2-builtin-statements`).
    pub(crate) fn stmt_call(&mut self, id: BuiltinId, args: &[StmtArg], out: &mut Vec<String>) {
        let form = stmt_find(id).unwrap_or_else(|| unreachable!("{} is no compiled statement", id.get().name));
        match form.rule {
            StmtRule::Plain => out.push(self.plain_stmt(id, args)),
            // One call per number, in order; every file when there is none.
            StmtRule::Close => {
                for arg in args {
                    let StmtArg::Value(n) = arg else {
                        unreachable!("`CLOSE` takes values (`validate`)");
                    };
                    out.push(format!("sub_close({},1);", self.value(n)));
                }
                if args.is_empty() {
                    out.push("sub_close(NULL,0);".into());
                }
            }
        }
        if args.iter().any(|a| a.uses_strings(self.p)) {
            out.push("qbs_cleanup(qbs_tmp_base,0);".into());
        }
    }

    /// A statement call by the table's `callname` and the old compiler's template rule
    /// (`qb64rust_builtins::passing`): each argument and each several-alternative choice is a C argument (the
    /// value, the alternative's number from 1, `NULL` when left out), and the `passed` mask comes last where the
    /// template has optional parts that need a flag.
    fn plain_stmt(&mut self, id: BuiltinId, args: &[StmtArg]) -> String {
        let b = id.get();
        let plan = plan(&b.template());
        let mut c_args = Vec::new();
        let mut mask = 0u32;
        for (part, arg) in plan.parts.iter().zip(args) {
            let text = match arg {
                StmtArg::Value(v) => self.value(v),
                StmtArg::Word(w) => (u32::from(*w) + 1).to_string(),
                StmtArg::Absent => "NULL".to_string(),
                StmtArg::Place(_) => unreachable!("`{}` takes no place (`validate`)", b.name),
            };
            if !matches!(arg, StmtArg::Absent) {
                mask |= part.flag;
            }
            if part.passed {
                c_args.push(text);
            }
        }
        if plan.mask {
            c_args.push(mask.to_string());
        }
        format!("{}({});", b.callname, c_args.join(","))
    }

    /// A call by the table's `callname`. A built-in with optional slots gets `NULL` for each absent argument, as the
    /// old compiler writes it, and, last, the `passed` mask: bit n set when the n-th optional slot is present (`study\02` §4.3). An argument
    /// of an any-numeric slot is cast to its C type, so libqb's overload for it is chosen (`func_abs((int16)(…))`:
    /// an INTEGER literal is otherwise a C `int`).
    fn table_call(&mut self, id: BuiltinId, args: &[Option<Expr>], slots: &[Slot]) -> String {
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
                    let x = self.value(a);
                    parts.push(match slots.get(i) {
                        Some(Slot::AnyNumeric) => format!("(({})({x}))", c_type(a.ty)),
                        Some(Slot::Long | Slot::Int64 | Slot::Double | Slot::Float | Slot::Str) | None => x,
                    });
                }
                None => parts.push("NULL".into()),
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
