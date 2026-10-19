//! The certification of the `CONST` evaluator's functions whose old value depends on the CPU and the system's
//! library (design D3, D4 of `m3-constant-evaluation`).
//!
//! The old evaluator calls the old compiler's own run-time functions inside `qb64pe.exe`: the x87's `fsin`,
//! `fcos`, `fptan`, `fpatan` through mingw's `long double` functions, mingw's `expl` and `powl`, and UCRT's `double`
//! functions inside libqb's `func_log`, `func_arcsec` … `func_cot` (read from the C++ `qb64pe.exe` runs and from
//! `extended_math.h`). None is correctly rounded. Each function is written here as the expression its library
//! computes: the exact steps exactly ([`F80`] for `long double`, `f64` for `double`), and each library call as a
//! **fuzzy step**: the values of its format within the call's error bound of the true value (a high-precision
//! reference). The result is a [`Range`] of the values the old compiler might hold; the evaluator's
//! re-read settles it to one value or calls it "not supported yet".
//!
//! The reference is `astro_float_num`'s, guarded: computed at two precisions that must agree (design D3).

use crate::consteval::{Problem, R, Range, error, fell_back, unsupported};
use crate::f80::{EXP_MAX, EXP_MIN, F80, Integral};
use astro_float_num::{BigFloat, Consts, RoundingMode};
use std::cell::RefCell;
use std::cmp::Ordering;

/// The two precisions of the reference (design D3, guard 1) and how close they must agree.
const P1: usize = 192;
const P2: usize = 320;
const AGREE: i32 = 180;

/// The error bounds of the library calls, in units in the last place of the call's format, and the bound of the
/// x87's argument reduction relative to the argument (design D4; calibrated by `verification\v45_sample`, task 1.3).
pub const X87_ULPS: f64 = 1.0;
/// `fsin`, `fcos`, `fptan` reduce with an internal π of 66 bits: an absolute error up to |x| × 2^-66.
pub const X87_REDUCTION: i32 = -66;
// Measured in the sample: `expl` 0.98 ulp, `powl` 6.6 ulp; the bounds are those widened by half.
pub const EXPL_ULPS: f64 = 1.5;
pub const POWL_ULPS: f64 = 10.0;
/// UCRT's `double` functions, faithful but not correctly rounded. The smallest error the sample shows for each
/// (directly, or through the exact outer step of a libqb helper): `log` 0.4993 ulp (correctly rounded there; kept
/// at half an ulp and a little), `exp` 0.51, `tan` 0.58, `acos` 0.58, `sin` 0.60, `asin` 0.61, `cos` 0.62, `atan`
/// 0.72; the bounds are those widened by half.
pub const UCRT_LOG_ULPS: f64 = 0.5 + 1.0 / 512.0;
pub const UCRT_EXP_ULPS: f64 = 0.76;
pub const UCRT_TAN_ULPS: f64 = 0.87;
pub const UCRT_ACOS_ULPS: f64 = 0.88;
pub const UCRT_SIN_ULPS: f64 = 0.90;
pub const UCRT_ASIN_ULPS: f64 = 0.92;
pub const UCRT_COS_ULPS: f64 = 0.93;
pub const UCRT_ATAN_ULPS: f64 = 1.08;

/// The largest argument the x87's trigonometric instructions take (beyond it mingw reduces with `fprem1` and a
/// 64-bit π, which no bound here covers).
const X87_TRIG_MAX: f64 = 9.223_372_036_854_776e18;

thread_local! {
    static CONSTS: RefCell<Consts> = RefCell::new(Consts::new().expect("constants of the reference"));
}

/// Why a reference is not taken: its two precisions do not agree (design D3, guard 1).
const UNSETTLED: &str = "a `CONST` value whose reference does not agree with itself";

/// The reference value of `f` (at two precisions, which must agree) or why there is none.
fn reference(f: impl Fn(usize, &mut Consts) -> BigFloat) -> R<BigFloat> {
    CONSTS.with(|cc| {
        let mut cc = cc.borrow_mut();
        let a = f(P1, &mut cc);
        let b = f(P2, &mut cc);
        if a.is_nan() || b.is_nan() {
            return unsupported("a `CONST` value its reference does not give");
        }
        if b.is_zero() {
            return if a.is_zero() { Ok(b) } else { unsupported(UNSETTLED) };
        }
        let diff = a.sub(&b, P2, RoundingMode::ToEven).abs();
        let scale = b.exponent().unwrap_or(0) - AGREE;
        let mut tol = BigFloat::from_u64(1, 64);
        tol.set_exponent(scale + 1);
        if diff.partial_cmp(&tol) == Some(Ordering::Greater) {
            return unsupported(UNSETTLED);
        }
        Ok(b)
    })
}

/// A big float of `k` units at binary exponent `e` (value `k × 2^e`, `k` positive).
fn units(k: f64, e: i32) -> BigFloat {
    // k as a multiple of 1/1024.
    let n = (k * 1024.0).ceil();
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a small positive count"
    )]
    let n = n as u64;
    let mut b = BigFloat::from_u64(n, 64);
    let bits = 64 - i32::try_from(n.leading_zeros()).expect("small");
    b.set_exponent(e - 10 + bits);
    b
}

