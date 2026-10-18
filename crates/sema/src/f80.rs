//! The x87 extended format, QB64pe's `_FLOAT`, in software (design D1 of `m3-constant-evaluation`).
//!
//! The old compiler's `CONST` evaluator computes floats in `_FLOAT` on the x87 with its precision control at 64
//! bits (measured, `verification\v45_a_precision`), prints each value with `%.19LG` and reads it back with
//! `%Lf` (mingw's gdtoa, both correctly rounded). Every operation here is rounded to nearest, ties to even, at a
//! 64-bit mantissa with the x87's exponent range, so the arithmetic and the text are the old compiler's to the
//! bit. The exact decimal conversions use `astro_float_num`'s big floats at a precision that makes each step exact
//! (or decide a near tie by an exact comparison), then this module's own rounding.

use astro_float_num::{BigFloat, RoundingMode, Sign, Word};
use std::cmp::Ordering;

/// An x87 extended value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F80 {
    /// `±mant × 2^exp`, `mant` not 0; its top bit is set unless `exp` is [`EXP_MIN`] (a subnormal).
    Finite {
        neg: bool,
        mant: u64,
        exp: i32,
    },
    Zero {
        neg: bool,
    },
    Inf {
        neg: bool,
    },
    Nan,
}

/// The exponent of the mantissa's lowest bit for the smallest normal value (2^-16382) and the subnormals.
pub const EXP_MIN: i32 = -16382 - 63;
/// The exponent of the mantissa's lowest bit for the largest values (below 2^16384).
pub const EXP_MAX: i32 = 16383 - 63;

/// A format to round to: the bits of its mantissa and the range of the exponent of the mantissa's lowest bit.
#[derive(Clone, Copy)]
struct Format {
    bits: u32,
    exp_min: i32,
    exp_max: i32,
}

const X87: Format = Format {
    bits: 64,
    exp_min: EXP_MIN,
    exp_max: EXP_MAX,
};

const DOUBLE: Format = Format {
    bits: 53,
    exp_min: -1074,
    exp_max: 1023 - 52,
};

/// A rounded value of a format: `±mant × 2^exp`, zero or infinite.
enum Rounded {
    Finite { mant: u64, exp: i32 },
    Zero,
    Inf,
}

/// Rounds `m × 2^e`, plus something below the lowest bit of `m` when `sticky`, to `fmt`, to nearest, ties to even.
fn round(m: u128, e: i32, sticky: bool, fmt: Format) -> Rounded {
    if m == 0 {
        // A sticky remainder alone is below every subnormal of these formats' callers' ranges.
        return Rounded::Zero;
    }
    let len = 128 - i32::try_from(m.leading_zeros()).expect("at most 128");
    let bits = i32::try_from(fmt.bits).expect("small");
    // The exponent of the lowest kept bit.
    let mut low = e + len - bits;
    if low < fmt.exp_min {
        low = fmt.exp_min;
    }
    let shift = low - e;
    let (mut mant, round_up) = if shift <= 0 {
        // Exact: shift left.
        let s = u32::try_from(-shift).expect("non-negative");
        let mant = if s >= 128 { 0 } else { m << s };
        (mant, false)
    } else {
        let s = u32::try_from(shift).expect("positive");
        if s > 128 {
            // Everything is below half the lowest bit.
            (0, false)
        } else {
            let kept = if s == 128 { 0 } else { m >> s };
            let half = 1u128 << (s - 1);
            let rest = m & ((half << 1).wrapping_sub(1));
            let above = rest > half || (rest == half && sticky);
            let tie = rest == half && !sticky;
            (kept, above || (tie && kept & 1 == 1))
        }
    };
    if round_up {
        mant += 1;
    }
    let mut exp = low;
    if mant >> fmt.bits != 0 {
        mant >>= 1;
        exp += 1;
    }
    if mant == 0 {
        return Rounded::Zero;
    }
    if exp > fmt.exp_max {
        return Rounded::Inf;
    }
    Rounded::Finite {
        mant: u64::try_from(mant).expect("fits the format"),
        exp,
    }
}

fn f80_of(neg: bool, r: Rounded) -> F80 {
    match r {
        Rounded::Finite { mant, exp } => F80::Finite { neg, mant, exp },
        Rounded::Zero => F80::Zero { neg },
        Rounded::Inf => F80::Inf { neg },
    }
}

/// `m × 2^e` with `m`'s top bit set (a subnormal's mantissa shifted up, its exponent below [`EXP_MIN`]).
fn normalised(mant: u64, exp: i32) -> (u64, i32) {
    let z = mant.leading_zeros();
    (mant << z, exp - i32::try_from(z).expect("small"))
}

