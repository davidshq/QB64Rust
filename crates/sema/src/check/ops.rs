//! Operators: the typing rules of every operator (design D4: the one place that types an operator) and the
//! folding of integer constants.
//!
//! Every rule works on an operand's two types (the `sema` crate doc): the held type decides what C++ computes in
//! ([`held`]: C's usual arithmetic conversions on the C types the old compiler emits), the believed type decides
//! what the old compiler believes the result is (its "markup", `study\02` §1.4, [`int_believed`]).

use crate::{BIT_VALUE_UNREACHABLE, BinOp, NEW_TYPE_UNREACHABLE, Ty, UnOp};

/// An operator, as the typing rules see it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Op {
    Bin(BinOp),
    Un(UnOp),
}

/// How a numeric operator is computed (design D4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Typing {
    /// The type every operand is converted to.
    pub(super) operands: Ty,
    /// Whether a float operand is first rounded half to even to `_INTEGER64` (`qbr`), for operators on integers.
    pub(super) round: bool,
    /// The type the result is held in.
    pub(super) ty: Ty,
    /// The type the old compiler believes the result has.
    pub(super) qb: Ty,
    /// Whether the result, computed in `ty`, is then rounded half to even to `_INTEGER64` (`qbr`): `*` with a float
    /// operand and `/` when an operand is an `_OFFSET` (`qb64pe.bas` 19963–19980, 20175).
    pub(super) round_result: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Typed {
    Num(Typing),
    /// String `+`.
    Concat,
    /// A comparison of two strings.
    StrCompare,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TypeError {
    /// Strings on both sides of an operator that takes numbers, or a string operand of a unary operator.
    OnStrings,
    /// A string and a number.
    Mixed,
    /// `^` with an `_OFFSET` operand (measured: "Operator '^' cannot be used with an _OFFSET", `v21_x28`).
    OffsetPow,
}

/// Whether a type is `_OFFSET` or `_UNSIGNED _OFFSET`.
fn is_offset(t: Ty) -> bool {
    matches!(t, Ty::Off | Ty::UOff)
}

/// The typing rules of every operator, from the operands' (held, believed) types (`study\02` §1.4, design D4); for
/// a unary operator `r` is `None`. The only place in `sema` that matches on [`Ty`] to type an operator.
pub(super) fn op_typing(op: Op, l: (Ty, Ty), r: Option<(Ty, Ty)>) -> Result<Typed, TypeError> {
    let ((lt, lq), r) = (l, r);
    let strings = (lt == Ty::Str, r.is_some_and(|(rt, _)| rt == Ty::Str));
    let bin = match op {
        Op::Bin(b) => b,
        Op::Un(_) if strings.0 => return Err(TypeError::OnStrings),
        Op::Un(u) => return Ok(Typed::Num(unary_typing(u, lt, lq))),
    };
    let (rt, rq) = r.expect("a binary operator has a right operand");
    match strings {
        (true, true) if bin == BinOp::Add => return Ok(Typed::Concat),
        (true, true) if bin.is_comparison() => return Ok(Typed::StrCompare),
        (true, true) => return Err(TypeError::OnStrings),
        (true, false) | (false, true) => return Err(TypeError::Mixed),
        (false, false) => {}
    }
    if is_offset(lq) || is_offset(rq) {
        return offset_typing(bin, (lt, lq), (rt, rq)).map(Typed::Num);
    }
    // C's usual arithmetic conversions on the held types.
    let c_common = held(lt, rt);
    let float_qb = [lq, rq].into_iter().filter(|t| t.is_float()).reduce(wider_float);
    // Operators on integers: float operands rounded to `_INTEGER64` first (`qbr`), then C's conversions; the
    // markup sees a rounded operand as `_INTEGER64`.
    let int_operand = |t: Ty| if t.is_float() { Ty::I64 } else { t };
    let int_common = held(int_operand(lt), int_operand(rt));
    let int_qb = int_believed(int_operand(lq), int_operand(rq));
    let plain = |operands: Ty, round: bool, ty: Ty, qb: Ty| Typing {
        operands,
        round,
        ty,
        qb,
        round_result: false,
    };
    Ok(Typed::Num(match bin {
        BinOp::Add | BinOp::Sub | BinOp::Mul => plain(c_common, false, c_common, float_qb.unwrap_or(int_qb)),
        // Integer / integer: the right operand is made `_FLOAT` (`study\02` §1.4).
        BinOp::Div => match float_qb {
            None => plain(Ty::F80, false, Ty::F80, Ty::F80),
            Some(qb) => plain(c_common, false, c_common, qb),
        },
        // Two floats compare at the narrower believed type (`S! = 2.1` is true); otherwise C's conversions on the
        // held types (`-1 < u~&` is false).
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
            let operands = if lq.is_float() && rq.is_float() {
                if lq.is_wider_than(rq) { rq } else { lq }
            } else {
                c_common
            };
            plain(operands, false, Ty::I32, Ty::I32)
        }
        BinOp::And | BinOp::Or | BinOp::Xor | BinOp::Eqv | BinOp::Imp | BinOp::IDiv | BinOp::Mod => {
            plain(int_common, true, int_common, int_qb)
        }
        // `-(a&&b)`, `-(a||b)`: a C `int`, believed as the other integer operators.
        BinOp::AndAlso | BinOp::OrElse => plain(int_common, true, Ty::I32, int_qb),
        // `pow2` computes in `long double`; an integer operand is believed SINGLE, DOUBLE or `_FLOAT` by its width.
        BinOp::Pow => plain(Ty::F80, false, Ty::F80, wider_float(pow_qb(lq), pow_qb(rq))),
    }))
}

