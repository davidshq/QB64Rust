# Spec Delta

## Purpose

Named constants (`CONST`) and the declaration rule `OPTION _EXPLICIT`: where a constant is visible, what type and
value it has (the old compiler evaluates `CONST` with its own evaluator), and when an implicit variable is an
error.

## ADDED Requirements

### Requirement: CONST visibility
`CONST name = expr` SHALL define a constant visible from its line to the end of its body: the main module's
constants SHALL also be visible in procedures that come later in the file; a procedure's constants only in that
procedure. A use of the name before the `CONST` line SHALL mean what it would mean without the constant.

#### Scenario: Constant used in a later SUB
- **WHEN** the main module has `CONST limit = 3` and a SUB defined after it prints `limit`
- **THEN** the SUB prints ` 3 `

### Requirement: CONST values
A constant's value SHALL be computed when compiling, as the old compiler's constant evaluator does: integer
arithmetic in 64 bits, `/` in floating point, `^` right-associative, comparisons giving -1 or 0, string `+` on
string constants. A suffix on the name SHALL convert the value to that type (rounding half to even). An expression
the evaluator cannot compute SHALL be a compile error; a function in the expression SHALL be "not supported yet".

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
After `OPTION _EXPLICIT`, a variable used without a declaration (`DIM`, `SHARED`, `STATIC`, a parameter, or a
FUNCTION's own result) SHALL be a compile error. `OPTION _EXPLICITARRAY` SHALL leave implicit scalars allowed.
Where the `OPTION` line may stand SHALL follow the old compiler.

#### Scenario: Undeclared variable
- **WHEN** a program starts with `OPTION _EXPLICIT` and then has `x = 1` without `DIM x`
- **THEN** it is a compile error naming `x`

#### Scenario: Declared variable and constant
- **WHEN** a program starts with `OPTION _EXPLICIT`, then `DIM x AS LONG`, `CONST k = 2`, `x = k`
- **THEN** it compiles
