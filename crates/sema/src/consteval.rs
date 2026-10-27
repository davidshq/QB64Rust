//! The arithmetic of the old compiler's `CONST` evaluator (`const_eval.bas`, `elements.bas`; design D2 of
//! `m3-constant-evaluation`), on values; the walk over the tree is `check\constants.rs`.
//!
//! The old evaluator holds each value as a `_FLOAT`, an `_INTEGER64` and an `_UNSIGNED _INTEGER64` with a type,
//! and an operator works in one of three lanes: on floats when either operand is one, else on unsigned 64-bit
//! integers when either is unsigned, else on signed ones (`/` always on floats). Floats are x87 extended values
//! ([`F80`], to the bit). Each parenthesised group, function result and whole value is printed with `_TOSTR$` and
//! read back, typed by its text ([`reread`]).
//!
//! A float is a [`Range`]: the values the old compiler might hold. It is one value except after a function whose
//! library is not correctly rounded (`certify.rs`). Where a re-read, a comparison or a conversion to an integer
//! settles a range to one outcome, the constant is the old compiler's on any machine; where it does not, the
//! outcome of the correctly rounded value is taken and noted ([`take_fallback`]; `QB64PE-DIVERGENCES.md` D-026, the user,
//! 2026-10-18), as the old compiler's own value then depends on the CPU and the system's library.

use crate::f80::{F80, Integral};
use crate::literal::{self, LitError, NumLit};
use crate::{BinOp, Ty, UnOp};
use std::cmp::Ordering;

/// A value of the evaluator.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// The signed lane.
    Int(i64),
    /// The unsigned lane.
    UInt(u64),
    /// The float lane: the value and the float type its text gives it.
    Float(Range, FloatTy),
    Str(Vec<u8>),
}

/// The float type the old evaluator reads from a number's text (`elementGetNumericValue`): `F` or `##` a
/// `_FLOAT`, `E` or `!` a SINGLE, `D`, `#` or a point a DOUBLE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatTy {
    Single,
    Double,
    Float,
}

impl FloatTy {
    pub fn ty(self) -> Ty {
        match self {
            FloatTy::Single => Ty::F32,
            FloatTy::Double => Ty::F64,
            FloatTy::Float => Ty::F80,
        }
    }
}

/// The values a float of the old compiler might be, `lo <= hi` (one value when they are equal), and `mid`, the value
/// a correctly rounded library gives, which is taken where the others do not settle to one outcome (the user,
/// `DECISIONS.md` 2026-10-18; `QB64PE-DIVERGENCES.md` D-026).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    pub lo: F80,
    pub hi: F80,
    pub mid: F80,
}

impl Range {
    pub fn exact(v: F80) -> Range {
        Range { lo: v, hi: v, mid: v }
    }

    /// The value, when it is one.
    pub fn one(self) -> Option<F80> {
        (self.lo == self.hi).then_some(self.lo)
    }

    /// The value: the one value, or the correctly rounded one (noted, D-026).
    pub fn settle(self) -> F80 {
        self.one().unwrap_or_else(|| {
            fell_back();
            self.mid
        })
    }
}

thread_local! {
    static FELL_BACK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Notes that a value was settled to its correctly rounded value (D-026).
pub fn fell_back() {
    FELL_BACK.with(|c| c.set(true));
}

/// Whether a value was settled to its correctly rounded value since the last call.
pub fn take_fallback() -> bool {
    FELL_BACK.with(|c| c.replace(false))
}

impl Value {
    pub fn float(v: F80, ty: FloatTy) -> Value {
        Value::Float(Range::exact(v), ty)
    }
}

/// Why a value cannot be computed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    /// The old compiler rejects the expression too.
    Error(String),
    /// Not supported yet; names what.
    Unsupported(String),
}

pub type R<T> = Result<T, Problem>;

pub fn unsupported<T>(what: &str) -> R<T> {
    Err(Problem::Unsupported(what.to_string()))
}

pub fn error<T>(msg: &str) -> R<T> {
    Err(Problem::Error(msg.to_string()))
}

// ---- Numbers from text ----

/// A number literal of the source as the old evaluator reads it (`elementGetNumericValue`): an integer with a suffix
/// in its lane (unsigned when the suffix is), a plain integer as an `_INTEGER64`, a float by `VAL` in `_FLOAT`, typed
/// by its letter or suffix. A plain integer beyond `_INTEGER64` is read as a statement reads it, in the unsigned
/// lane, and beyond `_UNSIGNED _INTEGER64` it is an error (`QB64PE-DIVERGENCES.md` D-027: the old evaluator clamps it to
/// 9223372036854775807, measured, `verification\v45_a_lanes`).
pub fn literal(text: &[u8]) -> R<Value> {
    let plain = !text.is_empty() && text.iter().all(u8::is_ascii_digit);
    if plain {
        let digits: Vec<u8> = text.iter().copied().skip_while(|&c| c == b'0').collect();
        // At most 20 digits can be within 64 bits; a `u128` holds them with room to spare.
        let n = (digits.len() <= 20).then(|| digits.iter().fold(0u128, |v, &c| v * 10 + u128::from(c - b'0')));
        return match n {
            Some(n) if n <= u128::from(i64::MAX.cast_unsigned()) => Ok(Value::Int(i64::try_from(n).expect("in range"))),
            Some(n) if n <= u128::from(u64::MAX) => Ok(Value::UInt(u64::try_from(n).expect("in range"))),
            _ => error("this number is beyond `_UNSIGNED _INTEGER64`'s range"),
        };
    }
    match literal::number(text, false) {
        Ok(NumLit::Int { value, qb, .. }) => Ok(if qb.is_unsigned() {
            Value::UInt(value.cast_unsigned())
        } else {
            Value::Int(value)
        }),
        Ok(NumLit::Float { text, ty }) => Ok(Value::float(decimal(&text), fty_of(ty))),
        Err(LitError::Overflow) => error("overflow"),
        Err(LitError::Error(msg)) => error(msg),
        Err(LitError::Unsupported(what)) => unsupported(what),
    }
}

