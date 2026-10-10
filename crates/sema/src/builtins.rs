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

use crate::{Expr, ExprKind, PLACE_ONLY_UNREACHABLE, Ty};
use qb64rust_builtins::passing::plan;
use qb64rust_builtins::{Builtin, BuiltinId, find_function, find_statements, find_stub};

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
    // File functions (`verification\v22_b_funcs`); `FREEFILE` and `_CWD$` have no slot and are called bare.
    ("EOF", Rule::Plain),
    ("LOF", Rule::Plain),
    ("LOC", Rule::Plain),
    ("SEEK", Rule::Plain),
    ("FREEFILE", Rule::Plain),
    ("_FILEEXISTS", Rule::Plain),
    ("_DIREXISTS", Rule::Plain),
    ("_CWD$", Rule::Plain),
];

/// How the old compiler treats a built-in statement that is one call of the runtime (design D3 of
/// `m2-builtin-statements`): one variant per kind of special-casing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StmtRule {
    /// Slots and choices from the table entry and its template ([`stmt_slots`]); each argument converted by its
    /// slot type; passed as the old compiler's template rule says (`qb64rust_builtins::passing`).
    Plain,
    /// `CLOSE`: any number of file numbers, each a LONG value; one call per number, in order, or one call that
    /// closes every file when there is none (measured from the C++: `sub_close(n,1)`, `sub_close(NULL,0)`).
    Close,
}

/// Every built-in statement `sema` compiles as a call of the runtime, as written, and its rule. A name stands for
/// every form the table has for it (`OPEN` two, `SHELL` three). The coverage check of tier 1 requires each to be
/// used in a `slice.list` program and in an `ir` front-end test.
pub const STATEMENTS: &[(&str, StmtRule)] = &[
    // The file-system statements and `ENVIRON` (`verification\v22_a_*`).
    ("KILL", StmtRule::Plain),
    ("MKDIR", StmtRule::Plain),
    ("RMDIR", StmtRule::Plain),
    ("CHDIR", StmtRule::Plain),
    ("NAME", StmtRule::Plain),
    ("ENVIRON", StmtRule::Plain),
    // Files (`verification\v22_b_*`): `OPEN` in both forms.
    ("OPEN", StmtRule::Plain),
    ("CLOSE", StmtRule::Close),
    ("SEEK", StmtRule::Plain),
];

/// One form of a supported built-in statement: its table entry and rule.
#[derive(Clone, Copy, Debug)]
pub struct StmtSupported {
    pub id: BuiltinId,
    pub rule: StmtRule,
    /// The name as written, with its required suffix.
    pub name: &'static str,
}

/// The forms of the supported statement a name as written stands for (`name` in upper case without suffix, `string`
/// when it carries `$`), in table order; empty for any other name.
pub fn stmt_lookup(name: &str, string: bool) -> Vec<StmtSupported> {
    STATEMENTS
        .iter()
        .filter(|&&(written, _)| {
            let bare = written.strip_suffix('$').unwrap_or(written);
            bare == name && (bare.len() != written.len()) == string
        })
        .flat_map(|&(written, rule)| {
            let forms = match rule {
                StmtRule::Plain => find_statements(name.as_bytes(), string),
                // The statements the old compiler writes itself are named by their stub entry.
                StmtRule::Close => find_stub(name.as_bytes(), string).into_iter().collect(),
            };
            forms.into_iter().map(move |id| StmtSupported {
                id,
                rule,
                name: written,
            })
        })
        .collect()
}

/// Every form of every supported statement.
pub fn statements() -> Vec<StmtSupported> {
    STATEMENTS
        .iter()
        .flat_map(|&(written, _)| {
            let bare = written.strip_suffix('$').unwrap_or(written);
            stmt_lookup(bare, bare.len() != written.len())
        })
        .collect()
}

/// The supported statement form with this table entry.
pub fn stmt_find(id: BuiltinId) -> Option<StmtSupported> {
    statements().into_iter().find(|s| s.id == id)
}

/// One slot of a built-in statement: an argument or a choice of its template, in template order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StmtSlot {
    /// An argument of this slot type; `optional` when it may be left out.
    Arg { slot: Slot, optional: bool },
    /// A choice among `alts` alternatives; `optional` when it may be left out.
    Choice { alts: u8, optional: bool },
}

