//! Generator of the differential programs (spec `testing/differential-tests`, design D2 of `m2-numeric-types`):
//! BASIC programs that apply every operator to every ordered pair of numeric types, store every type into every
//! type and print every type's values, at boundary values and seeded random ones. Their expected output is recorded
//! once with `qb64pe.exe`; tier 2 compares the new compiler's output with it (`tests\differential\README.md`).
//!
//! Deterministic: one fixed seed and a small xorshift, nothing from the platform or the clock, so [`generate`] gives
//! the same bytes every time and tier 1 can check that the files in the repository are fresh ([`stale`]).

use qb64rust_sema::Ty;
use std::path::Path;

/// The seed of every random value. Changing it changes every program, which then needs a new recording.
pub const SEED: u64 = 0x5EED_D1FF_2026_1008;

/// A numeric type as the generator writes it.
#[derive(Debug)]
pub struct NumTy {
    /// The type's name in printed labels; lower-cased, part of its variables' names.
    pub key: &'static str,
    /// What follows `AS`.
    pub decl: &'static str,
    /// The literal suffix. `None` for the `_OFFSET` types, which have none ("Cannot use _OFFSET symbols after
    /// numbers"): their values are written as 64-bit literals of the same signedness.
    pub suffix: Option<&'static str>,
    pub kind: Kind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// An integer type of `bits` bits; `bit` for the `_BIT` types.
    Int {
        bits: u32,
        signed: bool,
        bit: bool,
    },
    Float(FloatKind),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatKind {
    Single,
    Double,
    Float,
}

const fn int(key: &'static str, decl: &'static str, suffix: Option<&'static str>, bits: u32, signed: bool) -> NumTy {
    NumTy {
        key,
        decl,
        suffix,
        kind: Kind::Int {
            bits,
            signed,
            bit: false,
        },
    }
}

const fn bit(key: &'static str, decl: &'static str, suffix: &'static str, bits: u32, signed: bool) -> NumTy {
    NumTy {
        key,
        decl,
        suffix: Some(suffix),
        kind: Kind::Int {
            bits,
            signed,
            bit: true,
        },
    }
}

const fn float(key: &'static str, decl: &'static str, suffix: &'static str, kind: FloatKind) -> NumTy {
    NumTy {
        key,
        decl,
        suffix: Some(suffix),
        kind: Kind::Float(kind),
    }
}

/// Every numeric type of QB64pe. `_BIT * n` is represented by four widths: the 1-bit `_BIT`, a narrow unsigned one,
/// a signed one that needs `int32` storage, and an unsigned one wider than 32 bits (`int64` storage, D-009).
pub static TYPES: &[NumTy] = &[
    int("BYTE", "_BYTE", Some("%%"), 8, true),
    int("UBYTE", "_UNSIGNED _BYTE", Some("~%%"), 8, false),
    int("INTEGER", "INTEGER", Some("%"), 16, true),
    int("UINTEGER", "_UNSIGNED INTEGER", Some("~%"), 16, false),
    int("LONG", "LONG", Some("&"), 32, true),
    int("ULONG", "_UNSIGNED LONG", Some("~&"), 32, false),
    int("INT64", "_INTEGER64", Some("&&"), 64, true),
    int("UINT64", "_UNSIGNED _INTEGER64", Some("~&&"), 64, false),
    int("OFFSET", "_OFFSET", None, 64, true),
    int("UOFFSET", "_UNSIGNED _OFFSET", None, 64, false),
    bit("BIT", "_BIT", "`", 1, true),
    bit("UBIT7", "_UNSIGNED _BIT * 7", "~`7", 7, false),
    bit("BIT24", "_BIT * 24", "`24", 24, true),
    bit("UBIT40", "_UNSIGNED _BIT * 40", "~`40", 40, false),
    float("SINGLE", "SINGLE", "!", FloatKind::Single),
    float("DOUBLE", "DOUBLE", "#", FloatKind::Double),
    float("FLOAT", "_FLOAT", "##", FloatKind::Float),
];

/// The generator's type for a `sema` type; `None` for the non-numeric ones and for a `_BIT * n` whose width
/// [`TYPES`] does not represent. The match is exhaustive, so a new variant of `Ty` fails this crate's build until
/// it is mapped here and covered by [`TYPES`] (design D2).
pub fn for_sema(ty: Ty) -> Option<&'static NumTy> {
    let key = match ty {
        Ty::I8 => "BYTE",
        Ty::U8 => "UBYTE",
        Ty::I16 => "INTEGER",
        Ty::U16 => "UINTEGER",
        Ty::I32 => "LONG",
        Ty::U32 => "ULONG",
        Ty::I64 => "INT64",
        Ty::U64 => "UINT64",
        Ty::Off => "OFFSET",
        Ty::UOff => "UOFFSET",
        Ty::Bit { width: 1, signed: true } => "BIT",
        Ty::Bit {
            width: 7,
            signed: false,
        } => "UBIT7",
        Ty::Bit {
            width: 24,
            signed: true,
        } => "BIT24",
        Ty::Bit {
            width: 40,
            signed: false,
        } => "UBIT40",
        Ty::Bit { .. } => return None,
        Ty::F32 => "SINGLE",
        Ty::F64 => "DOUBLE",
        Ty::F80 => "FLOAT",
        Ty::Str | Ty::FixedStr(_) | Ty::User(_) => return None,
    };
    TYPES.iter().find(|t| t.key == key)
}