/// A decimal in the C form of [`literal::NumLit::Float`] (`-1.5E+2`), or any `[-]d[.d][E±n]`, read as `%Lf` reads it.
pub fn decimal(text: &str) -> F80 {
    let (neg, rest) = text.strip_prefix('-').map_or((false, text), |r| (true, r));
    let (mant, exp) = match rest.find(['E', 'e', 'D', 'd', 'F', 'f']) {
        Some(i) => (&rest[..i], rest[i + 1..].parse::<i64>().unwrap_or(0)),
        None => (rest, 0),
    };
    let (whole, frac) = mant.split_once('.').unwrap_or((mant, ""));
    let digits = format!("{whole}{frac}");
    let frac_len = i64::try_from(frac.len()).expect("short");
    F80::from_decimal(neg, digits.as_bytes(), exp - frac_len)
}

/// `_TOSTR$` of a `_FLOAT` (`%.19LG`, then `E` made `F`), in the C form here (`E` kept).
pub fn g19(v: F80) -> String {
    v.to_g19()
}

/// The `double` C++ reads from the 19-digit text the old compiler writes for a SINGLE or DOUBLE constant.
pub fn double_of(v: F80) -> f64 {
    let t = v.to_g19();
    t.parse::<f64>().unwrap_or_else(|_| v.to_f64())
}

/// The text the old compiler writes into C++ for a float constant (`qb64pe.bas` 24063–24094): `_TOSTR$`'s 19
/// digits as they are, a point and `0` added where there is none, `E+0` after plain digits
/// (`0.3333333333333333333E+0`, `3.333333333333333333E+29`).
pub fn c_text(v: F80) -> String {
    let t = v.to_g19();
    match t.split_once('E') {
        Some((mant, exp)) => {
            let mant = if mant.contains('.') {
                mant.to_string()
            } else {
                format!("{mant}.0")
            };
            let exp = if exp.starts_with(['+', '-']) {
                exp.to_string()
            } else {
                format!("+{exp}")
            };
            format!("{mant}E{exp}")
        }
        None => {
            let t = if t.contains('.') { t } else { format!("{t}.0") };
            format!("{t}E+0")
        }
    }
}

/// The shortest text that reads back as `v`, in the C form `d.dddE+n`.
pub fn shortest_text(v: f64) -> String {
    let rust = format!("{v:e}");
    let (mant, exp) = rust.split_once('e').expect("exponent form");
    let mant = if mant.contains('.') {
        mant.to_string()
    } else {
        format!("{mant}.0")
    };
    let exp = if exp.starts_with('-') {
        exp.to_string()
    } else {
        format!("+{exp}")
    };
    format!("{mant}E{exp}")
}

/// What the old evaluator does with a value it prints and reads back (a parenthesised group, a function result,
/// the whole value; design D2): an integer comes back in its lane; a float as `_TOSTR$` prints it, read as its
/// text says: an exponent a `_FLOAT`, a point a DOUBLE, plain digits an `_INTEGER64` within its range, and beyond
/// it the `_FLOAT` it is (`QB64PE-DIVERGENCES.md` D-027: the old evaluator clamps it). `INF` is an error, as the whole
/// value too (D-029: there `VAL` reads it as 0, measured, `verification\v45_x25`, `x38`). `NAN` inside an
/// expression is an error, and as the whole value 0.
pub fn reread(v: Value, outermost: bool) -> R<Value> {
    let Value::Float(r, _) = v else {
        return Ok(v);
    };
    let (a, b) = (r.lo.to_g19(), r.hi.to_g19());
    if a != b {
        // The libraries' possible results print differently: the correctly rounded one (D-026).
        fell_back();
        return read_text(&r.mid.to_g19(), outermost);
    }
    read_text(&a, outermost)
}

/// The value of a text `_TOSTR$` gave (design D2).
fn read_text(t: &str, outermost: bool) -> R<Value> {
    let upper = t.trim_start_matches('-');
    if upper == "INF" {
        return error("this `CONST` overflows: its value is beyond `_FLOAT`'s range");
    }
    if upper == "NAN" {
        if !outermost {
            return error(&format!("the value `{t}` cannot be used in a `CONST`"));
        }
        // `VAL` of `NAN` (plain, an `_INTEGER64`) is 0.
        return Ok(Value::Int(0));
    }
    if t.contains('E') {
        return Ok(Value::float(decimal(t), FloatTy::Float));
    }
    if t.contains('.') {
        return Ok(Value::float(decimal(t), FloatTy::Double));
    }
    // Plain digits, perhaps with a minus.
    Ok(match t.parse::<i64>() {
        Ok(v) => Value::Int(v),
        Err(_) => Value::float(decimal(t), FloatTy::Float),
    })
}

// ---- The lanes ----

/// `qbr` (libqb's `rounding.h`): a `_FLOAT` to `_INTEGER64` as the x87's `fistp` stores it, half to even; above
/// 2^63 - 1 the value less 2^63 with the top bit set; out of range or not a number `i64::MIN`.
pub fn qbr(f: F80) -> i64 {
    let max = F80::from_i64(i64::MAX);
    let two63 = F80::from_u64(1 << 63);
    let (f, top) = if f.partial_cmp(max) == Some(Ordering::Greater) {
        (f - two63, true)
    } else {
        (f, false)
    };
    let i = fistp(f);
    if top { i | i64::MIN } else { i }
}

fn fistp(f: F80) -> i64 {
    let r = f.round_integral(Integral::NearestEven);
    match r.to_i128() {
        Some(v) => i64::try_from(v).unwrap_or(i64::MIN),
        None => i64::MIN,
    }
}

/// `qbr` of a range, when every value gives the same integer.
fn qbr_range(r: Range) -> R<i64> {
    let (a, b) = (qbr(r.lo), qbr(r.hi));
    if a == b {
        Ok(a)
    } else {
        fell_back();
        Ok(qbr(r.mid))
    }
}