impl F80 {
    pub const ZERO: F80 = F80::Zero { neg: false };

    /// Rounds `±m × 2^e` (plus a remainder below its lowest bit when `sticky`).
    pub fn rounded(neg: bool, m: u128, e: i32, sticky: bool) -> F80 {
        f80_of(neg, round(m, e, sticky, X87))
    }

    pub fn from_i64(i: i64) -> F80 {
        F80::rounded(i < 0, u128::from(i.unsigned_abs()), 0, false)
    }

    pub fn from_u64(u: u64) -> F80 {
        F80::rounded(false, u128::from(u), 0, false)
    }

    /// An `f64` exactly.
    pub fn from_f64(d: f64) -> F80 {
        let bits = d.to_bits();
        let neg = bits >> 63 == 1;
        let biased = i32::try_from((bits >> 52) & 0x7ff).expect("11 bits");
        let frac = bits & ((1 << 52) - 1);
        match biased {
            0 if frac == 0 => F80::Zero { neg },
            0 => F80::rounded(neg, u128::from(frac), -1074, false),
            0x7ff if frac == 0 => F80::Inf { neg },
            0x7ff => F80::Nan,
            _ => F80::rounded(neg, u128::from(frac | (1 << 52)), biased - 1075, false),
        }
    }

    /// Rounded to an `f64`, to nearest, ties to even (a C++ `(double)` cast).
    pub fn to_f64(self) -> f64 {
        match self {
            F80::Nan => f64::NAN,
            F80::Inf { neg } => sign(neg, f64::INFINITY),
            F80::Zero { neg } => sign(neg, 0.0),
            F80::Finite { neg, mant, exp } => match round(u128::from(mant), exp, false, DOUBLE) {
                Rounded::Zero => sign(neg, 0.0),
                Rounded::Inf => sign(neg, f64::INFINITY),
                Rounded::Finite { mant, exp } => {
                    // `mant × 2^exp` with at most 53 bits and `exp` in range: exact in `f64`.
                    let (field, frac) = if mant >> 52 == 0 {
                        (0u64, mant)
                    } else {
                        (u64::try_from(exp + 1075).expect("in range"), mant & ((1 << 52) - 1))
                    };
                    let bits = (u64::from(neg) << 63) | (field << 52) | frac;
                    f64::from_bits(bits)
                }
            },
        }
    }

    pub fn is_neg(self) -> bool {
        match self {
            F80::Finite { neg, .. } | F80::Zero { neg } | F80::Inf { neg } => neg,
            F80::Nan => false,
        }
    }

    pub fn is_finite(self) -> bool {
        matches!(self, F80::Finite { .. } | F80::Zero { .. })
    }

    pub fn is_zero(self) -> bool {
        matches!(self, F80::Zero { .. })
    }

    pub fn abs(self) -> F80 {
        if self.is_neg() { -self } else { self }
    }

    pub fn sqrt(self) -> F80 {
        match self {
            F80::Nan => F80::Nan,
            F80::Zero { .. } => self,
            F80::Inf { neg: false } => self,
            F80::Inf { neg: true } | F80::Finite { neg: true, .. } => F80::Nan,
            F80::Finite { mant, exp, .. } => {
                let (m, e) = normalised(mant, exp);
                // A radicand of 127 or 128 bits, so the root has 64; its exponent halves.
                let (r, e) = if e % 2 == 0 {
                    (u128::from(m) << 64, e - 64)
                } else {
                    (u128::from(m) << 63, e - 63)
                };
                let s = r.isqrt();
                // Round to nearest: up when sqrt(r) >= s + 1/2, that is r > s^2 + s (no tie is possible).
                let up = r - s * s > s;
                F80::rounded(false, s + u128::from(up), e / 2, false)
            }
        }
    }

