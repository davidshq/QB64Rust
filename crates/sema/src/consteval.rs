//! The arithmetic of the old compiler's `CONST` evaluator (`const_eval.bas`, `study\02` §7; design D6 of
//! `m2-control-flow-slice`), on values; the walk over the tree is `check\constants.rs`.
//!
//! The old evaluator computes integers in 64 bits (wrapping) and floats in `_FLOAT` (x87, 64-bit mantissa), and
//! re-reads each parenthesised group and the result from 19-digit text. Rust has no 80-bit float, so floats are
//! computed in `f64` and each carries `exact`: whether it is the exact result. A float operation needs exact
//! operands; its result is exact, or rounded and far enough from a rounding boundary that the old path (round to
//! 64 bits, print 19 digits, read back) gives the same `double`; anything else is "not supported yet". So a
//! constant either has the old compiler's value or is not compiled.

use crate::{BinOp, Ty, UnOp};

/// A value of the evaluator.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Float(Float),
    Str(Vec<u8>),
}

/// A float, computed in `f64`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float {
    pub v: f64,
    /// Whether `v` is the exact value. When false, `v` is the `double` the old compiler's path gives, but the old
    /// `_FLOAT` value differs from `v`, so the value must not take part in another operation.
    pub exact: bool,
}

/// Why a value cannot be computed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    /// The old compiler rejects the expression too.
    Error(String),
    /// Not supported yet; names what.
    Unsupported(String),
}

type R<T> = Result<T, Problem>;

fn unsupported<T>(what: &str) -> R<T> {
    Err(Problem::Unsupported(what.to_string()))
}

fn error<T>(msg: &str) -> R<T> {
    Err(Problem::Error(msg.to_string()))
}

/// Relative distance within which the old path (rounding to 64 bits, then to 19 significant digits) may move a
/// value: 2^-64 plus 5E-19, rounded up.
const OLD_PATH: f64 = 1e-18;

/// 2^53: integers up to this size are exact in `f64`.
const EXACT_INT: i64 = 1 << 53;

/// A float literal in the C form of [`crate::literal::NumLit::Float`] (`1.5E+2`).
pub fn float_literal(text: &str) -> R<Float> {
    let v: f64 = text
        .parse()
        .map_err(|_| Problem::Unsupported(format!("the number `{text}` in a `CONST`")))?;
    if !v.is_finite() {
        return unsupported("a number beyond `DOUBLE` range in a `CONST`");
    }
    let (digits, exp) = decimal_parts(text);
    if decimal_parts(&exact_text(v)) == (digits.clone(), exp) {
        return Ok(Float { v, exact: true });
    }
    // Rounded: the same `double` for every value within OLD_PATH of the literal?
    if digits.len() > 21 {
        return unsupported("a number with more than 21 digits in a `CONST`");
    }
    let scale = 21 - digits.len();
    let n: u128 = digits.parse::<u128>().expect("decimal digits") * 10u128.pow(to_u32(scale));
    let at = |m: u128| -> f64 {
        let e = exp - i32::try_from(scale).expect("at most 21");
        format!("{m}E{e}").parse().unwrap_or(f64::NAN)
    };
    // `n` has 21 digits, so 1000 is more than OLD_PATH of it.
    if at(n - 1000) != v || at(n + 1000) != v {
        return unsupported("a number this close to a rounding boundary in a `CONST`");
    }
    Ok(Float { v, exact: false })
}

fn to_u32(n: usize) -> u32 {
    u32::try_from(n).expect("small count")
}

/// The digits of a decimal without leading or trailing zeros, and the exponent `e` such that its magnitude is
/// `digits * 10^e`. Zero is `("", 0)`. Accepts `[-]d[.d][E[+-]n]`.
fn decimal_parts(text: &str) -> (String, i32) {
    let text = text.trim_start_matches('-');
    let (mant, exp) = match text.find(['E', 'e']) {
        Some(i) => (&text[..i], text[i + 1..].parse::<i32>().unwrap_or(0)),
        None => (text, 0),
    };
    let (whole, frac) = mant.split_once('.').unwrap_or((mant, ""));
    let all = format!("{whole}{frac}");
    let frac_len = i32::try_from(frac.len()).expect("short fraction");
    let trimmed = all.trim_end_matches('0');
    let dropped = i32::try_from(all.len() - trimmed.len()).expect("short");
    let digits = trimmed.trim_start_matches('0').to_string();
    if digits.is_empty() {
        return (digits, 0);
    }
    (digits, exp - frac_len + dropped)
}