/// A value in the signed lane (`num.i`).
pub fn as_i64(v: &Value) -> R<i64> {
    match v {
        Value::Int(i) => Ok(*i),
        Value::UInt(u) => Ok(u.cast_signed()),
        Value::Float(r, _) => qbr_range(*r),
        Value::Str(_) => unreachable!("strings are handled first"),
    }
}

/// A value in the unsigned lane (`num.ui`; from a float by `qbr` too, measured: `-1.5 OR 0~&&` is 2^64 - 2).
pub fn as_u64(v: &Value) -> R<u64> {
    Ok(as_i64(v)?.cast_unsigned())
}

/// A value in the float lane (`num.f`): an integer exactly.
pub fn as_float(v: &Value) -> Range {
    match v {
        Value::Int(i) => Range::exact(F80::from_i64(*i)),
        Value::UInt(u) => Range::exact(F80::from_u64(*u)),
        Value::Float(r, _) => *r,
        Value::Str(_) => unreachable!("strings are handled first"),
    }
}

fn is_float(v: &Value) -> bool {
    matches!(v, Value::Float(..))
}

fn is_unsigned(v: &Value) -> bool {
    matches!(v, Value::UInt(_))
}

/// A float result of the float lane (`ParseNumSetF … FLOATTYPE`).
fn float_result(r: Range) -> Value {
    Value::Float(r, FloatTy::Float)
}

/// `a op b` on ranges, for an operation that is monotone in each operand (the four corners).
fn corners(a: Range, b: Range, op: impl Fn(F80, F80) -> F80) -> R<Range> {
    let mid = op(a.mid, b.mid);
    let vals = [op(a.lo, b.lo), op(a.lo, b.hi), op(a.hi, b.lo), op(a.hi, b.hi)];
    if vals.contains(&F80::Nan) {
        if a.one().is_none() || b.one().is_none() {
            fell_back();
        }
        return Ok(Range::exact(mid));
    }
    let mut lo = vals[0];
    let mut hi = vals[0];
    for &v in &vals[1..] {
        if v.partial_cmp(lo) == Some(Ordering::Less) {
            lo = v;
        }
        if v.partial_cmp(hi) == Some(Ordering::Greater) {
            hi = v;
        }
    }
    Ok(Range { lo, hi, mid })
}

/// The comparison of two ranges, when every pair of values gives the same answer.
fn compare(a: Range, b: Range) -> R<Option<Ordering>> {
    let x = a.lo.partial_cmp(b.hi);
    let y = a.hi.partial_cmp(b.lo);
    if x == y || (a.one().is_some() && b.one().is_some()) {
        Ok(x)
    } else {
        fell_back();
        Ok(a.mid.partial_cmp(b.mid))
    }
}

fn truth(c: bool) -> i64 {
    if c { -1 } else { 0 }
}

/// `a op b` (design D2; `const_eval.bas` `Term`, `Factor`, `Relation`, `IntDiv`, `ParseMod`, `LogicalAnd` …).
pub fn binary(op: BinOp, a: &Value, b: &Value) -> R<Value> {
    match (a, b) {
        (Value::Str(x), Value::Str(y)) => {
            return match op {
                BinOp::Add => Ok(Value::Str([x.as_slice(), y.as_slice()].concat())),
                BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
                    error("strings cannot be compared in a `CONST`")
                }
                BinOp::Sub
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
                | BinOp::Pow => error("this operator cannot be used on strings"),
            };
        }
        (Value::Str(_), _) | (_, Value::Str(_)) => return error("cannot mix strings and numbers"),
        _ => {}
    }
    let floats = is_float(a) || is_float(b);
    let unsigned = is_unsigned(a) || is_unsigned(b);
    let int = |f: fn(i64, i64) -> i64| -> R<Value> { Ok(Value::Int(f(as_i64(a)?, as_i64(b)?))) };
    let uint = |f: fn(u64, u64) -> u64| -> R<Value> { Ok(Value::UInt(f(as_u64(a)?, as_u64(b)?))) };
    // An integer operation by the unsigned lane when either operand is unsigned, else the signed one.
    let lanes = |fi: fn(i64, i64) -> i64, fu: fn(u64, u64) -> u64| if unsigned { uint(fu) } else { int(fi) };
    match op {
        BinOp::Add | BinOp::Sub | BinOp::Mul if floats => {
            let (x, y) = (as_float(a), as_float(b));
            let r = if op == BinOp::Add {
                corners(x, y, |p, q| p + q)?
            } else if op == BinOp::Sub {
                corners(x, y, |p, q| p - q)?
            } else {
                corners(x, y, |p, q| p * q)?
            };
            Ok(float_result(r))
        }
        BinOp::Add => lanes(i64::wrapping_add, u64::wrapping_add),
        BinOp::Sub => lanes(i64::wrapping_sub, u64::wrapping_sub),
        BinOp::Mul => lanes(i64::wrapping_mul, u64::wrapping_mul),
        BinOp::Div => {
            let (x, y) = (as_float(a), as_float(b));
            if y.lo.is_zero() || y.hi.is_zero() || compare(y, Range::exact(F80::ZERO))? == Some(Ordering::Equal) {
                // The old compiler gives 0 (`QB64PE-DIVERGENCES.md` D-007).
                return error("division by zero");
            }
            if y.lo.is_neg() != y.hi.is_neg() {
                fell_back();
                if y.mid.is_zero() {
                    return error("division by zero");
                }
                return Ok(float_result(Range::exact(x.settle() / y.mid)));
            }
            Ok(float_result(corners(x, y, |p, q| p / q)?))
        }
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
            if floats {
                // The float lane gives a float -1 or 0.
                let o = compare(as_float(a), as_float(b))?;
                let t = o.is_some_and(|o| holds(op, o));
                Ok(float_result(Range::exact(F80::from_i64(truth(t)))))
            } else if unsigned {
                let (x, y) = (as_u64(a)?, as_u64(b)?);
                // -1 stored into an `_UNSIGNED _INTEGER64`.
                Ok(Value::UInt(truth(holds(op, x.cmp(&y))).cast_unsigned()))
            } else {
                let (x, y) = (as_i64(a)?, as_i64(b)?);
                Ok(Value::Int(truth(holds(op, x.cmp(&y)))))
            }
        }
        BinOp::And => lanes(|x, y| x & y, |x, y| x & y),
        BinOp::Or => lanes(|x, y| x | y, |x, y| x | y),
        BinOp::Xor => lanes(|x, y| x ^ y, |x, y| x ^ y),
        BinOp::Eqv => lanes(|x, y| !(x ^ y), |x, y| !(x ^ y)),
        BinOp::Imp => lanes(|x, y| !x | y, |x, y| !x | y),
        BinOp::IDiv | BinOp::Mod => {
            if unsigned {
                let (x, y) = (as_u64(a)?, as_u64(b)?);
                if y == 0 {
                    // Measured: the old compiler crashes ("Division by zero").
                    return error("division by zero");
                }
                return Ok(Value::UInt(if op == BinOp::IDiv { x / y } else { x % y }));
            }
            let (x, y) = (as_i64(a)?, as_i64(b)?);
            if y == 0 {
                return error("division by zero");
            }
            if y == -1 && x == i64::MIN {
                return unsupported("the smallest `_INTEGER64` divided by -1 in a `CONST`");
            }
            Ok(Value::Int(if op == BinOp::IDiv { x / y } else { x % y }))
        }
        BinOp::Pow => power(a, b),
        BinOp::AndAlso | BinOp::OrElse => unsupported("`_ANDALSO` and `_ORELSE` in a `CONST`"),
    }
}

