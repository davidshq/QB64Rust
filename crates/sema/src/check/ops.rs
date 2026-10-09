//! Operators: the typing rules of every operator (design D4: the one place that types an operator) and the
//! folding of integer constants.

use crate::{BinOp, NEW_TYPE_UNREACHABLE, Ty, UnOp};

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
}

/// The typing rules of every operator, from the operands' (held, believed) types (`study\02` §1.4, design D4); for
/// a unary operator `r` is `None`. The only place in `sema` that matches on [`Ty`] to type an operator.
pub(super) fn op_typing(op: Op, l: (Ty, Ty), r: Option<(Ty, Ty)>) -> Result<Typed, TypeError> {
    let ((lt, lq), r) = (l, r);
    let strings = (lt == Ty::Str, r.is_some_and(|(rt, _)| rt == Ty::Str));
    let bin = match op {
        Op::Bin(b) => b,
        Op::Un(_) if strings.0 => return Err(TypeError::OnStrings),
        Op::Un(u) => {
            // A float operand of `NOT` or `_NEGATE` is rounded to `_INTEGER64` first.
            let rounded = if lt.is_float() { Ty::I64 } else { lt };
            return Ok(Typed::Num(match u {
                // Negating an integer is believed `_INTEGER64`, a float keeps its type (measured: `-x%` with
                // `x% = -32768` prints ` 32768 `).
                UnOp::Neg => Typing {
                    operands: promote(lt),
                    round: false,
                    ty: promote(lt),
                    qb: if lq.is_int() { Ty::I64 } else { lq },
                },
                // `~(x)`: in the operand's promoted width (measured: `NOT i%` computes in 32 bits).
                UnOp::Not => Typing {
                    operands: promote(rounded),
                    round: true,
                    ty: promote(rounded),
                    qb: Ty::I64,
                },
                // `-(!(x))`: a C `int`.
                UnOp::Negate => Typing {
                    operands: promote(rounded),
                    round: true,
                    ty: Ty::I32,
                    qb: Ty::I64,
                },
            }));
        }
    };
    let (rt, rq) = r.expect("a binary operator has a right operand");
    match strings {
        (true, true) if bin == BinOp::Add => return Ok(Typed::Concat),
        (true, true) if bin.is_comparison() => return Ok(Typed::StrCompare),
        (true, true) => return Err(TypeError::OnStrings),
        (true, false) | (false, true) => return Err(TypeError::Mixed),
        (false, false) => {}
    }
    // C's usual arithmetic conversions on the held types.
    let c_common = wider(promote(lt), promote(rt));
    let float_qb = [lq, rq].into_iter().filter(|t| t.is_float()).reduce(wider);
    // Operators on integers: float operands rounded to `_INTEGER64` first, then C's conversions.
    let int_common = wider(
        promote(if lt.is_float() { Ty::I64 } else { lt }),
        promote(if rt.is_float() { Ty::I64 } else { rt }),
    );
    Ok(Typed::Num(match bin {
        BinOp::Add | BinOp::Sub | BinOp::Mul => Typing {
            operands: c_common,
            round: false,
            ty: c_common,
            qb: float_qb.unwrap_or(Ty::I64),
        },
        // Integer / integer: the right operand is made `_FLOAT` (`study\02` §1.4).
        BinOp::Div => match float_qb {
            None => Typing {
                operands: Ty::F80,
                round: false,
                ty: Ty::F80,
                qb: Ty::F80,
            },
            Some(qb) => Typing {
                operands: c_common,
                round: false,
                ty: c_common,
                qb,
            },
        },
        // Two floats compare at the narrower believed type (`S! = 2.1` is true); otherwise C's conversions.
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => Typing {
            operands: if lq.is_float() && rq.is_float() {
                if lq.is_wider_than(rq) { rq } else { lq }
            } else {
                c_common
            },
            round: false,
            ty: Ty::I32,
            qb: Ty::I32,
        },
        BinOp::And | BinOp::Or | BinOp::Xor | BinOp::Eqv | BinOp::Imp | BinOp::IDiv | BinOp::Mod => Typing {
            operands: int_common,
            round: true,
            ty: int_common,
            qb: Ty::I64,
        },
        // `-(a&&b)`, `-(a||b)`: a C `int`.
        BinOp::AndAlso | BinOp::OrElse => Typing {
            operands: int_common,
            round: true,
            ty: Ty::I32,
            qb: Ty::I64,
        },
        // `pow2` computes in `long double`; an integer operand is believed SINGLE, DOUBLE or `_FLOAT` by its width.
        BinOp::Pow => Typing {
            operands: Ty::F80,
            round: false,
            ty: Ty::F80,
            qb: wider(pow_qb(lq), pow_qb(rq)),
        },
    }))
}

