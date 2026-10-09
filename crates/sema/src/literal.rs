//! Numeric literal typing (spec `language/numeric-semantics`), following the old compiler's `lineformat$` and
//! `fixoperationorder$` step H (`study\02` §1.2–1.3), and the type suffixes of names and literals.
//!
//! A literal with an integer suffix has two types (design D7 of `m2-numeric-types`, measured
//! `verification\v21_a_literals`, `v21_f_literal_uses`, `v21_f_radix`): the old compiler emits it as its digits (`ll`
//! added for `&&`, `ull` for `~&&`) and only *believes* it has the suffix's type. So it is held in the type C++
//! gives that text, and converted to the suffix's type only where the generated code casts to it (`PRINT`, `STR$`,
//! `ABS`): `300~%% + 0` is 300, `PRINT 300~%%` 44.

use crate::Ty;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumLit {
    /// An integer literal: `value` held in `ty` (for `U64`, its 64 bits as an `i64`), believed to be `qb`. Without
    /// a suffix the two types are one.
    Int { value: i64, ty: Ty, qb: Ty },
    /// A float literal. `text` is the decimal value in C form (`1.5E+2`, `-0.1E+0`); a SINGLE or DOUBLE literal
    /// is the nearest `double` to it and a `_FLOAT` literal the nearest `long double`, as in the old compiler,
    /// which emits the same text into C++.
    Float { text: String, ty: Ty },
}

#[derive(Debug, PartialEq, Eq)]
pub enum LitError {
    /// A value beyond 64 bits (the old compiler's C++ build fails), or a signed `&H`/`&O`/`&B` literal wider than
    /// its type ("Overflow" in the old compiler, `verification\v21_x37`–`x41`).
    Overflow,
    /// Another compile error of the old compiler, with its reason.
    Error(&'static str),
    Unsupported(&'static str),
}

/// The type a suffix names, on a name or a number: `%`, `&`, `&&`, `!`, `#`, `##`, `$`, `%%`, `~%%`, `~%`, `~&`,
/// `~&&`, `%&`, `~%&`, `` ` ``, `` `n ``, `` ~` ``, `` ~`n `` (measured, `verification\v21_a_suffixes`). `None` for
/// any other text; an error for a `_BIT` width outside 1 to 64 (`` `0 ``, `` `65 ``: `v21_x14`, `x15`).
pub fn suffix_type(s: &[u8]) -> Option<Result<Ty, &'static str>> {
    let bit = |rest: &[u8], signed: bool| {
        if rest.is_empty() {
            return Ok(Ty::Bit { width: 1, signed });
        }
        match decimal(rest).map(u8::try_from) {
            Some(Ok(width @ 1..=64)) => Ok(Ty::Bit { width, signed }),
            Some(_) => Err("a `_BIT` suffix takes 1 to 64 bits"),
            None => Err("a `_BIT` suffix takes a number of bits"),
        }
    };
    Some(Ok(match s {
        b"%" => Ty::I16,
        b"&" => Ty::I32,
        b"&&" => Ty::I64,
        b"!" => Ty::F32,
        b"#" => Ty::F64,
        b"##" => Ty::F80,
        b"$" => Ty::Str,
        b"%%" => Ty::I8,
        b"~%%" => Ty::U8,
        b"~%" => Ty::U16,
        b"~&" => Ty::U32,
        b"~&&" => Ty::U64,
        b"%&" => Ty::Off,
        b"~%&" => Ty::UOff,
        _ => {
            return match s {
                [b'~', b'`', rest @ ..] => Some(bit(rest, false)),
                [b'`', rest @ ..] => Some(bit(rest, true)),
                _ => None,
            };
        }
    }))
}

/// The value of a run of decimal digits; `None` when empty, when a byte is no digit, or beyond `u128` (any such
/// value is out of every range a caller accepts).
pub fn decimal(digits: &[u8]) -> Option<u128> {
    if digits.is_empty() {
        return None;
    }
    digits.iter().try_fold(0u128, |acc, &b| {
        let d = b.is_ascii_digit().then(|| u128::from(b - b'0'))?;
        acc.checked_mul(10)?.checked_add(d)
    })
}

/// The integer type an integer suffix on a number names; `Ok(None)` without a suffix.
fn int_suffix(s: &[u8]) -> Result<Option<Ty>, LitError> {
    if s.is_empty() {
        return Ok(None);
    }
    match suffix_type(s) {
        // Measured: "Cannot use _OFFSET symbols after numbers" (`v21_x10_offset_literal`).
        Some(Ok(Ty::Off | Ty::UOff)) => Err(LitError::Error("an `_OFFSET` suffix cannot follow a number")),
        Some(Ok(t)) if t.is_int() => Ok(Some(t)),
        Some(Err(e)) => Err(LitError::Error(e)),
        Some(Ok(_)) | None => Err(LitError::Unsupported("this type suffix")),
    }
}

/// The type C++ gives a suffixed integer literal as the old compiler emits it: its digits (with the minus of the
/// value), `ll` added when the suffix's type is wider than 32 bits and signed, `ull` when unsigned (design D7;
/// `qb64pe.bas` 19730: `&&`, `~&&`, and `` `n ``, `` ~`n `` with n above 32). An `ull` literal is `uint64`; an `ll` one
/// `int64` unless its digits are beyond `int64` (then `uint64`, as C++ types such a literal); any other `int32` if its
/// digits fit, else `int64`, else `uint64`. An INTEGER literal in INTEGER's range is held as INTEGER (C++ widens it
/// to `int32` in every operation, so the two compute alike).
fn held_type(value: i128, believed: Ty) -> Ty {
    let magnitude = value.unsigned_abs();
    let bits = if let Ty::Bit { width, .. } = believed {
        u32::from(width)
    } else {
        believed.int_bits().expect("an integer suffix")
    };
    if believed == Ty::I16 && i16::try_from(value).is_ok() {
        Ty::I16
    } else if bits > 32 && believed.is_unsigned() {
        Ty::U64
    } else if magnitude <= i32::MAX as u128 && bits <= 32 {
        Ty::I32
    } else if magnitude <= i64::MAX as u128 {
        Ty::I64
    } else {
        Ty::U64
    }
}

/// A suffixed integer literal of value `value` (at most 64 bits wide) believed `qb`, held as [`held_type`] says.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "a `uint64` value is kept as its 64 bits (a negative one as C++ wraps it): the truncation is the point"
)]
fn suffixed(value: i128, qb: Ty) -> NumLit {
    let ty = held_type(value, qb);
    let value = if ty == Ty::U64 {
        value as u64 as i64
    } else {
        i64::try_from(value).expect("a held signed value fits its type")
    };
    NumLit::Int { value, ty, qb }
}

