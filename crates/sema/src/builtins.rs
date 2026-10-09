//! Built-in functions (design D2–D4 of `m2-core-builtins`): which are compiled, the rule each follows, and the
//! types of their results. The arguments are checked in `check\builtins.rs`; the emitter finds a call's rule here
//! ([`rule`]) and writes what the old compiler writes for it.
//!
//! The built-in table (`qb64rust_builtins`, generated from the old compiler's registrations) gives each function's
//! slots, optional mask and plain return type. The old compiler special-cases about a third of the functions in
//! `evaluatefunc` (`study\02` §5.5); each kind of special-casing is one [`Rule`]. The table is never edited: where it
//! is wrong, a rule overrides it, citing the measurement (`verification\v20_*`, `study\00` §5).
//!
//! Each call has a held type (the C++ type of the libqb call for the arguments' held types, so arithmetic on it is
//! computed as in the old compiler) and a believed type (the old compiler's, which `PRINT`, `HEX$` and later
//! operators see), as operators have ([`crate::Expr`]).

use crate::{BIT_VALUE_UNREACHABLE, Expr, ExprKind, LATER_TYPE_UNREACHABLE, NEW_TYPE_UNREACHABLE, Ty};
use qb64rust_builtins::{Builtin, BuiltinId, find_function};

/// How the old compiler treats a built-in function: one variant per kind of special-casing, not per function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    /// Slots, optional mask and result from the table; each argument converted by its slot type ([`Slot`]).
    Plain,
    /// The result has the argument's type, the argument converted as its slot says (`ABS`).
    ResultOfArg,
    /// `INT`, `FIX`: the argument as it is, the result held and believed as the argument.
    IntFix,
    /// `SIN`, `COS`, `TAN`, `ATN`, `SQR`, `LOG`: believed SINGLE, DOUBLE or `_FLOAT` by the argument's believed type
    /// (`qb64pe.bas` 22071).
    FloatByArg,
    /// `EXP`: believed SINGLE for an integer of 16 bits or fewer and for SINGLE, else `_FLOAT` (`qb64pe.bas` 21066).
    Exp,
    /// `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND`: the result type fixed, a range-checked libqb entry chosen by the
    /// argument's believed type (`qb64pe.bas` 21116–21215).
    Convert(Ty),
    /// The result type overrides the table, no arguments to convert (`ERR` LONG, `ERL` DOUBLE; `LBOUND`/`UBOUND`
    /// `_INTEGER64`, measured `v18_e_bounds*`, checked as [`ExprKind::Bound`]).
    Fixed(Ty),
    /// `LEN`: a string expression's length, or a place's size (D4).
    Len,
    /// `VAL(s$[, type])` (D4).
    Val,
    /// `HEX$` (4 bits per digit), `OCT$` (3), `_BIN$` (1): the width of a negative value from the argument's type.
    Radix(u32),
    /// `ASC(s$[, position])`: two slots, against the table's one.
    Asc,
    /// `STRING$(n, code)` or `STRING$(n, s$)`: the second argument a number or a string (`qb64pe.bas` 21294).
    StringFill,
}