/// The binary exponent `e` of a big float (value `0.m × 2^e`).
fn exp_of(b: &BigFloat) -> i32 {
    b.exponent().unwrap_or(0)
}

/// The largest `_FLOAT` at or below `x`, and the smallest at or above it.
fn f80_down(x: &BigFloat) -> F80 {
    let mut v = F80::from_big(x);
    while v.to_big().partial_cmp(x) == Some(Ordering::Greater) {
        v = next_down(v);
    }
    v
}

fn f80_up(x: &BigFloat) -> F80 {
    let mut v = F80::from_big(x);
    while v.to_big().partial_cmp(x) == Some(Ordering::Less) {
        v = next_up(v);
    }
    v
}

fn next_up(v: F80) -> F80 {
    match v {
        F80::Zero { .. } => F80::Finite {
            neg: false,
            mant: 1,
            exp: EXP_MIN,
        },
        F80::Finite { neg: false, mant, exp } => {
            if mant == u64::MAX {
                if exp == EXP_MAX {
                    F80::Inf { neg: false }
                } else {
                    F80::Finite {
                        neg: false,
                        mant: 1 << 63,
                        exp: exp + 1,
                    }
                }
            } else {
                F80::Finite {
                    neg: false,
                    mant: mant + 1,
                    exp,
                }
            }
        }
        F80::Finite { neg: true, .. } => -next_down(-v),
        F80::Inf { .. } | F80::Nan => v,
    }
}

fn next_down(v: F80) -> F80 {
    match v {
        F80::Zero { .. } => -next_up(F80::ZERO),
        F80::Finite { neg: false, mant, exp } => {
            if mant == 1 << 63 && exp > EXP_MIN {
                F80::Finite {
                    neg: false,
                    mant: u64::MAX,
                    exp: exp - 1,
                }
            } else if mant == 1 {
                F80::ZERO
            } else {
                F80::Finite {
                    neg: false,
                    mant: mant - 1,
                    exp,
                }
            }
        }
        F80::Finite { neg: true, .. } => -next_up(-v),
        F80::Inf { .. } | F80::Nan => v,
    }
}

/// The largest `double` at or below `x`, and the smallest at or above it.
fn f64_down(x: &BigFloat) -> f64 {
    let mut d = F80::from_big(x).to_f64();
    while F80::from_f64(d).to_big().partial_cmp(x) == Some(Ordering::Greater) {
        d = d.next_down();
    }
    d
}

fn f64_up(x: &BigFloat) -> f64 {
    let mut d = F80::from_big(x).to_f64();
    while F80::from_f64(d).to_big().partial_cmp(x) == Some(Ordering::Less) {
        d = d.next_up();
    }
    d
}

/// A `long double` library call: the values within `ulps` units in the last place of the 64-bit format of the
/// reference, plus an absolute error `extra`.
fn fuzzy80(r: &BigFloat, ulps: f64, extra: Option<BigFloat>) -> Range {
    if r.is_zero() && extra.is_none() {
        return Range::exact(F80::ZERO);
    }
    // A true value the format holds is returned as it is (`expl(0)`, `atanl(0)`).
    let v = F80::from_big(r);
    if extra.is_none() && v.to_big().partial_cmp(r) == Some(Ordering::Equal) {
        return Range::exact(v);
    }
    let mut b = units(ulps, exp_of(r) - 64);
    if let Some(e) = extra {
        b = b.add(&e, P2, RoundingMode::ToEven);
    }
    let lo = r.sub(&b, P2 + 64, RoundingMode::ToEven);
    let hi = r.add(&b, P2 + 64, RoundingMode::ToEven);
    // The `_FLOAT` values inside the interval, and the correctly rounded one.
    Range {
        lo: f80_up(&lo),
        hi: f80_down(&hi),
        mid: v,
    }
}

/// A range of `double` values.
#[derive(Clone, Copy, Debug)]
struct D {
    lo: f64,
    hi: f64,
    /// The value with correctly rounded library calls.
    mid: f64,
}

impl D {
    fn exact(v: f64) -> D {
        D { lo: v, hi: v, mid: v }
    }

    fn corners(self, other: D, op: impl Fn(f64, f64) -> f64) -> D {
        let v = [
            op(self.lo, other.lo),
            op(self.lo, other.hi),
            op(self.hi, other.lo),
            op(self.hi, other.hi),
        ];
        D {
            lo: v.iter().copied().fold(f64::INFINITY, f64::min),
            hi: v.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            mid: op(self.mid, other.mid),
        }
    }

    /// A divisor of libqb's helpers, checked as they check it (`== 0` is error 5): zero error, a range that holds
    /// zero and other values the correctly rounded value (D-026).
    fn divisor(self, name: &str, x: F80) -> R<D> {
        if self.lo == 0.0 && self.hi == 0.0 {
            return Err(inside(name, x));
        }
        if self.lo <= 0.0 && self.hi >= 0.0 {
            fell_back();
            if self.mid == 0.0 {
                return Err(inside(name, x));
            }
            return Ok(D::exact(self.mid));
        }
        Ok(self)
    }

    fn into_range(self) -> Range {
        Range {
            lo: F80::from_f64(self.lo),
            hi: F80::from_f64(self.hi),
            mid: F80::from_f64(self.mid),
        }
    }
}