/// Types a numeric literal token. `negative` is true when a unary minus directly precedes it (folded into the
/// literal, step C of `fixoperationorder$`); it is ignored for `&H`/`&O`/`&B` literals, which the caller negates.
pub fn number(text: &[u8], negative: bool) -> Result<NumLit, LitError> {
    if text.first() == Some(&b'&') {
        return radix(text);
    }
    let mut i = 0;
    let digits = |i: &mut usize| {
        let s = *i;
        while *i < text.len() && text[*i].is_ascii_digit() {
            *i += 1;
        }
        text[s..*i].iter().map(|&b| char::from(b)).collect::<String>()
    };
    let mut whole = digits(&mut i);
    let mut frac = String::new();
    let mut dot = false;
    if text.get(i) == Some(&b'.') {
        dot = true;
        i += 1;
        frac = digits(&mut i);
    }
    let mut letter: Option<u8> = None;
    let mut exp_neg = false;
    let mut exp = String::new();
    if let Some(&c) = text.get(i)
        && matches!(c.to_ascii_uppercase(), b'E' | b'D' | b'F')
    {
        letter = Some(c.to_ascii_uppercase());
        i += 1;
        if let Some(&s) = text.get(i)
            && (s == b'+' || s == b'-')
        {
            exp_neg = s == b'-';
            i += 1;
        }
        exp = digits(&mut i);
    }
    let suffix = &text[i..];
    let whole_trim = whole.trim_start_matches('0').to_string();
    whole = whole_trim;
    let frac_trim = frac.trim_end_matches('0').to_string();
    frac = frac_trim;
    let exp = exp.trim_start_matches('0').to_string();
    let float = dot || letter.is_some();

    // An explicit float type: exponent letter or `!`, `#`, `##`.
    let forced = match (letter, suffix) {
        (Some(b'E'), _) | (None, b"!") => Some(Ty::F32),
        (Some(b'D'), _) | (None, b"#") => Some(Ty::F64),
        (Some(b'F'), _) | (None, b"##") => Some(Ty::F80),
        _ => None,
    };
    if forced.is_some() || float {
        let ty = match forced {
            Some(t) => t,
            None => {
                if !suffix.is_empty() {
                    return Err(LitError::Unsupported("this suffix on a number with a decimal point"));
                }
                auto_float_type(&whole, &frac)
            }
        };
        let text = format!(
            "{}{}.{}E{}{}",
            if negative { "-" } else { "" },
            if whole.is_empty() { "0" } else { &whole },
            if frac.is_empty() { "0" } else { &frac },
            if exp_neg { "-" } else { "+" },
            if exp.is_empty() { "0" } else { &exp },
        );
        return Ok(NumLit::Float { text, ty });
    }

    // Integer.
    let magnitude: u128 = if whole.is_empty() {
        0
    } else {
        whole.parse().map_err(|_| LitError::Overflow)?
    };
    // No literal type is wider than 64 bits; checked before the cast so a huge magnitude cannot wrap into range.
    if magnitude > 1 << 64 {
        return Err(LitError::Overflow);
    }
    let magnitude_signed = i128::try_from(magnitude).map_err(|_| LitError::Overflow)?;
    let value = if negative { -magnitude_signed } else { magnitude_signed };
    if let Some(qb) = int_suffix(suffix)? {
        // Held as written, in range of the suffix's type or not (design D7); C++ has no wider literal than 64 bits.
        if magnitude > u128::from(u64::MAX) {
            return Err(LitError::Overflow);
        }
        return Ok(suffixed(value, qb));
    }
    let ty = match suffix {
        b"" => {
            if negative {
                if magnitude <= 32768 {
                    Ty::I16
                } else if magnitude < 2147483648 {
                    // -2147483648 itself falls through to _INTEGER64 in the old compiler (`study\02` §1.3 H).
                    Ty::I32
                } else {
                    Ty::I64
                }
            } else if magnitude <= 32767 {
                Ty::I16
            } else if magnitude <= 2147483647 {
                Ty::I32
            } else if magnitude <= i64::MAX as u128 {
                Ty::I64
            } else {
                return Err(LitError::Unsupported("unsigned literals"));
            }
        }
        _ => return Err(LitError::Unsupported("this type suffix")),
    };
    Ok(NumLit::Int {
        value: i64::try_from(value).map_err(|_| LitError::Overflow)?,
        ty,
        qb: ty,
    })
}