/// The exact decimal value of `v` (every `f64` has a finite one, at most 767 significant digits), in the C form
/// `d.dddE+n`.
pub fn exact_text(v: f64) -> String {
    c_form(&format!("{v:.780e}"))
}

/// The shortest text that reads back as `v`, in the C form `d.dddE+n`.
pub fn shortest_text(v: f64) -> String {
    c_form(&format!("{v:e}"))
}

/// Rust's `1.2500e2` as `1.25E+2`: trailing zeros dropped, at least one digit after the point, a signed exponent.
fn c_form(rust: &str) -> String {
    let (mant, exp) = rust.split_once('e').expect("exponent form");
    let mant = if mant.contains('.') {
        let m = mant.trim_end_matches('0');
        if m.ends_with('.') {
            format!("{m}0")
        } else {
            m.to_string()
        }
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

/// The number of significant digits of `v`'s exact decimal value.
fn exact_digits(v: f64) -> usize {
    decimal_parts(&exact_text(v)).0.len()
}

/// What the old evaluator does with a value it prints and reads back (a parenthesised group, the result): an
/// integer-valued float within `_INTEGER64` range comes back an integer; one beyond that range stays a float
/// (measured: `1E+30` and `1E+19 / 1` are DOUBLE); an exact float with more than 19 significant digits comes back
/// rounded.
pub fn reread(v: Value) -> R<Value> {
    let Value::Float(f) = v else {
        return Ok(v);
    };
    if !f.v.is_finite() {
        return unsupported("a `CONST` value beyond `DOUBLE` range");
    }
    let exact = f.exact && exact_digits(f.v) <= 19;
    if f.v.abs() >= TWO_63 {
        return Ok(Value::Float(Float { v: f.v, exact }));
    }
    if near_integer(f) {
        return unsupported("a rounded `CONST` value next to an integer");
    }
    if let Some(i) = int_of(f.v) {
        return Ok(Value::Int(i));
    }
    Ok(Value::Float(Float { v: f.v, exact }))
}

/// 2^63, the first float beyond `_INTEGER64` range.
const TWO_63: f64 = 9_223_372_036_854_775_808.0;

/// Whether a rounded float is within 2 ulp of an integer (the old value might be one, or on the other side).
fn near_integer(f: Float) -> bool {
    !f.exact && (f.v - f.v.round()).abs() <= 2.0 * ulp(f.v)
}

fn ulp(v: f64) -> f64 {
    v.abs().next_up() - v.abs()
}

/// `v` as an integer, when it is one within `_INTEGER64` range.
#[expect(clippy::cast_possible_truncation, reason = "checked: `v` is an integer within range")]
fn int_of(v: f64) -> Option<i64> {
    (v.fract() == 0.0 && (-TWO_63..TWO_63).contains(&v)).then_some(v as i64)
}

/// An integer as a float of the old evaluator (exactly, in `_FLOAT`); rounded beyond 2^53.
#[expect(
    clippy::cast_precision_loss,
    reason = "the result is marked inexact when the conversion rounds"
)]
fn float_of_int(i: i64) -> Float {
    Float {
        v: i as f64,
        exact: (-EXACT_INT..=EXACT_INT).contains(&i),
    }
}

fn as_float(v: &Value) -> Float {
    match v {
        Value::Int(i) => float_of_int(*i),
        Value::Float(f) => *f,
        Value::Str(_) => unreachable!("strings are handled first"),
    }
}

/// A float rounded half to even to an integer, as the old compiler converts `_FLOAT` to `_INTEGER64`.
fn round_to_int(f: Float) -> R<i64> {
    let frac = (f.v - f.v.trunc()).abs();
    if !f.exact && ((frac - 0.5).abs() <= 2.0 * ulp(f.v) || near_integer(f)) {
        return unsupported("rounding a rounded `CONST` value next to a boundary");
    }
    match int_of(f.v.round_ties_even()) {
        Some(i) => Ok(i),
        None => unsupported("a `CONST` value beyond `_INTEGER64` range"),
    }
}

