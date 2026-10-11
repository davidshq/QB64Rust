# Spec Delta

## MODIFIED Requirements

### Requirement: Procedure definitions
A program SHALL be able to define procedures with `SUB name [(params)]` … `END SUB` and
`FUNCTION name[suffix] [(params)]` … `END FUNCTION`, before or after main-module code. A parameter SHALL be
`name[suffix] [AS type]` with a scalar type, of any numeric type other than `_BIT` or `STRING`, an array
(`name[suffix]() [AS type]`, requirement "Array parameters") or a user type (requirement "User-type
parameters"); without a suffix or `AS` clause its type SHALL be the default type of its name
(`language/default-types`). A `_BIT` or `_BIT *
n` parameter (by `AS` or by suffix) SHALL be a compile error: the old compiler accepts it but its C++ build fails,
called or not (`DECISIONS.md`, 2026-10-08). A parameter declared `STRING * n` or `name$n` SHALL be a `STRING`
parameter, its value neither cut nor padded, except that `LEN` of the parameter itself SHALL be the constant n, as
in the old compiler (`verification\v21_c_fixed_args`, `v21_f_fixed_param`); an n that is 0 or negative once read
in 32 bits, and `_UNSIGNED STRING` with or without a length, SHALL be compile errors on a parameter, as in the old
compiler ("Illegal SUB/FUNCTION parameter", `verification\v21_x42`–`x45`). A FUNCTION's type SHALL be given
by its suffix, of any numeric type or `$`, the default type of its name without one; a call of a FUNCTION named
with `` ` `` or `` ~` ``
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

## ADDED Requirements

### Requirement: Array parameters
A parameter written `name()` (also with commas between the parentheses, which are ignored) SHALL be an array
parameter. Its argument SHALL be a whole array written `name()`, of exactly the parameter's element type (and
length, for a fixed-length string), static or dynamic; anything else SHALL be a compile error. The array SHALL be
passed by reference: the procedure reads, stores, `REDIM`s and `ERASE`s the caller's array, and `LBOUND`/`UBOUND`
give its bounds. An array of one number of dimensions passed where the procedure uses another SHALL be a compile
error.

#### Scenario: Elements changed in the procedure
- **WHEN** `SUB fill (a() AS LONG)` sets `a(i) = i * 2` from `LBOUND(a)` to `UBOUND(a)` and is called as `fill x()`
  for `DIM x(1 TO 3) AS LONG`
- **THEN** `PRINT x(1); x(2); x(3)` prints ` 2  4  6 `

#### Scenario: REDIM in the procedure
- **WHEN** `SUB grow (a() AS LONG)` does `REDIM _PRESERVE a(10) AS LONG` and is called for `REDIM d(2) AS LONG` with
  `d(2) = 5`
- **THEN** `PRINT UBOUND(d); d(2)` prints ` 10  5 ` in the main module afterwards

#### Scenario: Passed on
- **WHEN** `SUB outer (a() AS LONG)` calls `inner a()` and `SUB inner (b() AS LONG)` sets `b(1) = 4`
- **THEN** the caller's array holds 4 at index 1

#### Scenario: Another element type
- **WHEN** an INTEGER array is passed to `SUB fill (a() AS LONG)`
- **THEN** it is a compile error

#### Scenario: Not an array
- **WHEN** `fill x(1)` or `fill y` is compiled for that SUB
- **THEN** it is a compile error

### Requirement: User-type parameters
A parameter declared `name AS type` with a user type SHALL take a place of that type (a variable, an array
element, a member) by reference: the procedure's stores to its members change the caller's. An argument of another
type SHALL be a compile error.

#### Scenario: Member changed in the procedure
- **WHEN** `SUB move (p AS pt)` does `p.x = p.x + 1` and is called as `move q` and as `move a(2)`
- **THEN** `q.x` and `a(2).x` are each 1 higher afterwards

#### Scenario: Another type
- **WHEN** a variable of another user type, or a number, is passed to `SUB move (p AS pt)`
- **THEN** it is a compile error