/// Every built-in function `sema` compiles, as written (with its required suffix), and its rule. The coverage check
/// of tier 1 requires each to be called in a `slice.list` program and in a `typed` front-end test.
pub const SUPPORTED: &[(&str, Rule)] = &[
    ("INSTR", Rule::Plain),
    ("CHR$", Rule::Plain),
    // String functions (`v20_c_string_edges`).
    ("LEFT$", Rule::Plain),
    ("RIGHT$", Rule::Plain),
    ("MID$", Rule::Plain),
    ("SPACE$", Rule::Plain),
    ("STRING$", Rule::StringFill),
    ("LTRIM$", Rule::Plain),
    ("RTRIM$", Rule::Plain),
    ("_TRIM$", Rule::Plain),
    ("UCASE$", Rule::Plain),
    ("LCASE$", Rule::Plain),
    ("STR$", Rule::Plain),
    ("_TOSTR$", Rule::Plain),
    ("LEN", Rule::Len),
    ("ASC", Rule::Asc),
    ("VAL", Rule::Val),
    ("HEX$", Rule::Radix(4)),
    ("OCT$", Rule::Radix(3)),
    ("_BIN$", Rule::Radix(1)),
    // Math functions (`v20_b_result_types`, `v20_f_math_edges`).
    ("ABS", Rule::ResultOfArg),
    ("INT", Rule::IntFix),
    ("FIX", Rule::IntFix),
    ("SGN", Rule::Plain),
    ("SIN", Rule::FloatByArg),
    ("COS", Rule::FloatByArg),
    ("TAN", Rule::FloatByArg),
    ("ATN", Rule::FloatByArg),
    ("SQR", Rule::FloatByArg),
    ("LOG", Rule::FloatByArg),
    ("EXP", Rule::Exp),
    ("CINT", Rule::Convert(Ty::I16)),
    // LONG: the table's INTEGER is wrong (`qb64pe.bas` 21190, `v20_b_result_types`).
    ("CLNG", Rule::Convert(Ty::I32)),
    ("CSNG", Rule::Convert(Ty::F32)),
    ("CDBL", Rule::Convert(Ty::F64)),
    ("_ROUND", Rule::Convert(Ty::I64)),
    ("_ATAN2", Rule::Plain),
    ("_HYPOT", Rule::Plain),
    ("_PI", Rule::Plain),
    ("LBOUND", Rule::Fixed(Ty::I64)),
    ("UBOUND", Rule::Fixed(Ty::I64)),
    // `ERR` is LONG, not `_UNSIGNED LONG` as in the table (design D5 of `m2-procedures-and-errors`).
    ("ERR", Rule::Fixed(Ty::I32)),
    ("ERL", Rule::Fixed(Ty::F64)),
];

/// A supported built-in: its table entry and rule.
#[derive(Clone, Copy, Debug)]
pub struct Supported {
    pub id: BuiltinId,
    pub rule: Rule,
    /// The name as written, with its required suffix (`CHR$`).
    pub name: &'static str,
}

/// The supported built-in a name as written stands for (`name` in upper case without suffix, `string` when it
/// carries `$`); `None` for any other name.
pub fn lookup(name: &str, string: bool) -> Option<Supported> {
    SUPPORTED.iter().find_map(|&(written, rule)| {
        let (bare, dollar) = match written.strip_suffix('$') {
            Some(b) => (b, true),
            None => (written, false),
        };
        (bare == name && dollar == string).then(|| Supported {
            id: find_function(bare.as_bytes()).expect("every supported built-in is in the table"),
            rule,
            name: written,
        })
    })
}

/// Every supported built-in.
pub fn supported() -> impl Iterator<Item = Supported> {
    SUPPORTED.iter().map(|&(written, _)| {
        let bare = written.strip_suffix('$').unwrap_or(written);
        lookup(bare, bare.len() != written.len()).expect("listed")
    })
}

/// The rule of a supported built-in; `None` for one `sema` does not compile.
pub fn rule(id: BuiltinId) -> Option<Rule> {
    find(id).map(|s| s.rule)
}

/// The supported built-in with this table entry.
pub fn find(id: BuiltinId) -> Option<Supported> {
    supported().find(|s| s.id == id)
}

/// The number of argument slots a call of the built-in has in the typed tree: the table's, except where a rule says
/// otherwise.
pub fn slot_count(id: BuiltinId) -> usize {
    match rule(id) {
        Some(Rule::Asc) => 2,
        Some(
            Rule::Plain
            | Rule::ResultOfArg
            | Rule::IntFix
            | Rule::FloatByArg
            | Rule::Exp
            | Rule::Convert(_)
            | Rule::Fixed(_)
            | Rule::Len
            | Rule::Val
            | Rule::Radix(_)
            | Rule::StringFill,
        )
        | None => id.get().arg_types.len(),
    }
}

