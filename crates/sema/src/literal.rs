//! Numeric literal typing (spec `language/numeric-semantics`), following the old compiler's `lineformat$` and
//! `fixoperationorder$` step H (`study\02` §1.2–1.3).

use crate::Ty;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumLit {
    Int {
        value: i64,
        ty: Ty,
    },
    /// A float literal. `text` is the decimal value in C form (`1.5E+2`, `-0.1E+0`); a SINGLE or DOUBLE literal
    /// is the nearest `double` to it and a `_FLOAT` literal the nearest `long double`, as in the old compiler,
    /// which emits the same text into C++.
    Float {
        text: String,
        ty: Ty,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum LitError {
    Overflow,
    Unsupported(&'static str),
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
        std::str::from_utf8(&text[s..*i]).unwrap().to_string()
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
    if let Some(&c) = text.get(i) {
        if matches!(c.to_ascii_uppercase(), b'E' | b'D' | b'F') {
            letter = Some(c.to_ascii_uppercase());
            i += 1;
            if let Some(&s) = text.get(i) {
                if s == b'+' || s == b'-' {
                    exp_neg = s == b'-';
                    i += 1;
                }
            }
            exp = digits(&mut i);
        }
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
    let value: i128 = if negative {
        -(magnitude as i128)
    } else {
        magnitude as i128
    };
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
        b"%" => Ty::I16,
        b"&" => Ty::I32,
        b"&&" => Ty::I64,
        _ => return Err(LitError::Unsupported("this type suffix")),
    };
    let (lo, hi) = range(ty);
    if value < lo || value > hi {
        return Err(LitError::Overflow);
    }
    Ok(NumLit::Int {
        value: value as i64,
        ty,
    })
}

/// SINGLE when at most 7 significant digits and the first one's position is within SINGLE's range; DOUBLE when
/// at most 16 and within DOUBLE's range; `_FLOAT` otherwise. As in `lineformat$`, the position is taken from the
/// digits alone (only literals without an exponent letter get here).
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

pub fn range(ty: Ty) -> (i128, i128) {
    match ty {
        Ty::I16 => (i16::MIN as i128, i16::MAX as i128),
        Ty::I32 => (i32::MIN as i128, i32::MAX as i128),
        Ty::I64 => (i64::MIN as i128, i64::MAX as i128),
        Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str => unreachable!("range of {ty:?}"),
    }
}

/// `&H`, `&O`, `&B`: the type is the explicit suffix or, without one, the smallest of INTEGER, LONG, `_INTEGER64`
/// whose *unsigned* width holds the value; the value is then read as signed (`&HFFFF` is -1).
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
    let ty = match &text[i..] {
        b"" if value <= 0xFFFF => Ty::I16,
        b"" if value <= 0xFFFF_FFFF => Ty::I32,
        b"" => Ty::I64,
        b"%" => Ty::I16,
        b"&" => Ty::I32,
        b"&&" => Ty::I64,
        _ => return Err(LitError::Unsupported("this type suffix")),
    };
    let bits = match ty {
        Ty::I16 => 16,
        Ty::I32 => 32,
        Ty::I64 => 64,
        Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str => unreachable!("radix literal of {ty:?}"),
    };
    if bits < 64 && value >> bits != 0 {
        return Err(LitError::Overflow);
    }
    let signed = if bits == 64 {
        value as u64 as i64
    } else if value >> (bits - 1) != 0 {
        value as i64 - (1i64 << bits)
    } else {
        value as i64
    };
    Ok(NumLit::Int { value: signed, ty })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(t: &str, neg: bool) -> (i64, Ty) {
        match number(t.as_bytes(), neg).unwrap() {
            NumLit::Int { value, ty } => (value, ty),
            other => panic!("{other:?}"),
        }
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
        assert_eq!(number(b"40000%", false), Err(LitError::Overflow));
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