/// The believed type an operand of `^` gives its result.
fn pow_qb(t: Ty) -> Ty {
    match t {
        Ty::I16 => Ty::F32,
        Ty::I32 => Ty::F64,
        Ty::I64 => Ty::F80,
        Ty::F32 | Ty::F64 | Ty::F80 => t,
        Ty::Str => unreachable!("strings are refused before `^` is typed"),
        Ty::User(_) => unreachable!("a whole `TYPE` value is never an operand"),
        crate::unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
    }
}

/// Integer operands are computed in at least 32 bits (C promotion).
pub(super) fn promote(t: Ty) -> Ty {
    match t {
        Ty::I16 => Ty::I32,
        Ty::I32 | Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str => t,
        Ty::User(_) => unreachable!("a whole `TYPE` value is never an operand"),
        crate::unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
    }
}

/// The wider of two numeric types by [`Ty::is_wider_than`] (what the derived order's `max` was). Two integers of
/// one width and different types (signedness, `_OFFSET`) are the business of design D4's `held` (task 5.1), not of
/// this function.
pub(super) fn wider(a: Ty, b: Ty) -> Ty {
    assert!(
        a == b || a.is_wider_than(b) || b.is_wider_than(a),
        "{a:?} and {b:?} have one width: design D4's `held` decides"
    );
    if b.is_wider_than(a) { b } else { a }
}

/// The folded value of a binary operator on two integer constants already converted to the computation type, the
/// result wrapped to `ty` (D-001, D-002); `None` where the operator is left to run time: `/` and `^` (computed in
/// floating point), `\` and `MOD` by 0 (error 11) and the smallest value of `ty` by -1 (unspecified).
pub(super) fn fold_binary(op: BinOp, a: i64, b: i64, ty: Ty) -> Option<i64> {
    let truth = |c: bool| if c { -1 } else { 0 };
    // Wrapping in 64 bits, then to `ty`, keeps the same low bits as the exact result.
    let v = match op {
        BinOp::Add => a.wrapping_add(b),
        BinOp::Sub => a.wrapping_sub(b),
        BinOp::Mul => a.wrapping_mul(b),
        BinOp::Div | BinOp::Pow => return None,
        BinOp::Eq => truth(a == b),
        BinOp::Ne => truth(a != b),
        BinOp::Lt => truth(a < b),
        BinOp::Gt => truth(a > b),
        BinOp::Le => truth(a <= b),
        BinOp::Ge => truth(a >= b),
        BinOp::And => a & b,
        BinOp::Or => a | b,
        BinOp::Xor => a ^ b,
        BinOp::Eqv => !(a ^ b),
        BinOp::Imp => !a | b,
        BinOp::AndAlso => truth(a != 0 && b != 0),
        BinOp::OrElse => truth(a != 0 || b != 0),
        BinOp::IDiv | BinOp::Mod if b == 0 || (b == -1 && a == int_min(ty)) => return None,
        BinOp::IDiv => a / b,
        BinOp::Mod => a % b,
    };
    Some(wrap(v, ty))
}

/// The smallest value of an integer type.
fn int_min(ty: Ty) -> i64 {
    match ty {
        Ty::I16 => i16::MIN.into(),
        Ty::I32 => i32::MIN.into(),
        Ty::I64 => i64::MIN,
        Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str | Ty::User(_) => unreachable!("integer constant folded to {ty:?}"),
        crate::unproduced_types!() => unreachable!("{NEW_TYPE_UNREACHABLE}"),
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

/// Keeps the low bits of `v` that fit `ty` (integer overflow wraps, D-001, D-002).
#[expect(
    clippy::cast_possible_truncation,
    reason = "BASIC integer overflow wraps: the truncation is the point"
)]
pub(super) fn wrap(v: i64, ty: Ty) -> i64 {
    match ty {
        Ty::I16 => i64::from(v as i16),
        Ty::I32 => i64::from(v as i32),
        Ty::I64 => v,
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
        fold_binary(op, a, b, ty)
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
    fn wider_replaces_the_old_order() {
        // The derived order was I16 < I32 < I64 < F32 < F64 < F80; `wider` is its `max` on these types.
        let old = [Ty::I16, Ty::I32, Ty::I64, Ty::F32, Ty::F64, Ty::F80];
        for (i, &a) in old.iter().enumerate() {
            for (j, &b) in old.iter().enumerate() {
                assert_eq!(wider(a, b), old[i.max(j)], "{a:?} {b:?}");
            }
        }
    }

    #[test]
    #[should_panic(expected = "one width")]
    fn wider_leaves_mixed_signedness_to_held() {
        wider(Ty::I32, Ty::U32);
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
