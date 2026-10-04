# Spec Delta

## Purpose

How SUB and FUNCTION procedures are defined and called, how arguments are passed, and which variables a statement
inside a procedure can see. Every rule is measured with `qb64pe.exe`; the scenarios are tests in
`tests\corpus\slice\` or `tests\frontend\`.

## ADDED Requirements

### Requirement: Procedure definitions
A program SHALL be able to define procedures with `SUB name [(params)]` … `END SUB` and
`FUNCTION name[suffix] [(params)]` … `END FUNCTION`, before or after main-module code. A parameter SHALL be
`name[suffix] [AS type]` with a scalar type. A FUNCTION's type SHALL be given by its suffix, SINGLE without one.
Procedures SHALL NOT be nested, and two procedures SHALL NOT share a name.

#### Scenario: SUB defined before the main code
- **WHEN** a program defines `SUB foo (a AS LONG, b AS LONG)` first and calls `CALL foo(1, 2)` after it
- **THEN** it prints ` foo: 1  2 ` (`PRINT "foo:"; a; b` in the SUB)

#### Scenario: Function called before its definition
- **WHEN** main code prints `twice&(3)` and `FUNCTION twice& (a AS LONG)` assigns `twice& = a * 2` further down
- **THEN** it prints ` 6 `

### Requirement: Calls
A SUB SHALL be called with `CALL name[(args)]` or `name [args]`. A FUNCTION SHALL be called in an expression,
with an argument list or, without parameters, by its name alone; inside the FUNCTION, assigning to its name (with
or without the suffix) SHALL set the result, and any other use of the name SHALL be a call. A call with the wrong
number of arguments, or a string for a number parameter or the reverse, SHALL be a compile error. `EXIT SUB` and
`EXIT FUNCTION` SHALL leave the procedure they are in, whichever kind it is, and SHALL be compile errors in the
main module.

#### Scenario: Function result printed with its own type
- **WHEN** `PRINT twice&(n)` is compiled
- **THEN** the value is formatted as a LONG (not as the `_INTEGER64` an integer operation is believed to be)

#### Scenario: EXIT FUNCTION before the result is set
- **WHEN** a FUNCTION executes `EXIT FUNCTION` before assigning its name
- **THEN** the call returns 0 (numeric) or an empty string

### Requirement: Argument passing
An argument that is a variable written plainly (not in parentheses) with exactly the parameter's type SHALL be
passed by reference: the procedure's assignments to the parameter change the variable. Any other argument SHALL be
converted as for an assignment to the parameter's type and passed as a temporary copy whose changes are lost. A
string variable passed to a string parameter SHALL be passed by reference.

#### Scenario: By reference
- **WHEN** `n` is a LONG holding 1 and `SUB bump (x AS LONG)` does `x = x + 1`, called as `CALL bump(n)` and as
  `bump n`
- **THEN** `n` is 3 afterwards

#### Scenario: Parentheses and expressions pass a copy
- **WHEN** the same SUB is called as `bump (n)` and as `bump n + 1`
- **THEN** `n` is unchanged

#### Scenario: Different type passes a rounded copy
- **WHEN** `2.5` is passed to an INTEGER parameter
- **THEN** the procedure sees 2 (half to even, as for an assignment)

### Requirement: Procedure scopes
Inside a procedure a name SHALL refer to, in this order: a parameter; the FUNCTION's result (when assigned); a
`STATIC` variable of the procedure; a variable named in a `SHARED` statement of the procedure (the main-module
variable of that name and type, created if the main module has none); a main-module variable declared with
`DIM SHARED` **earlier in the file** than the procedure; otherwise a local variable, declared with `DIM` or
implicitly, that is new on every call. A `STATIC` variable SHALL keep its value between calls. Main-module
variables SHALL NOT be visible in a procedure otherwise.

#### Scenario: Implicit local hides nothing
- **WHEN** the main module sets `x = 99` and a SUB sets `x = 5` without `SHARED`
- **THEN** the main module's `x` is still 99 after the call

#### Scenario: STATIC counter
- **WHEN** a SUB with `STATIC n AS LONG` and `n = n + 1: PRINT "count:"; n` is called three times
- **THEN** it prints `count: 1 `, `count: 2 `, `count: 3 `

#### Scenario: SHARED variable
- **WHEN** the main module sets a LONG `g` to 10 and a SUB with `SHARED g AS LONG` prints it and sets it to 20
- **THEN** the SUB prints 10 and the main module then prints 20

#### Scenario: DIM SHARED after the procedure
- **WHEN** a SUB that prints `g` comes before `DIM SHARED g AS LONG` and `g = 7` in the file
- **THEN** the SUB prints 0 (its `g` is a local)

### Requirement: DECLARE lines
`DECLARE SUB` and `DECLARE FUNCTION` lines SHALL be accepted when well formed and SHALL otherwise be ignored, as
by the old compiler: a `DECLARE` that disagrees with the definition, or names a procedure that does not exist, is
not an error; calls are checked against the definition.

#### Scenario: DECLARE before use
- **WHEN** a program starts with `DECLARE SUB show (v AS LONG)` and defines `SUB show (v AS LONG)` at the end
- **THEN** it compiles and runs as without the `DECLARE` line

#### Scenario: DECLARE that disagrees
- **WHEN** a program has `DECLARE SUB s (a AS LONG)`, calls `s "x"`, and defines `SUB s (a AS STRING)`
- **THEN** it compiles and prints `x`

### Requirement: Reserved names
A variable, parameter or procedure SHALL NOT have a name that, without its suffix, is a keyword or a built-in
written without a required suffix (such as `LEN`, `CLS`, `NAME`, `ERR`), whatever suffix it carries; nor the name of
a built-in with its required suffix (such as `LEFT$`). Such a program SHALL be rejected at compile time. A name that
matches a built-in only without its required suffix (such as `left`, `chr`) SHALL be allowed. A main-module
variable SHALL NOT have the name of a procedure.

#### Scenario: Parameter named after a statement
- **WHEN** a program declares `FUNCTION greet$ (name AS STRING)`
- **THEN** compiling it fails with an error naming `name`

#### Scenario: Suffix does not free a name
- **WHEN** a program defines `FUNCTION len& (a AS LONG)`
- **THEN** compiling it fails with an error naming `len`

#### Scenario: Name free without the required suffix
- **WHEN** a program defines `SUB chr (a AS LONG)` and has a variable `left`
- **THEN** it compiles