/// SINGLE when at most 7 significant digits and the first one's position is within SINGLE's range; DOUBLE when
/// at most 16 and within DOUBLE's range; `_FLOAT` otherwise. As in `lineformat$`, the position is taken from the
/// digits alone (only literals without an exponent letter get here).
#[expect(
    clippy::cast_possible_wrap,
    reason = "digit counts are string lengths, at most isize::MAX, which fits i64"
)]
fn auto_float_type(whole: &str, frac: &str) -> Ty {
    let (offset, sig): (i64, usize) = if !whole.is_empty() {
        (whole.len() as i64 - 1, whole.len() + frac.len())
    } else if !frac.is_empty() {
        let zeros = frac.len() - frac.trim_start_matches('0').len();
        (-1 - zeros as i64, frac.len() - zeros)
    } else {
        (0, 0)
    };
    let sig_digits = format!("{whole}{frac}");
    let sig_digits = &sig_digits[sig_digits.len() - sig..];
    let fits = |max_off: i64, hi: &str, lo: &str| {
        if offset > max_off || offset < -max_off {
            return false;
        }
        !(offset == max_off && sig_digits > hi || offset == -max_off && sig_digits < lo)
    };
    if sig <= 7 && fits(38, "3402823", "1175494") {
        Ty::F32
    } else if sig <= 16 && fits(308, "1797693134862315", "2225073858507201") {
        Ty::F64
    } else {
        Ty::F80
    }
}

/// The values an integer type holds: its width, with its signedness; a `_BIT * n` its n bits.
pub fn range(ty: Ty) -> (i128, i128) {
    let bits = if let Ty::Bit { width, .. } = ty {
        u32::from(width)
    } else {
        ty.int_bits().unwrap_or_else(|| unreachable!("range of {ty:?}"))
    };
    if ty.is_unsigned() {
        (0, (1i128 << bits) - 1)
    } else {
        (-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1)
    }
}