/// The slots of a [`StmtRule::Plain`] statement form: its template's arguments and choices, each argument typed by
/// the table (the table lists the types of the C arguments, [`qb64rust_builtins::passing`]). Empty for a stub
/// entry, whose rule says what its slots are.
pub fn stmt_slots(id: BuiltinId) -> Vec<StmtSlot> {
    let b = id.get();
    if b.callname == "sub_stub" {
        return Vec::new();
    }
    let plan = plan(&b.template());
    let mut c_arg = 0;
    plan.parts
        .iter()
        .map(|part| {
            let ty = part.passed.then(|| {
                c_arg += 1;
                b.arg_types.get(c_arg - 1).copied()
            });
            match part.alts {
                0 => {
                    let t = ty
                        .flatten()
                        .unwrap_or_else(|| unreachable!("`{}`: an argument without a table type", b.name));
                    StmtSlot::Arg {
                        slot: stmt_slot_type(b, c_arg - 1, t),
                        optional: part.optional,
                    }
                }
                alts => StmtSlot::Choice {
                    alts,
                    optional: part.optional,
                },
            }
        })
        .collect()
}

/// The type of C argument `k` of a statement entry: the table's, except where the libqb entry takes another type.
fn stmt_slot_type(b: &Builtin, k: usize, table: &str) -> Slot {
    match (b.callname, k) {
        // `sub_seek(int32 i, int64 pos)`: the table says LONG, but the old compiler passes the position as it is
        // and C++ keeps its 64 bits (measured from the C++, `verification\v22_a_args`).
        ("sub_seek", 1) => Slot::Int64,
        _ => Slot::from_table(table),
    }
}

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
    /// Converted as a store into an `_INTEGER64` (a float rounded half to even); only where a rule says so: the
    /// table has no such slot.
    Int64,
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
                held_override(b.callname).unwrap_or(qb)
            };
            (held, qb)
        }
        Rule::ResultOfArg => (first().qb, first().qb),
        Rule::IntFix => (first().ty, first().qb),
        Rule::FloatByArg => {
            let a = first();
            // A float keeps its rank; an integer of 16 bits or fewer is SINGLE, of 32 DOUBLE, wider `_FLOAT`; a
            // `_BIT * n` counts n bits (measured, `v21_d_builtins`: `SQR` of a `_BIT * 3` is SINGLE).
            let qb = match (a.qb.float_rank(), qb_bits(a.qb)) {
                (Some(1), _) => Ty::F32,
                (Some(2), _) => Ty::F64,
                (Some(_), _) => Ty::F80,
                (None, Some(b)) if b <= 16 => Ty::F32,
                (None, Some(b)) if b <= 32 => Ty::F64,
                (None, Some(_)) => Ty::F80,
                (None, None) => unreachable!("a numeric argument"),
            };
            // `std::sin` and the others follow C++ overloading; `func_sqr` and `func_log` take a `double`.
            let held = if b.callname.starts_with("std::") {
                overload([a.ty])
            } else {
                Ty::F64
            };
            (held, qb)
        }
        Rule::Exp => {
            if exp_single(first().qb) {
                (Ty::F64, Ty::F32)
            } else {
                (Ty::F80, Ty::F80)
            }
        }
        // `_ROUND` of an `_OFFSET` keeps its type (`qb64pe.bas` 21129; measured, `v21_d_builtins`).
        Rule::Convert(Ty::I64) if matches!(first().qb, Ty::Off | Ty::UOff) => (first().ty, first().qb),
        Rule::Convert(to) => (convert_held(to, first()), to),
        Rule::Fixed(t) => (t, t),
        Rule::Len | Rule::Asc => (Ty::I32, Ty::I32),
        Rule::Radix(_) | Rule::StringFill => (Ty::Str, Ty::Str),
        Rule::Val => unreachable!("typed by its type argument (check::builtins)"),
    }
}

