//! Values from the debuggee's bytes, shown by their BASIC type (design D7 of `m5-dap-proxy`): integers exact,
//! `SINGLE` and `DOUBLE` by their shortest round-trip text, `_FLOAT` through `f64`, `_OFFSET` in hex, strings quoted
//! with the bytes outside 32–126 written as BASIC would build them. The display formats are this adapter's own.

/// What a type is, as its text in the symbol file spells it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Integer { size: u64, signed: bool },
    Single,
    Double,
    Float,
    Offset { signed: bool },
    String,
    Fixed(u64),
    Bit,
    Mem,
    User(String),
}

impl Kind {
    pub fn of(ty: &str) -> Kind {
        let ty = ty.trim();
        let (unsigned, rest) = match ty.strip_prefix("_UNSIGNED ") {
            Some(r) => (true, r.trim()),
            None => (false, ty),
        };
        let int = |size| Kind::Integer {
            size,
            signed: !unsigned,
        };
        match rest.to_ascii_uppercase().as_str() {
            "_BYTE" => int(1),
            "INTEGER" => int(2),
            "LONG" => int(4),
            "_INTEGER64" => int(8),
            "_OFFSET" => Kind::Offset { signed: !unsigned },
            "SINGLE" => Kind::Single,
            "DOUBLE" => Kind::Double,
            "_FLOAT" => Kind::Float,
            "STRING" => Kind::String,
            "_MEM" => Kind::Mem,
            r if r == "_BIT" || r.starts_with("_BIT ") || r.starts_with("_BIT*") => Kind::Bit,
            r => match r.strip_prefix("STRING").map(str::trim_start) {
                Some(n) if n.starts_with('*') => n[1..].trim().parse().map_or(Kind::User(ty.into()), Kind::Fixed),
                _ => Kind::User(ty.to_string()),
            },
        }
    }

    /// The bytes of a scalar of this kind outside an array (a `_FLOAT` takes 16, its 10 bytes then padding); `None`
    /// for a `STRING` (a `qbs`), a `_BIT` and a user type, whose size is elsewhere.
    pub fn size(&self) -> Option<u64> {
        match self {
            Kind::Integer { size, .. } => Some(*size),
            Kind::Single => Some(4),
            Kind::Double | Kind::Offset { .. } => Some(8),
            Kind::Float => Some(16),
            Kind::Fixed(n) => Some(*n),
            Kind::String | Kind::Bit | Kind::Mem | Kind::User(_) => None,
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Kind::Integer { .. } | Kind::Offset { .. })
    }
}

/// How an integer is shown.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Radix {
    #[default]
    Decimal,
    Hex,
    Binary,
    Octal,
}

fn le(bytes: &[u8]) -> u64 {
    bytes.iter().rev().fold(0, |n, &b| (n << 8) | u64::from(b))
}

/// The integer in `bytes` (little-endian, `bytes.len()` of them), sign-extended when `signed`.
pub fn integer(bytes: &[u8], signed: bool) -> i128 {
    let raw = le(bytes);
    let bits = 8 * u32::try_from(bytes.len()).unwrap_or(8);
    if signed && bits < 64 && raw & (1 << (bits - 1)) != 0 {
        i128::from(raw) - (1i128 << bits)
    } else if signed && bits == 64 {
        i128::from(raw.cast_signed())
    } else {
        i128::from(raw)
    }
}

/// An integer shown in `radix`: decimal as it is, the others as BASIC writes them (`&HFF`), a negative number by
/// its bytes, as `HEX$` does.
pub fn show_integer(bytes: &[u8], signed: bool, radix: Radix) -> String {
    let raw = le(bytes);
    match radix {
        Radix::Decimal => integer(bytes, signed).to_string(),
        Radix::Hex => format!("&H{raw:X}"),
        Radix::Binary => format!("&B{raw:b}"),
        Radix::Octal => format!("&O{raw:o}"),
    }
}