impl NumTy {
    /// Whether a variable of this type needs a `_BIT * 32` pad DIMmed just before it: a `_BIT * n` with n > 32
    /// overwrites the `_BIT` scalar allocated before it in the old compiler (`DIVERGENCES.md` D-009).
    pub fn needs_pad(&self) -> bool {
        matches!(self.kind, Kind::Int { bits, bit: true, .. } if bits > 32)
    }

    fn has_literal(&self) -> bool {
        self.suffix.is_some()
    }

    /// The smallest and largest value of an integer type.
    fn range(&self) -> Option<(i128, i128)> {
        match self.kind {
            Kind::Int { bits, signed: true, .. } => Some((-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1)),
            Kind::Int {
                bits, signed: false, ..
            } => Some((0, (1i128 << bits) - 1)),
            Kind::Float(_) => None,
        }
    }
}

/// A value, as the generator writes it in a literal.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i128),
    /// The text of a float literal without its suffix; an exponent letter (`E`, `D`, `F`) types it by itself.
    Float(String),
}

impl Value {
    /// The value as a double (infinite beyond the double range), for the generator's own decisions.
    fn approx(&self) -> f64 {
        match self {
            #[expect(clippy::cast_precision_loss, reason = "an estimate for the exclusion rules")]
            Value::Int(v) => *v as f64,
            Value::Float(text) => text.replace(['D', 'F'], "E").parse().expect("float literal"),
        }
    }
}

/// The literal of `v` in type `t`: digits and suffix, a float with an exponent letter without one. The smallest
/// `_INTEGER64` is written as `(-9223372036854775807&& - 1)`, as its digits are no C++ literal.
pub fn literal(t: &NumTy, v: &Value) -> String {
    match v {
        Value::Int(n) => {
            let suffix = match (t.suffix, t.kind) {
                (Some(s), _) => s,
                (None, Kind::Int { signed: true, .. }) => "&&",
                (None, Kind::Int { signed: false, .. } | Kind::Float(_)) => "~&&",
            };
            if *n == i128::from(i64::MIN) {
                format!("(-9223372036854775807{suffix} - 1)")
            } else {
                format!("{n}{suffix}")
            }
        }
        Value::Float(text) if text.contains(['E', 'D', 'F']) => text.clone(),
        Value::Float(text) => format!("{text}{}", t.suffix.unwrap_or("")),
    }
}

/// `text` in parentheses when it starts with a minus sign, so that it stays one operand (`-2 ^ 2` is `-(2 ^ 2)`).
fn operand(text: String) -> String {
    if text.starts_with('-') {
        format!("({text})")
    } else {
        text
    }
}