/// The slots of a call: their types and whether each is optional.
pub fn slots(s: Supported) -> Vec<(Slot, bool)> {
    let b = s.id.get();
    match s.rule {
        Rule::Asc => vec![(Slot::Str, false), (Slot::Long, true)],
        Rule::Plain
        | Rule::ResultOfArg
        | Rule::IntFix
        | Rule::FloatByArg
        | Rule::Exp
        | Rule::Convert(_)
        | Rule::Fixed(_)
        | Rule::Len
        | Rule::Val
        | Rule::Radix(_)
        | Rule::StringFill => {
            let optional = b.optional.unwrap_or(&[]);
            b.arg_types
                .iter()
                .enumerate()
                .map(|(i, t)| (Slot::from_table(t), optional.get(i).copied().unwrap_or(false)))
                .collect()
        }
    }
}

/// The type of an argument slot, as decoded in the table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    /// Converted as a store into a LONG: a float rounded half to even to `_INTEGER64`, then the low 32 bits
    /// (measured `v20_a_slots`).
    Long,
    /// Converted exactly to DOUBLE (C's implicit conversion of a `double` parameter).
    Double,
    /// Passed in its own type (`func_sqr(*__INTEGER_I)`, measured from the C++).
    Float,
    /// Cast to its believed type (`func_abs((int16)(…))`, `qbs_str((int16)(…))`).
    AnyNumeric,
    Str,
}

impl Slot {
    fn from_table(t: &str) -> Slot {
        match t {
            "LONG" => Slot::Long,
            "DOUBLE" => Slot::Double,
            "_FLOAT" => Slot::Float,
            "any-numeric" => Slot::AnyNumeric,
            "STRING" => Slot::Str,
            other => unreachable!("slot type {other} of a supported built-in"),
        }
    }
}

/// The held and believed types of a call's result, from its (converted) arguments; `None` slots are absent.
pub(crate) fn result_types(s: Supported, args: &[Option<Expr>]) -> (Ty, Ty) {
    let b = s.id.get();
    let first = || {
        args[0]
            .as_ref()
            .expect("the first argument of a typed-by-argument built-in")
    };
    match s.rule {
        Rule::Plain => {
            let qb = table_ty(b);
            let held = if b.callname.starts_with("std::") {
                overload(args.iter().flatten().map(|a| a.ty))
            } else {
                qb
            };
            (held, qb)
        }
        Rule::ResultOfArg => (first().qb, first().qb),
        Rule::IntFix => (first().ty, first().qb),
        Rule::FloatByArg => {
            let a = first();
            let qb = match a.qb {
                Ty::I16 | Ty::F32 => Ty::F32,
                Ty::I32 | Ty::F64 => Ty::F64,
                Ty::I64 | Ty::F80 => Ty::F80,
                Ty::Str | Ty::User(_) => unreachable!("a numeric argument"),
                crate::unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
                crate::later_types!() => unreachable!("{LATER_TYPE_UNREACHABLE}"),
                crate::Ty::Bit { .. } => unreachable!("{BIT_VALUE_UNREACHABLE}"),
            };
            // `std::sin` and the others follow C++ overloading; `func_sqr` and `func_log` take a `double`.
            let held = if b.callname.starts_with("std::") {
                overload([a.ty])
            } else {
                Ty::F64
            };
            (held, qb)
        }
        Rule::Exp => match first().qb {
            Ty::I16 | Ty::F32 => (Ty::F64, Ty::F32),
            Ty::I32 | Ty::I64 | Ty::F64 | Ty::F80 => (Ty::F80, Ty::F80),
            Ty::Str | Ty::User(_) => unreachable!("a numeric argument"),
            crate::unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
            crate::later_types!() => unreachable!("{LATER_TYPE_UNREACHABLE}"),
            crate::Ty::Bit { .. } => unreachable!("{BIT_VALUE_UNREACHABLE}"),
        },
        Rule::Convert(to) => (convert_held(to, first()), to),
        Rule::Fixed(t) => (t, t),
        Rule::Len | Rule::Asc => (Ty::I32, Ty::I32),
        Rule::Radix(_) | Rule::StringFill => (Ty::Str, Ty::Str),
        Rule::Val => unreachable!("typed by its type argument (check::builtins)"),
    }
}