fn to_int(v: &Value) -> R<i64> {
    match v {
        Value::Int(i) => Ok(*i),
        Value::Float(f) => round_to_int(*f),
        Value::Str(_) => unreachable!("strings are handled first"),
    }
}

fn truth(c: bool) -> i64 {
    if c { -1 } else { 0 }
}

/// A float operation's result `r` with the exact error `err` (true value minus `r`): exact, or rounded far enough
/// from the rounding boundaries that the old path gives the same `double`.
fn rounded(r: f64, err: f64) -> R<Value> {
    if !r.is_finite() {
        return unsupported("a `CONST` value beyond `DOUBLE` range");
    }
    if err == 0.0 {
        return Ok(Value::Float(Float { v: r, exact: true }));
    }
    let half_gap = ((r - r.next_down()).min(r.next_up() - r)) / 2.0;
    if err.abs() + r.abs() * 2.0 * OLD_PATH >= half_gap * 0.999 {
        return unsupported("a `CONST` value this close to a rounding boundary");
    }
    Ok(Value::Float(Float { v: r, exact: false }))
}

/// Both operands exact, or "not supported yet" (the old operation works on the `_FLOAT` value, which `f64` does
/// not have).
fn exact_pair(a: Float, b: Float) -> R<()> {
    if a.exact && b.exact {
        Ok(())
    } else {
        unsupported("a `CONST` computed from a rounded value")
    }
}