/// A UCRT `double` call at an exact argument.
/// A UCRT `double` call at an exact argument, within `ulps` of the true value.
fn ucrt_within(x: f64, ulps: f64, f: impl Fn(&BigFloat, usize, &mut Consts) -> BigFloat) -> R<D> {
    let xb = F80::from_f64(x).to_big();
    let r = reference(|p, cc| f(&xb, p, cc))?;
    if r.is_zero() {
        return Ok(D::exact(0.0));
    }
    // A true value a `double` holds is returned as it is (`exp(0)`, `cos(0)`).
    let d = F80::from_big(&r).to_f64();
    if F80::from_f64(d).to_big().partial_cmp(&r) == Some(Ordering::Equal) {
        return Ok(D::exact(d));
    }
    let b = units(ulps, exp_of(&r) - 53);
    let lo = r.sub(&b, P2 + 64, RoundingMode::ToEven);
    let hi = r.add(&b, P2 + 64, RoundingMode::ToEven);
    // The `double` values inside the interval, and the nearest to the reference.
    Ok(D {
        lo: f64_up(&lo),
        hi: f64_down(&hi),
        mid: f64_nearest(&r),
    })
}

/// The `double` nearest to `r` (half to even). Beyond `f64::MAX` an infinity stands for 2^1024, as IEEE rounding
/// takes it, so a value from `MAX` plus half its last place up is the infinity.
fn f64_nearest(r: &BigFloat) -> f64 {
    let (below, above) = (f64_down(r), f64_up(r));
    let dist = |d: f64| {
        let b = if d.is_infinite() {
            let half = BigFloat::from_f64(2f64.powi(512), 64);
            let p = half.mul(&half, 64, RoundingMode::ToEven);
            if d < 0.0 { p.neg() } else { p }
        } else {
            F80::from_f64(d).to_big()
        };
        b.sub(r, P2 + 64, RoundingMode::ToEven).abs()
    };
    match dist(below).partial_cmp(&dist(above)) {
        Some(Ordering::Less) => below,
        Some(Ordering::Greater) => above,
        _ => {
            if below.to_bits() & 1 == 0 {
                below
            } else {
                above
            }
        }
    }
}

/// The error the old compiler stops with when its own run time raises one inside `qb64pe.exe` ("UNEXPECTED
/// INTERNAL COMPILER ERROR!", nothing built; measured, `verification\v45_x04` …).
pub fn inside(name: &str, x: F80) -> Problem {
    Problem::Error(format!(
        "`{name}({})` is an error of the old compiler's own run time, which stops it (\"UNEXPECTED INTERNAL \
         COMPILER ERROR\")",
        x.to_g19()
    ))
}

fn one(r: Range) -> R<F80> {
    Ok(r.settle())
}

fn is_whole(x: F80) -> bool {
    x.round_integral(Integral::Trunc) == x
}

/// The x87 trigonometric call `f` at `x`: 1 ulp and the reduction's error (design D4).
fn x87_trig(name: &str, x: F80, f: impl Fn(&BigFloat, usize, &mut Consts) -> BigFloat) -> R<Range> {
    if !x.is_finite() {
        return unsupported(&format!("`{name}` of an infinite value in a `CONST`"));
    }
    let xb = x.to_big();
    let r = reference(|p, cc| f(&xb, p, cc))?;
    if x.to_f64().abs() >= X87_TRIG_MAX {
        // Beyond the instructions' range mingw reduces with `fprem1` and a 64-bit pi, which no bound here covers
        // (measured, `verification\v45_x26`, `x27`): any value, the correctly rounded one taken (D-026).
        let inf = F80::Inf { neg: false };
        return Ok(Range {
            lo: -inf,
            hi: inf,
            mid: F80::from_big(&r),
        });
    }
    let extra = if x.is_zero() {
        None
    } else {
        let e = exp_of(&xb) + X87_REDUCTION;
        Some(units(1.0, e))
    };
    Ok(fuzzy80(&r, X87_ULPS, extra))
}

/// `atan2(y, x)` as the x87's `fpatan` gives it (the quadrant by the signs).
fn atan2_ref(y: &BigFloat, x: &BigFloat, p: usize, cc: &mut Consts) -> BigFloat {
    let rm = RoundingMode::ToEven;
    if x.is_zero() {
        let half_pi = cc.pi(p, rm).div(&BigFloat::from_u64(2, 64), p, rm);
        return if y.is_negative() { half_pi.neg() } else { half_pi };
    }
    let a = y.div(x, p + 64, rm).atan(p, rm, cc);
    if x.is_negative() {
        let pi = cc.pi(p, rm);
        if y.is_negative() {
            a.sub(&pi, p, rm)
        } else {
            a.add(&pi, p, rm)
        }
    } else {
        a
    }
}