/// Whether comparison `op` holds for operands ordered `o`.
fn holds(op: BinOp, o: Ordering) -> bool {
    match op {
        BinOp::Eq => o.is_eq(),
        BinOp::Ne => o.is_ne(),
        BinOp::Lt => o.is_lt(),
        BinOp::Gt => o.is_gt(),
        BinOp::Le => o.is_le(),
        BinOp::Ge => o.is_ge(),
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
        | BinOp::Pow => unreachable!("{op:?} is not a comparison"),
    }
}

/// `a ^ b` (`Exponent`): libqb's `pow2` (`powl`, certified), a float in every lane, as `/` gives one and `^` does in
/// a statement (`QB64PE-DIVERGENCES.md` D-028: in an integer lane the old evaluator stores the power back by `qbr`, so
/// `2 ^ -1` is 0, measured, `verification\v45_b_funcs`; D-007: beyond `_INTEGER64` `i64::MIN`). As every float, it
/// is read back by its text where a group or the whole value ends, so `2 ^ 3` is still the `_INTEGER64` 8. 0 to a
/// negative power is a division by zero.
fn power(a: &Value, b: &Value) -> R<Value> {
    let (x, y) = (as_float(a), as_float(b));
    if x.one().is_some_and(F80::is_zero) && y.one().is_some_and(|y| y.is_neg() && !y.is_zero()) {
        return error("division by zero");
    }
    Ok(float_result(crate::certify::pow2(x, y)?))
}

/// `a ROOT b` (`const_eval.bas` 659–672): `a ^ (1 / b)` in the float lane, with the sign of a negative `a` kept
/// when `b` is at least 1, and when `1 / b` is not whole and `b` is below 1 the sign of `a` times `|a| ^ (1 / b)`
/// (measured, `verification\v45_b_funcs`: `-8 ROOT 3` is -2, `20 ROOT 3` 2.714417616594906572; `v45_x25`: `2 ROOT 0`
/// is infinite, 0 as the whole value).
pub fn root(a: &Value, b: &Value) -> R<Value> {
    if matches!(a, Value::Str(_)) || matches!(b, Value::Str(_)) {
        return error("cannot mix strings and numbers");
    }
    let (mut x, n) = (as_float(a).settle(), as_float(b).settle());
    let unit = F80::from_i64(1);
    let at_least_one = n.partial_cmp(unit).is_some_and(Ordering::is_ge);
    let mut sig = if x.is_neg() && !x.is_zero() && at_least_one {
        x = -x;
        -1
    } else {
        1
    };
    let expon = unit / n;
    if expon != expon.round_integral(Integral::Floor) && !at_least_one {
        sig = if x.is_zero() {
            0
        } else if x.is_neg() {
            -1
        } else {
            1
        };
        x = x.abs();
    }
    let p = crate::certify::pow2(Range::exact(x), Range::exact(expon))?;
    let s = Range::exact(F80::from_i64(sig));
    Ok(float_result(corners(s, p, |q, r| q * r)?))
}

/// A prefix operator: `-` and `NOT` (`Unary`, `LogicalNot`). (`_NEGATE` is not an operator of the old evaluator.)
pub fn unary(op: UnOp, a: &Value) -> R<Value> {
    match (op, a) {
        (_, Value::Str(_)) => error("this operator cannot be used on strings"),
        (UnOp::Neg, Value::Float(r, _)) => Ok(float_result(Range {
            lo: -r.hi,
            hi: -r.lo,
            mid: -r.mid,
        })),
        // An unsigned value is negated in the signed lane (measured: `-18446744073709551615~&&` is 1).
        (UnOp::Neg, _) => Ok(Value::Int(as_i64(a)?.wrapping_neg())),
        (UnOp::Not, Value::UInt(u)) => Ok(Value::UInt(!u)),
        (UnOp::Not, _) => Ok(Value::Int(!as_i64(a)?)),
        (UnOp::Negate, _) => unsupported("`_NEGATE` in a `CONST`"),
    }
}

