# language/procedures Specification

## Purpose
How SUB and FUNCTION procedures are defined and called, how arguments are passed, and which variables a statement
inside a procedure can see. Every rule is measured with `qb64pe.exe`; the scenarios are tests in
`tests\corpus\slice\` or `tests\frontend\`.

## Requirements

### Requirement: Procedure definitions
A program SHALL be able to define procedures with `SUB name [(params)]` … `END SUB` and
`FUNCTION name[suffix] [(params)]` … `END FUNCTION`, before or after main-module code. A parameter SHALL be
`name[suffix] [AS type]` with a scalar type, of any numeric type other than `_BIT` or `STRING`. A `_BIT` or `_BIT *
n` parameter (by `AS` or by suffix) SHALL be a compile error: the old compiler accepts it but its C++ build fails,
called or not (`DECISIONS.md`, 2026-10-08). A parameter declared `STRING * n` or `name$n` SHALL be a `STRING`
parameter, its value neither cut nor padded, except that `LEN` of the parameter itself SHALL be the constant n, as
in the old compiler (`verification\v21_c_fixed_args`, `v21_f_fixed_param`); an n that is 0 or negative once read
in 32 bits, and `_UNSIGNED STRING` with or without a length, SHALL be compile errors on a parameter, as in the old
compiler ("Illegal SUB/FUNCTION parameter", `verification\v21_x42`–`x45`). A FUNCTION's type SHALL be given
by its suffix, of any numeric type or `$`, SINGLE without one; a call of a FUNCTION named with `` ` `` or `` ~` ``
and no width SHALL be a compile error, as in the old compiler ("Name already in use"), and a FUNCTION named with
`$n` SHALL return a string of exactly n bytes, its result cut or padded with spaces as a store into a `STRING * n`,
as the old compiler does (`verification\v21_c_fixed_args`); it SHALL be called and assigned as `f$n`, `f$` or `f`,
and another length (`f$4` for `f$5`) SHALL be a compile error ("Name already in use", `verification\v21_g_fixed_function`,
`v21_x49`). Each call's result SHALL be its own string: two calls in one expression keep both results (the old
compiler returns bytes it has released, so the second call overwrites the first, `DIVERGENCES.md` D-014). Procedures SHALL NOT be nested, and two procedures SHALL
NOT share a name.

#### Scenario: SUB defined before the main code
- **WHEN** a program defines `SUB foo (a AS LONG, b AS LONG)` first and calls `CALL foo(1, 2)` after it
- **THEN** it prints ` foo: 1  2 ` (`PRINT "foo:"; a; b` in the SUB)

#### Scenario: Function called before its definition
- **WHEN** main code prints `twice&(3)` and `FUNCTION twice& (a AS LONG)` assigns `twice& = a * 2` further down
- **THEN** it prints ` 6 `

#### Scenario: Unsigned function and parameter
- **WHEN** `FUNCTION big~& (x AS _UNSIGNED _BYTE)` assigns `big~& = x * 16843009` and main prints `big~&(255)`
- **THEN** it prints ` 4294967295 `

#### Scenario: Bit parameter
- **WHEN** a program defines `SUB s (x AS _BIT * 5)`, called or not
- **THEN** it is a compile error

#### Scenario: Unsigned string parameter
- **WHEN** a program defines `SUB q (t AS _UNSIGNED STRING)`
- **THEN** it is a compile error

#### Scenario: Fixed-length FUNCTION result
- **WHEN** `FUNCTION fs$5 (x AS STRING)` assigns `fs$5 = x` and main prints `"["; fs$5("ab"); "]"; LEN(fs$5("ab"))`
- **THEN** it prints `[ab   ] 5 `

#### Scenario: Two fixed-length FUNCTION results in one expression
- **WHEN** the same `fs$5` prints `"["; fs$5("ab") + fs$5("cd"); "]"`
- **THEN** it prints `[ab   cd   ]` (the old compiler prints `[cd   cd   ]`, D-014)

#### Scenario: Fixed-length string parameter
- **WHEN** `SUB sp (t AS STRING * 5)` prints `"["; t; "]"; LEN(t); LEN(t + "!")` and assigns `t = "much longer
  text"`, and is called as `sp s$` with `s$ = "hello world"` (a SUB named `s` would clash with `s$`: "Name already
  in use")