/// The value of a [`suffixed`] literal of believed type `qb` whose digits are an evaluator's 64-bit value `v` (a
/// suffixed `CONST`, design D7): a negative value under an unsigned suffix is written as its unsigned 64-bit value,
/// as the old compiler writes it (`CONST b~%% = -1` holds 2^64-1).
pub fn constant_literal(v: i64, qb: Ty) -> NumLit {
    let digits = if qb.is_unsigned() && v < 0 {
        i128::from(v.cast_unsigned())
    } else {
        i128::from(v)
    };
    suffixed(digits, qb)
}

/// `&H`, `&O`, `&B`. Without a suffix the type is the smallest of INTEGER, LONG, `_INTEGER64` whose *unsigned*
/// width holds the value; the value is then read as signed (`&HFFFF` is -1). With an integer suffix the bits are read
/// with the type's signedness when they fit its width (`&HFF%%` is -1); an unsigned one wider than its type holds its
/// whole value and a signed one is an error (measured, `verification\v21_f_radix`, `v21_x37`–`x41`); the literal is
/// then held as its value's digits ([`suffixed`]).
fn radix(text: &[u8]) -> Result<NumLit, LitError> {
    let base: u32 = match text[1].to_ascii_uppercase() {
        b'H' => 16,
        b'O' => 8,
        _ => 2,
    };
    let mut i = 2;
    let mut value: u128 = 0;
    while i < text.len() && (text[i] as char).is_digit(base) {
        value = value * base as u128 + (text[i] as char).to_digit(base).unwrap() as u128;
        if value > u64::MAX as u128 {
            return Err(LitError::Overflow);
        }
        i += 1;
    }
    let suffix = int_suffix(&text[i..])?;
    if let Some(Ty::Bit { .. }) = suffix {
        return Err(LitError::Unsupported(
            "a `_BIT` suffix on an `&H`, `&O` or `&B` literal",
        ));
    }
    let ty = match suffix {
        Some(t) => t,
        None if value <= 0xFFFF => Ty::I16,
        None if value <= 0xFFFF_FFFF => Ty::I32,
        None => Ty::I64,
    };
    let bits = ty.int_bits().expect("a radix literal is an integer");
    let fits = bits == 64 || value >> bits == 0;
    let whole = i128::try_from(value).expect("at most 64 bits (checked above)");
    let read = if !fits {
        if ty.is_signed() {
            return Err(LitError::Overflow);
        }
        // An unsigned literal wider than its type holds its whole value.
        whole
    } else if ty.is_signed() && value >> (bits - 1) != 0 {
        whole - (1i128 << bits)
    } else {
        whole
    };
    match suffix {
        Some(qb) => Ok(suffixed(read, qb)),
        None => Ok(NumLit::Int {
            value: i64::try_from(read).expect("a signed radix value fits 64 bits"),
            ty,
            qb: ty,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Value and type of a literal whose held and believed types are one.
    fn int(t: &str, neg: bool) -> (i64, Ty) {
        let (value, ty, qb) = held(t, neg);
        assert_eq!(ty, qb, "{t}");
        (value, ty)
    }

    /// Value, held type and believed type of a literal.
    fn held(t: &str, neg: bool) -> (i64, Ty, Ty) {
        match number(t.as_bytes(), neg).unwrap() {
            NumLit::Int { value, ty, qb } => (value, ty, qb),
            other => panic!("{other:?}"),
        }
    }

    const fn bit(width: u8, signed: bool) -> Ty {
        Ty::Bit { width, signed }
    }

    #[test]
    fn suffixes() {
        let cases: [(&str, Ty); 14] = [
            ("%", Ty::I16),
            ("&", Ty::I32),
            ("&&", Ty::I64),
            ("##", Ty::F80),
            ("$", Ty::Str),
            ("%%", Ty::I8),
            ("~%%", Ty::U8),
            ("~%", Ty::U16),
            ("~&", Ty::U32),
            ("~&&", Ty::U64),
            ("%&", Ty::Off),
            ("~%&", Ty::UOff),
            ("`", bit(1, true)),
            ("~`64", bit(64, false)),
        ];
        for (s, t) in cases {
            assert_eq!(suffix_type(s.as_bytes()), Some(Ok(t)), "{s}");
        }
        assert_eq!(suffix_type(b"`7"), Some(Ok(bit(7, true))));
        assert!(matches!(suffix_type(b"`0"), Some(Err(_))));
        assert!(matches!(suffix_type(b"`65"), Some(Err(_))));
        assert!(matches!(suffix_type(b"~`999"), Some(Err(_))));
        assert_eq!(suffix_type(b"~"), None);
        assert_eq!(suffix_type(b"%%%"), None);
    }

    #[test]
    fn suffixed_literals_are_held_as_written() {
        // `verification\v21_a_literals`: believed the suffix's type, held as C++ types the digits.
        let cases: [(&str, bool, (i64, Ty, Ty)); 21] = [
            ("300~%%", false, (300, Ty::I32, Ty::U8)),
            ("255~%%", false, (255, Ty::I32, Ty::U8)),
            ("1~%%", true, (-1, Ty::I32, Ty::U8)),
            ("200%%", false, (200, Ty::I32, Ty::I8)),
            ("65536~%", false, (65536, Ty::I32, Ty::U16)),
            ("4294967295~&", false, (4294967295, Ty::I64, Ty::U32)),
            ("1~&", true, (-1, Ty::I32, Ty::U32)),
            ("1~&&", true, (-1, Ty::U64, Ty::U64)),
            ("18446744073709551615~&&", false, (-1, Ty::U64, Ty::U64)),
            ("5~&&", false, (5, Ty::U64, Ty::U64)),
            ("5&&", false, (5, Ty::I64, Ty::I64)),
            ("2147483647&&", false, (2147483647, Ty::I64, Ty::I64)),
            ("9223372036854775808&&", false, (i64::MIN, Ty::U64, Ty::I64)),
            // INTEGER in range stays INTEGER; out of range it is held as LONG (`PRINT 32768%` is -32768).
            ("5%", false, (5, Ty::I16, Ty::I16)),
            ("32768%", true, (-32768, Ty::I16, Ty::I16)),
            ("32768%", false, (32768, Ty::I32, Ty::I16)),
            ("40000%", false, (40000, Ty::I32, Ty::I16)),
            // C++ reads `-2147483648` as `-(2147483648)`, an `int64` (task 2.6: `(-2147483648&) * (-32768%)`).
            ("2147483648&", true, (-2147483648, Ty::I64, Ty::I32)),
            ("2147483648&", false, (2147483648, Ty::I64, Ty::I32)),
            ("9`3", false, (9, Ty::I32, bit(3, true))),
            ("1~`", true, (-1, Ty::I32, bit(1, false))),
        ];
        for (text, neg, want) in cases {
            assert_eq!(held(text, neg), want, "{}{text}", if neg { "-" } else { "" });
        }
        assert_eq!(number(b"18446744073709551616~&&", false), Err(LitError::Overflow));
        assert!(matches!(number(b"5%&", false), Err(LitError::Error(_))));
        assert!(matches!(number(b"5~%&", false), Err(LitError::Error(_))));
        assert!(matches!(number(b"5`65", false), Err(LitError::Error(_))));
    }

    #[test]
    fn constants_as_literals() {
        let lit = |v, qb| match constant_literal(v, qb) {
            NumLit::Int { value, ty, qb } => (value, ty, qb),
            other => panic!("{other:?}"),
        };
        assert_eq!(lit(-1, Ty::U8), (-1, Ty::U64, Ty::U8));
        assert_eq!(lit(200, Ty::I8), (200, Ty::I32, Ty::I8));
        assert_eq!(lit(40000, Ty::I16), (40000, Ty::I32, Ty::I16));
        assert_eq!(lit(-5, Ty::I32), (-5, Ty::I32, Ty::I32));
        assert_eq!(lit(-1, bit(3, false)), (-1, Ty::U64, bit(3, false)));
        assert_eq!(lit(5, bit(3, true)), (5, Ty::I32, bit(3, true)));
    }

    #[test]
    fn ranges() {
        assert_eq!(range(Ty::U8), (0, 255));
        assert_eq!(range(Ty::I8), (-128, 127));
        assert_eq!(range(Ty::U64), (0, i128::from(u64::MAX)));
        assert_eq!(range(Ty::I64), (i128::from(i64::MIN), i128::from(i64::MAX)));
        assert_eq!(range(bit(3, true)), (-4, 3));
        assert_eq!(range(bit(3, false)), (0, 7));
    }

    fn float(t: &str) -> (String, Ty) {
        match number(t.as_bytes(), false).unwrap() {
            NumLit::Float { text, ty } => (text, ty),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn integer_typing() {
        assert_eq!(int("3", true), (-3, Ty::I16));
        assert_eq!(int("40000", false), (40000, Ty::I32));
        assert_eq!(int("32768", true), (-32768, Ty::I16));
        assert_eq!(int("32769", true), (-32769, Ty::I32));
        assert_eq!(int("2147483648", true), (-2147483648, Ty::I64));
        assert_eq!(int("2147483648", false), (2147483648, Ty::I64));
        assert_eq!(int("5&&", false), (5, Ty::I64));
        assert_eq!(
            number(b"9223372036854775808", false),
            Err(LitError::Unsupported("unsigned literals"))
        );
    }

    #[test]
    fn radix_typing() {
        assert_eq!(int("&HFF", false), (255, Ty::I16));
        assert_eq!(int("&HFFFF", false), (-1, Ty::I16));
        assert_eq!(int("&H8000", false), (-32768, Ty::I16));
        assert_eq!(int("&H10000", false), (65536, Ty::I32));
        assert_eq!(int("&HFFFF&", false), (65535, Ty::I32));
        assert_eq!(int("&HFFFFFFFF", false), (-1, Ty::I32));
        assert_eq!(int("&H100000000", false), (4294967296, Ty::I64));
        assert_eq!(int("&O177777", false), (-1, Ty::I16));
        assert_eq!(int("&B1111111111111111", false), (-1, Ty::I16));
        // With a suffix: the type's signedness when the bits fit (`verification\v21_f_radix`).
        assert_eq!(held("&HFF%%", false), (-1, Ty::I32, Ty::I8));
        assert_eq!(held("&H80%%", false), (-128, Ty::I32, Ty::I8));
        assert_eq!(held("&HFF~%%", false), (255, Ty::I32, Ty::U8));
        assert_eq!(held("&HFFFFFFFF~&", false), (4294967295, Ty::I64, Ty::U32));
        assert_eq!(held("&HFFFFFFFFFFFFFFFF~&&", false), (-1, Ty::U64, Ty::U64));
        assert_eq!(held("&HFFFFFFFFFFFFFFFF&&", false), (-1, Ty::I64, Ty::I64));
        // An unsigned one wider than its type holds its whole value; a signed one is an error.
        assert_eq!(held("&H1FF~%%", false), (511, Ty::I32, Ty::U8));
        assert_eq!(held("&H1FFFFFFFF~&", false), (8589934591, Ty::I64, Ty::U32));
        assert_eq!(held("&O777~%%", false), (511, Ty::I32, Ty::U8));
        for t in ["&H1FF%%", "&H1FFFF%", "&H1FFFFFFFF&", "&O777%%", "&B111111111%%"] {
            assert_eq!(number(t.as_bytes(), false), Err(LitError::Overflow), "{t}");
        }
        assert_eq!(number(b"&H10000000000000000", false), Err(LitError::Overflow));
    }

    #[test]
    fn float_typing() {
        assert_eq!(float("1.5E2"), ("1.5E+2".into(), Ty::F32));
        assert_eq!(float("1D5"), ("1.0E+5".into(), Ty::F64));
        assert_eq!(float("1.5##"), ("1.5E+0".into(), Ty::F80));
        assert_eq!(float(".5"), ("0.5E+0".into(), Ty::F32));
        assert_eq!(float("1234567."), ("1234567.0E+0".into(), Ty::F32));
        assert_eq!(float("12345678."), ("12345678.0E+0".into(), Ty::F64));
        assert_eq!(float("1.23456789012345678").1, Ty::F80);
        assert_eq!(float("7E38"), ("7.0E+38".into(), Ty::F32));
        assert_eq!(float("0.1#"), ("0.1E+0".into(), Ty::F64));
        assert_eq!(float("1.50000000"), ("1.5E+0".into(), Ty::F32));
    }
}