/// The type the left operand of `op`, held in `lt`, is converted to: `typing.operands`, except for `EQV` and `IMP`,
/// which the old compiler writes `~a^b` and `~a|b` (`study\02` §1.4): `~` complements the left operand in its own
/// promoted width, and C's conversions to the common type come after (measured, the differential `ops` programs:
/// `ULONG EQV INT64` zero-extends the complemented 32 bits).
pub(super) fn left_operand(op: BinOp, typing: &Typing, lt: Ty) -> Ty {
    match op {
        BinOp::Eqv | BinOp::Imp => promote(if lt.is_float() { Ty::I64 } else { lt }),
        BinOp::Add
        | BinOp::Sub
        | BinOp::Mul
        | BinOp::Div
        | BinOp::Eq
        | BinOp::Ne
        | BinOp::Lt
        | BinOp::Gt
        | BinOp::Le
        | BinOp::Ge
        | BinOp::And
        | BinOp::Or
        | BinOp::Xor
        | BinOp::AndAlso
        | BinOp::OrElse
        | BinOp::IDiv
        | BinOp::Mod
        | BinOp::Pow => typing.operands,
    }
}

/// The unary operators. The old compiler's markup sees the operand as both of its sides (`qb64pe.bas` 19888–19893).
fn unary_typing(u: UnOp, lt: Ty, lq: Ty) -> Typing {
    // A float operand of `NOT` or `_NEGATE` is rounded to `_INTEGER64` first.
    let (rounded, rounded_qb) = if lt.is_float() { (Ty::I64, Ty::I64) } else { (lt, lq) };
    let int_qb = |q: Ty| if is_offset(q) { q } else { int_believed(q, q) };
    match u {
        // Negating an integer is believed `_INTEGER64` (an `_UNSIGNED _INTEGER64` itself, an `_OFFSET` its offset
        // type), a float keeps its type (measured: `-x%` with `x% = -32768` prints ` 32768 `).
        UnOp::Neg => Typing {
            operands: promote(lt),
            round: false,
            ty: promote(lt),
            qb: if lq.is_float() { lq } else { int_qb(lq) },
            round_result: false,
        },
        // `~(x)`: in the operand's promoted width (measured: `NOT i%` computes in 32 bits).
        UnOp::Not => Typing {
            operands: promote(rounded),
            round: true,
            ty: promote(rounded),
            qb: int_qb(rounded_qb),
            round_result: false,
        },
        // `-(!(x))`: a C `int`.
        UnOp::Negate => Typing {
            operands: promote(rounded),
            round: true,
            ty: Ty::I32,
            qb: int_qb(rounded_qb),
            round_result: false,
        },
    }
}