/// `CHR$(code)` of the old evaluator (`const_eval.bas` 1027; measured, `verification\v43_c_chr_const`, `v43_x*`):
/// the code in the unsigned lane (`qbr` of a float: `-0.5` is 0, `65.5` 66, `-0.6` 2^64 - 1) must be at most
/// 255. A string is an error.
pub fn chr(code: &Value) -> R<Value> {
    if matches!(code, Value::Str(_)) {
        return error("`CHR$` in a `CONST` needs a number, not a string");
    }
    match u8::try_from(as_u64(code)?) {
        Ok(b) => Ok(Value::Str(vec![b])),
        Err(_) => error("`CHR$` in a `CONST` needs a code from 0 to 255"),
    }
}

// ---- The functions ----

/// The functions of the old evaluator (`Set_ConstFunctions`) with their argument count: a positive count is
/// exact and takes numbers only; -1 is a varying count that also takes strings.
pub const FUNCTIONS: &[(&str, i32)] = &[
    ("_PI", 1),
    ("_ACOS", 1),
    ("_ASIN", 1),
    ("_ARCSEC", 1),
    ("_ARCCSC", 1),
    ("_ARCCOT", 1),
    ("_SECH", 1),
    ("_CSCH", 1),
    ("_COTH", 1),
    ("COS", 1),
    ("SIN", 1),
    ("TAN", 1),
    ("LOG", 1),
    ("EXP", 1),
    ("ATN", 1),
    ("SQR", 1),
    ("_D2R", 1),
    ("_D2G", 1),
    ("_R2D", 1),
    ("_R2G", 1),
    ("_G2D", 1),
    ("_G2R", 1),
    ("ABS", 1),
    ("SGN", 1),
    ("INT", 1),
    ("_ROUND", 1),
    ("_CEIL", 1),
    ("FIX", 1),
    ("_SEC", 1),
    ("_CSC", 1),
    ("_COT", 1),
    ("_RGB32", -1),
    ("_RGBA32", 4),
    ("_RGBA", 5),
    ("_RGB", 4),
    ("_RED32", 1),
    ("_GREEN32", 1),
    ("_BLUE32", 1),
    ("_ALPHA32", 1),
    ("_RED", 2),
    ("_GREEN", 2),
    ("_BLUE", 2),
    ("_ALPHA", 2),
    ("CHR$", 1),
    ("ASC", -1),
];

/// The argument count of a function of the evaluator, if `name` (upper case) is one.
pub fn function_arity(name: &str) -> Option<i32> {
    FUNCTIONS.iter().find(|(n, _)| *n == name).map(|&(_, a)| a)
}

/// π rounded to `_FLOAT` (`3.14159265358979323846264338327950288##` in `const_eval.bas`; measured,
/// `verification\v45_b_funcs`: `3.141592653589793239`).
pub fn pi() -> F80 {
    decimal("3.14159265358979323846264338327950288")
}

/// An argument's float slot (`args(i).f`), as `elementGetNumericValue` fills it from the argument's text: an
/// unsigned integer by its signed bits.
fn arg_f(v: &Value) -> R<F80> {
    match v {
        Value::Int(i) => Ok(F80::from_i64(*i)),
        Value::UInt(u) => Ok(F80::from_i64(u.cast_signed())),
        Value::Float(r, _) => Ok(r.settle()),
        Value::Str(_) => unreachable!("checked by the caller"),
    }
}

/// An `_INTEGER64` result (`n&& = n1`: `qbr`).
fn int_result(n1: F80) -> Value {
    Value::Int(qbr(n1))
}

/// `name(args…)` of the old evaluator (`EvaluateFunction$`; design D4, D5). The arguments are already read back
/// as the evaluator reads them. A float result is the value before `_TOSTR$` (the caller reads it back).
pub fn function(name: &str, args: &[Value]) -> R<Value> {
    let Some(arity) = function_arity(name) else {
        return error(&format!("`{name}` cannot be used in a `CONST`"));
    };
    let count = i32::try_from(args.len()).unwrap_or(i32::MAX);
    if arity > 0 && count != arity {
        return error(&format!("wrong number of arguments to `{name}` in a `CONST`"));
    }
    if arity > 0 && args.iter().any(|a| matches!(a, Value::Str(_))) {
        return error(&format!("`{name}` in a `CONST` takes numbers, not strings"));
    }
    let float = |v: F80| Ok(Value::Float(Range::exact(v), FloatTy::Float));
    let fuzzy = |name: &str| -> R<Value> {
        let r = crate::certify::function(name, arg_f(&args[0])?)?;
        Ok(Value::Float(r, FloatTy::Float))
    };
    match name {
        "_PI" => float(pi() * arg_f(&args[0])?),
        "COS" | "SIN" | "TAN" | "ATN" | "LOG" | "EXP" | "_ACOS" | "_ASIN" | "_ARCCOT" | "_SECH" | "_CSCH" | "_COTH"
        | "_SEC" | "_CSC" | "_COT" => fuzzy(name),
        "_ARCSEC" | "_ARCCSC" => {
            if arg_f(&args[0])?.abs().partial_cmp(F80::from_i64(1)) == Some(Ordering::Less) {
                // Measured, `verification\v45_x02`, `x03`: "ABS(_ARCSEC) value < 1".
                return error(&format!("`{name}` in a `CONST` needs a value of at least 1 in size"));
            }
            fuzzy(name)
        }
        "SQR" => {
            let f = arg_f(&args[0])?;
            if f.is_neg() && !f.is_zero() {
                // Measured, `v45_x01`: "Argument to SQR cannot be negative".
                return error("`SQR` in a `CONST` cannot take a negative number");
            }
            // `func_sqr(double)`: a correctly rounded `double` root.
            float(F80::from_f64(f.to_f64().sqrt()))
        }
        // A `double` literal times the argument in `_FLOAT` (`const_eval.bas` 1011–1016).
        "_D2R" => float(F80::from_f64(0.0174532925) * arg_f(&args[0])?),
        "_D2G" => float(F80::from_f64(1.1111111111) * arg_f(&args[0])?),
        "_R2D" => float(F80::from_f64(57.2957795) * arg_f(&args[0])?),
        "_R2G" => float(F80::from_f64(0.015707963) * arg_f(&args[0])?),
        "_G2D" => float(F80::from_f64(0.9) * arg_f(&args[0])?),
        "_G2R" => float(F80::from_f64(63.661977237) * arg_f(&args[0])?),
        "ABS" => Ok(int_result(arg_f(&args[0])?.abs())),
        "SGN" => {
            let f = arg_f(&args[0])?;
            Ok(Value::Int(if f.is_zero() {
                0
            } else if f.is_neg() {
                -1
            } else {
                1
            }))
        }
        "INT" => Ok(int_result(arg_f(&args[0])?.round_integral(Integral::Floor))),
        "_CEIL" => Ok(int_result(arg_f(&args[0])?.round_integral(Integral::Ceil))),
        "FIX" => Ok(int_result(arg_f(&args[0])?.round_integral(Integral::Trunc))),
        "_ROUND" => Ok(Value::Int(qbr(arg_f(&args[0])?))),
        "CHR$" => chr(&args[0]),
        "ASC" => asc(args),
        "_RGB32" | "_RGBA32" | "_RED32" | "_GREEN32" | "_BLUE32" | "_ALPHA32" | "_RGB" | "_RGBA" | "_RED"
        | "_GREEN" | "_BLUE" | "_ALPHA" => crate::colors::function(name, args),
        _ => unreachable!("{name} is in FUNCTIONS"),
    }
}

