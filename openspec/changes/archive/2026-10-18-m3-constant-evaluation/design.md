# Design

## Context

Motivation, scope and the first measurements: `proposal.md` (scratch programs of 2026-10-17, recorded as
`verification\v45_*` in task 1.1). Where this compiler stands:

- **`sema` `consteval.rs`** holds the arithmetic of the old evaluator on `Value::{Int(i64), Float, Str}`. A float is
  an `f64` with an `exact` flag; any operation on an inexact float, any value near a rounding boundary and any
  `_FLOAT` constant beyond `DOUBLE` precision is "not supported yet". `chr` is the only function.
- **`sema` `check\constants.rs`** walks the tree of a `CONST` value (`const_value`, `const_binary`); a
  parenthesised group and the result are read back (`consteval::reread`) as the old evaluator re-reads its text.
  A suffixed constant inside a `CONST` is "not supported yet"; the evaluator's other functions are listed in
  `FUNCTIONS` and "not supported yet". `const_use` turns a use into a literal node (`literal_expr`).
- **`sema` `check\auto.rs`** reads the expressions of `beforefirstline.bi` (from `qb64rust_builtins::AUTO_CONSTANTS`)
  in three fixed shapes, with its own small parser.
- **`$COLOR`** is "not supported yet" (a metacommand the syntax crate marks).
- **The old evaluator** (`..\QB64pe\source\utilities\const_eval.bas`, `elements.bas`; `qb64pe.bas` 2600–2720,
  24030–24090): each value is a `ParseNum` with `f AS _FLOAT`, `i AS _INTEGER64`, `ui AS _UNSIGNED _INTEGER64` and a
  type; an operator takes the float lane if either operand is a float, else the unsigned lane if either is
  unsigned, else the signed lane; `/` is always float. Each parenthesised group and each function result is
  printed (`_TOSTR$`, `%.19LG`) and read back (`elementGetNumericValue`: `VAL`, typed by the text: `F` or `##` a
  `_FLOAT`, `E` or `!` a SINGLE, `D`, `#` or `.` a DOUBLE, `~&&` unsigned, `&&` signed, plain digits `VAL(…,
  _INTEGER64)`). The functions run inside `qb64pe.exe` (`EvaluateFunction$`), on the libraries it was built with.

## Goals / Non-Goals

**Goals:** every `CONST` the old evaluator computes, with its value, type and errors; where its value depends on the
machine, the value certified or "not supported yet" (user, 2026-10-17); the remaining auto-included constants;
`$COLOR`.

**Non-Goals:** a correctly rounded result where the old compiler's is not (that would be a divergence nobody
asked for); the run-time functions of programs (`SIN(x)` in a statement is unchanged); `_CAST` and the other
lines of the demand report.

## Decisions

### D1. A software `_FLOAT`

New module `crates\sema\src\f80.rs`: `F80 { neg: bool, exp: i32, mant: u64 }` (the x87 extended format: an explicit
integer bit, a 15-bit exponent range, subnormals, ±0, infinities, NaN). Operations, each rounded to nearest, ties
to even, at 64 bits, as the x87 does with its precision control at 64 bits (measured, `1## / 3##`): `add`, `sub`,
`mul`, `div`, `sqrt`, `neg`, comparisons; exact conversion from `i64`, `u64` and `f64`; conversion to `f64` rounded
to nearest even (a `(double)` cast). The work is done in `u128` (a 64 × 64 product, a 128 / 64 quotient with a
sticky bit). `f64` operations needed by D4 use Rust's `f64`, which is IEEE double with the same rounding as the
SSE2 code of `qb64pe.exe`.

Text, in the same module:
- `F80::to_g19(self) -> String`: `__mingw_snprintf("%.19LG")` (`qbs__tostr.cpp`): 19 significant digits,
  correctly rounded (mingw's printf is gdtoa's), `%G`'s choice of plain or exponent form (exponent below -4 or at
  least 19), trailing zeros and a trailing point dropped, an exponent of at least two digits with its sign; then
  `E` becomes `F` as `qbs__tostr` does.
