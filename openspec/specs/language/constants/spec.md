# language/constants Specification

## Purpose
Named constants (`CONST`) and the declaration rule `OPTION _EXPLICIT`: where a constant is visible, what type and
value it has (the old compiler evaluates `CONST` with its own evaluator), and when an implicit variable is an
error.

## Requirements

### Requirement: CONST visibility
`CONST name = expr` SHALL define a constant when compiling, whatever control flow surrounds its line, visible
from its line to the end of its body: the main module's constants SHALL also be visible in procedures that come
later in the file; a procedure's constants only in that procedure, where they MAY reuse the name of a main constant
or variable. A use of a main constant's name before its `CONST` line, in the main module or in an earlier
procedure, with or without a type suffix, SHALL be a compile error, and so SHALL a `DIM` of a constant's name and a second `CONST` of the name
with another value. A constant used with a numeric suffix SHALL be the constant; a numeric constant used with `$`
SHALL be a compile error.

#### Scenario: Constant used in a later SUB
- **WHEN** the main module has `CONST limit = 3` and a SUB defined after it prints `limit`
- **THEN** the SUB prints ` 3 `

#### Scenario: Name used before its CONST line
- **WHEN** a program has `PRINT c1` and then `CONST c1 = 5`
- **THEN** it is a compile error at the `PRINT`

#### Scenario: Procedure constant shadows a main constant
- **WHEN** main has `CONST k = 1`, a later SUB has `CONST k = 2` and prints `k`, and main prints `k` after calling it
- **THEN** the SUB prints ` 2 ` and main ` 1 `

#### Scenario: CONST inside a skipped block
- **WHEN** `IF 0 THEN` … `CONST skipped = 8` … `END IF: PRINT skipped` runs
- **THEN** it prints ` 8 `

### Requirement: CONST values
A constant's value SHALL be computed when compiling, as the old compiler's constant evaluator does: integer
arithmetic in 64 bits, `/` in floating point, `^` right-associative, comparisons giving -1 or 0, string `+` on
string constants. An integer result, and a floating-point result with an integer value within `_INTEGER64` range,
SHALL be typed `_INTEGER64`; any other floating-point result DOUBLE. A suffix on the name SHALL convert the value
to that type (rounding half to even). An expression the old evaluator rejects (a variable, a string mixed with a
number, a string comparison, a function outside its list, integer division or `MOD` by 0, a suffix of the wrong
kind) SHALL be a compile error; a function from its list, a `/` by 0, a power of two integers outside
`_INTEGER64` range, and a floating-point value the compiler cannot show to equal the old evaluator's `_FLOAT`
result SHALL be "not supported yet". A floating-point result outside `_INTEGER64` range SHALL be DOUBLE.

#### Scenario: Float beyond _INTEGER64
- **WHEN** `CONST a = 1E+19 / 1: PRINT a` runs
- **THEN** it prints ` 1D+19 `

#### Scenario: Integer-valued constant is 64-bit
- **WHEN** `CONST i3 = 3, f2 = 4 / 2: PRINT i3 * 1000000000; f2 * 4611686018427387904` runs
- **THEN** it prints ` 3000000000 -9223372036854775808 ` (`3 * 1000000000` with the literal wraps at 32 bits)

#### Scenario: Right-associative power
- **WHEN** `CONST c = 2 ^ 3 ^ 2: PRINT c; 2 ^ 3 ^ 2` runs
- **THEN** it prints ` 512  64 ` (the run-time `^` is left-associative)

#### Scenario: Suffix rounds
- **WHEN** `CONST c% = 3.7: PRINT c%` runs
- **THEN** it prints ` 4 `

#### Scenario: Assignment to a constant
- **WHEN** a program has `CONST c = 1` and later `c = 2`
- **THEN** it is a compile error

### Requirement: OPTION _EXPLICIT
`OPTION _EXPLICIT` SHALL apply to the whole program, wherever it stands (also inside a block or a procedure, and
after the variables it concerns): a variable used without a declaration (`DIM`, `DIM SHARED`, `SHARED` of a
declared main-module variable, `STATIC`, `CONST`, a parameter, or a FUNCTION's own result) in any body SHALL be a
compile error. A `FOR` variable is not a declaration. `OPTION _EXPLICITARRAY` SHALL leave implicit scalars
allowed. `OPTION EXPLICIT` without the underscore SHALL be a compile error.

#### Scenario: OPTION inside a SUB
- **WHEN** the main module has `y = 1` and a SUB defined later holds `OPTION _EXPLICIT`
- **THEN** it is a compile error naming `y`

#### Scenario: Undeclared variable
- **WHEN** a program starts with `OPTION _EXPLICIT` and then has `x = 1` without `DIM x`
- **THEN** it is a compile error naming `x`

#### Scenario: SHARED before the main module's DIM
- **WHEN** a program has `OPTION _EXPLICIT`, a SUB with `SHARED w AS LONG`, and after the SUB `DIM w AS LONG`
- **THEN** it is a compile error naming `w` at the `SHARED`

#### Scenario: Declared variable and constant
- **WHEN** a program starts with `OPTION _EXPLICIT`, then `DIM x AS LONG`, `CONST k = 2`, `x = k`
- **THEN** it compiles