/// A floating value in the shortest text that reads back the same, in exponent form when very large or small.
fn show_float(x: f64, shortest: impl Fn() -> String, exponent: impl Fn() -> String) -> String {
    if x.is_nan() {
        "NaN".into()
    } else if x.is_infinite() {
        if x > 0.0 { "Infinity".into() } else { "-Infinity".into() }
    } else if x != 0.0 && !(1e-6..1e16).contains(&x.abs()) {
        exponent()
    } else {
        shortest()
    }
}

pub fn show_single(bytes: &[u8]) -> String {
    let x = f32::from_bits(u32::try_from(le(&bytes[..4])).unwrap_or(0));
    show_float(f64::from(x), || x.to_string(), || format!("{x:e}"))
}

pub fn show_double(bytes: &[u8]) -> String {
    let x = f64::from_bits(le(&bytes[..8]));
    show_float(x, || x.to_string(), || format!("{x:e}"))
}

/// An 80-bit x87 extended value (its first 10 bytes: the 64-bit significand with its integer bit, then the sign and
/// the 15-bit exponent) as the nearest `f64`; out of `f64`'s range it is an infinity or zero.
pub fn float80(bytes: &[u8]) -> f64 {
    let significand = le(&bytes[..8]);
    let top = u16::try_from(le(&bytes[8..10])).unwrap_or(0);
    let negative = top & 0x8000 != 0;
    let exponent = i32::from(top & 0x7FFF);
    let magnitude = if exponent == 0x7FFF {
        if significand << 1 == 0 { f64::INFINITY } else { f64::NAN }
    } else {
        // value = significand * 2^(exponent - 16383 - 63), scaled in steps that stay in f64's exponent range.
        #[expect(
            clippy::cast_precision_loss,
            reason = "the display of a _FLOAT goes through f64 (design D7)"
        )]
        let mut x = significand as f64;
        let mut e = exponent - 16383 - 63;
        while e > 0 {
            let step = e.min(1000);
            x *= 2f64.powi(step);
            e -= step;
        }
        while e < 0 {
            let step = e.max(-1000);
            x *= 2f64.powi(step);
            e -= step;
        }
        x
    };
    if negative { -magnitude } else { magnitude }
}

/// An `f64` as the 10 bytes of an x87 extended value (exact: every `f64` is one).
pub fn to_float80(x: f64) -> [u8; 10] {
    let bits = x.to_bits();
    let sign = u16::from(x.is_sign_negative()) << 15;
    let exponent = i32::try_from((bits >> 52) & 0x7FF).unwrap_or(0);
    let fraction = bits & ((1 << 52) - 1);
    let (e, significand) = match exponent {
        0 if fraction == 0 => (0, 0),
        0 => {
            // A subnormal: normalise its fraction.
            let shift = fraction.leading_zeros();
            (16383 - 1022 - i32::try_from(shift).unwrap_or(0) + 11, fraction << shift)
        }
        0x7FF => (0x7FFF, (1 << 63) | (fraction << 11)),
        e => (e - 1023 + 16383, (1 << 63) | (fraction << 11)),
    };
    let mut out = [0; 10];
    out[..8].copy_from_slice(&significand.to_le_bytes());
    out[8..].copy_from_slice(&(sign | u16::try_from(e).unwrap_or(0)).to_le_bytes());
    out
}

pub fn show_float80(bytes: &[u8]) -> String {
    let x = float80(bytes);
    show_float(x, || x.to_string(), || format!("{x:e}"))
}

/// A string's bytes as BASIC would build them: printable runs quoted, every other byte (and `"`) as `CHR$(n)`,
/// joined by ` + `; `""` when empty.
pub fn show_string(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "\"\"".into();
    }
    let mut parts = Vec::new();
    let mut run = String::new();
    for &b in bytes {
        if (32..=126).contains(&b) && b != b'"' {
            run.push(char::from(b));
        } else {
            if !run.is_empty() {
                parts.push(format!("\"{run}\""));
                run.clear();
            }
            parts.push(format!("CHR$({b})"));
        }
    }
    if !run.is_empty() {
        parts.push(format!("\"{run}\""));
    }
    parts.join(" + ")
}