- `F80::from_decimal(digits, exp10)`: `__mingw_sscanf("%Lf")` (gdtoa's `strtold`), correctly rounded, after
  `qbs_val`'s normalisation (leading zeros dropped, exponent digits after the 18th ignored).

Both are exact conversions between a decimal and a binary value. They use `astro-float-num` (D3) at a precision that
makes every intermediate exact (a 19-digit integer times 10^±4951 needs under 17,000 bits), then this module's own
rounding to 64 bits. Unit tests hold known pairs (`_FLOAT_MAX` and `_FLOAT_MIN_FRAC` from `v45`, the boundaries
of each decade, halfway cases), and a round trip `from_decimal(to_g19(x))` for a sweep of `x`.

*Alternative considered*: keep `f64` with the `exact` flag and widen its margins. Rejected: a `_FLOAT` constant has
64 bits the `f64` cannot hold (`const/const_float`, `_FLOAT_MAX`), and every rounded value stays "not supported
yet".

### D2. Values, lanes and types

`consteval::Value` becomes `Int(i64)`, `UInt(u64)`, `Float(Range)`, `Str(Vec<u8>)`, where `Range { lo: F80, hi:
F80 }` is the set of values the old compiler might hold (`lo == hi` when it is certain; D4). Each value also
carries the type the old evaluator gives it (`ValTy`: signed, unsigned, SINGLE, DOUBLE, `_FLOAT`), which decides
the lane and, at the end, the constant's type.

Each operator follows `const_eval.bas` (functions named there): `+ - *` and comparisons in `Term`, `Factor`,
`Relation` by the three lanes; `/` always float; `\`, `MOD`, `AND` … `IMP` and `NOT` by the unsigned or signed
lane; `^` by the three lanes (`Exponent`); unary minus float or signed (an unsigned operand goes through the signed
lane). A value moves between lanes as `ParseNumSetF`, `ParseNumSetI` and `ParseNumSetUI` move it: from a float to
an integer as `qb64pe.exe`'s C++ converts a `_FLOAT` to `_INTEGER64` and to `_UNSIGNED _INTEGER64`, which task 1.2
measures (`study\00` §5 records the path of a statement: `qbr`, half to even); from an integer to a float
exactly; between the two integer lanes by the bits. A comparison of floats is a float -1 or 0, of integers
an integer.

The re-read (`consteval::reread`, at each parenthesised group, each function result and the end) prints the value
as the old evaluator does (`_TOSTR$(f)`; an integer as `n&&` or `n~&&`) and reads it back with
`elementGetNumericValue`'s typing. So a float result whose text has an exponent becomes a `_FLOAT`, one without a
DOUBLE (measured: `1E+30 / 3` is `_FLOAT`, `1 / 3` DOUBLE), and an integer-valued float below 10^19 prints as
plain digits (`2`), which are read back as an `_INTEGER64` (today's rule, now following from the text). A literal of the source is typed and read the same way, which gives the clamp of an unsuffixed
literal beyond `_INTEGER64` (`VAL(…, _INTEGER64)`, measured: `18446744073709551615` is 9223372036854775807).

`settle` and `convert` (the constant's type from its suffix, the type of a use) keep their rules; their float
side works on the certain `F80` (a `Range` that is not certain at the end is "not supported yet", D4). The
constant's type without a suffix is the value's `ValTy` (a float `_FLOAT` or DOUBLE by its text, an unsigned
`_UNSIGNED _INTEGER64`), where today it is `_INTEGER64` or DOUBLE.

`check\constants.rs` `literal_expr` writes a `_FLOAT` constant as its 19-digit text with the `L` suffix, as the old
compiler writes it (`qbs__tostr((long double)( 3.333333333333333333E+29L ))`), and a DOUBLE as its 19-digit text
without it (`0.3333333333333333333E+0`); `codegen-cpp` writes `ExprKind::Float` as it is given. `ir` and
`codegen-cpp` change only if a `_FLOAT` literal's text lacks a way to carry the `L` (checked in task 2.3 with a
`cpp` snapshot).

### D3. The high-precision reference: `astro-float-num` with two guards

`astro-float-num` (pure Rust, MIT; the number crate of `astro-float`, taken without its proc-macro; user, 2026-10-17) is added to `sema` only, for the reference values of D4 and the
exact conversions of D1. `deny.toml` gets its licence if it is not allowed already; `DECISIONS.md` a row (the first
dependency of the front end that is neither protocol nor test infrastructure; the policy row of 2026-10-07 is
widened, not replaced).

The guards, in `crates\sema\src\certify.rs` (`reference`):
1. Each reference value is computed at 192 and at 320 bits; when the two differ by more than 2^-180 relative, the
   value is "not supported yet" (a fault of the crate at one precision cannot pass).
2. Unit tests compare the reference of each function at a set of arguments with values written into the tests,
   computed once with Python's `mpmath` at 50 digits (`tools\builtins\README.md` names the script; `mpmath` is
   needed only to regenerate them).
3. Task 3.4 checks every recorded constant of `v45`: certified and equal to the old compiler's, or "not supported
   yet"; never different.

*Alternatives considered*: our own big float (user's choice was the crate); `rug` (MPFR: proven, but a C build and
LGPL).

### D4. Certification of the functions

`certify.rs` writes each function of the evaluator as the expression its library computes, read from libqb's
source and from the disassembly of the toolchain's libraries (`llvm-objdump` of `libmingwex.a`; the double
functions are UCRT's, `libucrt.a` imports). In that expression the exact steps are exact (D1 for `long double`,
`f64` for `double`), and each step that is not correctly rounded is a **fuzzy step**: the reference value of the
true result, widened by the step's error bound: the values of its format within that bound, and the correctly
rounded value (`mid`). A true value the format holds is returned as it is (`exp(0)`). The steps and their bounds,
set from the sample (task 1.3; the smallest error each result needs, widened by half, except where the sample shows
a correctly rounded library):

| Library | Step | Seen in the sample | Bound |
|---|---|---|---|
| x87 (mingwex) | **`sinl`**, **`cosl`**, **`tanl`** (`fsin`, `fcos`, `fptan`) | 0.48–0.55 ulp for \|x\| < pi/4; beyond, at most 0.023 \|x\| 2^-64 more (the internal pi) | 1 ulp (Intel's documented accuracy) and \|x\| 2^-66 |
| x87 | **`atanl`** (`fpatan`); `acosl`, `asinl` as `1 - x*x`, `fsqrt`, `fpatan` | 0.57 ulp | 1 ulp |
| x87 | **`expl`** | 0.98 ulp | 1.5 ulp |
| x87 | **`powl`** (a whole exponent with an exact power: that power) | 6.6 ulp | 10 ulp |
| UCRT | **`log`** | 0.4993 ulp | 0.5 ulp + 1/512 |
| UCRT | **`exp`**, **`tan`**, **`acos`**, **`sin`**, **`asin`**, **`cos`**, **`atan`** (inside `func_arcsec` … `func_cot`) | 0.51, 0.58, 0.58, 0.60, 0.61, 0.62, 0.72 ulp | 0.76, 0.87, 0.88, 0.90, 0.92, 0.93, 1.08 ulp |

An argument beyond 2^63 for `fsin`, `fcos`, `fptan` (mingw then reduces with `fprem1` and a 64-bit pi, which no
bound covers) is "not supported yet". A `Range` goes through exact steps by its ends (each step is monotone, or the
four corners for `*` and `/`) and its `mid` by the same step. At each re-read the two ends are printed: if their
texts are equal the `Range` becomes that one value, which is then the old compiler's on any machine. **Where they
are not (and where a comparison, a conversion to an integer or a sign differs across the `Range`), the outcome of
`mid` is taken** (the user, 2026-10-18, `DIVERGENCES.md` D-026): the old compiler's own value then depends on the
CPU and the system's library. `consteval::fell_back` notes it, and the `CONST` check warns (`-w`) that QB64pe's
value depends on the CPU and the system's library and may differ. With these bounds the share of the sample whose result certifies is: `LOG`,
`SQR` all; `_ACOS`, `ATN` 78–86 %; `_ASIN`, `TAN` 60–65 %; `EXP`, `_SECH`, `_CSCH`, `_COTH` 43–52 %; the UCRT
helpers 8–38 %; `SIN`, `COS` (\|x\| up to 8) 4–5 %; `^` none.

The functions whose result is exact need no reference: `INT`, `FIX`, `_CEIL`, `_ROUND`, `ABS`, `SGN` (then
converted to `_INTEGER64` as `EvaluateFunction$` does), `SQR` (`func_sqr` is a correctly rounded `double` square
root), `_PI` (π rounded to `_FLOAT`, times the argument in `_FLOAT`; measured `3.141592653589793239`), `_D2R` … `_G2R` (a `double` literal times the
argument in `_FLOAT`), `ASC`, `CHR$`, and the colour functions (D5). Their errors are the evaluator's (`SQR` of a
negative, `_ARCSEC` and `_ARCCSC` below 1 in size, a wrong argument count, a string where a number is wanted) and,
where the old compiler raises a run-time error inside `qb64pe.exe` (`LOG(0)`, `_SECH(100)`, `_ACOS(2)` giving a
NaN), what task 1.2 measures: a compile error here, with a `DIVERGENCES.md` row only if the old compiler builds a
program from it.

Validation (task 1.3): `verification\v45_sample.bas`, built with `qb64pe.exe`, calls each of the 19 functions
that are not exact at run time with a `_FLOAT` argument, which compiles to the C++ `qb64pe.exe`'s evaluator runs
(read from its C++: `std::sin`, `func_log`, `func_arcsec`, `pow2`, …), at 500 exact binary arguments each, and
prints the 10 bytes of each result (270 KB; whole functions rather than library steps, so the test checks the
whole chain of each function's model, and 500 rather than 10,000 arguments to keep the file small).
`certify.rs`'s test reproduces the arguments and checks that every recorded result lies in the `Range` its model
gives. A model that a result breaks gets its bound widened to cover the sample, then by half again.

### D5. The colour functions

`_RGB32`, `_RGBA32`, `_RED32` … `_ALPHA32` are bit arithmetic on the unsigned or signed lane as `EvaluateFunction$`
passes them (`args(i).ui`, `args(i).i`). `_RGB`, `_RGBA`, `_RED` … `_ALPHA` take a screen mode (0 to 2, 7 to 13,
256, 32, else the evaluator's error) and work on a new 1 × 1 image of that mode: for a palette mode the nearest
colour of the mode's default palette by libqb's `matchcol` (and `_RED` … of a palette index the palette's
component), for mode 32 the 32-bit value. The default palettes are copied from libqb by
`tools\builtins\extract_builtins.py` into `crates\builtins\src\palettes.rs` (QB64pe's MIT licence, as
`afterlastline.bm`); `matchcol` is rewritten in `consteval.rs` from libqb's source. Checked by the `v45` colour
programs and `const/math_funcs`.

### D6. A suffixed constant inside a CONST

`ParseNumHashLookup&` strips the suffix (`tryRemoveSymbol$`) and finds the constant by its name: `check\constants.rs`
`const_value` does the same, with the constant's own value and type. What a suffix of the wrong kind does (`c$` of
a number, `s%` of a string) is measured in task 1.2 and written as found (an error or the constant).

### D7. The auto-included constants through the evaluator's paths

`check\auto.rs` keeps its reader of the file's three shapes (one term; two terms with `+`, `*` or `/`; a term an
operand or a call of one operand), which covers every expression of QB64pe 4.7.0's file, but its literals and calls
take the paths of a program's `CONST`: `consteval::literal` (the clamp of `_INTEGER64_MIN`, `_UINTEGER64_MAX`; the
`_FLOAT` limits), `consteval::function` (`EXP(1.0##)` for `_E`), `consteval::lookup` for a constant of the file
used in another. (Parsing the text with `qb64rust_syntax` was the plan; it would need a source file of its own in
the source map for spans, for no gain on this file.) The 12 constants not computed before follow from D1, D2 and
D4 (`_E` is `expl`, measured `2.718281828459045235`; where it does not certify, D-026). A shape the reader does
not know stays "not supported yet" where it is used.

### D8. `$COLOR`

`tools\builtins\extract_builtins.py` copies `internal\support\color\color0.bi` and `color32.bi` byte for byte into
`crates\builtins\src\` (as `afterlastline.bm`, design D7 of `m3-network-embed`). `syntax` reads `$COLOR:0` and
`$COLOR:32` (any other `$COLOR` form: the old compiler's error, measured in task 1.2). `driver` `program.rs`
`parse_round`, when the main file or an included file has one, parses the colour file before the main file as an
included tree named `internal\support\color\color0.bi` (the old compiler's `autoIncludeBuffer`, `qb64pe.bas`
1734), so its `CONST` lines are main-module constants and a program name they take is "name already in use" by the
existing rules. Both metacommands in one program: the old compiler's error at the second. Where the metacommand may
stand (inside a SUB, after the names are used, in an included file) is measured in task 1.2.

### D9. Tests

- `verification\v45_*` (task 1): the lanes, typing by text, the clamp, each function at chosen arguments with its
  19-digit result, the errors, suffixed references, `$COLOR` placement and errors; and the sample of D4.
- Slice programs, recorded with `qb64pe.exe`: `s142_const_float` (the lanes, `_FLOAT` constants and their text),
  `s143_const_funcs` (exact functions, colour functions, certified functions whose value certifies on the
  recording machine), `s144_color0`, `s145_color32`.
- Front end: `const_funcs_errors` (`check-fail`), `const_lanes_typed`, `const_float_cpp` (the `L` text),
  `color_errors`.
- Unit tests: `f80.rs` (operations against `f64` where both are exact, halfway cases, subnormals, text both ways),
  `certify.rs` (the reference against the `mpmath` values; the sample of D4).
- Upstream: the eight programs of the proposal added to `tests\upstream\pass.list` when they pass; one whose
  constants do not certify stays out with its reason in `tests\known_clean_not_passing.list`.

### D10. What task 1.2 settled (`study\00` §5, `v45`)

- **Float to integer lane**: `qbr` (`rounding.h`): half to even; above 2^63, less 2^63 with the top bit set; out of
  range `i64::MIN`. Both `num.i = f` and `num.ui = f` take it (`1E+19 OR 0~&&` is 10^19, `-1.5 OR 0~&&` 2^64 - 2).
  `consteval::qbr`.
- **An integer `^`** is the float power converted by `qbr` (`2 ^ -1` 0, `0 ^ -1` `i64::MIN`); beyond range D-007
  stays (a float).
- **`INF` and `NaN`**: a value printed as `INF` or `NAN` and read back inside an expression (a group, a function
  argument) is the error "Unexpected element 'INF'"; as the whole value it is read by `VAL(…, _INTEGER64)`, 0
  (`x25`, `x38`). `consteval::reread` has a flag for the outermost read.
- **An error inside `qb64pe.exe`** ("UNEXPECTED INTERNAL COMPILER ERROR!", nothing built: `LOG(0)`, `_COT(0)`,
  `EXP(1000)`, `ASC("ab", 5)`, `(-8) ^ 0.5`, a palette index outside the mode's palette) is a compile error here
  naming the function and its argument. No `DIVERGENCES.md` row: the old compiler builds nothing.
- **`$COLOR`** counts wherever it stands (a SUB, an included file, after the names are used, twice, lower case),
  not in a `$IF` branch not taken; a `CONST` above the first `$COLOR` line in file order that names a colour
  constant is "Unexpected element" (the first pass meets it before the file is included, `x59`); errors as
  measured (`x45`–`x56`). `check\constants.rs` knows the offset of the first `$COLOR` line.
- **The colour functions** fail inside `qb64pe.exe` for an index outside the mode's palette, except mode 0, which
  gives 0 (and `_ALPHA` 255) for any index (`x60`, `x64`, `x65`).

## Risks / Trade-offs

- **The bounds of D4 are an assumption for UCRT**, whose accuracy is not documented, and for `expl` and `powl`,
  whose bounds come from their code and a sample. A Windows or CPU whose library is less accurate than the bound
  could make the old compiler's value differ from a certified one. Mitigation: the sample on this machine; bounds
  widened by half beyond what it shows; the recordings checked in task 3.4. What is left is a constant whose last
  bit differs from an old compiler run on another machine, the same difference two runs of the old compiler can
  show there.
- **`astro-float-num` is a new dependency** of the front end, unmaintained since its last release. Mitigation: D3's
  guards; it is used only for reference values and conversions, so a fault shows as "not supported yet" or as a
  failing unit test, not as a wrong constant.
- **The type of a float constant changes** for a result printed with an exponent: DOUBLE today, `_FLOAT` after
  this change (as the old compiler has it). The C++ of such programs changes; their output does not, except where
  the extra precision shows (`_TOSTR$`, and arithmetic with the constant). `DIVERGENCES.md` D-007's text ("`2 ^ 70`
  is a DOUBLE") is corrected to "a float, `_FLOAT` by its text".
- **Size**: `f80.rs` and `certify.rs` are new code of some hundreds of lines each, with tests.

## Migration Plan

None: a front-end change. `tests\known_unsupported_rejections.list` and the demand reports shrink.

## Open Questions

- **The clamped limits** (`_INTEGER64_MIN` -9223372036854775807, `_UINTEGER64_MAX` and `_UOFFSET_MAX`
  9223372036854775807, and `CONST big~&& = 18446744073709551615`): built QB64pe's way, recorded in `SOMEDAY.md`'s
  "QB64pe behaviours to review" and put to the user at the close, with whatever else task 1.2 finds.