    /// The order of two values; `None` when one is a NaN.
    pub fn partial_cmp(self, other: F80) -> Option<Ordering> {
        if self == F80::Nan || other == F80::Nan {
            return None;
        }
        // A key that orders the magnitudes: zero, finite values by (exponent, mantissa) normalised, infinity.
        let key = |v: F80| -> (u8, i32, u64) {
            match v {
                F80::Zero { .. } => (0, 0, 0),
                F80::Finite { mant, exp, .. } => {
                    let (m, e) = normalised(mant, exp);
                    (1, e, m)
                }
                F80::Inf { .. } | F80::Nan => (2, 0, 0),
            }
        };
        let (ka, kb) = (key(self), key(other));
        let (na, nb) = (self.is_neg() && ka.0 != 0, other.is_neg() && kb.0 != 0);
        Some(match (na, nb) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => ka.cmp(&kb),
            (true, true) => kb.cmp(&ka),
        })
    }

    /// The integer nearest to the value in the given way, as an `F80` (exact).
    pub fn round_integral(self, how: Integral) -> F80 {
        let F80::Finite { neg, mant, exp } = self else {
            return self;
        };
        if exp >= 0 {
            return self;
        }
        let s = u32::try_from(-exp).expect("positive");
        let m = u128::from(mant);
        let (int, frac_nonzero, half, above_half) = if s >= 66 {
            (0, true, false, false)
        } else {
            let int = m >> s;
            let rest = m & ((1u128 << s) - 1);
            let half_bit = 1u128 << (s - 1);
            (int, rest != 0, rest == half_bit, rest > half_bit)
        };
        let up = match how {
            Integral::Trunc => false,
            Integral::Floor => neg && frac_nonzero,
            Integral::Ceil => !neg && frac_nonzero,
            Integral::NearestEven => above_half || (half && int & 1 == 1),
        };
        let int = int + u128::from(up);
        if int == 0 {
            return F80::Zero { neg };
        }
        F80::rounded(neg, int, 0, false)
    }

    /// The value as an integer when it is one within `i128` range.
    pub fn to_i128(self) -> Option<i128> {
        match self {
            F80::Zero { .. } => Some(0),
            F80::Finite { neg, mant, exp } => {
                if exp < 0 {
                    let s = u32::try_from(-exp).expect("positive");
                    if s >= 64 || mant & ((1u64 << s) - 1) != 0 {
                        return None;
                    }
                    let v = i128::from(mant >> s);
                    return Some(if neg { -v } else { v });
                }
                if exp > 62 {
                    return None;
                }
                let v = i128::from(mant) << exp;
                Some(if neg { -v } else { v })
            }
            F80::Inf { .. } | F80::Nan => None,
        }
    }

    /// The value as a big float, exactly.
    pub fn to_big(self) -> BigFloat {
        match self {
            F80::Finite { neg, mant, exp } => {
                let mut b = BigFloat::from_u64(mant, 64);
                let bits = 64 - i32::try_from(mant.leading_zeros()).expect("small");
                b.set_exponent(exp + bits);
                if neg { b.neg() } else { b }
            }
            F80::Zero { neg } => {
                let b = BigFloat::from_u64(0, 64);
                if neg { b.neg() } else { b }
            }
            F80::Inf { neg: false } => astro_float_num::INF_POS,
            F80::Inf { neg: true } => astro_float_num::INF_NEG,
            F80::Nan => astro_float_num::NAN,
        }
    }

    /// A big float rounded to `_FLOAT`. Exact when `b` is exact (its inexact flag clear); otherwise `b` must have
    /// been rounded toward zero at a precision far above 64 bits and not lie next to a rounding boundary (callers
    /// check with [`near_tie`]).
    pub fn from_big(b: &BigFloat) -> F80 {
        if b.is_nan() {
            return F80::Nan;
        }
        if b.is_inf() {
            return F80::Inf { neg: b.is_inf_neg() };
        }
        let Some((words, bits, sign, e, inexact)) = b.as_raw_parts() else {
            return F80::Nan;
        };
        let neg = sign == Sign::Neg;
        if bits == 0 {
            return F80::Zero { neg };
        }
        let (top, rest_nonzero) = top_bits(words);
        // `top` holds the 128 top bits of the mantissa: value = 0.top... × 2^e.
        F80::rounded(neg, top, e - 128, rest_nonzero || inexact)
    }
}

impl std::ops::Neg for F80 {
    type Output = F80;

    fn neg(self) -> F80 {
        match self {
            F80::Finite { neg, mant, exp } => F80::Finite { neg: !neg, mant, exp },
            F80::Zero { neg } => F80::Zero { neg: !neg },
            F80::Inf { neg } => F80::Inf { neg: !neg },
            F80::Nan => F80::Nan,
        }
    }
}

impl std::ops::Add for F80 {
    type Output = F80;