/// `ASC(s[, i])` (`const_eval.bas` 1033–1046; measured, `verification\v45_b_asc_suffix`, `v45_x08`–`x10`, `x19`,
/// `x20`).
fn asc(args: &[Value]) -> R<Value> {
    if args.is_empty() || args.len() > 2 {
        return error("wrong number of arguments to `ASC` in a `CONST`");
    }
    let Value::Str(s) = &args[0] else {
        return error("`ASC` in a `CONST` needs a string");
    };
    if s.is_empty() {
        return error("`ASC` in a `CONST` cannot take an empty string");
    }
    let pos = match args.get(1) {
        None => 1,
        Some(Value::Str(_)) => return error("`ASC` in a `CONST` needs a number as its position"),
        Some(v) => as_i64(v)?,
    };
    match usize::try_from(pos).ok().filter(|&p| p >= 1).and_then(|p| s.get(p - 1)) {
        Some(&b) => Ok(Value::Int(i64::from(b))),
        // libqb's error 5 inside `qb64pe.exe` (measured, `v45_x09`, `x10`).
        None => error(&format!(
            "`ASC` at position {pos} of a string of {} bytes is an error of the old compiler's own run time, which \
             stops it",
            s.len()
        )),
    }
}

// ---- The constant's type ----

/// The type and value of a constant (`qb64pe.bas` 2600–2625): without a suffix the value's own type (signed
/// `_INTEGER64`, unsigned `_UNSIGNED _INTEGER64`, a float's by its text, a string); a float suffix gives the
/// value as a float (an unsigned value through its signed bits, as `constval## = constval&&`); an integer or
/// `_BIT` suffix keeps the value's 64 bits (a float by `qbr`): the constant is then a literal of that suffix with
/// those digits (design D7 of `m2-numeric-types`, [`crate::literal::constant_literal`]; measured
/// `verification\v21_d_const`: `CONST c%% = 200` prints -56, `c%% + 0` is 200). Floats are stored as their one
/// value; integers as `Value::Int` holding the 64 bits.
pub fn settle(v: Value, suffix: Option<Ty>) -> R<(Ty, Value)> {
    // A value the libraries do not settle is its correctly rounded one (D-026).
    let v = if let Value::Float(r, fty) = v {
        Value::float(r.settle(), fty)
    } else {
        v
    };
    match (suffix, v) {
        (None | Some(Ty::Str), Value::Str(s)) => Ok((Ty::Str, Value::Str(s))),
        (Some(_), Value::Str(_)) | (Some(Ty::Str), _) => error("type mismatch"),
        (Some(t), v) if t.is_int() => Ok((t, Value::Int(as_i64(&v)?))),
        (Some(t @ (Ty::F32 | Ty::F64 | Ty::F80)), v) => {
            let f = match v {
                Value::Int(i) => F80::from_i64(i),
                Value::UInt(u) => F80::from_i64(u.cast_signed()),
                Value::Float(r, _) => r.lo,
                Value::Str(_) => unreachable!("handled above"),
            };
            Ok((t, Value::float(f, fty_of(t))))
        }
        (Some(t), _) => unreachable!("a suffix's type is a number or a string, not {t:?}"),
        (None, Value::Int(i)) => Ok((Ty::I64, Value::Int(i))),
        (None, Value::UInt(u)) => Ok((Ty::U64, Value::Int(u.cast_signed()))),
        (None, Value::Float(r, fty)) => Ok((fty.ty(), Value::Float(r, fty))),
    }
}

/// The float type of a float type (a DOUBLE for any other type, which no caller gives).
fn fty_of(t: Ty) -> FloatTy {
    if t == Ty::F32 {
        FloatTy::Single
    } else if t == Ty::F80 {
        FloatTy::Float
    } else {
        FloatTy::Double
    }
}

/// A stored constant of type `ty` as the old evaluator reads it inside another `CONST` (`ParseNumHashLookup&`):
/// a float its `_FLOAT` value, an unsigned type the unsigned lane, any other integer the signed lane.
pub fn lookup(ty: Ty, v: &Value) -> Value {
    match (v, ty) {
        (Value::Int(i), t) if t.is_unsigned() => Value::UInt(i.cast_unsigned()),
        _ => v.clone(),
    }
}