/// A function of the evaluator whose library is not correctly rounded, at an exact argument (design D4): the
/// values the old compiler might hold, or its error.
pub fn function(name: &str, x: F80) -> R<Range> {
    let rm = RoundingMode::ToEven;
    match name {
        "SIN" => x87_trig(name, x, |a, p, cc| a.sin(p, rm, cc)),
        "COS" => x87_trig(name, x, |a, p, cc| a.cos(p, rm, cc)),
        "TAN" => x87_trig(name, x, |a, p, cc| a.tan(p, rm, cc)),
        "ATN" => {
            let xb = x.to_big();
            let r = reference(|p, cc| xb.atan(p, rm, cc))?;
            Ok(fuzzy80(&r, X87_ULPS, None))
        }
        // `acosl` and `asinl`: 1 - x * x, its root, and `fpatan` (mingwex).
        "_ACOS" | "_ASIN" => {
            let one = F80::from_i64(1);
            let s = (one - x * x).sqrt();
            if s == F80::Nan {
                // NaN: the evaluator reads "NAN" and stops (measured, `v45_x07`).
                return error("the value `NAN` cannot be used in a `CONST`");
            }
            let (xb, sb) = (x.to_big(), s.to_big());
            let r = if name == "_ACOS" {
                reference(|p, cc| atan2_ref(&sb, &xb, p, cc))?
            } else {
                reference(|p, cc| atan2_ref(&xb, &sb, p, cc))?
            };
            Ok(fuzzy80(&r, X87_ULPS, None))
        }
        "EXP" => {
            // `func_exp_float`: above 709.782712893 error 6.
            if x.partial_cmp(crate::consteval::decimal("709.782712893")) == Some(Ordering::Greater) {
                return Err(inside(name, x));
            }
            let xb = x.to_big();
            let r = reference(|p, cc| xb.exp(p, rm, cc))?;
            Ok(fuzzy80(&r, EXPL_ULPS, None))
        }
        "LOG" => {
            // `func_log(double)`: at most 0 error 5.
            let d = x.to_f64();
            if d <= 0.0 {
                return Err(inside(name, x));
            }
            Ok(ucrt_within(d, UCRT_LOG_ULPS, |a, p, cc| a.ln(p, rm, cc))?.into_range())
        }
        "_ARCSEC" | "_ARCCSC" => {
            let d = x.to_f64();
            if d.abs() < 1.0 {
                return Err(inside(name, x));
            }
            let inv = 1.0 / d;
            let r = if name == "_ARCSEC" {
                ucrt_within(inv, UCRT_ACOS_ULPS, |a, p, cc| a.acos(p, rm, cc))?
            } else {
                ucrt_within(inv, UCRT_ASIN_ULPS, |a, p, cc| a.asin(p, rm, cc))?
            };
            Ok(r.into_range())
        }
        // `2 * std::atan(1) - std::atan(num)`.
        "_ARCCOT" => {
            let quarter = ucrt_within(1.0, UCRT_ATAN_ULPS, |a, p, cc| a.atan(p, rm, cc))?;
            let a = ucrt_within(x.to_f64(), UCRT_ATAN_ULPS, |a, p, cc| a.atan(p, rm, cc))?;
            let two = D::exact(2.0);
            Ok(two.corners(quarter, |p, q| p * q).corners(a, |p, q| p - q).into_range())
        }
        "_SECH" | "_CSCH" => {
            let d = x.to_f64();
            if d > 88.02969 {
                return Err(inside(name, x));
            }
            let e1 = ucrt_within(d, UCRT_EXP_ULPS, |a, p, cc| a.exp(p, rm, cc))?;
            let e2 = ucrt_within(-d, UCRT_EXP_ULPS, |a, p, cc| a.exp(p, rm, cc))?;
            let den = if name == "_SECH" {
                e1.corners(e2, |p, q| p + q)
            } else {
                e1.corners(e2, |p, q| p - q)
            };
            let den = den.divisor(name, x)?;
            Ok(D::exact(2.0).corners(den, |p, q| p / q).into_range())
        }
        // The hyperbolic cotangent of the `double` argument, as the `double` libqb's `func_coth` returns, correctly
        // rounded (`DIVERGENCES.md` D-031: the old compiler's is `2 * std::exp(num) - 1`, measured, `v45_x32`). At 0 an
        // error, as libqb's error 5.
        "_COTH" => {
            let d = x.to_f64();
            if d == 0.0 {
                return error("`_COTH(0)` in a `CONST` is a division by zero");
            }
            let xb = F80::from_f64(d).to_big();
            let one = BigFloat::from_u64(1, 64);
            let r = reference(|p, cc| one.div(&xb.tanh(p + 64, rm, cc), p, rm))?;
            Ok(Range::exact(F80::from_f64(f64_nearest(&r))))
        }
        "_SEC" | "_CSC" | "_COT" => {
            let d = x.to_f64();
            let v = match name {
                "_SEC" => ucrt_within(d, UCRT_COS_ULPS, |a, p, cc| a.cos(p, rm, cc))?,
                "_CSC" => ucrt_within(d, UCRT_SIN_ULPS, |a, p, cc| a.sin(p, rm, cc))?,
                _ => ucrt_within(d, UCRT_TAN_ULPS, |a, p, cc| a.tan(p, rm, cc))?,
            };
            let v = v.divisor(name, x)?;
            Ok(D::exact(1.0).corners(v, |p, q| p / q).into_range())
        }
        _ => unreachable!("{name} is not a function certified here"),
    }
}