    fn add(self, other: F80) -> F80 {
        match (self, other) {
            (F80::Nan, _) | (_, F80::Nan) => F80::Nan,
            (F80::Inf { neg: a }, F80::Inf { neg: b }) => {
                if a == b {
                    self
                } else {
                    F80::Nan
                }
            }
            (F80::Inf { .. }, _) => self,
            (_, F80::Inf { .. }) => other,
            (F80::Zero { neg: a }, F80::Zero { neg: b }) => F80::Zero { neg: a && b },
            (F80::Zero { .. }, _) => other,
            (_, F80::Zero { .. }) => self,
            (
                F80::Finite {
                    neg: na,
                    mant: ma,
                    exp: ea,
                },
                F80::Finite {
                    neg: nb,
                    mant: mb,
                    exp: eb,
                },
            ) => {
                let (ma, ea) = normalised(ma, ea);
                let (mb, eb) = normalised(mb, eb);
                // The larger magnitude first.
                let ((na, ma, ea), (nb, mb, eb)) = if (ea, ma) >= (eb, mb) {
                    ((na, ma, ea), (nb, mb, eb))
                } else {
                    ((nb, mb, eb), (na, ma, ea))
                };
                let a = u128::from(ma) << 62;
                let b_full = u128::from(mb) << 62;
                let d = u32::try_from(ea - eb).expect("ordered");
                let (b, sticky) = if d >= 128 {
                    (0, b_full != 0)
                } else {
                    (b_full >> d, b_full & ((1u128 << d) - 1) != 0)
                };
                let e = ea - 62;
                if na == nb {
                    F80::rounded(na, a + b, e, sticky)
                } else {
                    let diff = if sticky { a - b - 1 } else { a - b };
                    if diff == 0 && !sticky {
                        return F80::ZERO;
                    }
                    F80::rounded(na, diff, e, sticky)
                }
            }
        }
    }
}

impl std::ops::Sub for F80 {
    type Output = F80;

    fn sub(self, other: F80) -> F80 {
        self + (-other)
    }
}

impl std::ops::Mul for F80 {
    type Output = F80;

    fn mul(self, other: F80) -> F80 {
        let neg = self.is_neg() != other.is_neg();
        match (self, other) {
            (F80::Nan, _) | (_, F80::Nan) => F80::Nan,
            (F80::Inf { .. }, F80::Zero { .. }) | (F80::Zero { .. }, F80::Inf { .. }) => F80::Nan,
            (F80::Inf { .. }, _) | (_, F80::Inf { .. }) => F80::Inf { neg },
            (F80::Zero { .. }, _) | (_, F80::Zero { .. }) => F80::Zero { neg },
            (F80::Finite { mant: ma, exp: ea, .. }, F80::Finite { mant: mb, exp: eb, .. }) => {
                F80::rounded(neg, u128::from(ma) * u128::from(mb), ea + eb, false)
            }
        }
    }
}

impl std::ops::Div for F80 {
    type Output = F80;

    fn div(self, other: F80) -> F80 {
        let neg = self.is_neg() != other.is_neg();
        match (self, other) {
            (F80::Nan, _) | (_, F80::Nan) => F80::Nan,
            (F80::Inf { .. }, F80::Inf { .. }) | (F80::Zero { .. }, F80::Zero { .. }) => F80::Nan,
            (F80::Inf { .. }, _) | (_, F80::Zero { .. }) => F80::Inf { neg },
            (F80::Zero { .. }, _) | (_, F80::Inf { .. }) => F80::Zero { neg },
            (F80::Finite { mant: ma, exp: ea, .. }, F80::Finite { mant: mb, exp: eb, .. }) => {
                let (ma, ea) = normalised(ma, ea);
                let (mb, eb) = normalised(mb, eb);
                let mb = u128::from(mb);
                let n1 = u128::from(ma) << 64;
                let (q1, r1) = (n1 / mb, n1 % mb);
                let n2 = r1 << 64;
                let (q2, r2) = (n2 / mb, n2 % mb);
                let q = (q1 << 62) | (q2 >> 2);
                let sticky = q2 & 3 != 0 || r2 != 0;
                F80::rounded(neg, q, ea - eb - 64 - 62, sticky)
            }
        }
    }
}

/// The 128 top bits of a mantissa (the top bit of the last word first) and whether any lower bit is set.
fn top_bits(words: &[Word]) -> (u128, bool) {
    let n = words.len();
    let w = |i: usize| -> u128 { if i < n { u128::from(words[i]) } else { 0 } };
    let (hi, lo) = if n >= 2 { (w(n - 1), w(n - 2)) } else { (w(n - 1), 0) };
    let rest = n > 2 && words[..n - 2].iter().any(|&x| x != 0);
    ((hi << 64) | lo, rest)
}

