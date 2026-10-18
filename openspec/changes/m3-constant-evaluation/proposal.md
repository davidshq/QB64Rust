# Proposal

## Why

The next step of M3 is the user's choice (user, 2026-10-17: "Do full constant evaluation"): the roadmap item
"Constant evaluation" (`ROADMAP.md` M3), which the upstream demand report names in eight programs, and in 12
constants of the auto-included file that a program still cannot use. Today `consteval.rs` computes floats in
`f64` and calls "not supported yet" whatever it cannot show equals the old `_FLOAT` result; the functions of the
old evaluator other than `CHR$` are all "not supported yet".

| Upstream program | Held back by |
|---|---|
| `const/const_float` | a `_FLOAT` constant that needs more than `DOUBLE` precision |
| `const/const_replace` | a constant with a type suffix inside a `CONST` |
| `const/rho_test` | a constant with a type suffix inside a `CONST` |
| `const/math_funcs` | every function of the evaluator, `_PI`, `ROOT`, `ASC` |
| `func_tostr/mmConst` | `_DOUBLE_MAX`, `_DOUBLE_MIN`, `_FLOAT_MAX`, `_FLOAT_MIN` |
| `auto_include/general` | `_E` (`EXP(1.0##)`) |
| `auto_include/color0`, `color32` | `$COLOR:0`, `$COLOR:32` |

Measured before this proposal (scratch programs, built and run with `qb64pe.exe`; task 1.1 records them as
`verification\v45_*` with the rest of the family):

- **The old evaluator's floats are x87 extended precision with a 64-bit mantissa**: `CONST third## = 1## / 3##`
  is `0.3333333333333333333` by `_TOSTR$`. Each value is printed with `%.19LG` (`__mingw_snprintf`) and read
  back with `__mingw_sscanf("%Lf")` (`qbs__tostr.cpp`, `qbs_val.cpp`), both correctly rounded, so `+ - * /` and
  the read-back are reproducible to the bit.
- **The functions of the evaluator are the old compiler's own run-time functions, inside `qb64pe.exe`**. Read
  from its C++ for a `_FLOAT` argument: `COS`, `SIN`, `TAN`, `ATN`, `_ACOS`, `_ASIN` are `std::` functions on
  `long double` (mingw's `cosl`, `sinl`, `tanl`, `atanl`, `acosl`, `asinl`: x87 `fcos`, `fsin`, `fptan`,
  `fpatan`); `EXP` is `func_exp_float` (`expl`, x87); `^` and `ROOT` are `pow2` (`powl`); `LOG`, `SQR` and the
  libqb functions `_ARCSEC` … `_COT` take and give `double` (`func_log`, `func_sqr`, `extended_math.h`) and call
  UCRT's `double` functions; `INT`, `_CEIL`, `FIX`, `ABS`, `_ROUND` are exact. **x87 transcendental instructions
  and UCRT's functions are not correctly rounded**, and their last bit can differ between CPUs and Windows
  versions, so the old compiler's own value of `CONST c = SIN(.2)` is not fixed.
- `_PI` is π in `_FLOAT` (`3.141592653589793239` as a `##` constant; a plain `CONST p = _PI` is DOUBLE by its
  text and prints `3.141592653589793`); `_PI(2)` is that times 2 in `_FLOAT` (corrected by task 1.2). `20 ROOT 3` is `2.714417616594907`. `_E` is
  `2.718281828459045235`.
- **The evaluator has an unsigned lane**: `CONST a = 18446744073709551615~&&: CONST b = a + 1` gives `b` 0. A
  constant used with a suffix inside a `CONST` is the constant (`CONST c% = 4 + 4: CONST d = c% * 2` is 16).
- **An unsuffixed integer literal beyond `_INTEGER64` is clamped** by `VAL(…, _INTEGER64)`: `CONST big~&& =
  18446744073709551615` is 9223372036854775807, and the auto-included `_UINTEGER64_MAX` and `_UOFFSET_MAX` are
  9223372036854775807, `_INTEGER64_MIN` and `_OFFSET_MIN` -9223372036854775807 (the minus applies after the
  clamp).
- `$COLOR:0` and `$COLOR:32` include QB64pe's `color0.bi` (17 constants, `Black~%%` …) or `color32.bi` (270
  constants, `AliceBlue~&` …) before the program, as the auto-included file is; both in one program are an error
  (`qb64pe.bas` 1734, 3538).