/// The C++ result type of a `std::` math function for arguments of these held types: `long double` if any is,
/// `float` if all are `float`, else `double` (integers convert to `double`).
fn overload(args: impl IntoIterator<Item = Ty>) -> Ty {
    let mut all_float = true;
    let mut any_long = false;
    for t in args {
        all_float &= t == Ty::F32;
        any_long |= t == Ty::F80;
    }
    if any_long {
        Ty::F80
    } else if all_float {
        Ty::F32
    } else {
        Ty::F64
    }
}

/// The held type of `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND` of an argument, by the libqb entry the old compiler
/// picks for the argument's believed type (`rounding.h`; measured from the C++): the argument itself where no entry
/// is called.
fn convert_held(to: Ty, a: &Expr) -> Ty {
    let wide_float = a.qb == Ty::F80;
    match (to, a.qb.is_float()) {
        // `func_cint_float` returns `int64`, `func_cint_double` `int32`; `func_cint_long`/`_int64` `int16`.
        (Ty::I16, true) => {
            if wide_float {
                Ty::I64
            } else {
                Ty::I32
            }
        }
        (Ty::I16, false) => {
            if a.qb == Ty::I16 {
                a.ty
            } else {
                Ty::I16
            }
        }
        // `func_clng_float` returns `int64`, `func_clng_double` and `func_clng_int64` `int32`.
        (Ty::I32, true) => {
            if wide_float {
                Ty::I64
            } else {
                Ty::I32
            }
        }
        (Ty::I32, false) => {
            if a.qb == Ty::I64 {
                Ty::I32
            } else {
                a.ty
            }
        }
        // `CSNG` of a SINGLE and `CDBL` of a SINGLE or DOUBLE emit the argument itself; the entries return `double`;
        // an integer is `(double)(e)` (not narrowed, `v20_b_result_types`).
        (Ty::F32, true) if a.qb == Ty::F32 => a.ty,
        (Ty::F64, true) if a.qb != Ty::F80 => a.ty,
        (Ty::F32 | Ty::F64, _) => Ty::F64,
        // `func_round_*` return `int64`; an integer is emitted as it is.
        (Ty::I64, true) => Ty::I64,
        (Ty::I64, false) => a.ty,
        (Ty::F80 | Ty::Str | Ty::User(_), _) => unreachable!("no conversion function to {to:?}"),
        (crate::unproduced_types!(), _) => unreachable!("{NEW_TYPE_UNREACHABLE}"),
        (crate::later_types!(), _) => unreachable!("{LATER_TYPE_UNREACHABLE}"),
        (crate::Ty::Bit { .. }, _) => unreachable!("{BIT_VALUE_UNREACHABLE}"),
    }
}

/// The table's return type of a plain function.
fn table_ty(b: &Builtin) -> Ty {
    match b.ret {
        Some("INTEGER") => Ty::I16,
        Some("LONG") => Ty::I32,
        Some("_INTEGER64") => Ty::I64,
        Some("SINGLE") => Ty::F32,
        Some("DOUBLE") => Ty::F64,
        Some("_FLOAT") => Ty::F80,
        Some("STRING") => Ty::Str,
        other => unreachable!("return type {other:?} of a plain supported built-in"),
    }
}

/// The width a negative value is printed with by `HEX$`, `OCT$` or `_BIN$` (D4, `qb64pe.bas` 20978–21062): from the
/// argument's believed type, 16 bits 4 hex digits and 32 bits 8; 64 bits 16 only for a place (variable, element,
/// member), 0 (the shortest form) for any other expression. For `OCT$` and `_BIN$` the number of bits. `None` for a
/// float argument (the `_float` entries take no width).
pub fn radix_width(bits_per_digit: u32, arg: &Expr) -> Option<u32> {
    let bits = match arg.qb {
        Ty::I16 => 16,
        Ty::I32 => 32,
        Ty::I64 if matches!(arg.kind, ExprKind::Load(_)) => 64,
        Ty::I64 => 0,
        Ty::F32 | Ty::F64 | Ty::F80 => return None,
        Ty::Str | Ty::User(_) => unreachable!("a numeric argument"),
        crate::unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
        crate::later_types!() => unreachable!("{LATER_TYPE_UNREACHABLE}"),
        crate::Ty::Bit { .. } => unreachable!("{BIT_VALUE_UNREACHABLE}"),
    };
    Some(if bits_per_digit == 4 { bits / 4 } else { bits })
}