fn sign(neg: bool, v: f64) -> f64 {
    if neg { -v } else { v }
}

/// How [`F80::round_integral`] rounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Integral {
    Trunc,
    Floor,
    Ceil,
    NearestEven,
}

// ---- Decimal text ----

/// Precision used where a division cannot be exact: far above the 64 bits rounded to.
const WIDE: usize = 320;

/// `10^n` exactly.
fn pow10(n: u32) -> BigFloat {
    // 10^n = 5^n × 2^n: about 2.33 n significant bits.
    let p = (usize::try_from(n).expect("small") * 7 / 3 + 128).max(128);
    let p = p.div_ceil(64) * 64;
    let ten = BigFloat::from_u64(10, p);
    let r = ten.powi(usize::try_from(n).expect("small"), p, RoundingMode::None);
    debug_assert!(!r.inexact(), "10^{n} exact");
    r
}

/// The decimal digits `digits` (ASCII, at least one) as an exact big float.
fn digits_big(digits: &[u8]) -> BigFloat {
    let p = (digits.len() * 4 + 128).div_ceil(64) * 64;
    let mut acc = BigFloat::from_u64(0, p);
    for chunk in digits.chunks(19) {
        let mut v: u64 = 0;
        for &c in chunk {
            v = v * 10 + u64::from(c - b'0');
        }
        let scale = BigFloat::from_u64(10u64.pow(u32::try_from(chunk.len()).expect("19")), 64);
        acc = acc
            .mul(&scale, p, RoundingMode::None)
            .add(&BigFloat::from_u64(v, 64), p, RoundingMode::None);
    }
    acc
}

fn bits_needed(b: &BigFloat) -> usize {
    b.mantissa_max_bit_len().unwrap_or(64)
}

/// Whether a value rounded toward zero at [`WIDE`] bits lies so close to a midpoint of the rounding to `kept`
/// bits that its remaining bits cannot decide it: the guard bit 0 and every bit after it 1, or the guard bit 1 and
/// every bit after it 0, down to 64 bits from the end.
fn near_tie(b: &BigFloat, kept: u32) -> bool {
    let Some((words, bits, _, _, _)) = b.as_raw_parts() else {
        return false;
    };
    if bits == 0 {
        return false;
    }
    let total = words.len() * 64;
    // `i` counts from the top bit (0).
    let bit = |i: usize| -> bool {
        let pos = total - 1 - i;
        words[pos / 64] >> (pos % 64) & 1 == 1
    };
    let guard = usize::try_from(kept).expect("small");
    let end = total.saturating_sub(64);
    if guard + 1 >= end {
        return true;
    }
    let g = bit(guard);
    (guard + 1..end).all(|i| bit(i) != g)
}

/// The top `kept` bits of a big float's mantissa (at most 64).
fn top_kept(b: &BigFloat, kept: u32) -> u64 {
    let (words, _, _, _, _) = b.as_raw_parts().expect("finite");
    let (top, _) = top_bits(words);
    if kept == 0 {
        0
    } else {
        u64::try_from(top >> (128 - kept)).expect("at most 64 bits")
    }
}

/// The number of mantissa bits a value of `_FLOAT` keeps at binary exponent `e` (value = 0.m × 2^e): 64, fewer
/// for a subnormal.
fn kept_bits(e: i32) -> u32 {
    // The lowest kept bit's exponent is e - 64, at least EXP_MIN.
    let low = (e - 64).max(EXP_MIN);
    u32::try_from((e - low).clamp(0, 64)).expect("0 to 64")
}

impl F80 {
    /// `±digits × 10^exp10`, correctly rounded (gdtoa's `strtold`, as `__mingw_sscanf("%Lf")`). `digits` are ASCII
    /// decimal digits, leading zeros allowed.
    pub fn from_decimal(neg: bool, digits: &[u8], exp10: i64) -> F80 {
        let digits: Vec<u8> = digits.iter().copied().skip_while(|&c| c == b'0').collect();
        if digits.is_empty() {
            return F80::Zero { neg };
        }
        let len = i64::try_from(digits.len()).expect("short");
        // Beyond these the value is infinite or zero whatever the digits.
        if exp10 + len > 4934 {
            return F80::Inf { neg };
        }
        if exp10 + len < -4952 {
            return F80::Zero { neg };
        }
        let d = digits_big(&digits);
        let r = if exp10 >= 0 {
            let t = pow10(u32::try_from(exp10).expect("in range"));
            let p = (bits_needed(&d) + bits_needed(&t) + 64).div_ceil(64) * 64;
            let prod = d.mul(&t, p, RoundingMode::None);
            debug_assert!(!prod.inexact());
            F80::from_big(&prod)
        } else {
            let k = u32::try_from(-exp10).expect("in range");
            let t = pow10(k);
            let q = d.div(&t, WIDE, RoundingMode::ToZero);
            let e = q.exponent().expect("finite");
            let kept = kept_bits(e);
            if near_tie(&q, kept) {
                // Decide exactly: compare d with the boundary × 10^k.
                exact_quotient_round(&d, &t, &q, e, kept)
            } else {
                F80::from_big(&q)
            }
        };
        if neg { -r } else { r }
    }

