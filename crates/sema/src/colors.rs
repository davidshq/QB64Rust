//! The colour functions of the `CONST` evaluator (design D5 of `m3-constant-evaluation`): `_RGB32` … `_ALPHA32` as
//! bit arithmetic (libqb's `graphics.h`), and `_RGB`, `_RGBA`, `_RED` … `_ALPHA` on a new 1 × 1 image of a screen
//! mode, as `EvaluateFunction$` makes one (`_NEWIMAGE(1, 1, mode)`), with libqb's default palette of the mode
//! (`restorepalette`) and its nearest-colour search (`matchcol`, `func__rgb`, `func__red`, … in `libqb.cpp`).
//! Measured: `verification\v45_b_colors32`, `v45_b_colors_m*`, `v45_x60`–`x66`.

use crate::consteval::{R, Value, as_i64, as_u64, error, unsupported};
use qb64rust_builtins::palettes::{PALETTE_64, PALETTE_256};

/// A screen mode's image: its palette (256 entries, as libqb allocates them, unused ones 0), the number of
/// colours `matchcol` searches and the mask an index must not exceed.
struct Image {
    pal: [u32; 256],
    colours: usize,
    mask: u32,
    bgra: bool,
}

/// The image of `mode` (0 to 2, 7 to 13, 256, 32), or `None` for another mode.
fn image(mode: u64) -> Option<Image> {
    let mut pal = [0u32; 256];
    let bpp: u32 = match mode {
        0 => 16,
        1 | 10 => 2,
        2 | 11 => 1,
        7 | 8 | 9 | 12 => 4,
        13 | 256 => 8,
        32 => {
            return Some(Image {
                pal,
                colours: 0,
                mask: u32::MAX,
                bgra: true,
            });
        }
        _ => return None,
    };
    match mode {
        1 => {
            for (i, &k) in [0, 3, 5, 7].iter().enumerate() {
                pal[i] = PALETTE_256[k];
            }
        }
        2 | 11 => pal[1] = 0xFF_FFFF,
        9 => {
            let picks = [0, 1, 2, 3, 4, 5, 20, 7, 56, 57, 58, 59, 60, 61, 62, 63];
            for (i, &k) in picks.iter().enumerate() {
                pal[i] = PALETTE_64[k];
            }
        }
        // Mode 10 keeps its colours as indexes in entries 4 to 7; entries 0 to 3 stay 0 (measured: every
        // colour of mode 10 reads 0).
        10 => {
            pal[4] = 0;
            pal[5] = 4;
            pal[6] = 6;
            pal[7] = 8;
        }
        13 | 256 => pal = PALETTE_256,
        _ => pal[..16].copy_from_slice(&PALETTE_256[..16]),
    }
    let mask = (1u32 << bpp) - 1;
    // A text image (mode 0) is searched over 16 colours.
    let colours = if mode == 0 {
        16
    } else {
        usize::try_from(mask).expect("small") + 1
    };
    Some(Image {
        pal,
        colours,
        mask,
        bgra: false,
    })
}

/// `matchcol`: the first palette entry nearest to the colour by the sum of the differences of its components.
fn matchcol(im: &Image, r: i32, g: i32, b: i32) -> u32 {
    let mut best = 0;
    let mut v = 1000;
    for (n, &c) in im.pal[..im.colours].iter().enumerate() {
        let c = i32::try_from(c & 0xFF_FFFF).expect("24 bits");
        let v2 = (b - (c & 0xFF)).abs() + (g - (c >> 8 & 0xFF)).abs() + (r - (c >> 16 & 0xFF)).abs();
        if v2 < v {
            let n = u32::try_from(n).expect("at most 256");
            if v2 == 0 {
                return n;
            }
            v = v2;
            best = n;
        }
    }
    best
}

/// An argument's unsigned slot (`args(i).ui`) as a C++ `int32_t` parameter takes it (the low 32 bits).
fn int32(v: &Value) -> R<i32> {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the C++ conversion of a 64-bit value to `int32_t`"
    )]
    let low = as_u64(v)? as u32;
    Ok(low.cast_signed())
}

fn clamp(n: i32) -> u32 {
    n.clamp(0, 255).cast_unsigned()
}