/// xorshift64*: small, fast, and the same on every platform (as in the driver's mutation test).
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Rng {
        // Mix the seed so that neighbouring seeds give unrelated streams; never zero.
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        Rng((z ^ (z >> 31)) | 1)
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A number in `0..n` (`n` > 0).
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// A random value of `t`: uniform over an integer type's range; for a float, a decimal of up to the type's
/// significant digits with the point at a random place.
fn random(t: &NumTy, rng: &mut Rng) -> Value {
    match t.kind {
        Kind::Int { bits, signed, .. } => {
            let raw = rng.next();
            let v = i128::from(if bits == 64 { raw } else { raw & ((1u64 << bits) - 1) });
            Value::Int(if signed && v >= 1i128 << (bits - 1) {
                v - (1i128 << bits)
            } else {
                v
            })
        }
        Kind::Float(k) => {
            let max_digits = match k {
                FloatKind::Single => 7,
                FloatKind::Double => 15,
                FloatKind::Float => 18,
            };
            let n = 1 + rng.below(max_digits);
            let mut digits = String::new();
            for i in 0..n {
                let d = if i == 0 { 1 + rng.below(9) } else { rng.below(10) };
                digits.push(char::from(b'0' + u8::try_from(d).expect("a digit")));
            }
            let point = usize::try_from(rng.below(n + 1)).expect("small");
            let sign = if rng.next() & 1 == 1 { "-" } else { "" };
            Value::Float(match point {
                0 => format!("{sign}0.{digits}"),
                p if p == digits.len() => format!("{sign}{digits}"),
                p => format!("{sign}{}.{}", &digits[..p], &digits[p..]),
            })
        }
    }
}

/// The value slots of every type: one variable each in the programs that use variables.
pub const SLOTS: [&str; 7] = ["min", "max", "m1", "zero", "one", "r1", "r2"];

/// The values of `t`'s slots ([`SLOTS`]): the extremes (for a float the largest finite literal), -1 (for an unsigned
/// type its largest value, the bits of -1), 0, 1 (0 for the 1-bit `_BIT`, whose range is -1 to 0) and two seeded
/// random values.
pub fn values(t: &NumTy) -> [Value; 7] {
    let index = TYPES.iter().position(|x| x.key == t.key).expect("a type of TYPES");
    let mut rng = Rng::new(SEED ^ (u64::try_from(index).expect("small") + 1).wrapping_mul(0xC2B2_AE3D_27D4_EB4F));
    let (min, max, m1, zero, one) = match (t.range(), t.kind) {
        (Some((lo, hi)), Kind::Int { signed, .. }) => (
            Value::Int(lo),
            Value::Int(hi),
            Value::Int(if signed { -1 } else { hi }),
            Value::Int(0),
            Value::Int(hi.min(1)),
        ),
        (_, kind) => {
            let max = match kind {
                Kind::Float(FloatKind::Single) => "3.402823E+38",
                Kind::Float(FloatKind::Double) => "1.797693134862315D+308",
                // The largest literal the old compiler can write: it emits a `_FLOAT` literal as a C++ double
                // (measured: `1.18973149535723176F+4932` is infinite, see `beyond`).
                Kind::Float(FloatKind::Float) | Kind::Int { .. } => "1.797693134862315F+308",
            };
            let f = |s: &str| Value::Float(s.to_string());
            (f(&format!("-{max}")), f(max), f("-1"), f("0"), f("1"))
        }
    };
    let r1 = random(t, &mut rng);
    let r2 = random(t, &mut rng);
    [min, max, m1, zero, one, r1, r2]
}

/// The value pairs of every type pair, as slot indexes into [`SLOTS`]: min/min, max/max, min/max, max/min, -1/1,
/// 0/1 and two random pairs.
pub const PAIRS: [(usize, usize); 8] = [(0, 0), (1, 1), (0, 1), (1, 0), (2, 4), (3, 4), (5, 6), (6, 5)];

/// One step beyond a type's range, for the literals of the `fold` and `print` groups: an integer type's next values;
/// for `_FLOAT` the extremes of `long double`, which the old compiler writes as an infinite double. `None` for a
/// 64-bit type, whose next value is no literal of the old compiler (`18446744073709551616~&&` fails its C++ build),
/// and for `SINGLE` and `DOUBLE`.
fn beyond(t: &NumTy) -> Option<(Value, Value)> {
    match (t.kind, t.range()) {
        (Kind::Int { bits, .. }, Some((lo, hi))) if bits < 64 => Some((Value::Int(lo - 1), Value::Int(hi + 1))),
        (Kind::Float(FloatKind::Float), _) => Some((
            Value::Float("-1.18973149535723176F+4932".into()),
            Value::Float("1.18973149535723176F+4932".into()),
        )),
        _ => None,
    }
}

/// How an operator's exclusions work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpClass {
    /// `\` and `MOD`: no divisor that rounds to 0, no smallest integer by -1.
    IntDiv,
    /// `^`: no `_OFFSET` operand.
    Pow,
    Other,
}