    /// The text of `%.19LG` (`__mingw_snprintf`): 19 significant digits, correctly rounded, `%G`'s form.
    pub fn to_g19(self) -> String {
        let (neg, mant, exp) = match self {
            F80::Nan => return "NAN".to_string(),
            F80::Inf { neg } => return if neg { "-INF" } else { "INF" }.to_string(),
            F80::Zero { neg } => return if neg { "-0" } else { "0" }.to_string(),
            F80::Finite { neg, mant, exp } => (neg, mant, exp),
        };
        let x = F80::Finite { neg: false, mant, exp }.to_big();
        let (n, k) = digits19(&x);
        let s = format_g(n, k);
        if neg { format!("-{s}") } else { s }
    }
}

/// The rounding of `d / t` (both positive) decided exactly when the quotient `q` (toward zero, `WIDE` bits,
/// binary exponent `e`) is next to the midpoint of the rounding to `kept` bits: `d` is compared with the midpoint
/// times `t`.
fn exact_quotient_round(d: &BigFloat, t: &BigFloat, q: &BigFloat, e: i32, kept: u32) -> F80 {
    let m = top_kept(q, kept);
    // q = 0.(m)(rest) × 2^e, so m's unit is 2^(e - kept) and the midpoint is (2m + 1) × 2^(e - kept - 1).
    let unit_exp = e - i32::try_from(kept).expect("small");
    let mut mid = BigFloat::from_u128(2 * u128::from(m) + 1, 128);
    let bits = 128 - i32::try_from((2 * u128::from(m) + 1).leading_zeros()).expect("small");
    mid.set_exponent(unit_exp - 1 + bits);
    let pp = (bits_needed(&mid) + bits_needed(t) + 64).div_ceil(64) * 64;
    let mt = mid.mul(t, pp, RoundingMode::None);
    let up = match d.partial_cmp(&mt).unwrap_or(Ordering::Equal) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => m & 1 == 1,
    };
    F80::rounded(false, u128::from(m) + u128::from(up), unit_exp, false)
}

/// For a positive `x`, the 19-digit integer `n` (10^18 <= n < 10^19) and the decimal exponent `k` with
/// `x ≈ n × 10^(k - 18)`, `n` correctly rounded (ties to even).
fn digits19(x: &BigFloat) -> (u64, i32) {
    let e = x.exponent().expect("finite");
    // log10(x) is about (e - 1) × log10(2); a first guess, corrected below.
    // 30103 / 100000 is log10(2) to five digits; the loop below corrects the guess.
    let mut k = i32::try_from((i64::from(e) - 1) * 30_103 / 100_000).expect("small");
    loop {
        let n = scaled_round(x, 18 - k);
        if n >= 10_000_000_000_000_000_000 {
            k += 1;
            continue;
        }
        if n < 1_000_000_000_000_000_000 {
            k -= 1;
            continue;
        }
        return (u64::try_from(n).expect("19 digits"), k);
    }
}

/// `x × 10^s` rounded to an integer, ties to even, for a positive `x` whose product is below 2^127.
fn scaled_round(x: &BigFloat, s: i32) -> u128 {
    let p = WIDE;
    if s >= 0 {
        let t = pow10(u32::try_from(s).expect("non-negative"));
        let pp = (bits_needed(x) + bits_needed(&t) + 64).div_ceil(64) * 64;
        let y = x.mul(&t, pp, RoundingMode::None);
        round_big_int(&y, None)
    } else {
        let t = pow10(u32::try_from(-s).expect("positive"));
        let y = x.div(&t, p, RoundingMode::ToZero);
        round_big_int(&y, Some((x, &t)))
    }
}