/// A binary operator with an `_OFFSET` operand (`qb64pe.bas` 19950–19988, 20127–20131): `*` with a float operand
/// and `/` compute in `long double` and round the result (`qbr`); every other operator takes integers (a float
/// operand rounded first); the result is believed `_OFFSET`, unsigned unless an `_OFFSET` operand is signed, except
/// a comparison's (LONG). `^` is an error.
fn offset_typing(bin: BinOp, (lt, lq): (Ty, Ty), (rt, rq): (Ty, Ty)) -> Result<Typing, TypeError> {
    let qb = if lq == Ty::Off || rq == Ty::Off {
        Ty::Off
    } else {
        Ty::UOff
    };
    let int_operand = |t: Ty| if t.is_float() { Ty::I64 } else { t };
    let int_common = held(int_operand(lt), int_operand(rt));
    let integral = |ty: Ty, qb: Ty| Typing {
        operands: int_common,
        round: true,
        ty,
        qb,
        round_result: false,
    };
    // Integer operands made `long double`, float ones kept: C computes in the widest float, `long double`.
    let float = Typing {
        operands: Ty::F80,
        round: false,
        ty: Ty::F80,
        qb,
        round_result: true,
    };
    Ok(match bin {
        BinOp::Pow => return Err(TypeError::OffsetPow),
        BinOp::Div => float,
        BinOp::Mul if lt.is_float() || rt.is_float() => float,
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => integral(Ty::I32, Ty::I32),
        BinOp::AndAlso | BinOp::OrElse => integral(Ty::I32, qb),
        BinOp::Add
        | BinOp::Sub
        | BinOp::Mul
        | BinOp::And
        | BinOp::Or
        | BinOp::Xor
        | BinOp::Eqv
        | BinOp::Imp
        | BinOp::IDiv
        | BinOp::Mod => integral(int_common, qb),
    })
}

/// The believed type an operand of `^` gives its result.
fn pow_qb(t: Ty) -> Ty {
    if t.is_float() {
        return t;
    }
    match markup_bits(t) {
        ..=16 => Ty::F32,
        17..=32 => Ty::F64,
        _ => Ty::F80,
    }
}

/// The width the old compiler's markup sees for a believed integer type: a `_BIT * n` its n, every other its
/// [`Ty::int_bits`].
fn markup_bits(t: Ty) -> u32 {
    if let Ty::Bit { width, .. } = t {
        return u32::from(width);
    }
    t.int_bits().unwrap_or_else(|| unreachable!("{t:?} is no integer type"))
}

/// What the old compiler believes an integer operation on operands believed `a` and `b` gives (`qb64pe.bas`
/// 20076–20083): `_INTEGER64`, or `_UNSIGNED _INTEGER64` when both are unsigned and one is 64 bits wide.
pub(super) fn int_believed(a: Ty, b: Ty) -> Ty {
    if a.is_unsigned() && b.is_unsigned() && markup_bits(a).max(markup_bits(b)) == 64 {
        Ty::U64
    } else {
        Ty::I64
    }
}

/// C's integer promotion of a held type: below 32 bits to `int32`; `_OFFSET` is `int64` (`ptrszint`), `_UNSIGNED
/// _OFFSET` `uint64`. A float or a string is itself.
pub(super) fn promote(t: Ty) -> Ty {
    match t {
        Ty::I8 | Ty::U8 | Ty::I16 | Ty::U16 => Ty::I32,
        Ty::Off => Ty::I64,
        Ty::UOff => Ty::U64,
        Ty::I32 | Ty::U32 | Ty::I64 | Ty::U64 | Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str => t,
        Ty::Bit { .. } => unreachable!("{BIT_VALUE_UNREACHABLE}"),
        Ty::User(_) => unreachable!("a whole `TYPE` value is never an operand"),
        crate::unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
    }
}

/// The C++ type of `a op b` for operands held in `a` and `b` (design D4's `held`): C's usual arithmetic conversions.
/// A float operand makes it the widest float; otherwise each promoted ([`promote`]), then `uint64` if either is,
/// else `int64` if either is, else `uint32` if either is, else `int32`.
pub(super) fn held(a: Ty, b: Ty) -> Ty {
    if a.is_float() || b.is_float() {
        return [a, b]
            .into_iter()
            .filter(|t| t.is_float())
            .reduce(wider_float)
            .expect("a float operand");
    }
    let (a, b) = (promote(a), promote(b));
    if a == Ty::U64 || b == Ty::U64 {
        Ty::U64
    } else if a == Ty::I64 || b == Ty::I64 {
        Ty::I64
    } else if a == Ty::U32 || b == Ty::U32 {
        Ty::U32
    } else {
        Ty::I32
    }
}

/// The wider of two float types.
fn wider_float(a: Ty, b: Ty) -> Ty {
    if b.is_wider_than(a) { b } else { a }
}