/// `a op b` (design D6). `op` is one of the operators the old evaluator knows; `_ANDALSO` and `_ORELSE` are not.
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
    let floats = matches!(a, Value::Float(_)) || matches!(b, Value::Float(_));
    let int = |f: fn(i64, i64) -> i64| -> R<Value> { Ok(Value::Int(f(to_int(a)?, to_int(b)?))) };
    match op {
        BinOp::Add if !floats => int(i64::wrapping_add),
        BinOp::Sub if !floats => int(i64::wrapping_sub),
        BinOp::Mul if !floats => int(i64::wrapping_mul),
        BinOp::Add | BinOp::Sub => {
            let (x, y) = (as_float(a), as_float(b));
            exact_pair(x, y)?;
            let y = if op == BinOp::Sub { -y.v } else { y.v };
            let r = x.v + y;
            // Knuth's two-sum: the exact error of the rounded sum.
            let bv = r - x.v;
            let err = (x.v - (r - bv)) + (y - bv);
            rounded(r, err)
        }
        BinOp::Mul => {
            let (x, y) = (as_float(a), as_float(b));
            exact_pair(x, y)?;
            let r = x.v * y.v;
            rounded(r, x.v.mul_add(y.v, -r))
        }
        BinOp::Div => {
            let (x, y) = (as_float(a), as_float(b));
            exact_pair(x, y)?;
            if y.v == 0.0 {
                // Measured: the old compiler gives 0.
                return unsupported("division by zero in a `CONST`");
            }
            let r = x.v / y.v;
            // x - r*y exactly, divided by y: the error to well within the margin.
            rounded(r, (-r).mul_add(y.v, x.v) / y.v)
        }
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
            let c = |o| holds(op, o);
            if floats {
                // The old evaluator gives a float -1 or 0 here.
                let (x, y) = (as_float(a), as_float(b));
                exact_pair(x, y)?;
                let o = x.v.partial_cmp(&y.v).expect("finite values");
                Ok(Value::Float(Float {
                    v: f64::from(i32::try_from(truth(c(o))).expect("-1 or 0")),
                    exact: true,
                }))
            } else {
                let (Value::Int(x), Value::Int(y)) = (a, b) else {
                    unreachable!()
                };
                Ok(Value::Int(truth(c(x.cmp(y)))))
            }
        }
        BinOp::And => int(|x, y| x & y),
        BinOp::Or => int(|x, y| x | y),
        BinOp::Xor => int(|x, y| x ^ y),
        BinOp::Eqv => int(|x, y| !(x ^ y)),
        BinOp::Imp => int(|x, y| !x | y),
        BinOp::IDiv | BinOp::Mod => {
            let (x, y) = (to_int(a)?, to_int(b)?);
            if y == 0 {
                // Measured: the old compiler crashes ("Division by zero").
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
fn holds(op: BinOp, o: std::cmp::Ordering) -> bool {
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

/// `a ^ b`. Two integers give an integer (the old evaluator stores the power back into `_INTEGER64`); otherwise
/// a float. Supported: a whole exponent with an exact result, and the exponent 0.5 (a square root).
fn power(a: &Value, b: &Value) -> R<Value> {
    if let (Value::Int(x), Value::Int(y)) = (a, b) {
        return match (x, u32::try_from(*y)) {
            (_, Ok(e)) => match x.checked_pow(e) {
                Some(p) => Ok(Value::Int(p)),
                None => unsupported("a power beyond `_INTEGER64` range in a `CONST`"),
            },
            (1, Err(_)) => Ok(Value::Int(1)),
            (-1, Err(_)) => Ok(Value::Int(if y % 2 == 0 { 1 } else { -1 })),
            _ => unsupported("a negative power of an integer in a `CONST`"),
        };
    }
    let (x, y) = (as_float(a), as_float(b));
    if x.v < 0.0 && y.v.fract() != 0.0 && !near_integer(y) {
        // Measured: `(-8) ^ (1 / 3)` is an internal error of the old compiler.
        return error("a negative number to a fractional power");
    }
    exact_pair(x, y)?;
    if y.v == 0.5 {
        let r = x.v.sqrt();
        // (x - r*r) / 2r: the error of the rounded root to well within the margin.
        let err = if r == 0.0 {
            0.0
        } else {
            (-r).mul_add(r, x.v) / (2.0 * r)
        };
        return rounded(r, err);
    }
    if y.v.fract() == 0.0 && (0.0..=64.0).contains(&y.v) {
        let mut p = Value::Float(Float { v: 1.0, exact: true });
        for _ in 0..int_of(y.v).expect("small whole exponent") {
            p = binary(BinOp::Mul, &p, &Value::Float(x))?;
            if matches!(p, Value::Float(f) if !f.exact) {
                return unsupported("a rounded power in a `CONST`");
            }
        }
        return Ok(p);
    }
    unsupported("this power in a `CONST`")
}

/// A prefix operator: `-` and `NOT`. (`_NEGATE` is not an operator of the old evaluator.)
pub fn unary(op: UnOp, a: &Value) -> R<Value> {
    match (op, a) {
        (_, Value::Str(_)) => error("this operator cannot be used on strings"),
        (UnOp::Neg, Value::Int(i)) => Ok(Value::Int(i.wrapping_neg())),
        (UnOp::Neg, Value::Float(f)) => Ok(Value::Float(Float {
            v: -f.v,
            exact: f.exact,
        })),
        (UnOp::Not, _) => Ok(Value::Int(!to_int(a)?)),
        (UnOp::Negate, _) => unsupported("`_NEGATE` in a `CONST`"),
    }
}

/// The type and value of a constant: the value's own (an integer `_INTEGER64`, a float DOUBLE, measured), or the
/// type of the suffix on its name, converted (a float rounded half to even to an integer).
pub fn settle(v: Value, suffix: Option<Ty>) -> R<(Ty, Value)> {
    let own = match &v {
        Value::Int(_) => Ty::I64,
        Value::Float(_) => Ty::F64,
        Value::Str(_) => Ty::Str,
    };
    convert(own, v, suffix.unwrap_or(own))
}

/// A constant of type `from` used as type `to` (by the suffix of its name or of a use).
pub fn convert(from: Ty, v: Value, to: Ty) -> R<(Ty, Value)> {
    if from == to {
        return Ok((to, v));
    }
    let v = match (v, to) {
        (Value::Str(_), _) | (_, Ty::Str | Ty::User(_)) => return error("type mismatch"),
        (_, crate::unproduced_types!()) => unreachable!("{}", crate::NEW_TYPE_UNREACHABLE),
        (v, Ty::I16 | Ty::I32 | Ty::I64) => {
            let i = to_int(&v)?;
            let (lo, hi) = crate::literal::range(to);
            if !(lo..=hi).contains(&i128::from(i)) {
                // The old compiler has no range check here.
                return unsupported("a `CONST` value beyond the range of its type");
            }
            Value::Int(i)
        }
        (Value::Int(i), Ty::F32 | Ty::F64) => Value::Float(Float {
            exact: true,
            ..float_of_int(i)
        }),
        (Value::Float(f), Ty::F32 | Ty::F64) => Value::Float(f),
        (v, Ty::F80) => {
            // A `_FLOAT` keeps the old value's 64 bits, which `f64` has only when it is exact.
            let f = as_float(&v);
            if !f.exact || exact_digits(f.v) > 19 {
                return unsupported("a `_FLOAT` `CONST` value that needs more than `DOUBLE` precision");
            }
            Value::Float(f)
        }
    };
    Ok((to, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(i: i64) -> Value {
        Value::Int(i)
    }

    fn flt(text: &str) -> Value {
        Value::Float(float_literal(text).unwrap())
    }

    fn bin(op: BinOp, a: &Value, b: &Value) -> Value {
        binary(op, a, b).unwrap()
    }

    fn is_unsupported<T: std::fmt::Debug>(r: R<T>) -> bool {
        matches!(r, Err(Problem::Unsupported(_)))
    }

    fn is_error<T: std::fmt::Debug>(r: R<T>) -> bool {
        matches!(r, Err(Problem::Error(_)))
    }

    #[test]
    fn integers_wrap_in_64_bits() {
        assert_eq!(bin(BinOp::Add, &int(i64::MAX), &int(1)), int(i64::MIN));
        assert_eq!(bin(BinOp::Add, &int(2147483647), &int(1)), int(2147483648));
        assert_eq!(bin(BinOp::Mul, &int(3000000000), &int(4)), int(12000000000));
        assert_eq!(bin(BinOp::Sub, &int(2), &int(3)), int(-1));
        assert_eq!(unary(UnOp::Neg, &int(i64::MIN)).unwrap(), int(i64::MIN));
    }

    #[test]
    fn each_operator() {
        assert_eq!(bin(BinOp::IDiv, &int(7), &int(2)), int(3));
        assert_eq!(bin(BinOp::IDiv, &int(-7), &int(2)), int(-3));
        assert_eq!(bin(BinOp::Mod, &int(-7), &int(3)), int(-1));
        assert_eq!(bin(BinOp::IDiv, &flt("7.5E+0"), &int(2)), int(4));
        assert_eq!(bin(BinOp::Mod, &flt("7.5E+0"), &int(2)), int(0));
        assert_eq!(bin(BinOp::And, &int(6), &int(3)), int(2));
        assert_eq!(bin(BinOp::And, &flt("1.5E+0"), &int(3)), int(2));
        assert_eq!(bin(BinOp::Or, &int(4), &int(1)), int(5));
        assert_eq!(bin(BinOp::Xor, &int(5), &int(3)), int(6));
        assert_eq!(bin(BinOp::Eqv, &int(5), &int(3)), int(-7));
        assert_eq!(bin(BinOp::Imp, &int(5), &int(3)), int(-5));
        assert_eq!(unary(UnOp::Not, &int(0)).unwrap(), int(-1));
        assert_eq!(bin(BinOp::Gt, &int(3), &int(2)), int(-1));
        assert_eq!(bin(BinOp::Gt, &int(2), &int(3)), int(0));
        // A comparison with a float operand gives a float.
        assert_eq!(
            bin(BinOp::Lt, &flt("1.5E+0"), &int(2)),
            Value::Float(Float { v: -1.0, exact: true })
        );
        assert_eq!(
            bin(BinOp::Add, &Value::Str(b"x".to_vec()), &Value::Str(b"y".to_vec())),
            Value::Str(b"xy".to_vec())
        );
    }

    #[test]
    fn division_is_float_and_rereads_as_integer_when_whole() {
        let half = bin(BinOp::Div, &int(4), &int(2));
        assert_eq!(half, Value::Float(Float { v: 2.0, exact: true }));
        assert_eq!(reread(half).unwrap(), int(2));
        let third = bin(BinOp::Div, &int(1), &int(3));
        assert_eq!(
            third,
            Value::Float(Float {
                v: 1.0 / 3.0,
                exact: false
            })
        );
        assert_eq!(settle(reread(third).unwrap(), None).unwrap().0, Ty::F64);
        assert_eq!(reread(bin(BinOp::Mul, &flt("2.5E+0"), &int(2))).unwrap(), int(5));
        assert_eq!(reread(flt("2.5E+10")).unwrap(), int(25000000000));
    }

    #[test]
    fn power_is_right_associative_at_the_tree_level_and_exact() {
        // 2 ^ (3 ^ 2) and (2 ^ 3) ^ 2: the tree decides the grouping; the values are integers.
        assert_eq!(bin(BinOp::Pow, &int(2), &bin(BinOp::Pow, &int(3), &int(2))), int(512));
        assert_eq!(bin(BinOp::Pow, &bin(BinOp::Pow, &int(2), &int(3)), &int(2)), int(64));
        assert_eq!(bin(BinOp::Pow, &int(2), &int(10)), int(1024));
        let root = bin(BinOp::Pow, &int(2), &flt("5.0E-1"));
        assert_eq!(
            root,
            Value::Float(Float {
                v: 2f64.sqrt(),
                exact: false
            })
        );
        assert_eq!(
            bin(BinOp::Pow, &flt("2.5E+0"), &int(2)),
            Value::Float(Float { v: 6.25, exact: true })
        );
        assert!(is_unsupported(binary(BinOp::Pow, &int(2), &int(70))));
        assert!(is_unsupported(binary(BinOp::Pow, &int(2), &int(-1))));
        assert!(is_error(binary(
            BinOp::Pow,
            &int(-8),
            &bin(BinOp::Div, &int(1), &int(3))
        )));
    }

    #[test]
    fn suffixes_round_half_to_even() {
        let cases = [
            ("3.7E+0", Ty::I16, 4),
            ("2.5E+0", Ty::I16, 2),
            ("3.5E+0", Ty::I16, 4),
            ("2.4999E+0", Ty::I16, 2),
        ];
        for (text, ty, want) in cases {
            assert_eq!(settle(flt(text), Some(ty)).unwrap(), (ty, int(want)), "{text}");
        }
        let neg = unary(UnOp::Neg, &flt("2.5E+0")).unwrap();
        assert_eq!(settle(neg, Some(Ty::I32)).unwrap(), (Ty::I32, int(-2)));
        // No range check in the old compiler: not supported yet.
        assert!(is_unsupported(settle(int(40000), Some(Ty::I16))));
        assert!(is_error(settle(Value::Str(b"x".to_vec()), Some(Ty::I16))));
        assert!(is_error(settle(int(5), Some(Ty::Str))));
    }

    #[test]
    fn errors_and_unsupported() {
        assert!(is_error(binary(BinOp::IDiv, &int(1), &int(0))));
        assert!(is_error(binary(BinOp::Mod, &int(5), &int(0))));
        assert!(is_unsupported(binary(BinOp::Div, &int(1), &int(0))));
        assert!(is_error(binary(BinOp::Add, &Value::Str(b"a".to_vec()), &int(1))));
        assert!(is_error(binary(
            BinOp::Lt,
            &Value::Str(b"a".to_vec()),
            &Value::Str(b"b".to_vec())
        )));
        assert!(is_unsupported(binary(BinOp::IDiv, &int(i64::MIN), &int(-1))));
        // An integer-valued float beyond `_INTEGER64` range stays a float; an integer power beyond it is not
        // supported (the old evaluator wraps it).
        let big = reread(bin(BinOp::Div, &flt("1.0E+19"), &int(1))).unwrap();
        assert_eq!(settle(big, None).unwrap().0, Ty::F64);
        assert_eq!(settle(reread(flt("1.0E+30")).unwrap(), None).unwrap().0, Ty::F64);
        // A rounded value in another operation.
        let tenth = flt("1.0E-1");
        assert!(is_unsupported(binary(BinOp::Mul, &tenth, &int(3))));
    }

    #[test]
    fn literals_and_text() {
        assert_eq!(float_literal("1.0E-1").unwrap(), Float { v: 0.1, exact: false });
        assert_eq!(float_literal("2.5E+0").unwrap(), Float { v: 2.5, exact: true });
        assert_eq!(float_literal("1.0E+30").unwrap(), Float { v: 1e30, exact: false });
        assert_eq!(shortest_text(1.0 / 3.0), "3.333333333333333E-1");
        assert_eq!(shortest_text(1e30), "1.0E+30");
        assert_eq!(exact_text(2.5), "2.5E+0");
        assert_eq!(exact_text(-0.375), "-3.75E-1");
        assert_eq!(decimal_parts("1234567.0E+0"), ("1234567".to_string(), 0));
        assert_eq!(decimal_parts("0.5E+0"), ("5".to_string(), -1));
    }
}