/// libqb's `pow2(x, y)` (`powl`): a negative `x` to a fractional `y` is error 5 inside `qb64pe.exe`; a whole `y`
/// whose exact power `_FLOAT` holds gives that power (mingw's `powl` multiplies for a whole exponent); otherwise
/// the reference within the bound of `powl`.
pub fn pow2(x: Range, y: Range) -> R<Range> {
    let (x, y) = (one(x)?, one(y)?);
    if let F80::Inf { neg } = y
        && x.is_finite()
    {
        // C's `pow` (and `powl`): |x| > 1 grows to infinity with a positive exponent, |x| < 1 to 0, 1 stays.
        let unit = F80::from_i64(1);
        return Ok(Range::exact(match x.abs().partial_cmp(unit) {
            Some(Ordering::Equal) => unit,
            Some(Ordering::Greater) if !neg => F80::Inf { neg: false },
            Some(Ordering::Less) if neg => F80::Inf { neg: false },
            _ => F80::ZERO,
        }));
    }
    if !x.is_finite() || !y.is_finite() {
        return unsupported("a power of an infinite value in a `CONST`");
    }
    if x.is_neg() && !x.is_zero() && !is_whole(y) {
        return Err(inside("^", x));
    }
    if y.is_zero() {
        return Ok(Range::exact(F80::from_i64(1)));
    }
    if x.is_zero() {
        return Ok(Range::exact(if y.is_neg() {
            F80::Inf { neg: false }
        } else {
            F80::ZERO
        }));
    }
    if is_whole(y)
        && let Some(e) = y.to_i128()
        && e.unsigned_abs() <= 16384
        && let Some(v) = exact_power(x, e)
    {
        return Ok(Range::exact(v));
    }
    let (xb, yb) = (x.abs().to_big(), y.to_big());
    let rm = RoundingMode::ToEven;
    let r = reference(|p, cc| xb.pow(&yb, p, rm, cc))?;
    let r = if x.is_neg() && y.to_i128().is_some_and(|e| e % 2 != 0) {
        r.neg()
    } else {
        r
    };
    if r.is_inf() {
        return Ok(Range::exact(F80::Inf { neg: r.is_inf_neg() }));
    }
    Ok(fuzzy80(&r, POWL_ULPS, None))
}