/// A constant of type `from` used as the float type `to` (by the suffix of a use): its value as a `_FLOAT`
/// (`v## = v&&`, an unsigned one through its signed bits).
pub fn convert(from: Ty, v: Value, to: Ty) -> R<(Ty, Value)> {
    if from == to {
        return Ok((to, v));
    }
    let f = match (&v, to) {
        (Value::Str(_), _) | (_, Ty::Str | Ty::User(_)) => return error("type mismatch"),
        (_, crate::place_only_types!()) => unreachable!("{}", crate::PLACE_ONLY_UNREACHABLE),
        (_, crate::Ty::Bit { .. }) => unreachable!("{}", crate::BIT_VALUE_UNREACHABLE),
        (_, Ty::I8 | Ty::U8 | Ty::I16 | Ty::U16 | Ty::I32 | Ty::U32 | Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff) => {
            unreachable!("integer uses are literals of their suffix (`const_use`)")
        }
        // An unsigned constant too goes through its signed bits (`v~&& = constuinteger: v&& = v~&&: v## = v&&`,
        // `qb64pe.bas` 24040; `tests\corpus\slice\s34_types_procs`: `nb!` of `CONST nb~%% = -1` is -1).
        (Value::Int(i), _) => F80::from_i64(*i),
        (Value::UInt(u), _) => F80::from_u64(*u),
        (Value::Float(r, _), _) => r.lo,
    };
    Ok((to, Value::float(f, fty_of(to))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(i: i64) -> Value {
        Value::Int(i)
    }

    fn flt(text: &str) -> Value {
        literal(text.as_bytes()).unwrap()
    }

    fn bin(op: BinOp, a: &Value, b: &Value) -> Value {
        binary(op, a, b).unwrap()
    }

    fn text(v: &Value) -> String {
        match v {
            Value::Float(r, _) => r.one().unwrap().to_g19(),
            Value::Int(i) => i.to_string(),
            Value::UInt(u) => format!("{u}~"),
            Value::Str(s) => s.iter().map(|&b| char::from(b)).collect(),
        }
    }

    fn is_error<T: std::fmt::Debug>(r: R<T>) -> bool {
        matches!(r, Err(Problem::Error(_)))
    }

    #[test]
    fn integers_wrap_in_64_bits() {
        assert_eq!(bin(BinOp::Add, &int(i64::MAX), &int(1)), int(i64::MIN));
        assert_eq!(bin(BinOp::Mul, &int(3000000000), &int(4)), int(12000000000));
        assert_eq!(unary(UnOp::Neg, &int(i64::MIN)).unwrap(), int(i64::MIN));
    }

    /// `verification\v45_a_lanes`, `v45_b_conv`.
    #[test]
    fn the_lanes() {
        let max = flt("18446744073709551615~&&");
        assert_eq!(max, Value::UInt(u64::MAX));
        assert_eq!(bin(BinOp::Add, &max, &int(1)), Value::UInt(0));
        assert_eq!(bin(BinOp::Sub, &max, &int(1)), Value::UInt(u64::MAX - 1));
        assert_eq!(unary(UnOp::Neg, &max).unwrap(), int(1));
        assert_eq!(unary(UnOp::Not, &Value::UInt(0)).unwrap(), Value::UInt(u64::MAX));
        assert_eq!(unary(UnOp::Not, &flt("2.5")).unwrap(), int(-3));
        let cases = [
            (BinOp::IDiv, "2.5", "1", "2"),
            (BinOp::IDiv, "3.5", "1", "4"),
            (BinOp::IDiv, "-2.5", "1", "-2"),
            (BinOp::Mod, "7.5", "4", "0"),
        ];
        for (op, a, b, want) in cases {
            let a = if let Some(p) = a.strip_prefix('-') {
                unary(UnOp::Neg, &flt(p)).unwrap()
            } else {
                flt(a)
            };
            assert_eq!(text(&bin(op, &a, &flt(b))), want, "{a:?} {op:?} {b}");
        }
        let neg = unary(UnOp::Neg, &flt("1.5")).unwrap();
        assert_eq!(bin(BinOp::Or, &neg, &Value::UInt(0)), Value::UInt(u64::MAX - 1));
        assert_eq!(
            bin(BinOp::Or, &flt("1E+19"), &Value::UInt(0)),
            Value::UInt(10_000_000_000_000_000_000)
        );
        assert_eq!(bin(BinOp::Or, &flt("1E+19"), &int(0)), int(-8446744073709551616));
        assert_eq!(bin(BinOp::Or, &flt("1E+30"), &int(0)), int(i64::MIN));
    }

    /// `verification\v45_a_precision`, `v45_b_conv`.
    #[test]
    fn floats_and_their_text() {
        let third = bin(BinOp::Div, &flt("1##"), &flt("3##"));
        assert_eq!(text(&third), "0.3333333333333333333");
        let rr = |v: Value| reread(v, true).unwrap();
        assert_eq!(
            rr(bin(BinOp::Div, &flt("1E+30"), &int(3))).clone(),
            Value::float(decimal("3.333333333333333333E+29"), FloatTy::Float)
        );
        assert!(matches!(
            rr(bin(BinOp::Div, &int(1), &int(3))),
            Value::Float(_, FloatTy::Double)
        ));
        assert_eq!(rr(bin(BinOp::Div, &int(4), &int(2))), int(2));
        assert_eq!(rr(bin(BinOp::Div, &flt("9.2E+18"), &int(1))), int(9200000000000000000));
        assert_eq!(text(&rr(bin(BinOp::Mul, &flt("0.1"), &int(3)))), "0.3");
        assert!(matches!(
            rr(bin(BinOp::Div, &flt("1E+19"), &int(1))),
            Value::Float(_, FloatTy::Float)
        ));
    }

    #[test]
    fn settle_and_suffixes() {
        assert_eq!(settle(flt("3.7"), Some(Ty::I16)).unwrap(), (Ty::I16, int(4)));
        assert_eq!(settle(flt("2.5"), Some(Ty::I16)).unwrap(), (Ty::I16, int(2)));
        assert_eq!(settle(int(40000), Some(Ty::I16)).unwrap(), (Ty::I16, int(40000)));
        assert_eq!(settle(Value::UInt(u64::MAX), None).unwrap(), (Ty::U64, int(-1)));
        // An unsigned constant used with a float suffix goes through its signed bits (`nb!` of `CONST nb~%% = -1`).
        let (_, v) = convert(Ty::U8, int(-1), Ty::F32).unwrap();
        assert_eq!(v, Value::float(F80::from_i64(-1), FloatTy::Single));
        assert!(is_error(settle(Value::Str(b"x".to_vec()), Some(Ty::I16))));
        assert!(is_error(settle(int(5), Some(Ty::Str))));
    }

    #[test]
    fn chr_rounds_half_to_even_within_a_byte() {
        let s = |b: u8| Ok(Value::Str(vec![b]));
        assert_eq!(chr(&int(65)), s(65));
        assert_eq!(chr(&flt("65.5")), s(66));
        assert_eq!(chr(&flt("66.5")), s(66));
        assert_eq!(chr(&unary(UnOp::Neg, &flt("0.5")).unwrap()), s(0));
        for bad in [int(256), int(-1), flt("255.5"), flt("1E+300")] {
            assert!(is_error(chr(&bad)), "{bad:?}");
        }
        assert!(is_error(chr(&unary(UnOp::Neg, &flt("0.6")).unwrap())));
    }

    #[test]
    fn errors() {
        assert!(is_error(binary(BinOp::IDiv, &int(1), &int(0))));
        assert!(is_error(binary(BinOp::Div, &int(1), &int(0))));
        assert!(is_error(binary(BinOp::Add, &Value::Str(b"a".to_vec()), &int(1))));
        assert!(is_error(reread(
            Value::float(F80::Inf { neg: false }, FloatTy::Float),
            false
        )));
        assert_eq!(reread(Value::float(F80::Nan, FloatTy::Float), true).unwrap(), int(0));
    }

    /// D-027: a plain integer beyond `_INTEGER64` is read as a statement reads it, and a float beyond it stays a
    /// float; the old evaluator clamps both to 9223372036854775807 (`verification\v45_a_lanes`, `v45_b_conv`).
    #[test]
    fn values_beyond_integer64_are_not_clamped() {
        assert_eq!(flt("9223372036854775807"), int(i64::MAX));
        assert_eq!(flt("9223372036854775808"), Value::UInt(1 << 63));
        assert_eq!(flt("18446744073709551615"), Value::UInt(u64::MAX));
        assert!(is_error(literal(b"18446744073709551616")));
        assert!(is_error(literal(b"123456789012345678901234567890")));
        // `-9223372036854775808` is the smallest `_INTEGER64`.
        assert_eq!(unary(UnOp::Neg, &flt("9223372036854775808")).unwrap(), int(i64::MIN));
        let rr = |v: Value| reread(v, true).unwrap();
        let big = rr(bin(BinOp::Div, &flt("9.3E+18"), &int(1)));
        assert_eq!(big, Value::float(decimal("9300000000000000000"), FloatTy::Float));
        let small = rr(unary(UnOp::Neg, &bin(BinOp::Div, &flt("9.3E+18"), &int(1))).unwrap());
        assert_eq!(small, Value::float(decimal("-9300000000000000000"), FloatTy::Float));
        assert_eq!(rr(Value::float(F80::from_i64(i64::MIN), FloatTy::Float)), int(i64::MIN));
    }

    /// D-028: `^` gives a float in every lane, read back by its text (`verification\v45_b_funcs`: the old
    /// evaluator's `2 ^ -1` is 0).
    #[test]
    fn integer_powers_are_floats() {
        let rr = |v: Value| reread(v, true).unwrap();
        assert_eq!(text(&rr(bin(BinOp::Pow, &int(2), &int(-1)))), "0.5");
        assert_eq!(text(&rr(bin(BinOp::Pow, &int(2), &int(-2)))), "0.25");
        assert!(matches!(
            rr(bin(BinOp::Pow, &int(2), &int(-1))),
            Value::Float(_, FloatTy::Double)
        ));
        assert_eq!(rr(bin(BinOp::Pow, &int(2), &int(3))), int(8));
        assert_eq!(rr(bin(BinOp::Pow, &Value::UInt(2), &int(3))), int(8));
        // Beyond `_INTEGER64` a float, from unsigned operands too.
        assert_eq!(
            rr(bin(BinOp::Pow, &Value::UInt(2), &int(63))),
            Value::float(decimal("9223372036854775808"), FloatTy::Float)
        );
        // Not wrapped in the signed lane: `2 ^ 62 * 4` is 2^64 as a float.
        let p = bin(BinOp::Mul, &bin(BinOp::Pow, &int(2), &int(62)), &int(4));
        assert_eq!(rr(p), Value::float(decimal("1.844674407370955162E+19"), FloatTy::Float));
        assert!(is_error(binary(BinOp::Pow, &int(0), &int(-1))));
        assert_eq!(rr(bin(BinOp::Pow, &int(0), &int(0))), int(1));
    }

    /// D-029: a value beyond `_FLOAT`'s range is an error as the whole value too (the old evaluator gives 0,
    /// `verification\v45_x25`, `x38`).
    #[test]
    fn a_value_beyond_float_is_an_error() {
        let huge = bin(BinOp::Mul, &flt("1F+4000"), &flt("1F+4000"));
        assert!(is_error(reread(huge.clone(), true)));
        assert!(is_error(reread(unary(UnOp::Neg, &huge).unwrap(), true)));
        assert!(is_error(reread(root(&int(2), &int(0)).unwrap(), true)));
    }

    #[test]
    fn c_texts() {
        // As the old compiler writes them (`verification\v45_a_precision`'s C++).
        assert_eq!(c_text(decimal("0.3333333333333333333")), "0.3333333333333333333E+0");
        assert_eq!(c_text(decimal("3.333333333333333333E+29")), "3.333333333333333333E+29");
        assert_eq!(
            c_text(decimal("1.189731495357231765E+4932")),
            "1.189731495357231765E+4932"
        );
        assert_eq!(c_text(decimal("32")), "32.0E+0");
        assert_eq!(c_text(decimal("1E-5")), "1.0E-05");
        assert_eq!(double_of(decimal("0.1")), 0.1);
    }
}