/// A non-negative big float rounded to an integer, ties to even. When it was rounded toward zero (`exact` gives
/// the numerator and denominator it came from), a fraction next to one half is decided exactly.
fn round_big_int(y: &BigFloat, exact: Option<(&BigFloat, &BigFloat)>) -> u128 {
    let int = y.floor();
    let frac = y.sub(&int, WIDE, RoundingMode::None);
    let n = big_to_u128(&int);
    let half = BigFloat::from_f64(0.5, 64);
    let mut ord = frac.partial_cmp(&half).unwrap_or(Ordering::Less);
    if let Some((num, den)) = exact {
        // frac is low by less than 2^-(WIDE - 128); next to one half, compare num with (n + 1/2) × den exactly.
        let diff = frac.sub(&half, WIDE, RoundingMode::None).abs();
        let eps = BigFloat::from_f64(2f64.powi(-150), 64);
        if diff.partial_cmp(&eps) != Some(Ordering::Greater) {
            let mid = BigFloat::from_u128(n, 192).add(&half, 192, RoundingMode::None);
            let pp = (bits_needed(&mid) + bits_needed(den) + 64).div_ceil(64) * 64;
            let m = mid.mul(den, pp, RoundingMode::None);
            ord = num.partial_cmp(&m).unwrap_or(Ordering::Equal);
        }
    }
    match ord {
        Ordering::Greater => n + 1,
        Ordering::Less => n,
        Ordering::Equal => n + (n & 1),
    }
}

/// A non-negative integer-valued big float below 2^128.
fn big_to_u128(b: &BigFloat) -> u128 {
    if b.is_zero() {
        return 0;
    }
    let (words, _, _, e, _) = b.as_raw_parts().expect("finite");
    let (top, _) = top_bits(words);
    let e = u32::try_from(e).expect("at least 1");
    assert!(e <= 128, "below 2^128");
    if e == 0 { 0 } else { top >> (128 - e) }
}

/// `%.19G` of `n × 10^(k - 18)` (`n` has 19 digits): plain when `-4 <= k < 19`, else `d.dddE±XX`; trailing zeros
/// and a trailing point dropped.
fn format_g(n: u64, k: i32) -> String {
    let digits = n.to_string();
    debug_assert_eq!(digits.len(), 19);
    if (-4..19).contains(&k) {
        let s = if k >= 0 {
            let point = usize::try_from(k).expect("non-negative") + 1;
            format!("{}.{}", &digits[..point], &digits[point..])
        } else {
            let zeros = "0".repeat(usize::try_from(-k - 1).expect("positive"));
            format!("0.{zeros}{digits}")
        };
        trim_fraction(&s)
    } else {
        let mant = trim_fraction(&format!("{}.{}", &digits[..1], &digits[1..]));
        let sign = if k < 0 { '-' } else { '+' };
        format!("{mant}E{sign}{:02}", k.unsigned_abs())
    }
}