/// The bytes of a string as a user types it: BASIC's own form (`"a" + CHR$(0) + "b"`, a `""` inside quotes being
/// one `"`), or else the text as it is.
pub fn parse_string(text: &str) -> Vec<u8> {
    fn terms(t: &str) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        let mut rest = t.trim();
        loop {
            if let Some(r) = rest.strip_prefix('"') {
                let mut i = 0;
                let b = r.as_bytes();
                loop {
                    match b.get(i) {
                        Some(b'"') if b.get(i + 1) == Some(&b'"') => {
                            out.push(b'"');
                            i += 2;
                        }
                        Some(b'"') => break,
                        Some(&c) => {
                            out.push(c);
                            i += 1;
                        }
                        None => return None,
                    }
                }
                rest = r[i + 1..].trim_start();
            } else {
                let upper = rest.get(..5)?.to_ascii_uppercase();
                if upper != "CHR$(" {
                    return None;
                }
                let close = rest.find(')')?;
                out.push(rest[5..close].trim().parse::<u8>().ok()?);
                rest = rest[close + 1..].trim_start();
            }
            if rest.is_empty() {
                return Some(out);
            }
            rest = rest.strip_prefix('+')?.trim_start();
        }
    }
    terms(text).unwrap_or_else(|| text.as_bytes().to_vec())
}

/// A scalar of `kind` from its bytes; `None` for a kind that is not shown from bytes alone.
pub fn show(kind: &Kind, bytes: &[u8], radix: Radix) -> Option<String> {
    Some(match kind {
        Kind::Integer { size, signed } => show_integer(&bytes[..usize::try_from(*size).ok()?], *signed, radix),
        Kind::Offset { signed } => match radix {
            Radix::Decimal => show_integer(&bytes[..8], false, Radix::Hex),
            r => show_integer(&bytes[..8], *signed, r),
        },
        Kind::Single => show_single(bytes),
        Kind::Double => show_double(bytes),
        Kind::Float => show_float80(bytes),
        Kind::Fixed(n) => show_string(&bytes[..usize::try_from(*n).ok()?]),
        Kind::String | Kind::Bit | Kind::Mem | Kind::User(_) => return None,
    })
}

/// The text a user typed for a number of `kind`, as its bytes: an integer in range (decimal, or `&H`, `&O`, `&B`)
/// or a floating value; an error saying why not.
pub fn parse_number(kind: &Kind, text: &str) -> Result<Vec<u8>, String> {
    let t = text.trim();
    match kind {
        Kind::Integer { size, signed } => {
            let n = parse_integer(t).ok_or_else(|| format!("`{t}` is not a whole number"))?;
            let bits = 8 * u32::try_from(*size).unwrap_or(8);
            let (lo, hi) = if *signed {
                (-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1)
            } else {
                (0, (1i128 << bits) - 1)
            };
            // `&HFFFF` for an INTEGER is -1, as BASIC reads it.
            let n = if *signed && t.starts_with('&') && n > hi && n < (1i128 << bits) {
                n - (1i128 << bits)
            } else {
                n
            };
            if n < lo || n > hi {
                return Err(format!("{t} is out of the range of the type ({lo} to {hi})"));
            }
            let size = usize::try_from(*size).unwrap_or(8);
            Ok(n.cast_unsigned().to_le_bytes()[..size].to_vec())
        }
        Kind::Offset { .. } => {
            let n = parse_integer(t).ok_or_else(|| format!("`{t}` is not a whole number"))?;
            let raw = i64::try_from(n)
                .map(i64::cast_unsigned)
                .or_else(|_| u64::try_from(n))
                .map_err(|_| format!("{t} is out of the range of the type"))?;
            Ok(raw.to_le_bytes().to_vec())
        }
        Kind::Single => {
            let x: f64 = parse_float(t)?;
            #[expect(clippy::cast_possible_truncation, reason = "a SINGLE is set to the nearest value")]
            let y = x as f32;
            if y.is_infinite() && x.is_finite() {
                return Err(format!("{t} is out of the range of the type"));
            }
            Ok(y.to_le_bytes().to_vec())
        }
        Kind::Double => Ok(parse_float(t)?.to_le_bytes().to_vec()),
        Kind::Float => Ok(to_float80(parse_float(t)?).to_vec()),
        _ => Err("this value cannot be set as a number".into()),
    }
}