/// The folded value of a binary operator on two integer constants, `a` already converted to its [`left_operand`]
/// type `lt` and `b` to `operands`, the result wrapped to `ty` (D-001, D-002); `None` where the operator is left to
/// run time: `/` and `^` (computed in floating point), `\` and `MOD` by 0 (error 11) and the smallest value of a
/// signed `ty` by -1 (unspecified). A 64-bit unsigned value is its 64 bits in the `i64`.
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "an unsigned 64-bit value is kept as its bits in an `i64`"
)]
pub(super) fn fold_binary(op: BinOp, (a, lt): (i64, Ty), (b, operands): (i64, Ty), ty: Ty) -> Option<i64> {
    let truth = |c: bool| if c { -1 } else { 0 };
    if matches!(op, BinOp::Div | BinOp::Pow) {
        return None;
    }
    // `~a` in the left operand's type, then converted to the common one (`EQV`, `IMP`).
    let not_a = wrap(wrap(!a, lt), operands);
    let a = wrap(a, operands);
    // 32-bit and narrower unsigned values are non-negative `i64`s, so only 64-bit unsigned ones need `u64`.
    let wide_unsigned = operands == Ty::U64;
    let (ua, ub) = (a as u64, b as u64);
    let order = if wide_unsigned { ua.cmp(&ub) } else { a.cmp(&b) };
    // Wrapping in 64 bits, then to `ty`, keeps the same low bits as the exact result.
    let v = match op {
        BinOp::Add => a.wrapping_add(b),
        BinOp::Sub => a.wrapping_sub(b),
        BinOp::Mul => a.wrapping_mul(b),
        BinOp::Div | BinOp::Pow => unreachable!("returned above"),
        BinOp::Eq => truth(order.is_eq()),
        BinOp::Ne => truth(order.is_ne()),
        BinOp::Lt => truth(order.is_lt()),
        BinOp::Gt => truth(order.is_gt()),
        BinOp::Le => truth(order.is_le()),
        BinOp::Ge => truth(order.is_ge()),
        BinOp::And => a & b,
        BinOp::Or => a | b,
        BinOp::Xor => a ^ b,
        BinOp::Eqv => not_a ^ b,
        BinOp::Imp => not_a | b,
        BinOp::AndAlso => truth(a != 0 && b != 0),
        BinOp::OrElse => truth(a != 0 || b != 0),
        BinOp::IDiv | BinOp::Mod if b == 0 => return None,
        BinOp::IDiv | BinOp::Mod if !operands.is_unsigned() && b == -1 && a == int_min(ty) => return None,
        BinOp::IDiv if wide_unsigned => (ua / ub) as i64,
        BinOp::Mod if wide_unsigned => (ua % ub) as i64,
        BinOp::IDiv => a / b,
        BinOp::Mod => a % b,
    };
    Some(wrap(v, ty))
}

/// The smallest value of a signed integer type.
fn int_min(ty: Ty) -> i64 {
    match ty.int_bits() {
        Some(8) => i8::MIN.into(),
        Some(16) => i16::MIN.into(),
        Some(32) => i32::MIN.into(),
        Some(_) => i64::MIN,
        None => unreachable!("integer constant folded to {ty:?}"),
    }
}

/// The folded value of a unary operator on an integer constant already converted to the computation type.
pub(super) fn fold_unary(op: UnOp, v: i64, ty: Ty) -> i64 {
    match op {
        UnOp::Neg => wrap(v.wrapping_neg(), ty),
        UnOp::Not => wrap(!v, ty),
        UnOp::Negate => {
            if v == 0 {
                -1
            } else {
                0
            }
        }
    }
}