#[derive(Debug)]
pub struct Op {
    /// The program's file name.
    pub file: &'static str,
    pub text: &'static str,
    pub class: OpClass,
}

const fn op(file: &'static str, text: &'static str) -> Op {
    Op {
        file,
        text,
        class: OpClass::Other,
    }
}

/// Every binary operator.
pub const OPS: &[Op] = &[
    op("add", "+"),
    op("sub", "-"),
    op("mul", "*"),
    op("div", "/"),
    Op {
        file: "idiv",
        text: "\\",
        class: OpClass::IntDiv,
    },
    Op {
        file: "mod",
        text: "MOD",
        class: OpClass::IntDiv,
    },
    Op {
        file: "pow",
        text: "^",
        class: OpClass::Pow,
    },
    op("eq", "="),
    op("ne", "<>"),
    op("lt", "<"),
    op("gt", ">"),
    op("le", "<="),
    op("ge", ">="),
    op("and", "AND"),
    op("or", "OR"),
    op("xor", "XOR"),
    op("eqv", "EQV"),
    op("imp", "IMP"),
    op("andalso", "_ANDALSO"),
    op("orelse", "_ORELSE"),
];

/// The unary operators.
pub const UNARY: &[&str] = &["-", "NOT", "_NEGATE"];

/// Why a value pair or type pair is left out of a program.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Skip {
    ByZero,
    D006,
    OffsetPow,
}

fn is_offset(t: &NumTy) -> bool {
    t.key.ends_with("OFFSET")
}

/// Whether `a op b` is left out (spec `testing/differential-tests`, "Safe programs").
fn skip(op: &Op, l: &NumTy, r: &NumTy, a: &Value, b: &Value) -> Option<Skip> {
    match op.class {
        OpClass::Pow if is_offset(l) || is_offset(r) => Some(Skip::OffsetPow),
        OpClass::IntDiv => {
            let d = b.approx();
            // Rounded half to even, a divisor in [-0.5, 0.5] is 0 and one in [-1.5, -0.5] may be -1.
            if d.abs() <= 0.5 {
                return Some(Skip::ByZero);
            }
            let smallest = match a {
                Value::Int(v) => *v == i128::from(i32::MIN) || *v == i128::from(i64::MIN),
                // A float is rounded to an integer first; one that rounds to the smallest LONG or beyond LONG may end
                // up as the smallest one (half to even: from 2147483647.5 on, and from -2147483647.5 down).
                Value::Float(_) => a.approx().abs() >= 2_147_483_647.5,
            };
            (smallest && (-1.5..=-0.5).contains(&d)).then_some(Skip::D006)
        }
        OpClass::Pow | OpClass::Other => None,
    }
}

/// One generated file: its path below `tests\differential` (with `/`) and its text.
#[derive(Debug)]
pub struct File {
    pub path: String,
    pub text: String,
}

/// A program being written: the body first, the header (which names what was left out) once the body is known.
struct Program {
    body: String,
    by_zero: usize,
    d006: usize,
    offset_pow: usize,
    pads: bool,
}

impl Program {
    fn new() -> Program {
        Program {
            body: String::new(),
            by_zero: 0,
            d006: 0,
            offset_pow: 0,
            pads: false,
        }
    }

    fn line(&mut self, s: &str) {
        self.body.push_str(s);
        self.body.push('\n');
    }

    fn count(&mut self, s: Skip) {
        match s {
            Skip::ByZero => self.by_zero += 1,
            Skip::D006 => self.d006 += 1,
            Skip::OffsetPow => self.offset_pow += 1,
        }
    }

    /// `DIM name AS type`, with the D-009 pad before a wide `_BIT`.
    fn dim(&mut self, name: &str, t: &NumTy) {
        if t.needs_pad() {
            self.pads = true;
            self.line(&format!("DIM pad_{name} AS _BIT * 32"));
        }
        self.line(&format!("DIM {name} AS {}", t.decl));
    }