/// The held type of a plain function whose libqb entry returns another C++ type than the table's return type says
/// (the type the old compiler believes): `func_lof`, `func_loc` and `func_seek` return `int64` and are believed
/// LONG, so arithmetic on them is computed in 64 bits (libqb's prototypes; `study\00` §5, "File functions").
fn held_override(callname: &str) -> Option<Ty> {
    match callname {
        "func_lof" | "func_loc" | "func_seek" => Some(Ty::I64),
        _ => None,
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
        // `func_cint_float` returns `int64`, `func_cint_double` `int32`; `func_cint_long`/`_ulong`/`_int64`/`_uint64`
        // `int16`.
        (Ty::I16, true) => {
            if wide_float {
                Ty::I64
            } else {
                Ty::I32
            }
        }
        // `func_clng_float` returns `int64`, `func_clng_double`, `_ulong`, `_int64` and `_uint64` `int32`.
        (Ty::I32, true) => {
            if wide_float {
                Ty::I64
            } else {
                Ty::I32
            }
        }
        (Ty::I16 | Ty::I32, false) => {
            if int_entry(to, a.qb).is_some() {
                to
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
        (
            Ty::I8
            | Ty::U8
            | Ty::U16
            | Ty::U32
            | Ty::U64
            | Ty::Off
            | Ty::UOff
            | Ty::Bit { .. }
            | Ty::F80
            | Ty::Str
            | Ty::User(_),
            _,
        ) => unreachable!("no conversion function to {to:?}"),
        (crate::place_only_types!(), _) => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
    }
}

/// The width in bits the old compiler sees in a value believed `t`: a `_BIT * n`'s n (its type's bits, not its
/// storage's), any other integer's [`Ty::int_bits`]; `None` for a float.
pub fn qb_bits(t: Ty) -> Option<u32> {
    match t {
        Ty::Bit { width, .. } => Some(u32::from(width)),
        Ty::I8
        | Ty::U8
        | Ty::I16
        | Ty::U16
        | Ty::I32
        | Ty::U32
        | Ty::I64
        | Ty::U64
        | Ty::Off
        | Ty::UOff
        | Ty::F32
        | Ty::F64
        | Ty::F80
        | Ty::Str
        | Ty::FixedStr(_)
        | Ty::User(_) => t.int_bits(),
    }
}

/// Whether `EXP` of a value believed `t` is SINGLE (`func_exp_single`): a SINGLE, or an integer of 16 bits or
/// fewer that is not a `_BIT` (`qb64pe.bas` 21066–21084; measured, `v21_d_builtins`: a `_BIT * 3` is `_FLOAT`).
pub fn exp_single(t: Ty) -> bool {
    match t {
        Ty::F32 => true,
        Ty::Bit { .. } | Ty::F64 | Ty::F80 => false,
        Ty::I8 | Ty::U8 | Ty::I16 | Ty::U16 | Ty::I32 | Ty::U32 | Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff => {
            t.int_bits().is_some_and(|b| b <= 16)
        }
        Ty::Str | Ty::User(_) => unreachable!("a numeric argument"),
        crate::place_only_types!() => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
    }
}

/// The libqb entry `CINT` (`to` INTEGER) or `CLNG` (`to` LONG) calls for an integer believed `qb`, `None` where the
/// old compiler writes the value itself (`qb64pe.bas` 21172–21215): `CINT` checks the range of an unsigned value of
/// 16 bits or more and of a signed one wider than 16, `CLNG` of an unsigned one of 32 bits or more and of a signed
/// one wider than 32 (measured, `v21_d_builtins`: `CINT` of a 65535 `_UNSIGNED INTEGER` raises error 6).
pub fn int_entry(to: Ty, qb: Ty) -> Option<&'static str> {
    let bits = qb_bits(qb).unwrap_or_else(|| unreachable!("an integer argument, not {qb:?}"));
    let unsigned = qb.is_unsigned();
    match (to, unsigned) {
        (Ty::I16, true) if bits > 32 => Some("func_cint_uint64"),
        (Ty::I16, true) if bits > 15 => Some("func_cint_ulong"),
        (Ty::I16, false) if bits > 32 => Some("func_cint_int64"),
        (Ty::I16, false) if bits > 16 => Some("func_cint_long"),
        (Ty::I32, true) if bits > 32 => Some("func_clng_uint64"),
        (Ty::I32, true) if bits == 32 => Some("func_clng_ulong"),
        (Ty::I32, false) if bits > 32 => Some("func_clng_int64"),
        (Ty::I16 | Ty::I32, _) => None,
        _ => unreachable!("`CINT` or `CLNG` only, not {to:?}"),
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
///
/// A `_BIT * n` (a place or a literal) has n bits, `HEX$` (n + 3) \ 4 digits, whether a place or not (measured,
/// `v21_d_builtins`: `HEX$` of a `_BIT * 3` holding -4 is `C`; the numeric-semantics scenario `HEX$(-1`5)` is `FF`).
pub fn radix_width(bits_per_digit: u32, arg: &Expr) -> Option<u32> {
    if let Ty::Bit { width, .. } = arg.qb {
        let bits = u32::from(width);
        return Some(if bits_per_digit == 4 { bits.div_ceil(4) } else { bits });
    }
    let bits = match qb_bits(arg.qb) {
        None => return None,
        Some(64) if matches!(arg.kind, ExprKind::Load(_)) => 64,
        Some(64) => 0,
        Some(b) => b,
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
    fn every_statement_row_is_in_the_table_once() {
        let mut names: Vec<&str> = STATEMENTS.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), STATEMENTS.len(), "a row is listed twice");
        for &(written, rule) in STATEMENTS {
            let bare = written.strip_suffix('$').unwrap_or(written);
            let forms = stmt_lookup(bare, bare.len() != written.len());
            assert!(!forms.is_empty(), "{written} has no statement entry in the table");
            for s in forms {
                assert_eq!(stmt_find(s.id).map(|f| f.rule), Some(rule), "{written}");
                // Every argument has a table type, and the table has no more types than C arguments and the mask.
                let slots = stmt_slots(s.id);
                let b = s.id.get();
                let p = plan(&b.template());
                let passed = p.parts.iter().filter(|part| part.passed).count();
                assert!(
                    b.arg_types.len() == passed || b.arg_types.len() == passed + usize::from(p.mask),
                    "{written}: {} table types for {passed} C arguments",
                    b.arg_types.len()
                );
                assert_eq!(slots.len(), p.parts.len());
            }
        }
        assert!(stmt_lookup("KILL", true).is_empty());
        assert!(stmt_lookup("NOPE", false).is_empty());
        let name = stmt_lookup("NAME", false);
        assert_eq!(name.len(), 1);
        assert_eq!(
            stmt_slots(name[0].id),
            [
                StmtSlot::Arg {
                    slot: Slot::Str,
                    optional: false
                },
                StmtSlot::Choice {
                    alts: 1,
                    optional: false
                },
                StmtSlot::Arg {
                    slot: Slot::Str,
                    optional: false
                },
            ]
        );
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
    fn new_types_special_cases() {
        let bit = |width, signed| Ty::Bit { width, signed };
        // `_BIT * n` counts its n bits, not its storage's.
        assert_eq!(qb_bits(bit(3, true)), Some(3));
        assert_eq!(qb_bits(Ty::U8), Some(8));
        assert_eq!(qb_bits(Ty::UOff), Some(64));
        assert_eq!(qb_bits(Ty::F32), None);
        // `HEX$`: (n + 3) \ 4 digits for a `_BIT * n`, place or not; 2 for a byte; 16 for a 64-bit place, else 0.
        let var = |t| e(Ty::I64, t, ExprKind::Load(crate::Place::Var(crate::VarId(0))));
        let op = |t| e(Ty::I64, t, ExprKind::Int(0));
        assert_eq!(radix_width(4, &var(bit(3, true))), Some(1));
        assert_eq!(radix_width(4, &op(bit(5, true))), Some(2));
        assert_eq!(radix_width(3, &var(bit(3, false))), Some(3));
        assert_eq!(radix_width(4, &var(Ty::U8)), Some(2));
        assert_eq!(radix_width(1, &var(Ty::I8)), Some(8));
        assert_eq!(radix_width(4, &var(Ty::Off)), Some(16));
        assert_eq!(radix_width(4, &op(Ty::U64)), Some(0));
        // `EXP`: SINGLE up to 16 bits, never for a `_BIT`.
        assert!(exp_single(Ty::U16) && exp_single(Ty::I8) && exp_single(Ty::F32));
        assert!(!exp_single(Ty::U32) && !exp_single(bit(3, true)) && !exp_single(Ty::F64));
        // `CINT`: unsigned from 16 bits, signed above 16; `CLNG`: unsigned at 32 or more, signed above 32.
        assert_eq!(int_entry(Ty::I16, Ty::U8), None);
        assert_eq!(int_entry(Ty::I16, Ty::U16), Some("func_cint_ulong"));
        assert_eq!(int_entry(Ty::I16, Ty::I16), None);
        assert_eq!(int_entry(Ty::I16, Ty::U32), Some("func_cint_ulong"));
        assert_eq!(int_entry(Ty::I16, Ty::UOff), Some("func_cint_uint64"));
        assert_eq!(int_entry(Ty::I16, bit(20, true)), Some("func_cint_long"));
        assert_eq!(int_entry(Ty::I16, bit(10, true)), None);
        assert_eq!(int_entry(Ty::I32, Ty::U16), None);
        assert_eq!(int_entry(Ty::I32, Ty::U32), Some("func_clng_ulong"));
        assert_eq!(int_entry(Ty::I32, bit(20, false)), None);
        assert_eq!(int_entry(Ty::I32, Ty::U64), Some("func_clng_uint64"));
        assert_eq!(int_entry(Ty::I32, Ty::Off), Some("func_clng_int64"));
        // The held type is the entry's (`int16`, `int32`) or the argument's own where none is called.
        assert_eq!(convert_held(Ty::I16, &var(Ty::U16)), Ty::I16);
        assert_eq!(convert_held(Ty::I16, &e(Ty::U8, Ty::U8, ExprKind::Int(0))), Ty::U8);
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
