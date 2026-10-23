# language/procedures Specification

## Purpose
How SUB and FUNCTION procedures are defined and called, how arguments are passed, and which variables a statement
inside a procedure can see. Every rule is measured with `qb64pe.exe`; the scenarios are tests in
`tests\corpus\slice\` or `tests\frontend\`.

## Requirements

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
written without a required suffix (such as `LEN`, `CLS`, `NAME`, `ERR`), when it has no suffix or `&`; nor the
name of a built-in with its required suffix (such as `LEFT$`). Such a program SHALL be rejected at compile time.
With any other suffix (`$`, `%`, `!`, `#`, `%%`, `&&`, `##`, `~` forms, `` ` ``), a variable, parameter or array of
such a name SHALL be accepted or rejected as the old compiler does, word by word and suffix by suffix, as
`verification\v52_builtin_name_suffixes` records it; where it is accepted it SHALL be an ordinary variable,
parameter or array of that type in every place a name may stand (`DIM`, `DIM SHARED`, `REDIM`, `COMMON SHARED`,
`STATIC`, `SHARED`, a parameter, a name not declared; `v52_x20`–`x37`). A procedure of such a name SHALL be
rejected: the family does not record these word by word, so the rejection is the mark "not supported yet", and
the old compiler rejects the form recorded (`FUNCTION exp$`, `v52_x28`). A `CONST` of such a name SHALL be
rejected with "Name already in use" whatever its suffix, since the old compiler looks a constant's name up without
its suffix (`CONST key$`, `exp$`, `key!`; `v52_x32`, `x67`, `x68`).
A name that matches a built-in only without its required suffix (such as `left`, `chr`) SHALL be
allowed. A main-module variable SHALL NOT have the name of a procedure.

#### Scenario: Parameter named after a statement
- **WHEN** a program declares `FUNCTION greet$ (name AS STRING)`
- **THEN** compiling it fails with an error naming `name`

#### Scenario: Suffix does not free a name
- **WHEN** a program defines `FUNCTION len& (a AS LONG)`
- **THEN** compiling it fails with an error naming `len`

#### Scenario: Name free without the required suffix
- **WHEN** a program defines `SUB chr (a AS LONG)` and has a variable `left`
- **THEN** it compiles

#### Scenario: A keyword with $ as a variable
- **WHEN** a program runs `DIM section$, key$: key$ = "k": PRINT key$` (QB64pe `source\utilities\ini-manager\ini.bm`)
- **THEN** it prints `k`

#### Scenario: A built-in function's name with $ as a parameter
- **WHEN** a program has `SUB s (exp$, base$)` and calls `s "1", "2"`, and the SUB prints `exp$ + base$`
- **THEN** it prints `12`

#### Scenario: The function beside the variable
- **WHEN** a program runs `exp$ = "2": PRINT exp$; EXP(0)`
- **THEN** it prints `2 1`

#### Scenario: A FUNCTION named after a built-in with $
- **WHEN** a program defines `FUNCTION exp$ (a$)`
- **THEN** compiling it fails with an error naming `exp$`

#### Scenario: A CONST named after a keyword with a suffix
- **WHEN** a program has `CONST key! = 1`
- **THEN** compiling it fails with an error naming `key!`

#### Scenario: Every recorded form
- **WHEN** a form of `v52_builtin_name_suffixes` is compiled
- **THEN** it is a variable where the record says "variable", and a compile error where it says "rejected"

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
error of the old compiler) SHALL be compile errors. A FUNCTION SHALL NOT have a user type as its result. The parameter's `TYPE` SHALL be defined by a block above the procedure's header in the file (at the top level or in a procedure above); one defined only below it SHALL be the old compiler's error "Illegal SUB/FUNCTION parameter".

#### Scenario: Member changed in the procedure
- **WHEN** `SUB move (p AS pt)` does `p.x = p.x + 1` and is called as `move q` and as `move a(2)`
- **THEN** `q.x` and `a(2).x` are each 1 higher afterwards

#### Scenario: Another type
- **WHEN** a variable of another user type, a number, or `(q)` in parentheses is passed to `SUB move (p AS pt)`
- **THEN** it is a compile error

#### Scenario: Element outside
- **WHEN** `move a(9)` runs for `DIM a(2) AS pt` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 and no element changes

#### Scenario: A parameter's type defined below the procedure
- **WHEN** `SUB use (p AS late)` stands above the program's `TYPE late` block, at the top level or in a SUB below
- **THEN** it is the compile error "Illegal SUB/FUNCTION parameter" at the header, as in the old compiler

#### Scenario: A parameter's type defined in a procedure above
- **WHEN** `TYPE late` is defined in a SUB above `SUB use (p AS late)`
- **THEN** the program compiles and runs as with the old compiler