/// Keeps the low bits of `v` that fit `ty`, read with its signedness (integer overflow wraps, D-001, D-002); a
/// 64-bit type keeps all 64. A `_BIT * n` wraps to its 32- or 64-bit storage type, not to its n bits: nothing folds
/// a constant to a `_BIT` type yet, and a caller that does must mask to n bits itself.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "BASIC integer overflow wraps: the truncation is the point"
)]
pub(super) fn wrap(v: i64, ty: Ty) -> i64 {
    match ty {
        Ty::I8 => i64::from(v as i8),
        Ty::U8 => i64::from(v as u8),
        Ty::I16 => i64::from(v as i16),
        Ty::U16 => i64::from(v as u16),
        Ty::I32 => i64::from(v as i32),
        Ty::U32 => i64::from(v as u32),
        Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff => v,
        Ty::Bit { .. } => wrap(v, ty.storage()),
        Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str | Ty::User(_) => unreachable!("integer constant folded to {ty:?}"),
        crate::unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I32_MAX: i64 = i32::MAX as i64;
    const I32_MIN: i64 = i32::MIN as i64;

    fn bin(op: BinOp, a: i64, b: i64, ty: Ty) -> Option<i64> {
        fold_binary(op, (a, ty), (b, ty), ty)
    }

    #[test]
    fn arithmetic_wraps_at_the_limits() {
        assert_eq!(bin(BinOp::Add, I32_MAX, 1, Ty::I32), Some(I32_MIN));
        assert_eq!(bin(BinOp::Sub, I32_MIN, 1, Ty::I32), Some(I32_MAX));
        assert_eq!(bin(BinOp::Mul, 65536, 65536, Ty::I32), Some(0));
        assert_eq!(bin(BinOp::Add, i64::MAX, 1, Ty::I64), Some(i64::MIN));
        assert_eq!(bin(BinOp::Mul, i64::MIN, -1, Ty::I64), Some(i64::MIN));
        assert_eq!(fold_unary(UnOp::Neg, I32_MIN, Ty::I32), I32_MIN);
        assert_eq!(fold_unary(UnOp::Neg, i64::MIN, Ty::I64), i64::MIN);
    }

    #[test]
    fn comparisons_give_minus_one_or_zero() {
        let cases = [
            (BinOp::Eq, 1, 1, -1),
            (BinOp::Eq, 1, 2, 0),
            (BinOp::Ne, 1, 2, -1),
            (BinOp::Lt, i64::MIN, i64::MAX, -1),
            (BinOp::Gt, i64::MIN, i64::MAX, 0),
            (BinOp::Le, 2, 2, -1),
            (BinOp::Ge, 2, 3, 0),
        ];
        for (op, a, b, want) in cases {
            assert_eq!(bin(op, a, b, Ty::I32), Some(want), "{op:?} {a} {b}");
        }
    }

    #[test]
    fn logic_is_bitwise() {
        assert_eq!(bin(BinOp::And, 6, 3, Ty::I32), Some(2));
        assert_eq!(bin(BinOp::Or, 6, 3, Ty::I32), Some(7));
        assert_eq!(bin(BinOp::Xor, 6, 3, Ty::I32), Some(5));
        assert_eq!(bin(BinOp::Eqv, 5, 3, Ty::I32), Some(-7));
        assert_eq!(bin(BinOp::Imp, 5, 3, Ty::I32), Some(-5));
        assert_eq!(bin(BinOp::Or, I32_MAX, I32_MIN, Ty::I32), Some(-1));
        assert_eq!(bin(BinOp::Xor, i64::MAX, i64::MIN, Ty::I64), Some(-1));
        assert_eq!(bin(BinOp::Imp, i64::MAX, 0, Ty::I64), Some(i64::MIN));
        assert_eq!(fold_unary(UnOp::Not, 5, Ty::I32), -6);
        assert_eq!(fold_unary(UnOp::Not, I32_MAX, Ty::I32), I32_MIN);
        assert_eq!(fold_unary(UnOp::Not, i64::MAX, Ty::I64), i64::MIN);
    }

    #[test]
    fn short_circuit_and_negate_give_minus_one_or_zero() {
        assert_eq!(bin(BinOp::AndAlso, 1, 2, Ty::I32), Some(-1));
        assert_eq!(bin(BinOp::AndAlso, 0, 2, Ty::I32), Some(0));
        assert_eq!(bin(BinOp::OrElse, 0, 0, Ty::I32), Some(0));
        assert_eq!(bin(BinOp::OrElse, 0, i64::MIN, Ty::I32), Some(-1));
        assert_eq!(fold_unary(UnOp::Negate, 0, Ty::I32), -1);
        assert_eq!(fold_unary(UnOp::Negate, 5, Ty::I32), 0);
    }

    #[test]
    fn integer_division_and_mod_truncate_and_stay_at_run_time_by_zero() {
        assert_eq!(bin(BinOp::IDiv, 7, 2, Ty::I32), Some(3));
        assert_eq!(bin(BinOp::IDiv, -7, 2, Ty::I32), Some(-3));
        assert_eq!(bin(BinOp::IDiv, 7, -2, Ty::I32), Some(-3));
        assert_eq!(bin(BinOp::Mod, -7, 3, Ty::I32), Some(-1));
        assert_eq!(bin(BinOp::Mod, 7, -3, Ty::I32), Some(1));
        assert_eq!(bin(BinOp::IDiv, i64::MAX, 2, Ty::I64), Some(i64::MAX / 2));
        // Error 11 belongs to run time; the smallest value by -1 is unspecified (the old program crashes).
        assert_eq!(bin(BinOp::IDiv, 1, 0, Ty::I32), None);
        assert_eq!(bin(BinOp::Mod, 1, 0, Ty::I64), None);
        assert_eq!(bin(BinOp::IDiv, I32_MIN, -1, Ty::I32), None);
        assert_eq!(bin(BinOp::Mod, i64::MIN, -1, Ty::I64), None);
        assert_eq!(bin(BinOp::IDiv, I32_MIN, -1, Ty::I64), Some(-I32_MIN));
    }

    #[test]
    fn floating_point_operators_are_not_folded() {
        assert_eq!(bin(BinOp::Div, 1, 2, Ty::F80), None);
        assert_eq!(bin(BinOp::Pow, 2, 3, Ty::F80), None);
    }

    fn num(op: Op, l: (Ty, Ty), r: Option<(Ty, Ty)>) -> Typing {
        match op_typing(op, l, r) {
            Ok(Typed::Num(t)) => t,
            other => panic!("{op:?}: {other:?}"),
        }
    }

    #[test]
    fn held_on_the_old_types_is_the_old_order() {
        // The derived order was I16 < I32 < I64 < F32 < F64 < F80; C's conversions on these types are its `max`, with
        // INTEGER promoted to LONG.
        let old = [Ty::I16, Ty::I32, Ty::I64, Ty::F32, Ty::F64, Ty::F80];
        for (i, &a) in old.iter().enumerate() {
            for (j, &b) in old.iter().enumerate() {
                assert_eq!(held(a, b), promote(old[i.max(j)]), "{a:?} {b:?}");
            }
        }
    }

    #[test]
    fn held_follows_cs_usual_arithmetic_conversions() {
        let cases = [
            // Below 32 bits, signed or not: `int`.
            (Ty::I8, Ty::U8, Ty::I32),
            (Ty::U16, Ty::U16, Ty::I32),
            (Ty::U8, Ty::I16, Ty::I32),
            // 32 bits: unsigned wins.
            (Ty::I32, Ty::U32, Ty::U32),
            (Ty::U16, Ty::U32, Ty::U32),
            // 64 bits: `int64` holds every `uint32`; unsigned 64 wins.
            (Ty::U32, Ty::I64, Ty::I64),
            (Ty::I64, Ty::U64, Ty::U64),
            (Ty::I8, Ty::U64, Ty::U64),
            // `_OFFSET` is `ptrszint`, `int64`.
            (Ty::Off, Ty::U32, Ty::I64),
            (Ty::Off, Ty::UOff, Ty::U64),
            // A float operand wins.
            (Ty::U64, Ty::F32, Ty::F32),
            (Ty::F64, Ty::UOff, Ty::F64),
        ];
        for (a, b, want) in cases {
            assert_eq!(held(a, b), want, "{a:?} {b:?}");
            assert_eq!(held(b, a), want, "{b:?} {a:?}");
        }
    }

    #[test]
    fn int_believed_is_unsigned_only_for_two_unsigned_with_64_bits() {
        let ubit = |width| Ty::Bit { width, signed: false };
        assert_eq!(int_believed(Ty::U32, Ty::U32), Ty::I64);
        assert_eq!(int_believed(Ty::U8, Ty::U64), Ty::U64);
        assert_eq!(int_believed(Ty::U64, Ty::I8), Ty::I64);
        assert_eq!(int_believed(ubit(7), Ty::U64), Ty::U64);
        // The markup sees a `_BIT * n` as n bits, not its 64-bit storage.
        assert_eq!(int_believed(ubit(40), ubit(40)), Ty::I64);
        assert_eq!(int_believed(ubit(64), ubit(1)), Ty::U64);
    }

    #[test]
    fn mixed_signedness_typing_of_each_family() {
        let ulong = (Ty::U32, Ty::U32);
        let long = (Ty::I32, Ty::I32);
        let uint64 = (Ty::U64, Ty::U64);
        let ubit7 = (
            Ty::U32,
            Ty::Bit {
                width: 7,
                signed: false,
            },
        );
        // `-1 < u~&` is false: compared as `uint32`.
        let t = num(Op::Bin(BinOp::Lt), long, Some(ulong));
        assert_eq!((t.operands, t.ty, t.qb), (Ty::U32, Ty::I32, Ty::I32));
        let t = num(Op::Bin(BinOp::Add), ubit7, Some(uint64));
        assert_eq!((t.operands, t.qb), (Ty::U64, Ty::U64));
        let t = num(Op::Bin(BinOp::Mul), ulong, Some(ulong));
        assert_eq!((t.ty, t.qb), (Ty::U32, Ty::I64));
        let t = num(Op::Bin(BinOp::And), uint64, Some((Ty::F32, Ty::F32)));
        assert_eq!((t.operands, t.round, t.qb), (Ty::U64, true, Ty::I64));
        let t = num(Op::Bin(BinOp::AndAlso), uint64, Some(uint64));
        assert_eq!((t.ty, t.qb), (Ty::I32, Ty::U64));
        let t = num(Op::Un(UnOp::Neg), uint64, None);
        assert_eq!((t.ty, t.qb), (Ty::U64, Ty::U64));
        let t = num(Op::Un(UnOp::Not), (Ty::U8, Ty::U8), None);
        assert_eq!((t.ty, t.qb), (Ty::I32, Ty::I64));
        assert_eq!(num(Op::Bin(BinOp::Pow), (Ty::U8, Ty::U8), Some(ubit7)).qb, Ty::F32);
        // `EQV` and `IMP` complement the left operand in its own width.
        let t = num(Op::Bin(BinOp::Eqv), ulong, Some((Ty::I64, Ty::I64)));
        assert_eq!(left_operand(BinOp::Eqv, &t, Ty::U32), Ty::U32);
        assert_eq!(left_operand(BinOp::Xor, &t, Ty::U32), Ty::I64);
        assert_eq!(left_operand(BinOp::Imp, &t, Ty::U8), Ty::I32);
    }

    #[test]
    fn offset_operands() {
        let off = (Ty::Off, Ty::Off);
        let uoff = (Ty::UOff, Ty::UOff);
        let single = (Ty::F32, Ty::F32);
        // Believed `_OFFSET`, unsigned unless an `_OFFSET` operand is signed.
        let t = num(Op::Bin(BinOp::Add), uoff, Some((Ty::I32, Ty::I32)));
        assert_eq!(
            (t.operands, t.ty, t.qb, t.round_result),
            (Ty::U64, Ty::U64, Ty::UOff, false)
        );
        assert_eq!(num(Op::Bin(BinOp::Sub), uoff, Some(off)).qb, Ty::Off);
        // Other operators take integers: a float operand is rounded first.
        let t = num(Op::Bin(BinOp::Add), off, Some(single));
        assert_eq!((t.operands, t.round), (Ty::I64, true));
        // `*` with a float and `/` compute in `long double`, then `qbr`.
        let t = num(Op::Bin(BinOp::Mul), off, Some(single));
        assert_eq!((t.operands, t.round_result, t.qb), (Ty::F80, true, Ty::Off));
        let t = num(Op::Bin(BinOp::Div), uoff, Some((Ty::I16, Ty::I16)));
        assert_eq!((t.operands, t.round_result, t.qb), (Ty::F80, true, Ty::UOff));
        assert!(!num(Op::Bin(BinOp::Mul), off, Some(uoff)).round_result);
        // Comparisons are LONG; `^` is an error.
        let t = num(Op::Bin(BinOp::Lt), off, Some(single));
        assert_eq!((t.operands, t.round, t.qb), (Ty::I64, true, Ty::I32));
        assert_eq!(
            op_typing(Op::Bin(BinOp::Pow), off, Some(single)),
            Err(TypeError::OffsetPow)
        );
        assert_eq!(num(Op::Un(UnOp::Neg), uoff, None).qb, Ty::UOff);
    }

    #[test]
    fn folding_with_unsigned_and_narrow_types() {
        let u64_max = -1; // 2^64-1 as its bits
        // Unsigned 64-bit comparisons and division.
        assert_eq!(
            fold_binary(BinOp::Lt, (1, Ty::U64), (u64_max, Ty::U64), Ty::I32),
            Some(-1)
        );
        assert_eq!(
            fold_binary(BinOp::IDiv, (u64_max, Ty::U64), (2, Ty::U64), Ty::U64),
            Some(i64::MAX)
        );
        assert_eq!(
            fold_binary(BinOp::Mod, (u64_max, Ty::U64), (10, Ty::U64), Ty::U64),
            Some(5)
        );
        // `uint32` wraps to its own range.
        assert_eq!(
            fold_binary(BinOp::Add, (4_294_967_295, Ty::U32), (1, Ty::U32), Ty::U32),
            Some(0)
        );
        assert_eq!(
            fold_binary(BinOp::Sub, (0, Ty::U32), (1, Ty::U32), Ty::U32),
            Some(4_294_967_295)
        );
        // No exception for the smallest signed value by -1 when the operands are unsigned.
        assert_eq!(
            fold_binary(BinOp::IDiv, (0, Ty::U32), (4_294_967_295, Ty::U32), Ty::U32),
            Some(0)
        );
        // `EQV`: `~a` in `uint32`, zero-extended to `int64`.
        assert_eq!(
            fold_binary(BinOp::Eqv, (4_294_967_295, Ty::U32), (0, Ty::I64), Ty::I64),
            Some(0)
        );
        assert_eq!(
            fold_binary(BinOp::Imp, (0, Ty::U32), (0, Ty::I64), Ty::I64),
            Some(4_294_967_295)
        );
        assert_eq!(wrap(300, Ty::U8), 44);
        assert_eq!(wrap(200, Ty::I8), -56);
        assert_eq!(wrap(-1, Ty::U16), 65535);
        assert_eq!(
            wrap(
                -1,
                Ty::Bit {
                    width: 7,
                    signed: false
                }
            ),
            4_294_967_295
        );
    }

    #[test]
    fn typing_of_each_family() {
        let i16_ = (Ty::I16, Ty::I16);
        let i32_ = (Ty::I32, Ty::I32);
        let i64_ = (Ty::I64, Ty::I64);
        let single = (Ty::F32, Ty::F32);
        let single_lit = (Ty::F64, Ty::F32);
        let double = (Ty::F64, Ty::F64);
        // Comparisons: two floats at the narrower believed type, otherwise C's conversions; LONG.
        let t = num(Op::Bin(BinOp::Eq), single, Some(single_lit));
        assert_eq!((t.operands, t.ty, t.qb), (Ty::F32, Ty::I32, Ty::I32));
        assert_eq!(
            num(Op::Bin(BinOp::Lt), double, Some((Ty::F80, Ty::F80))).operands,
            Ty::F64
        );
        assert_eq!(num(Op::Bin(BinOp::Lt), i64_, Some(single)).operands, Ty::F32);
        assert_eq!(num(Op::Bin(BinOp::Ne), i16_, Some(i16_)).operands, Ty::I32);
        // Logic, `\` and MOD: rounded floats, the promoted width, believed `_INTEGER64`.
        let t = num(Op::Bin(BinOp::And), i16_, Some(i16_));
        assert_eq!((t.operands, t.round, t.ty, t.qb), (Ty::I32, true, Ty::I32, Ty::I64));
        assert_eq!(num(Op::Bin(BinOp::Mod), single, Some(i16_)).operands, Ty::I64);
        assert_eq!(num(Op::Bin(BinOp::IDiv), i32_, Some(i64_)).ty, Ty::I64);
        let t = num(Op::Bin(BinOp::OrElse), i64_, Some(i16_));
        assert_eq!((t.operands, t.ty), (Ty::I64, Ty::I32));
        // Unary.
        let t = num(Op::Un(UnOp::Not), i16_, None);
        assert_eq!((t.operands, t.ty, t.qb), (Ty::I32, Ty::I32, Ty::I64));
        let t = num(Op::Un(UnOp::Negate), single, None);
        assert_eq!((t.operands, t.round, t.ty), (Ty::I64, true, Ty::I32));
        assert_eq!(num(Op::Un(UnOp::Neg), single_lit, None).qb, Ty::F32);
        // Power: `_FLOAT`, believed by the operand widths.
        let t = num(Op::Bin(BinOp::Pow), i16_, Some(single_lit));
        assert_eq!((t.operands, t.ty, t.qb), (Ty::F80, Ty::F80, Ty::F32));
        assert_eq!(num(Op::Bin(BinOp::Pow), i32_, Some(i16_)).qb, Ty::F64);
        assert_eq!(num(Op::Bin(BinOp::Pow), i64_, Some(single)).qb, Ty::F80);
    }

    #[test]
    fn strings() {
        let s = (Ty::Str, Ty::Str);
        let n = (Ty::I32, Ty::I32);
        assert_eq!(op_typing(Op::Bin(BinOp::Add), s, Some(s)), Ok(Typed::Concat));
        assert_eq!(op_typing(Op::Bin(BinOp::Ge), s, Some(s)), Ok(Typed::StrCompare));
        assert_eq!(op_typing(Op::Bin(BinOp::And), s, Some(s)), Err(TypeError::OnStrings));
        assert_eq!(op_typing(Op::Bin(BinOp::Eq), s, Some(n)), Err(TypeError::Mixed));
        assert_eq!(op_typing(Op::Un(UnOp::Not), s, None), Err(TypeError::OnStrings));
    }
}