/// `x^e` for a whole `e`, when `_FLOAT` holds it exactly.
fn exact_power(x: F80, e: i128) -> Option<F80> {
    let F80::Finite { mant, .. } = x else {
        return None;
    };
    // The exact power has at most |e| × 64 significant bits; beyond 4096 it cannot be a 64-bit mantissa unless x is
    // a power of two.
    let bits = usize::try_from(e.unsigned_abs()).ok()? * usize::try_from(64 - mant.trailing_zeros()).ok()?;
    if bits > 64 * 64 && mant.count_ones() != 1 {
        return None;
    }
    let p = (bits + 128).div_ceil(64) * 64;
    let xb = x.to_big();
    let pos = xb.powi(usize::try_from(e.unsigned_abs()).ok()?, p, RoundingMode::None);
    let v = if e < 0 {
        let one = BigFloat::from_u64(1, 64);
        let q = one.div(&pos, p + 128, RoundingMode::ToEven);
        if q.inexact() {
            return None;
        }
        q
    } else {
        pos
    };
    if v.inexact() {
        return None;
    }
    let r = F80::from_big(&v);
    (r.to_big().partial_cmp(&v) == Some(Ordering::Equal)).then_some(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consteval::decimal;

    fn g19(r: Range) -> (String, String) {
        (r.lo.to_g19(), r.hi.to_g19())
    }

    /// The recorded values of `verification\v45_b_funcs` lie in the ranges.
    #[test]
    fn recorded_values_lie_in_their_ranges() {
        // Some value of the range prints as the recorded text.
        let within = |r: Range, text: &str| {
            let mut v = r.lo;
            for _ in 0..5000 {
                if v.to_g19() == text {
                    return true;
                }
                if v == r.hi {
                    return false;
                }
                v = next_up(v);
            }
            false
        };
        let cases = [
            ("SIN", ".2", "0.1986693307950612155"),
            ("COS", "1", "0.5403023058681397174"),
            ("ATN", "1", "0.7853981633974483096"),
            ("EXP", "1", "2.718281828459045235"),
            ("LOG", "10", "2.302585092994045901"),
            ("_ACOS", ".2", "1.369438406004565828"),
            ("_ASIN", ".2", "0.2013579207903307915"),
            ("_ARCSEC", "1.2", "0.5856855434571508479"),
            ("_SECH", ".2", "0.9803279976447253352"),
            ("_SEC", "2", "-2.402997961722380893"),
            ("TAN", "1.5", "14.10141994717171939"),
        ];
        for (name, arg, want) in cases {
            let r = function(name, decimal(arg)).unwrap();
            assert!(within(r, want), "{name}({arg}): {:?} does not hold {want}", g19(r));
        }
    }

    /// `_COTH` is the hyperbolic cotangent as a correctly rounded `double` (D-031; the values are `mpmath`'s at 300
    /// bits, rounded to `double`), with no upper limit, and an error at 0.
    #[test]
    fn coth_is_the_hyperbolic_cotangent() {
        for (arg, want) in [
            (".2", 5.066_489_563_439_473),
            ("1", 1.313_035_285_499_331_2),
            ("-.5", -2.163_953_413_738_653),
            ("45", 1.0),
        ] {
            let x = if let Some(a) = arg.strip_prefix('-') {
                -decimal(a)
            } else {
                decimal(arg)
            };
            assert_eq!(
                function("_COTH", x).unwrap().one(),
                Some(F80::from_f64(want)),
                "_COTH({arg})"
            );
        }
        assert_eq!(
            function("_COTH", decimal(".2")).unwrap().one().unwrap().to_g19(),
            "5.066489563439472654"
        );
        assert!(matches!(function("_COTH", F80::ZERO), Err(Problem::Error(_))));
        // Beyond `double` range the result rounds to the infinity, as `1 / std::tanh` gives it (found in review).
        for (x, want) in [(1e-310, f64::INFINITY), (-1e-310, f64::NEG_INFINITY), (1e-308, 1e308)] {
            let got = function("_COTH", F80::from_f64(x)).unwrap().one().unwrap().to_f64();
            assert!(got == want || (got / want - 1.0).abs() < 1e-15, "_COTH({x}): {got}");
        }
    }

    #[test]
    fn exact_powers() {
        let p = |x: &str, y: &str| pow2(Range::exact(decimal(x)), Range::exact(decimal(y))).unwrap();
        assert_eq!(p("1.5", "2").one(), Some(decimal("2.25")));
        assert_eq!(p("2.5", "3").one(), Some(decimal("15.625")));
        assert_eq!(p("2", "-1").one(), Some(decimal("0.5")));
        assert_eq!(p("2", "70").one(), Some(decimal("1180591620717411303424")));
        assert!(p("3", "41").one().is_none());
        assert!(matches!(
            pow2(Range::exact(decimal("-8")), Range::exact(decimal("0.5"))),
            Err(Problem::Error(_))
        ));
    }

    /// The functions of `verification\v45_sample.bas`, in its order.
    const SAMPLE: [&str; 19] = [
        "SIN", "COS", "TAN", "ATN", "_ACOS", "_ASIN", "EXP", "LOG", "^", "_ARCSEC", "_ARCCSC", "_ARCCOT", "_SECH",
        "_CSCH", "_COTH", "_SEC", "_CSC", "_COT", "SQR",
    ];

    /// A `_FLOAT` from its 10 bytes (low byte first) as `_MK$` gives them.
    fn from_bytes(b: &[u8]) -> F80 {
        let mant = u64::from_le_bytes(b[..8].try_into().unwrap());
        let se = u16::from_le_bytes([b[8], b[9]]);
        let neg = se >> 15 == 1;
        let e = i32::from(se & 0x7fff);
        if mant == 0 {
            return F80::Zero { neg };
        }
        F80::Finite {
            neg,
            mant,
            exp: e.max(1) - 16383 - 63,
        }
    }

    /// The argument (and the exponent of `^`) of sample `i` of function `f`, as the sample program computes them.
    fn sample_args(f: usize, i: i64) -> (F80, F80) {
        let h = (i * 2_654_435_761) & 0xFFFFF;
        let h2 = (i * 40503 + 12345) & 0xFFFF;
        let u = F80::from_i64(h) / F80::from_i64(1 << 20);
        let v = F80::from_i64(h2) / F80::from_i64(1 << 16);
        let n = |k: i64| F80::from_i64(k);
        let tiny = F80::from_f64(0.0009765625);
        let x = match f {
            1 | 2 | 13 => u * n(16) - n(8),
            3 | 16 => u * n(3) - F80::from_f64(1.5),
            4 | 7 | 12 => u * n(32) - n(16),
            5 | 6 => u * n(2) - n(1),
            8 | 9 | 14 | 15 | 19 => u * n(16) + tiny,
            10 | 11 => u * n(16) + n(1),
            17 | 18 => u * n(3) + tiny,
            _ => unreachable!(),
        };
        (x, v * n(8) - n(4))
    }

    /// Every recorded result of the sample, with its function's range.
    #[expect(
        clippy::disallowed_methods,
        reason = "a recorded output of hex digits, not BASIC source"
    )]
    fn sample() -> Vec<(&'static str, F80, F80, F80, R<Range>)> {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../verification/v45_sample.out.txt");
        let text = std::fs::read_to_string(path).unwrap();
        let mut out = Vec::new();
        for line in text.lines().filter(|l| l.contains(':')) {
            let (key, hex) = line.split_once(' ').unwrap();
            let (f, i) = key.split_once(':').unwrap();
            let (f, i): (usize, i64) = (f.parse().unwrap(), i.parse().unwrap());
            let bytes: Vec<u8> = (0..10)
                .map(|k| u8::from_str_radix(&hex[2 * k..2 * k + 2], 16).unwrap())
                .collect();
            let got = from_bytes(&bytes);
            let (x, y) = sample_args(f, i);
            let name = SAMPLE[f - 1];
            let range = match name {
                "^" => pow2(Range::exact(x), Range::exact(y)),
                "SQR" => Ok(Range::exact(F80::from_f64(x.to_f64().sqrt()))),
                _ => function(name, x),
            };
            out.push((name, x, y, got, range));
        }
        out
    }

    /// `verification\v45_sample` (task 1.3): every result the old compiler's libraries gave lies in its range.
    #[test]
    fn the_sample_lies_in_its_ranges() {
        let mut bad = Vec::new();
        // `_COTH` is the hyperbolic cotangent here, not the old compiler's (D-031).
        for (name, x, _, got, range) in sample().into_iter().filter(|s| s.0 != "_COTH") {
            let ok = match range {
                Ok(r) => {
                    r.lo.partial_cmp(got) != Some(Ordering::Greater) && r.hi.partial_cmp(got) != Some(Ordering::Less)
                }
                Err(_) => false,
            };
            if !ok {
                bad.push(format!("{name}({}) = {}", x.to_g19(), got.to_g19()));
            }
        }
        let mut per: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
        for b in &bad {
            *per.entry(b.split('(').next().unwrap()).or_default() += 1;
        }
        assert!(
            bad.is_empty(),
            "{} outside their range ({per:?}), first: {:?}",
            bad.len(),
            &bad[..bad.len().min(5)]
        );
    }

    /// How far the sample's results lie from the reference, in units in the last place, per function: the
    /// calibration of the bounds (`cargo test -p qb64rust-sema calibrate -- --ignored --nocapture`).
    #[test]
    #[ignore = "prints the calibration of the bounds"]
    fn calibrate() {
        let mut worst: std::collections::BTreeMap<&str, f64> = std::collections::BTreeMap::new();
        let rm = RoundingMode::ToEven;
        for (name, x, y, got, _) in sample() {
            let xb = x.to_big();
            let (r, bits) = match name {
                "SIN" => (reference(|p, cc| xb.sin(p, rm, cc)), 64),
                "COS" => (reference(|p, cc| xb.cos(p, rm, cc)), 64),
                "TAN" => (reference(|p, cc| xb.tan(p, rm, cc)), 64),
                "ATN" => (reference(|p, cc| xb.atan(p, rm, cc)), 64),
                "EXP" => (reference(|p, cc| xb.exp(p, rm, cc)), 64),
                "LOG" => {
                    let d = F80::from_f64(x.to_f64()).to_big();
                    (reference(|p, cc| d.ln(p, rm, cc)), 53)
                }
                // Measured below, through the helper's outer step.
                "_SEC" | "_CSC" | "_COT" | "_COTH" | "_ARCCOT" => (Ok(BigFloat::from_u64(1, 64)), 53),
                "_ARCSEC" | "_ARCCSC" => {
                    let inv = F80::from_f64(1.0 / x.to_f64()).to_big();
                    if name == "_ARCSEC" {
                        (reference(|p, cc| inv.acos(p, rm, cc)), 53)
                    } else {
                        (reference(|p, cc| inv.asin(p, rm, cc)), 53)
                    }
                }
                "^" => {
                    let yb = y.to_big();
                    (reference(|p, cc| xb.pow(&yb, p, rm, cc)), 64)
                }
                _ => continue,
            };
            let Ok(r) = r else { continue };
            if r.is_zero() {
                continue;
            }
            // A UCRT call inside a helper: the `double`s `c` the exact outer step turns into the recorded result,
            // and the smallest error among them.
            if let Some((inner, outer)) = match name {
                "_SEC" | "_CSC" | "_COT" => Some((name, (|c: f64| 1.0 / c) as fn(f64) -> f64)),
                "_COTH" => Some(("exp via _COTH", (|e: f64| 2.0 * e - 1.0) as fn(f64) -> f64)),
                "_ARCCOT" => Some((
                    "atan via _ARCCOT",
                    (|a: f64| 2.0 * (std::f64::consts::FRAC_PI_4) - a) as fn(f64) -> f64,
                )),
                _ => None,
            } {
                let d = x.to_f64();
                let truth = match name {
                    "_SEC" => reference(|p, cc| F80::from_f64(d).to_big().cos(p, rm, cc)),
                    "_CSC" => reference(|p, cc| F80::from_f64(d).to_big().sin(p, rm, cc)),
                    "_COT" => reference(|p, cc| F80::from_f64(d).to_big().tan(p, rm, cc)),
                    "_COTH" => reference(|p, cc| F80::from_f64(d).to_big().exp(p, rm, cc)),
                    _ => reference(|p, cc| F80::from_f64(d).to_big().atan(p, rm, cc)),
                };
                let Ok(truth) = truth else { continue };
                let rec = got.to_f64();
                let guess = F80::from_big(&truth).to_f64();
                let mut c = guess;
                for _ in 0..8 {
                    c = c.next_down();
                }
                let mut best = f64::INFINITY;
                for _ in 0..17 {
                    if outer(c) == rec {
                        let diff = F80::from_f64(c).to_big().sub(&truth, P2, rm).abs();
                        let q = diff.div(&units(1.0, exp_of(&truth) - 53), 64, rm);
                        best = best.min(F80::from_big(&q).to_f64());
                    }
                    c = c.next_up();
                }
                let w = worst.entry(inner).or_insert(0.0);
                if best.is_finite() && best > *w {
                    *w = best;
                }
                continue;
            }
            let diff = got.to_big().sub(&r, P2, rm).abs();
            let ulp = units(1.0, exp_of(&r) - bits);
            let q = diff.div(&ulp, 64, rm);
            let q = F80::from_big(&q).to_f64();
            // For the trigonometric functions: in ulps where no reduction happens (|x| < pi/4), and elsewhere the
            // error beyond 1 ulp as a multiple of |x| × 2^-64.
            let small = x.to_f64().abs() < std::f64::consts::FRAC_PI_4;
            let key: &str = match (name, small) {
                ("SIN" | "COS" | "TAN", true) => &format!("{name} |x| < pi/4"),
                _ => name,
            };
            let key: &'static str = Box::leak(key.to_string().into_boxed_str());
            let w = worst.entry(key).or_insert(0.0);
            if q > *w {
                *w = q;
            }
            if matches!(name, "SIN" | "COS" | "TAN") && !small {
                let beyond = (F80::from_big(&diff).to_f64() - F80::from_big(&ulp).to_f64()).max(0.0);
                let c = beyond / (x.to_f64().abs() * 2f64.powi(-64));
                let w = worst
                    .entry(Box::leak(format!("{name} reduction per |x| 2^-64").into_boxed_str()))
                    .or_insert(0.0);
                if c > *w {
                    *w = c;
                }
            }
        }
        for (name, w) in worst {
            println!("{name}: {w:.4} ulp");
        }
    }

    /// How often a function's result certifies over the sample's arguments: its range prints as one 19-digit text
    /// (`cargo test -p qb64rust-sema certify_rate -- --ignored --nocapture`).
    #[test]
    #[ignore = "prints the share of the sample whose result certifies"]
    fn certify_rate() {
        let mut rate: std::collections::BTreeMap<&str, (usize, usize)> = std::collections::BTreeMap::new();
        for (name, _, _, _, range) in sample() {
            let e = rate.entry(name).or_default();
            e.1 += 1;
            if range.is_ok_and(|r| r.lo.to_g19() == r.hi.to_g19()) {
                e.0 += 1;
            }
        }
        for (name, (ok, all)) in rate {
            println!("{name}: {ok} of {all}");
        }
        // The constants of `const\math_funcs` and `_E`.
        for (name, arg) in [
            ("EXP", "1"),
            ("_ACOS", ".2"),
            ("_ASIN", ".2"),
            ("_ARCSEC", "1.2"),
            ("_ARCCOT", ".2"),
            ("_ARCCSC", "1.2"),
            ("_SECH", ".2"),
            ("_CSCH", ".2"),
            ("_COTH", ".2"),
            ("COS", ".2"),
            ("SIN", ".2"),
            ("TAN", ".2"),
            ("LOG", ".2"),
            ("EXP", ".2"),
            ("ATN", ".2"),
            ("_SEC", "2"),
            ("_CSC", "2"),
            ("_COT", "2"),
        ] {
            let r = function(name, crate::consteval::decimal(arg));
            let ok = r.as_ref().is_ok_and(|r| r.lo.to_g19() == r.hi.to_g19());
            println!("{name}({arg}): {}", if ok { "certified" } else { "NOT" });
        }
        let r = pow2(
            Range::exact(crate::consteval::decimal("20")),
            Range::exact(F80::from_i64(1) / F80::from_i64(3)),
        );
        println!(
            "20 ROOT 3: {}",
            if r.is_ok_and(|r| r.lo.to_g19() == r.hi.to_g19()) {
                "certified"
            } else {
                "NOT"
            }
        );
    }

    /// Guard 2 of design D3: the reference, rounded to `_FLOAT`, is `mpmath`'s value rounded to `_FLOAT`
    /// (`crate::mpmath_values`, written by `tools\builtins\mpmath_values.py`).
    #[test]
    fn the_reference_agrees_with_mpmath() {
        let rm = RoundingMode::ToEven;
        for &(name, a, b, want) in crate::mpmath_values::MPMATH {
            let x = decimal(a).to_big();
            let y = if b.is_empty() { None } else { Some(decimal(b).to_big()) };
            let r = reference(|p, cc| match name {
                "sin" => x.sin(p, rm, cc),
                "cos" => x.cos(p, rm, cc),
                "tan" => x.tan(p, rm, cc),
                "atan" => x.atan(p, rm, cc),
                "exp" => x.exp(p, rm, cc),
                "ln" => x.ln(p, rm, cc),
                "asin" => x.asin(p, rm, cc),
                "acos" => x.acos(p, rm, cc),
                "sqrt" => x.sqrt(p, rm),
                "pow" => x.pow(y.as_ref().unwrap(), p, rm, cc),
                _ => unreachable!("{name}"),
            })
            .unwrap();
            assert_eq!(
                F80::from_big(&r),
                decimal(want),
                "{name}({a}{}{b})",
                if b.is_empty() { "" } else { ", " }
            );
        }
    }

    #[test]
    fn errors_inside_the_old_compiler() {
        for (name, arg) in [
            ("LOG", "0"),
            ("_SECH", "100"),
            ("_COT", "0"),
            ("_CSC", "0"),
            ("_CSCH", "0"),
            ("EXP", "1000"),
        ] {
            assert!(
                matches!(function(name, decimal(arg)), Err(Problem::Error(_))),
                "{name}({arg})"
            );
        }
    }
}
