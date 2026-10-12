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
A parameter written `name()` (also with anything between the parentheses, which is ignored: `a(,)`, `a(1)`,
`a(n + 1)`, `a(1 TO 5)`; only a second `(` and the word `AS` there are compile errors) SHALL be
an array parameter, of any element type an array may have, `_BIT` included. Its argument SHALL be a whole array
written `name()`, static or dynamic, allocated or not, whose element type has the parameter's kind and width:
the same number of bits or the same user type, both strings or neither, the same length for a fixed-length
string, both floating-point or neither; `_UNSIGNED` and `_OFFSET` SHALL NOT be compared (an `_UNSIGNED LONG`
array goes to a LONG parameter, which then reads its elements as LONG). Anything else SHALL be a compile error,
also a whole array passed to a parameter that is no array, and an array argument in parentheses (the old compiler
accepts that one and its C++ build fails). A name that names no array, passed in the main module, SHALL become
an implicit array of one dimension, as a use of it with one index would make it; in a SUB or FUNCTION it SHALL be
a compile error. An array parameter and a scalar parameter of one name SHALL be two variables; two array
parameters of one name and type SHALL be a compile error.
The array SHALL be passed by reference: the procedure reads, stores, `DIM`s, `REDIM`s and `ERASE`s the caller's
array with the errors that array's kind gives (error 10 for a static one), and `LBOUND`/`UBOUND` give its bounds.
The number of dimensions of an array parameter SHALL be one number for the whole program: every index count and
`REDIM` in the procedure and every argument passed to it, directly or through another procedure's array
parameter, SHALL agree, wherever the call stands in the file; a disagreement SHALL be a compile error. `LBOUND`
and `UBOUND` of the parameter SHALL NOT count among them. An array listed by `COMMON a()` or `STATIC a()` that is
passed before anything gave it a number of dimensions SHALL have the parameter's from then on.

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
- **WHEN** `fill x(1)`, `fill y` or `fill (x())` is compiled for that SUB
- **THEN** it is a compile error

#### Scenario: Signedness is not compared
- **WHEN** an `_UNSIGNED LONG` array is passed to `SUB fill (a() AS LONG)`, which stores -1 in `a(1)`
- **THEN** it compiles and the caller's element prints as ` 4294967295 `

#### Scenario: Static argument and REDIM
- **WHEN** `SUB grow (a() AS LONG)` with `REDIM _PRESERVE a(10) AS LONG` is called for `DIM x(1 TO 3) AS LONG` under
  a handler that prints `ERR` and resumes next
- **THEN** the handler prints 10 and `UBOUND(x)` is still 3

#### Scenario: ERASE in the procedure
- **WHEN** a SUB that does `ERASE a` is called for a static array and for a dynamic one
- **THEN** the static array keeps its bounds with every element 0, and `UBOUND` of the dynamic one raises error 9

#### Scenario: Dimensions from the argument
- **WHEN** `SUB show (m() AS LONG)` prints `UBOUND(m, 2)` and never indexes `m`, and is called once for `DIM y(2, 4)
  AS LONG`
- **THEN** it prints ` 4 `, with the SUB above or below the call

#### Scenario: Dimensions that disagree
- **WHEN** a SUB that stores to `m(1, 2)` is called for an array of one dimension, or a SUB that never indexes its
  parameter is called for an array of one and for an array of two dimensions
- **THEN** each is a compile error

#### Scenario: A name that names no array
- **WHEN** `fill nx(): PRINT nx(1); UBOUND(nx)` is compiled in the main module for `SUB fill (a())` that sets
  `a(1) = 5`, and `fill loc()` stands in a SUB that has no array `loc`
- **THEN** the first prints ` 5  10 `, and the second is a compile error

#### Scenario: A listed array gets the parameter's dimensions
- **WHEN** `COMMON c() AS LONG: make c(): PRINT UBOUND(c, 1); UBOUND(c, 2); c(1, 2)` runs for `SUB make (a() AS
  LONG)` that does `REDIM a(3, 4) AS LONG: a(1, 2) = 7`
- **THEN** it prints ` 3  4  7 `

#### Scenario: Passed down a chain
- **WHEN** upstream's `arrays/array_arg_dimensions_transitive` is compiled (an array handed down three SUBs, the
  innermost printing `UBOUND` of each dimension)
- **THEN** its output equals the old compiler's

### Requirement: User-type parameters
A parameter declared `name AS type` with a user type SHALL take a place of that type (a variable, an array
element, a member, the procedure's own parameter) by reference: the procedure's stores to its members change the
caller's, in a SUB and in a FUNCTION. An element with an index outside its bounds SHALL raise error 9 and the
procedure SHALL NOT be entered. An argument of another type, a number, and a place in parentheses (an internal
error of the old compiler) SHALL be compile errors. A FUNCTION SHALL NOT have a user type as its result.

#### Scenario: Member changed in the procedure
- **WHEN** `SUB move (p AS pt)` does `p.x = p.x + 1` and is called as `move q` and as `move a(2)`
- **THEN** `q.x` and `a(2).x` are each 1 higher afterwards

#### Scenario: Another type
- **WHEN** a variable of another user type, a number, or `(q)` in parentheses is passed to `SUB move (p AS pt)`
- **THEN** it is a compile error

#### Scenario: Element outside
- **WHEN** `move a(9)` runs for `DIM a(2) AS pt` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 and no element changes