    /// Every type's slot variables, DIMmed and assigned from literals of their own type.
    fn slot_variables(&mut self, ts: &[&NumTy]) {
        for &t in ts {
            for slot in SLOTS {
                self.dim(&var(t, slot), t);
            }
        }
        for &t in ts {
            for (slot, v) in SLOTS.iter().zip(values(t)) {
                self.line(&format!("{} = {}", var(t, slot), literal(t, &v)));
            }
        }
    }

    fn finish(self, what: &str) -> String {
        let mut out = String::from("$CONSOLE:ONLY\n");
        out.push_str(&format!(
            "' Differential program (m2-numeric-types, design D2): {what}\n"
        ));
        out.push_str(
            "' Generated by crates/difftest (cargo run -p qb64rust-difftest -- gen): do not edit. The expected output\n\
             ' is recorded with qb64pe.exe (tests/differential/README.md). A runtime error prints ERR and resumes next.\n",
        );
        if self.by_zero > 0 {
            out.push_str(&format!(
                "' Left out: {} value pairs whose divisor rounds to 0 (fatal in the old compiler, design D2).\n",
                self.by_zero
            ));
        }
        if self.d006 > 0 {
            out.push_str(&format!(
                "' Left out: {} value pairs of the smallest LONG or _INTEGER64 (or a float beyond LONG) and a divisor\n\
                 ' of -1, which crash the old program (DIVERGENCES.md D-006).\n",
                self.d006
            ));
        }
        if self.offset_pow > 0 {
            out.push_str(&format!(
                "' Left out: the {} type pairs with an _OFFSET operand (\"Operator '^' cannot be used with an _OFFSET\").\n",
                self.offset_pow
            ));
        }
        if self.pads {
            out.push_str(
                "' Each _BIT * 40 has a _BIT * 32 pad DIMmed just before it, which the old compiler's overlap hits\n\
                 ' instead (DIVERGENCES.md D-009).\n",
            );
        }
        out.push_str("ON ERROR GOTO h\n");
        out.push_str(&self.body);
        out.push_str("SYSTEM\nh:\nPRINT \" error\"; ERR\nRESUME NEXT\n");
        out
    }
}

/// The variable of `t`'s `slot`.
fn var(t: &NumTy, slot: &str) -> String {
    format!("v{}_{slot}", t.key.to_ascii_lowercase())
}

/// `ops\<op>.bas`: `op` on every ordered pair of types, the operands in variables.
fn ops_program(op: &Op, ts: &[&NumTy]) -> String {
    let mut p = Program::new();
    p.slot_variables(ts);
    for &l in ts {
        let lv = values(l);
        for &r in ts {
            let rv = values(r);
            if op.class == OpClass::Pow && (is_offset(l) || is_offset(r)) {
                p.count(Skip::OffsetPow);
                continue;
            }
            for (i, &(a, b)) in PAIRS.iter().enumerate() {
                if let Some(s) = skip(op, l, r, &lv[a], &rv[b]) {
                    p.count(s);
                    continue;
                }
                p.line(&format!(
                    "PRINT \"{} {} {} {}:\"; {} {} {}",
                    op.text,
                    l.key,
                    r.key,
                    i + 1,
                    var(l, SLOTS[a]),
                    op.text,
                    var(r, SLOTS[b])
                ));
            }
        }
    }
    p.finish(&format!(
        "{} on every ordered pair of numeric types, operands in variables.",
        op.text
    ))
}

/// The values of `t` used as literals: its slots, then one step below and above its range where that is a
/// literal.
fn literal_values(t: &NumTy) -> Vec<(String, Value)> {
    let mut out: Vec<(String, Value)> = SLOTS.iter().map(|s| s.to_string()).zip(values(t)).collect();
    if let Some((lo, hi)) = beyond(t) {
        out.push(("below".into(), lo));
        out.push(("above".into(), hi));
    }
    out
}