## What Changes

- **`_FLOAT` arithmetic in software**: an 80-bit float (64-bit mantissa) with correctly rounded `+ - * /` and
  square root, and the old evaluator's printing and reading of values, so every constant computed from
  literals, constants and these operators has the old compiler's value. The "not supported yet" for rounded
  values, for values near a rounding boundary and for `_FLOAT` constants beyond `DOUBLE` precision goes.
- **The three lanes of the old evaluator**: float, unsigned 64-bit and signed 64-bit values, with its rules for
  which lane an operator takes and its conversions between them.
- **Every function of the evaluator's list**: `_PI`, `ROOT`, `INT`, `FIX`, `_CEIL`, `_ROUND`, `ABS`, `SGN`, `ASC`,
  `_RGB32`, `_RGBA32`, `_RED32` … `_ALPHA32`, `_RGB`, `_RGBA`, `_RED` … `_ALPHA` (with the default palettes of each
  screen mode), `_D2R` … `_G2R` computed exactly. **The transcendental functions** (`SIN`, `COS`, `TAN`, `ATN`,
  `LOG`, `EXP`, `SQR`, `^` with a fraction, `ROOT`, `_ACOS`, `_ASIN`, `_ARCSEC`, `_ARCCSC`, `_ARCCOT`, `_SECH`,
  `_CSCH`, `_COTH`, `_SEC`, `_CSC`, `_COT`) are **certified** (user, 2026-10-17): computed with a high-precision
  reference: when every result the old compiler's library could give within its error bound leads to the same
  constant, that is the constant; otherwise the correctly rounded value is taken, with a warning (user,
  2026-10-18, after the sample showed the libraries' errors: `DIVERGENCES.md` D-026; "not supported yet" until
  then).
- **A constant used with a type suffix inside a `CONST`** is the constant, as the old evaluator reads it.
- **The 12 auto-included constants not computed yet** compile, with the old compiler's values (the clamped
  limits included, kept QB64pe's way and put to the user at the close).
- **`$COLOR:0` and `$COLOR:32`**: the constants of QB64pe's `color0.bi` and `color32.bi`, from copies of the files.
- **New dependency**: `astro-float-num` in `sema`, for the high-precision reference (user, 2026-10-17), guarded by a
  second precision and by tests against values written into the tests.
- **Tests**: verification family `v45`; slice programs; front-end tests; the eight upstream programs added to
  `tests\upstream\pass.list` (those whose constants certify).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/constants`: the values of `CONST` (the `_FLOAT` arithmetic, the lanes, the functions, certification),
  suffixed constants inside a `CONST`, the auto-included constants, and `$COLOR`.
- `compiler/pipeline`: the scenarios of "No follow-on errors after an unsupported declaration" use `CONST c =
  EXP(1)` as their example of an unsupported declaration; it compiles now, so they use another.

## Impact

- Changed: `crates\sema` (`consteval.rs` rewritten over a new `f80.rs` and `certify.rs`; `check\constants.rs`, the
  walk, the functions and suffixed references; `check\auto.rs`, the general walk in place of its three shapes;
  `literal.rs` where a constant becomes a literal of `_FLOAT`), `crates\syntax` and `crates\driver` (`$COLOR`, the
  colour files parsed before the program), `crates\builtins` (copies of `color0.bi` and `color32.bi` and the
  palettes), `tools\builtins\extract_builtins.py` (copies the colour files), `crates\codegen-cpp` only if a
  `_FLOAT` constant's text needs the `L` form it does not write yet.
- New dependency `astro-float-num` (`sema`); `deny.toml` or its equivalent checked; a `DECISIONS.md` row.
- No change to libqb, the Makefile or the reference clone.
- `DIVERGENCES.md`: none planned; a row only where the old compiler cannot build a program at all (a math error
  inside `qb64pe.exe`), decided at the measurement.
- Not in this change: `_CAST`, `_CONTINUE`, `REDIM` without bounds and the other lines of the demand report;
  `$CHECKING`.
- `STATUS.md`, `ROADMAP.md`, `SOMEDAY.md`, `study\00` §5, `verification\README.md`, `DECISIONS.md` updated.