fn trim_fraction(s: &str) -> String {
    if !s.contains('.') {
        return s.to_string();
    }
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(d: f64) -> F80 {
        F80::from_f64(d)
    }

    fn dec(s: &str) -> F80 {
        let (neg, s) = s.strip_prefix('-').map_or((false, s), |r| (true, r));
        let (mant, e) = s
            .split_once(['E', 'e'])
            .map_or((s, 0), |(m, e)| (m, e.parse().unwrap()));
        let (w, frac) = mant.split_once('.').unwrap_or((mant, ""));
        let digits = format!("{w}{frac}");
        F80::from_decimal(neg, digits.as_bytes(), e - i64::try_from(frac.len()).unwrap())
    }

    #[test]
    fn operations_agree_with_f64_where_both_are_exact() {
        for (a, b) in [(1.5, 2.25), (-3.0, 0.125), (1e10, 3.0), (7.0, -7.0)] {
            assert_eq!((f(a) + f(b)).to_f64(), a + b);
            assert_eq!((f(a) - f(b)).to_f64(), a - b);
            assert_eq!((f(a) * f(b)).to_f64(), a * b);
        }
        assert_eq!((f(9.0) / f(4.0)).to_f64(), 2.25);
        assert_eq!(f(2.25).sqrt().to_f64(), 1.5);
        assert_eq!((f(7.0) - f(7.0)), F80::ZERO);
    }

    #[test]
    fn one_third_has_64_bits() {
        let third = f(1.0) / f(3.0);
        assert_eq!(
            third,
            F80::Finite {
                neg: false,
                mant: 0xAAAA_AAAA_AAAA_AAAB,
                exp: -65
            }
        );
        assert_eq!(third.to_g19(), "0.3333333333333333333");
        assert_eq!(third.to_f64(), 1.0 / 3.0);
    }

    #[test]
    fn ties_go_to_even() {
        // 2^64 + 1 is a tie between 2^64 and 2^64 + 2: even is 2^64.
        let r = F80::rounded(false, (1u128 << 64) + 1, 0, false);
        assert_eq!(
            r,
            F80::Finite {
                neg: false,
                mant: 1 << 63,
                exp: 1
            }
        );
        let r = F80::rounded(false, (1u128 << 64) + 3, 0, false);
        assert_eq!(
            r,
            F80::Finite {
                neg: false,
                mant: (1 << 63) + 2,
                exp: 1
            }
        );
        // Just above the tie goes up.
        let r = F80::rounded(false, (1u128 << 64) + 1, 0, true);
        assert_eq!(
            r,
            F80::Finite {
                neg: false,
                mant: (1 << 63) + 1,
                exp: 1
            }
        );
    }

    #[test]
    fn subnormals_and_overflow() {
        let min_normal = F80::Finite {
            neg: false,
            mant: 1 << 63,
            exp: EXP_MIN,
        };
        let half = min_normal / f(2.0);
        assert_eq!(
            half,
            F80::Finite {
                neg: false,
                mant: 1 << 62,
                exp: EXP_MIN
            }
        );
        assert_eq!((half * f(2.0)), min_normal);
        let max = F80::Finite {
            neg: false,
            mant: u64::MAX,
            exp: EXP_MAX,
        };
        assert_eq!((max * f(2.0)), F80::Inf { neg: false });
    }

    #[test]
    fn decimal_text_both_ways() {
        // Recorded by the old compiler (`verification\v45_a_*`, `v45_b_funcs`).
        for text in [
            "1.189731495357231765E+4932",
            "3.362103143112093506E-4932",
            "2.718281828459045235",
            "0.3333333333333333333",
            "3.141592653589793239",
            "0.006737946999085467097",
            "485165195.409790278",
            "9.999999999999999808E+149",
            "1.797693134862315E+308",
        ] {
            assert_eq!(dec(text).to_g19(), text, "{text}");
        }
        assert_eq!(dec("0.1").to_g19(), "0.1");
        assert_eq!((dec("0.1") * f(3.0)).to_g19(), "0.3");
        assert_eq!((dec("1E+30") / f(3.0)).to_g19(), "3.333333333333333333E+29");
        assert_eq!(f(1e19).to_g19(), "1E+19");
        assert_eq!(dec("0.0001").to_g19(), "0.0001");
        // The `f64` 0.0001 is 1.0000000000000000479…E-4.
        assert_eq!(f(0.0001).to_g19(), "0.0001000000000000000048");
        assert_eq!(dec("1E-5").to_g19(), "1E-05");
        assert_eq!(dec("9.3E+18").to_g19(), "9300000000000000000");
        assert_eq!(dec("2.5").to_g19(), "2.5");
    }

    #[test]
    fn decimal_rounds_correctly_at_the_boundaries() {
        // 1 + 2^-64 is the tie between 1 and 1 + 2^-63: even is 1.
        let tie = "1.0000000000000000000542101086242752217003726400434970855712890625";
        assert_eq!(dec(tie), f(1.0));
        let above = "1.0000000000000000000542101086242752217003726400434970855712890626";
        assert_eq!(
            dec(above),
            F80::Finite {
                neg: false,
                mant: (1 << 63) + 1,
                exp: -63
            }
        );
        // A round trip over a sweep of values.
        let mut x = dec("0.000123456789");
        for _ in 0..200 {
            let back = {
                let t = x.to_g19();
                dec(&t)
            };
            // 19 digits do not always identify a 64-bit value, but reading them gives the value nearest the text.
            let again = dec(&back.to_g19());
            assert_eq!(back, again);
            x = x * dec("1.7");
        }
    }

    #[test]
    fn integral_rounding() {
        let h = |d: f64, how| f(d).round_integral(how).to_f64();
        assert_eq!(h(2.5, Integral::NearestEven), 2.0);
        assert_eq!(h(3.5, Integral::NearestEven), 4.0);
        assert_eq!(h(-2.5, Integral::NearestEven), -2.0);
        assert_eq!(h(-20.2, Integral::Floor), -21.0);
        assert_eq!(h(-20.2, Integral::Ceil), -20.0);
        assert_eq!(h(-20.5, Integral::Trunc), -20.0);
        assert_eq!(f(2.0).to_i128(), Some(2));
        assert_eq!(f(2.5).to_i128(), None);
    }
}