fn rgb32(r: i32, g: i32, b: i32, a: Option<i32>) -> u32 {
    let a = a.map_or(0xFF, clamp);
    (a << 24) | (clamp(r) << 16) | (clamp(g) << 8) | clamp(b)
}

/// The mode argument, checked as `EvaluateFunction$` checks it (measured, `v45_x17`, `x18`: "Invalid Screen Mode").
fn mode_image(mode: u64) -> R<Image> {
    image(mode).map_or_else(
        || error(&format!("screen mode {mode} in a colour function of a `CONST`")),
        Ok,
    )
}

fn inside(name: &str) -> R<Value> {
    error(&format!(
        "`{name}` of a colour outside the mode's palette is an error of the old compiler's own run time, which stops it"
    ))
}

pub fn function(name: &str, args: &[Value]) -> R<Value> {
    if args.iter().any(|a| matches!(a, Value::Str(_))) {
        return unsupported(&format!("a string argument to `{name}` in a `CONST`"));
    }
    let n = |v: u32| Ok(Value::Int(i64::from(v)));
    match name {
        "_RGB32" => {
            let a: Vec<i32> = args.iter().map(int32).collect::<R<_>>()?;
            match a[..] {
                [i] => n(rgb32(i, i, i, None)),
                [i, al] => n(rgb32(i, i, i, Some(al))),
                [r, g, b] => n(rgb32(r, g, b, None)),
                [r, g, b, al] => n(rgb32(r, g, b, Some(al))),
                // Measured, `v45_x14`, `x15`.
                _ => error("wrong number of arguments to `_RGB32` in a `CONST`"),
            }
        }
        "_RGBA32" => n(rgb32(
            int32(&args[0])?,
            int32(&args[1])?,
            int32(&args[2])?,
            Some(int32(&args[3])?),
        )),
        "_RED32" | "_GREEN32" | "_BLUE32" | "_ALPHA32" => {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "the C++ conversion to a `uint32_t` parameter"
            )]
            let col = as_i64(&args[0])?.cast_unsigned() as u32;
            n(match name {
                "_RED32" => col >> 16 & 0xFF,
                "_GREEN32" => col >> 8 & 0xFF,
                "_BLUE32" => col & 0xFF,
                _ => col >> 24,
            })
        }
        "_RGB" | "_RGBA" => {
            let mode_arg = if name == "_RGB" { &args[3] } else { &args[4] };
            let im = mode_image(as_u64(mode_arg)?)?;
            let c = |i: usize| -> R<i32> { Ok(int32(&args[i])?.clamp(0, 255)) };
            let (r, g, b) = (c(0)?, c(1)?, c(2)?);
            let a = if name == "_RGBA" { c(3)? } else { 255 };
            if im.bgra {
                let a = a.cast_unsigned();
                return n((a << 24) | (r.cast_unsigned() << 16) | (g.cast_unsigned() << 8) | b.cast_unsigned());
            }
            // A new image has no transparent colour, so `_RGBA` with alpha 0 matches as `_RGB` does.
            n(matchcol(&im, r, g, b))
        }
        "_RED" | "_GREEN" | "_BLUE" | "_ALPHA" => {
            let im = mode_image(as_i64(&args[1])?.cast_unsigned())?;
            #[expect(
                clippy::cast_possible_truncation,
                reason = "the C++ conversion to a `uint32_t` parameter"
            )]
            let col = as_i64(&args[0])?.cast_unsigned() as u32;
            if im.bgra {
                return n(match name {
                    "_RED" => col >> 16 & 0xFF,
                    "_GREEN" => col >> 8 & 0xFF,
                    "_BLUE" => col & 0xFF,
                    _ => col >> 24,
                });
            }
            if col > im.mask {
                return inside(name);
            }
            if name == "_ALPHA" {
                return n(255);
            }
            let Some(&p) = im.pal.get(usize::try_from(col).expect("32 bits")) else {
                // Beyond libqb's 256 entries (a text image's mask is 65535): libqb reads past its palette.
                return unsupported(&format!(
                    "`{name}` of a colour beyond 255 in screen mode 0 in a `CONST`"
                ));
            };
            n(match name {
                "_RED" => p >> 16 & 0xFF,
                "_GREEN" => p >> 8 & 0xFF,
                _ => p & 0xFF,
            })
        }
        _ => unreachable!("{name} is a colour function"),
    }
}