/// `fold\<op>.bas`: as `ops`, the operands written as literals (every type with a literal suffix), plus one step
/// beyond each type's range.
fn fold_program(op: &Op, ts: &[&NumTy]) -> String {
    let mut p = Program::new();
    for &l in ts.iter().filter(|t| t.has_literal()) {
        let lv = values(l);
        let lb = beyond(l);
        for &r in ts.iter().filter(|t| t.has_literal()) {
            let rv = values(r);
            let rb = beyond(r);
            let mut pairs: Vec<(&Value, &Value)> = PAIRS.iter().map(|&(a, b)| (&lv[a], &rv[b])).collect();
            // Beyond the range: above/below and below/above, falling back to the extreme on a side that has none.
            if lb.is_some() || rb.is_some() {
                let (llo, lhi) = lb.as_ref().map_or((&lv[0], &lv[1]), |(lo, hi)| (lo, hi));
                let (rlo, rhi) = rb.as_ref().map_or((&rv[0], &rv[1]), |(lo, hi)| (lo, hi));
                pairs.push((lhi, rlo));
                pairs.push((llo, rhi));
            }
            for (i, (a, b)) in pairs.into_iter().enumerate() {
                if let Some(s) = skip(op, l, r, a, b) {
                    p.count(s);
                    continue;
                }
                p.line(&format!(
                    "PRINT \"{} {} {} {}:\"; {} {} {}",
                    op.text,
                    l.key,
                    r.key,
                    i + 1,
                    operand(literal(l, a)),
                    op.text,
                    operand(literal(r, b))
                ));
            }
        }
    }
    p.finish(&format!(
        "{} on every ordered pair of numeric types with a literal suffix, operands written as literals \
         (constant folding against the old compiler, which does not fold).",
        op.text
    ))
}

/// `unary\unary.bas`: every unary operator on every type's values.
fn unary_program(ts: &[&NumTy]) -> String {
    let mut p = Program::new();
    p.slot_variables(ts);
    for op in UNARY {
        let sep = if op.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
            " "
        } else {
            ""
        };
        for &t in ts {
            for slot in SLOTS {
                p.line(&format!("PRINT \"{op} {} {slot}:\"; {op}{sep}{}", t.key, var(t, slot)));
            }
        }
    }
    p.finish("-, NOT and _NEGATE on every numeric type's values.")
}

/// `store\<target>.bas`: every type's values stored into a variable of `target` and printed.
fn store_program(target: &NumTy, ts: &[&NumTy]) -> String {
    let mut p = Program::new();
    p.slot_variables(ts);
    p.dim("t", target);
    for &s in ts {
        for slot in SLOTS {
            p.line(&format!("PRINT \"= {} {} {slot}:\";", target.key, s.key));
            p.line(&format!("t = {}", var(s, slot)));
            p.line("PRINT t");
        }
    }
    p.finish(&format!("every numeric type's values stored into {}.", target.decl))
}

/// `print\print.bas`: `PRINT` and `STR$` of every type's values, from variables and as literals.
fn print_program(ts: &[&NumTy]) -> String {
    let mut p = Program::new();
    p.slot_variables(ts);
    for &t in ts {
        for slot in SLOTS {
            p.line(&format!("PRINT \"PRINT {} {slot}:\"; {}", t.key, var(t, slot)));
            p.line(&format!("PRINT \"STR$ {} {slot}:\"; STR$({})", t.key, var(t, slot)));
        }
    }
    for &t in ts.iter().filter(|t| t.has_literal()) {
        for (slot, v) in literal_values(t) {
            let lit = literal(t, &v);
            p.line(&format!("PRINT \"PRINT literal {} {slot}:\"; {lit}", t.key));
            p.line(&format!("PRINT \"STR$ literal {} {slot}:\"; STR$({lit})", t.key));
        }
    }
    p.finish("PRINT and STR$ of every numeric type's values, from variables and as literals.")
}

/// The six numeric types of the old `sema` (`INTEGER`, `LONG`, `_INTEGER64`, `SINGLE`, `DOUBLE`, `_FLOAT`): the
/// `old6` group repeats every program for these alone, so that what today's compiler handles is checked in tier 2
/// while the full programs cannot compile yet (`DECISIONS.md` 2026-10-08).
pub const OLD6: [&str; 6] = ["INTEGER", "LONG", "INT64", "SINGLE", "DOUBLE", "FLOAT"];