- **THEN** it prints `[hello world] 5  12 ` and `s$` is `much longer text` afterwards

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
passed by reference: the procedure's assignments to the parameter change the variable. An integer variable, array
element or member of the parameter's width but the other signedness, written plainly, SHALL be passed by reference
too, and so SHALL an `_INTEGER64` or `_UNSIGNED _INTEGER64` one to an `_OFFSET` parameter of either signedness and
the reverse, as the old compiler does (the procedure sees the same bytes read with the parameter's type;
`verification\v21_b_passing`, `v21_g_passing_places`); in parentheses it is a copy. A `_BIT` variable SHALL always be passed
as a copy, also to a parameter of its storage width. Any other argument SHALL be converted as for an assignment to
the parameter's type and passed as a temporary copy whose changes are lost. A string variable passed to a string
parameter SHALL be passed by reference.

#### Scenario: By reference
- **WHEN** `n` is a LONG holding 1 and `SUB bump (x AS LONG)` does `x = x + 1`, called as `CALL bump(n)` and as
  `bump n`
- **THEN** `n` is 3 afterwards

#### Scenario: Parentheses and expressions pass a copy
- **WHEN** the same SUB is called as `bump (n)` and as `bump n + 1`
- **THEN** `n` is unchanged

#### Scenario: String variable in parentheses
- **WHEN** `SUB addbang (t AS STRING)` appends `"!"` to `t` and is called as `addbang (s$)`
- **THEN** `s$` has the `"!"` afterwards (a string variable is passed by reference even in parentheses)

#### Scenario: Different type passes a rounded copy
- **WHEN** `2.5` is passed to an INTEGER parameter
- **THEN** the procedure sees 2 (half to even, as for an assignment)

#### Scenario: Same width, other signedness
- **WHEN** `n&` holds -1 and is passed to `SUB show (x AS _UNSIGNED LONG)`, which prints `x` and sets `x = 5`
- **THEN** the SUB prints ` 4294967295 ` and `n&` is 5 afterwards

#### Scenario: Element of the other signedness
- **WHEN** `a(1)` of `DIM a(2) AS _UNSIGNED INTEGER` is set to 65535 and passed to `SUB showsi (x AS INTEGER)`, which
  prints `x` and sets `x = -3`, first as `showsi a(1)`, then (set to 65535 again) as `showsi (a(1))`
- **THEN** the SUB prints ` -1 ` both times, and `a(1)` is 65533 after the first call and 65535 after the second

#### Scenario: _INTEGER64 to an _OFFSET parameter
- **WHEN** `q&&` holds -1 and is passed to `SUB show (x AS _UNSIGNED _OFFSET)`, which prints `x` and sets `x = 7`
- **THEN** the SUB prints ` 18446744073709551615 ` and `q&&` is 7 afterwards

#### Scenario: Bit variable passes a copy
- **WHEN** `b AS _BIT * 5` holds -16 and is passed to `SUB show (x AS LONG)`, which sets `x = 5`
- **THEN** `b` is still -16 afterwards

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

#### Scenario: SHARED with a type in an earlier procedure
- **WHEN** a SUB with `SHARED h AS LONG` comes first in the file and the main module then does `h = 9.5`
- **THEN** main's `h` is the LONG `h&` and holds 10

#### Scenario: SHARED without a type
- **WHEN** the main module has `DIM g AS LONG` and `g = 7`, and a SUB with `SHARED g` prints `g`
- **THEN** the SUB prints 0 (its `g` is main's SINGLE `g!`, not the LONG `g`)

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