#[cfg(test)]
mod tests {
    use super::*;
    use qb64rust_base::{FileId, Span};

    #[test]
    fn every_row_is_in_the_table_once() {
        let mut names: Vec<&str> = SUPPORTED.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), SUPPORTED.len(), "a row is listed twice");
        for s in supported() {
            let b = s.id.get();
            assert_eq!(b.musthave == Some("$"), s.name.ends_with('$'), "{}", s.name);
            assert_eq!(rule(s.id), Some(s.rule));
        }
    }

    #[test]
    fn lookup_needs_the_suffix_as_written() {
        assert!(lookup("CHR", true).is_some());
        assert!(lookup("CHR", false).is_none());
        assert!(lookup("INSTR", false).is_some());
        assert!(lookup("INSTR", true).is_none());
        assert!(lookup("NOPE", false).is_none());
    }

    fn e(ty: Ty, qb: Ty, kind: ExprKind) -> Expr {
        Expr {
            span: Span::new(FileId(0), 0, 0),
            ty,
            qb,
            kind,
        }
    }

    #[test]
    fn std_overloads() {
        assert_eq!(overload([Ty::F32, Ty::F32]), Ty::F32);
        assert_eq!(overload([Ty::F32, Ty::I16]), Ty::F64);
        assert_eq!(overload([Ty::I64]), Ty::F64);
        assert_eq!(overload([Ty::F64, Ty::F80]), Ty::F80);
    }

    #[test]
    fn radix_widths() {
        let var = |t| e(t, t, ExprKind::Load(crate::Place::Var(crate::VarId(0))));
        let op = |t| e(Ty::I32, t, ExprKind::Int(0));
        assert_eq!(radix_width(4, &var(Ty::I16)), Some(4));
        assert_eq!(radix_width(4, &var(Ty::I32)), Some(8));
        assert_eq!(radix_width(4, &var(Ty::I64)), Some(16));
        assert_eq!(radix_width(3, &var(Ty::I64)), Some(64));
        assert_eq!(radix_width(4, &op(Ty::I64)), Some(0));
        assert_eq!(radix_width(1, &op(Ty::I16)), Some(16));
        assert_eq!(radix_width(4, &op(Ty::F32)), None);
    }

    #[test]
    fn conversion_held_types() {
        let lit = |ty, qb| e(ty, qb, ExprKind::Int(0));
        // CINT of a DOUBLE: func_cint_double, int32; of a _FLOAT: int64; of a LONG: int16; of an INTEGER: itself.
        assert_eq!(convert_held(Ty::I16, &lit(Ty::F64, Ty::F64)), Ty::I32);
        assert_eq!(convert_held(Ty::I16, &lit(Ty::F80, Ty::F80)), Ty::I64);
        assert_eq!(convert_held(Ty::I16, &lit(Ty::I32, Ty::I32)), Ty::I16);
        assert_eq!(convert_held(Ty::I16, &lit(Ty::I16, Ty::I16)), Ty::I16);
        // CSNG of an integer is (double), not narrowed; of a SINGLE literal (held DOUBLE) the literal itself.
        assert_eq!(convert_held(Ty::F32, &lit(Ty::I32, Ty::I32)), Ty::F64);
        assert_eq!(convert_held(Ty::F32, &lit(Ty::F64, Ty::F32)), Ty::F64);
        assert_eq!(convert_held(Ty::F64, &lit(Ty::F32, Ty::F32)), Ty::F32);
        // CLNG of an _INTEGER64: func_clng_int64, int32; of an INTEGER: itself.
        assert_eq!(convert_held(Ty::I32, &lit(Ty::I64, Ty::I64)), Ty::I32);
        assert_eq!(convert_held(Ty::I32, &lit(Ty::I16, Ty::I16)), Ty::I16);
        assert_eq!(convert_held(Ty::I64, &lit(Ty::F32, Ty::F32)), Ty::I64);
    }
}