fn parse_integer(t: &str) -> Option<i128> {
    let (negative, t) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let upper = t.to_ascii_uppercase();
    let (radix, digits) = if let Some(d) = upper.strip_prefix("&H") {
        (16, d)
    } else if let Some(d) = upper.strip_prefix("&O") {
        (8, d)
    } else if let Some(d) = upper.strip_prefix("&B") {
        (2, d)
    } else {
        (10, upper.as_str())
    };
    if digits.is_empty() || digits.len() > 40 {
        return None;
    }
    let n = i128::from_str_radix(digits, radix).ok()?;
    Some(if negative { -n } else { n })
}

fn parse_float(t: &str) -> Result<f64, String> {
    let lower = t.to_ascii_lowercase();
    // BASIC writes a DOUBLE's exponent with D; a type suffix may end it.
    let lower = lower.trim_end_matches(['!', '#']).replace('d', "e");
    lower
        .parse::<f64>()
        .ok()
        .filter(|x| x.is_finite())
        .ok_or_else(|| format!("`{t}` is not a number"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds() {
        assert_eq!(Kind::of("_UNSIGNED INTEGER"), Kind::Integer { size: 2, signed: false });
        assert_eq!(Kind::of("LONG"), Kind::Integer { size: 4, signed: true });
        assert_eq!(Kind::of("STRING * 4"), Kind::Fixed(4));
        assert_eq!(Kind::of("STRING"), Kind::String);
        assert_eq!(Kind::of("_BIT * 3"), Kind::Bit);
        assert_eq!(Kind::of("_UNSIGNED _BIT"), Kind::Bit);
        assert_eq!(Kind::of("_MEM"), Kind::Mem);
        assert_eq!(Kind::of("PT"), Kind::User("PT".into()));
        assert_eq!(Kind::of("_UNSIGNED _OFFSET"), Kind::Offset { signed: false });
    }

    #[test]
    fn integers() {
        let k = Kind::of("INTEGER");
        assert_eq!(show(&k, &(-2i16).to_le_bytes(), Radix::Decimal).unwrap(), "-2");
        assert_eq!(show(&k, &(-1i16).to_le_bytes(), Radix::Hex).unwrap(), "&HFFFF");
        let u = Kind::of("_UNSIGNED INTEGER");
        assert_eq!(show(&u, &65535u16.to_le_bytes(), Radix::Decimal).unwrap(), "65535");
        let l = Kind::of("LONG");
        assert_eq!(show(&l, &255i32.to_le_bytes(), Radix::Hex).unwrap(), "&HFF");
        assert_eq!(show(&l, &5i32.to_le_bytes(), Radix::Binary).unwrap(), "&B101");
        assert_eq!(show(&l, &8i32.to_le_bytes(), Radix::Octal).unwrap(), "&O10");
        let q = Kind::of("_INTEGER64");
        assert_eq!(
            show(&q, &i64::MIN.to_le_bytes(), Radix::Decimal).unwrap(),
            i64::MIN.to_string()
        );
        let uq = Kind::of("_UNSIGNED _INTEGER64");
        assert_eq!(
            show(&uq, &u64::MAX.to_le_bytes(), Radix::Decimal).unwrap(),
            u64::MAX.to_string()
        );
        let b = Kind::of("_BYTE");
        assert_eq!(show(&b, &[0x80], Radix::Decimal).unwrap(), "-128");
    }

    #[test]
    fn offsets_in_hex() {
        let k = Kind::of("_OFFSET");
        assert_eq!(show(&k, &0x1234u64.to_le_bytes(), Radix::Decimal).unwrap(), "&H1234");
    }

    #[test]
    fn floats() {
        assert_eq!(show_single(&1.5f32.to_le_bytes()), "1.5");
        assert_eq!(show_single(&0.1f32.to_le_bytes()), "0.1");
        assert_eq!(show_single(&1.0f32.to_le_bytes()), "1");
        assert_eq!(show_double(&(1.0f64 / 3.0).to_le_bytes()), "0.3333333333333333");
        assert_eq!(show_double(&1e300f64.to_le_bytes()), "1e300");
        assert_eq!(show_double(&(-2.5e-9f64).to_le_bytes()), "-2.5e-9");
    }

    /// `_FLOAT` from its bytes: 1.5 is the significand 0xC000… with the exponent 16383; -2 has the sign bit and
    /// the exponent 16384; 0 is all zeros.
    #[test]
    fn float80_from_its_bytes() {
        let mut one_and_half = [0u8; 16];
        one_and_half[..8].copy_from_slice(&0xC000_0000_0000_0000u64.to_le_bytes());
        one_and_half[8..10].copy_from_slice(&16383u16.to_le_bytes());
        assert_eq!(show_float80(&one_and_half), "1.5");
        let mut minus_two = [0u8; 16];
        minus_two[..8].copy_from_slice(&0x8000_0000_0000_0000u64.to_le_bytes());
        minus_two[8..10].copy_from_slice(&(0x8000u16 | 16384).to_le_bytes());
        assert_eq!(show_float80(&minus_two), "-2");
        assert_eq!(show_float80(&[0u8; 16]), "0");
        let mut huge = [0u8; 16];
        huge[..8].copy_from_slice(&0x8000_0000_0000_0000u64.to_le_bytes());
        huge[8..10].copy_from_slice(&(16383u16 + 2000).to_le_bytes());
        assert_eq!(show_float80(&huge), "Infinity");
        for x in [1.5, -2.0, 0.0, 1e300, 1e-310, 0.1] {
            assert_eq!(float80(&to_float80(x)), x, "{x}");
        }
        assert_eq!(to_float80(1.5)[..8], one_and_half[..8]);
    }

    #[test]
    fn strings() {
        assert_eq!(show_string(b"hi"), "\"hi\"");
        assert_eq!(show_string(b""), "\"\"");
        assert_eq!(show_string(b"a\0b"), "\"a\" + CHR$(0) + \"b\"");
        assert_eq!(show_string(b"say \"x\""), "\"say \" + CHR$(34) + \"x\" + CHR$(34)");
        assert_eq!(show_string(&[200]), "CHR$(200)");
        assert_eq!(show(&Kind::Fixed(4), b"abcd", Radix::Decimal).unwrap(), "\"abcd\"");
    }

    #[test]
    fn parsing_strings() {
        assert_eq!(parse_string("\"abc\""), b"abc");
        assert_eq!(parse_string("\"a\" + CHR$(0) + \"b\""), b"a\0b");
        assert_eq!(parse_string("\"say \"\"x\"\"\""), b"say \"x\"");
        assert_eq!(parse_string("plain text"), b"plain text");
        assert_eq!(parse_string(&show_string(b"q\"\x01")), b"q\"\x01");
    }

    #[test]
    fn parsing_numbers() {
        let i = Kind::of("INTEGER");
        assert_eq!(parse_number(&i, "42").unwrap(), 42i16.to_le_bytes());
        assert_eq!(parse_number(&i, "-32768").unwrap(), i16::MIN.to_le_bytes());
        assert!(parse_number(&i, "40000").unwrap_err().contains("out of the range"));
        assert!(parse_number(&i, "x").unwrap_err().contains("not a whole number"));
        assert_eq!(parse_number(&i, "&HFFFF").unwrap(), (-1i16).to_le_bytes());
        let u = Kind::of("_UNSIGNED _BYTE");
        assert!(parse_number(&u, "-1").is_err());
        assert_eq!(parse_number(&u, "255").unwrap(), [255]);
        assert_eq!(parse_number(&Kind::Single, "1.5").unwrap(), 1.5f32.to_le_bytes());
        assert!(
            parse_number(&Kind::Single, "1e300")
                .unwrap_err()
                .contains("out of the range")
        );
        assert_eq!(parse_number(&Kind::Double, "2.5D3").unwrap(), 2500f64.to_le_bytes());
        assert!(parse_number(&Kind::Double, "abc").is_err());
    }
}