/// Every differential program, in a fixed order: the full programs, then the `old6` group (`<group>/<name>.bas` of
/// the six-type subset as `old6/<group>_<name>.bas`, with a header line naming the subset).
pub fn generate() -> Vec<File> {
    let mut out = generate_for(|_| true);
    out.extend(generate_for(|t| OLD6.contains(&t.key)).into_iter().map(|f| File {
        path: format!("old6/{}", f.path.replace('/', "_")),
        text: f.text.replacen(
            "$CONSOLE:ONLY\n",
            "$CONSOLE:ONLY\n' Group old6: the program for the six numeric types INTEGER, LONG, _INTEGER64, SINGLE, \
             DOUBLE and _FLOAT only.\n",
            1,
        ),
    }));
    out
}

/// The programs restricted to the types `keep` selects (`gen --types`), for a look at part of the type set: the
/// same values and lines as [`generate`] for those types, but not the files tier 1 and tier 2 use.
pub fn generate_for(keep: impl Fn(&NumTy) -> bool) -> Vec<File> {
    let ts: Vec<&NumTy> = TYPES.iter().filter(|t| keep(t)).collect();
    let mut out = Vec::new();
    for op in OPS {
        out.push(File {
            path: format!("ops/{}.bas", op.file),
            text: ops_program(op, &ts),
        });
    }
    out.push(File {
        path: "unary/unary.bas".into(),
        text: unary_program(&ts),
    });
    for &t in &ts {
        out.push(File {
            path: format!("store/{}.bas", t.key.to_ascii_lowercase()),
            text: store_program(t, &ts),
        });
    }
    for op in OPS {
        out.push(File {
            path: format!("fold/{}.bas", op.file),
            text: fold_program(op, &ts),
        });
    }
    out.push(File {
        path: "print/print.bas".into(),
        text: print_program(&ts),
    });
    out
}

