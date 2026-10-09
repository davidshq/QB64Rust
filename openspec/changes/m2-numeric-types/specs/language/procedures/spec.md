## MODIFIED Requirements

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
