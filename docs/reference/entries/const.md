# CONST

`CONST` gives a name to a value that is computed when the program is compiled and never changes:
`CONST MAXARMIES = 12`.

## Syntax

```text
CONST name = expression [, name = expression]...
```

## Arguments

- `name`: the constant's name, with or without a type suffix.
- `expression`: numbers, strings, other constants, operators, and the functions the compiler can compute, such as
  `ABS`, `INT` and `CHR$`.

## Behavior

The value is computed when compiling. It is defined whatever control flow surrounds its line, even inside an
`IF 0 THEN` block.
<!-- src: language/constants "CONST visibility" -->

A constant of the main module is visible from its line to the end of the main module, and in every procedure that
comes later in the file. A constant of a procedure is visible only in it, and may reuse the name of a constant or
variable of the main module.
<!-- src: language/constants "CONST visibility"; tests\reference\examples\const_1 -->

Integer arithmetic in a `CONST` is done in 64 bits, and a whole-number result is an `_INTEGER64`: with
`CONST BIG = 3`, `BIG * 1000000000` is 3000000000, where `3 * 1000000000` wraps at 32 bits.
<!-- src: language/constants "CONST values"; tests\reference\examples\const_1 -->

`/` gives a floating-point value, and so does `^`: `CONST HALF = 2 ^ -1` is 0.5. `^` is worked from right to left
in a `CONST`: `CONST c = 2 ^ 3 ^ 2` is 512, where the same expression in a statement is 64. A chain of `^` gets a
warning that says so.
<!-- src: language/constants "CONST values"; language/constants "Warning for a chained power in a CONST" -->

A string constant is a string literal or `+` of string constants. `CHR$(code)` gives the one-byte string.
<!-- src: language/constants "CONST values" -->

A floating-point suffix on the name converts the value as an assignment does: `CONST c% = 3.7` is 4. An integer
suffix makes the constant behave as a literal with that suffix: `CONST u~% = -1` prints 65535.
<!-- src: language/constants "CONST values" -->

A constant used with a numeric suffix is still the constant. A default type set by `DEFINT` or `_DEFINE` does not
type a constant: it takes the type of its value.
<!-- src: language/constants "CONST visibility"; language/default-types "What a default type gives its type to" -->

A label and a `CONST` may stand on one line: `lbl1: CONST k = 4`.
<!-- src: language/constants "CONST after a label" -->

## Errors

These are compile errors:

- A store into a constant: "cannot assign to the constant `c`".
  <!-- src: language/constants "CONST values" -->
- A use of the name above its `CONST` line: "name already in use: `k` (a `CONST` of this name follows on line 3)".
  <!-- src: language/constants "CONST visibility" -->
- A `DIM` of a constant's name, or a second `CONST` of it with another value: "name already in use: `k`".
  <!-- src: language/constants "CONST visibility" -->
- A variable in the expression: "`x` is a variable, not a constant".
  <!-- src: language/constants "CONST values"; language/constants "Malformed CONST expressions" -->
- A division by zero, as in `CONST c = 1 / 0`: "division by zero".
  <!-- src: language/constants "CONST values" -->
- A numeric constant used with `$`: "type mismatch".
  <!-- src: language/constants "CONST visibility" -->
- `_IIF`, `_MIN`, `_MAX`, `_CLAMP`, `_SHL`, `_SHR`, `_ROL`, `_ROR` and `_CAST` in a `CONST`.
  <!-- src: language/builtin-functions "Not in a CONST"; verification\v49_x11_cast_in_const -->

## Differences

From QB64pe:

- `^` of two integers in a `CONST` is rounded to an integer there: `CONST half = 2 ^ -1` is 0. QB64Rust gives 0.5,
  as a statement does (`DIVERGENCES-QB64PE.md` D-028).
  <!-- src: D-028 -->
- `CONST c = 1 / 0` is 0 there, and a power beyond the `_INTEGER64` range wraps. QB64Rust makes the first an error
  and the second a floating-point value (D-007).
  <!-- src: D-007 -->
- `lbl1: CONST k = 4` does not compile there (D-008).
  <!-- src: D-008 -->
- A `CONST` computed with `SIN`, `COS`, `TAN`, `ATN`, `EXP`, `LOG`, `ROOT` or `^` with a fraction takes its value
  there from the machine's math library, so its last digits can differ between machines. QB64Rust gives QB64pe's
  value when every value that library could give leads to one constant. Otherwise it takes the correctly rounded
  value, and `-w` warns: "this `CONST` is computed with correctly rounded math functions; QB64pe's value depends on
  the CPU and the system's library and may differ" (D-026).
  <!-- src: D-026 -->
- A value beyond `_INTEGER64` is clamped there: `CONST big~&& = 18446744073709551615` is 9223372036854775807, and so
  is `_UINTEGER64_MAX`. QB64Rust keeps the value as a statement reads it, and the limits hold their types' limits
  (D-027).
  <!-- src: D-027 -->
- A value beyond `_FLOAT`'s range, as `CONST a = 1F+4000 * 1F+4000`, is 0 there. QB64Rust gives the compile error
  "this `CONST` overflows: its value is beyond `_FLOAT`'s range" (D-029).
  <!-- src: D-029 -->
- `_RED`, `_GREEN` or `_BLUE` of a color above 255 in screen mode 0, as `CONST c = _RED(2222, 0)`, reads past the
  palette there, so its value changes from build to build. QB64Rust gives 0 (D-035).
  <!-- src: D-035 -->

From QuickBASIC 4.5: any `^` in a `CONST` is a syntax error there (`DIVERGENCES-QB45.md` Q-008).
<!-- src: Q-008 -->

Doubled signs in a `CONST`, as `CONST c = 2 * - -3`, are a compile error there too. QB64Rust, as QB64pe, gives 6
(`DIVERGENCES-QB45.md` Q-033).
<!-- src: Q-033 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:const_1 -->
```qb64
CONST MAXARMIES = 12, TITLE$ = "Campaign"
CONST HALF = MAXARMIES / 2
PRINT TITLE$; " allows"; MAXARMIES; "armies, half of them is"; HALF
CONST BIG = 3
PRINT "BIG * 1000000000 ="; BIG * 1000000000
PRINT "2 ^ 3 ^ 2 in a CONST and in a statement:";
CONST P = 2 ^ 3 ^ 2
PRINT P; 2 ^ 3 ^ 2
Report
END

SUB Report
    PRINT "The SUB sees"; MAXARMIES
END SUB
```

Output:

```text
Campaign allows 12 armies, half of them is 6 
BIG * 1000000000 = 3000000000 
2 ^ 3 ^ 2 in a CONST and in a statement: 512  64 
The SUB sees 12 

Press any key to continue
```
<!-- end generated -->

## See also

[`$COLOR`](meta-color.md), [`DIM`](dim.md), [`DEFINT`](defint.md).