/// The `.bas` files below `dir`'s group folders, as paths relative to `dir` with `/`, sorted.
pub fn existing_programs(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Ok(groups) = std::fs::read_dir(dir) else {
        return out;
    };
    for g in groups.flatten() {
        if !g.path().is_dir() {
            continue;
        }
        for f in std::fs::read_dir(g.path()).into_iter().flatten().flatten() {
            let p = f.path();
            if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("bas")) {
                out.push(format!(
                    "{}/{}",
                    g.file_name().to_string_lossy(),
                    p.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
        }
    }
    out.sort();
    out
}

/// What differs between [`generate`] and the programs in `dir`: missing, changed and extra `.bas` files, one line
/// each. Empty when the files are fresh.
pub fn stale(dir: &Path) -> Vec<String> {
    let files = generate();
    let mut problems = Vec::new();
    for f in &files {
        match std::fs::read(dir.join(&f.path)) {
            Err(_) => problems.push(format!("missing: {}", f.path)),
            Ok(bytes) if bytes != f.text.as_bytes() => problems.push(format!("differs from the generator: {}", f.path)),
            Ok(_) => {}
        }
    }
    for p in existing_programs(dir) {
        if !files.iter().any(|f| f.path == p) {
            problems.push(format!("not generated (remove it and its recording): {p}"));
        }
    }
    problems
}

/// The generated programs in `dir` without a recording (`.output` or `.err` beside them).
pub fn unrecorded(dir: &Path) -> Vec<String> {
    generate()
        .into_iter()
        .map(|f| f.path)
        .filter(|p| {
            let bas = dir.join(p);
            !bas.with_extension("output").is_file() && !bas.with_extension("err").is_file()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_numeric_sema_type_is_covered() {
        let bit = |width, signed| Ty::Bit { width, signed };
        let all = [
            Ty::I8,
            Ty::U8,
            Ty::I16,
            Ty::U16,
            Ty::I32,
            Ty::U32,
            Ty::I64,
            Ty::U64,
            Ty::Off,
            Ty::UOff,
            bit(1, true),
            bit(7, false),
            bit(24, true),
            bit(40, false),
            Ty::F32,
            Ty::F64,
            Ty::F80,
        ];
        let mut keys: Vec<&str> = all
            .iter()
            .map(|&ty| for_sema(ty).unwrap_or_else(|| panic!("{ty:?}")).key)
            .collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), TYPES.len(), "every generator type is some `sema` type");
        assert!(for_sema(Ty::Str).is_none());
        assert!(for_sema(Ty::FixedStr(4)).is_none());
    }

    #[test]
    fn generation_is_deterministic() {
        let a = generate();
        let b = generate();
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            assert_eq!((&x.path, &x.text), (&y.path, &y.text));
        }
    }

    /// Spec "Every pair covered": each `ops` program has a line for every ordered pair of types (`^` none with an
    /// `_OFFSET` operand, named in its header), and each `store` program one for every source type.
    #[test]
    fn every_pair_is_covered() {
        let files = generate();
        for op in OPS {
            let f = files.iter().find(|f| f.path == format!("ops/{}.bas", op.file)).unwrap();
            for l in TYPES {
                for r in TYPES {
                    let has = f.text.contains(&format!("PRINT \"{} {} {} ", op.text, l.key, r.key));
                    let offset_pow = op.class == OpClass::Pow && (is_offset(l) || is_offset(r));
                    assert_eq!(has, !offset_pow, "{} {} {}", op.text, l.key, r.key);
                }
            }
            if op.class == OpClass::Pow {
                assert!(f.text.contains("cannot be used with an _OFFSET"));
            }
        }
        for target in TYPES {
            let path = format!("store/{}.bas", target.key.to_ascii_lowercase());
            let f = files.iter().find(|f| f.path == path).unwrap();
            for s in TYPES {
                assert!(
                    f.text.contains(&format!("\"= {} {} ", target.key, s.key)),
                    "{path}: {}",
                    s.key
                );
            }
        }
    }

    /// Spec "Safe programs" and "Excluded case named".
    #[test]
    fn programs_are_safe() {
        for f in generate() {
            for line in f.text.lines().filter(|l| l.starts_with("PRINT")) {
                // No PRINT comma: every comma would be inside a string or a call; there are none.
                assert!(!line.contains(','), "{}: {line}", f.path);
            }
            assert!(f.text.contains("ON ERROR GOTO h\n") && f.text.ends_with("RESUME NEXT\n"));
            let pads = f.text.matches("DIM pad_").count();
            let wide = f.text.matches(" AS _UNSIGNED _BIT * 40\n").count();
            assert_eq!(pads, wide, "{}", f.path);
            if wide > 0 {
                assert!(f.text.contains("D-009"), "{}", f.path);
            }
        }
        let files = generate();
        let idiv = files.iter().find(|f| f.path == "ops/idiv.bas").unwrap();
        assert!(idiv.text.contains("D-006") && !idiv.text.contains("vlong_min \\ vlong_m1"));
        assert!(!idiv.text.contains("\\ vlong_zero"));
    }

    /// The `old6` group: one program per full program, for the six types only.
    #[test]
    fn old6_is_the_six_type_subset() {
        let files = generate();
        let full: Vec<&File> = files.iter().filter(|f| !f.path.starts_with("old6/")).collect();
        let old6: Vec<&File> = files.iter().filter(|f| f.path.starts_with("old6/")).collect();
        assert_eq!(
            old6.len(),
            full.len() - (TYPES.len() - OLD6.len()),
            "one store program per kept type"
        );
        for k in OLD6 {
            assert!(TYPES.iter().any(|t| t.key == k), "{k}");
        }
        for f in &old6 {
            assert!(f.text.contains("' Group old6:"), "{}", f.path);
            for t in TYPES.iter().filter(|t| !OLD6.contains(&t.key)) {
                assert!(!f.text.contains(&format!(" AS {}\n", t.decl)), "{}: {}", f.path, t.key);
            }
        }
    }

    #[test]
    fn literals() {
        let t = |k: &str| TYPES.iter().find(|t| t.key == k).unwrap();
        assert_eq!(literal(t("UBYTE"), &Value::Int(255)), "255~%%");
        assert_eq!(
            literal(t("INT64"), &Value::Int(i128::from(i64::MIN))),
            "(-9223372036854775807&& - 1)"
        );
        assert_eq!(literal(t("UOFFSET"), &Value::Int(7)), "7~&&");
        assert_eq!(literal(t("SINGLE"), &Value::Float("1.5".into())), "1.5!");
        assert_eq!(literal(t("DOUBLE"), &Value::Float("1D+3".into())), "1D+3");
        assert_eq!(values(t("BIT"))[..5], [-1, 0, -1, 0, 0].map(Value::Int));
        assert_eq!(values(t("UBIT7"))[..5], [0, 127, 127, 0, 1].map(Value::Int));
    }
}
