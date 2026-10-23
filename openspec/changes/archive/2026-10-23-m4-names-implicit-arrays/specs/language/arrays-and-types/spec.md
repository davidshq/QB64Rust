# Spec Delta

## ADDED Requirements

### Requirement: Where an implicit array is allocated
Below `$DYNAMIC` in the main module, the allocation of an array used without `DIM` SHALL run where the old
compiler's code evaluates the name. Inside a statement, that is after the items or targets before the one that
names it, and in a console `INPUT` after the prompt and before the line is read. On a block's line, it is before
an `IF`, `FOR`, `WHILE`, `DO` with its test at the top or `SELECT CASE`, once. For `ELSEIF`, `CASE` and a test at
`LOOP`, it is each time that part's test runs.

#### Scenario: A later item of PRINT
- **WHEN** below `$DYNAMIC`, `PRINT " x"; dp(1); " y";` runs twice under a handler that prints `ERR` and resumes
  next
- **THEN** it prints ` x 0  y`, then ` x` and the handler prints 10 (`verification\v24_x264`)

#### Scenario: WRITE writes its separator first
- **WHEN** below `$DYNAMIC`, `WRITE i, dr(1)` runs for `i` = 1 and 2 under the same handler
- **THEN** it prints `1,0`, then `2,` and the handler prints 10 (`verification\v24_x264`)

#### Scenario: INPUT shows its prompt and reads nothing
- **WHEN** below `$DYNAMIC`, `INPUT a, ia(1), b` runs twice under the same handler, then `INPUT "p"; ib(1), a`
  runs, with the lines `1,2,3` and `4,5` typed
- **THEN** the second `INPUT` shows `? `, the handler prints 10, and the next `INPUT` reads `4,5`
  (`verification\v24_x265`)

#### Scenario: The line of a block
- **WHEN** the programs of `verification\v24_x262` run: an array first named in the line of an `IF`, a `FOR`, a
  `WHILE`, a `DO WHILE`, a `LOOP UNTIL`, a `SELECT CASE`, a later `CASE` and an `ELSEIF`
- **THEN** each prints what the old compiler printed: the `WHILE` and `DO WHILE` raise no error on their second
  round, the `LOOP UNTIL` raises error 10 on its second, and the `ELSEIF` not reached allocates nothing (its
  `UBOUND` raises error 9)

#### Scenario: An error of the allocation in a block's line
- **WHEN** the programs of `verification\v55_a_implicit_headers` run, the allocation in the line of each block
  raising error 10 on its second run under `RESUME NEXT`
- **THEN** each goes on where the old compiler's program went on, as recorded: an `IF`, a `CASE`, a `WHILE` and
  a `DO` tested at the top run their body though the test is false, a `LOOP UNTIL` leaves its loop, an `ELSEIF`
  runs no branch's first statement, a `FOR` runs its body with its variable as it was, and after a `SELECT CASE`
  the `CASE` tests run as usual

### Requirement: Declared names with blanks around the dot
A name written with blanks around a dot after a plain name SHALL be the name without those blanks where it is
declared too: `DIM a . b AS LONG` SHALL declare the variable `a.b`, and `SUB a . b` the SUB `a.b`, as at a use
(requirement "Member access and dotted names"). A form `verification\v55` does not record SHALL stay "not
supported yet".

#### Scenario: DIM with blanks around the dot
- **WHEN** `DIM a . b AS LONG: a.b = 3: PRINT a.b; a . b` runs
- **THEN** it prints ` 3  3 ` (`verification\v47_x17`)

#### Scenario: A SUB with blanks around the dot
- **WHEN** `a . b 3` runs for `SUB a . b (n)` that prints `n`
- **THEN** it prints ` 3 ` (`verification\v47_x18`)

## MODIFIED Requirements

### Requirement: Implicit arrays
`name(index, …)` where no array of that name and type is visible, the name with its suffix may be a variable, no
SUB or FUNCTION of that name exists and the spelling calls no built-in function SHALL make an array of the scope
it stands in, with the bounds `OPTION BASE` to 10 in each of as many dimensions as indexes are written;
`LBOUND(name)` and `UBOUND(name)` of such a name, and the whole array `name()`, SHALL make one of one dimension
(`name()` then stands for its first element, requirement "A whole array as a value", and `LEN(name())` is its size
in bytes). `WIDTH (n) = v` SHALL store into an implicit array named `WIDTH`, and `WIDTH(n)` in an expression read
it, as in the old compiler (`verification\v27_x51`). In the main module, and in a
`SUB … STATIC` or `FUNCTION … STATIC`, the array SHALL be static (dynamic in the main module while `$DYNAMIC` is
in effect, requirement "Dynamic arrays"). In any other procedure, and in every procedure while `$DYNAMIC` is in
effect, it SHALL be a local dynamic array that is not allocated, so that every use raises error 9 until a `DIM`
or `REDIM` of it in the procedure runs, as in the old compiler. Under `OPTION _EXPLICITARRAY` or `OPTION
_EXPLICIT`, wherever in the program the statement stands, an implicit array SHALL be a compile error instead. A
scalar variable of the name, also one typed with `AS`, and a `CONST` of the name SHALL NOT keep the name from
being an array and SHALL NOT give it their type. `ERASE` of an implicit array SHALL clear a static one and
release a dynamic one; above its first use in the file it SHALL be a compile error. A later `DIM` of a static
one, and a use with another number of indexes, SHALL be compile errors. A name that may not be a variable (one leading underscore, a keyword, a built-in statement)
and a store to a spelling that is a FUNCTION or a built-in function SHALL be compile errors that are not "not
supported yet", as the old compiler's are ("Invalid array name", "Expected variable =";
requirement "Names that call no built-in" of `language/builtin-functions`).

#### Scenario: Zero to ten
- **WHEN** `z(10) = 4: PRINT z(10); UBOUND(z)` runs, and then `z(11) = 1` under a handler that prints `ERR`
- **THEN** it prints ` 4  10 `, and the handler prints 9

#### Scenario: Two indexes
- **WHEN** `m(2, 3) = 5: PRINT UBOUND(m, 1); UBOUND(m, 2)` runs
- **THEN** it prints ` 10  10 `

#### Scenario: Another type of the same name
- **WHEN** `DIM c(3) AS LONG: c!(7) = 1.5: PRINT c!(7); UBOUND(c)` runs
- **THEN** it prints ` 1.5  3 ` (`c!()` is another, implicit array)

#### Scenario: OPTION _EXPLICITARRAY
- **WHEN** `OPTION _EXPLICITARRAY: z(3) = 4` is compiled, or `z(3) = 4` with `OPTION _EXPLICITARRAY` on a later
  line, or `OPTION _EXPLICIT: z(3) = 4`
- **THEN** each is a compile error

#### Scenario: A scalar and a constant of the name
- **WHEN** `DIM x AS LONG: x = 7: x(3) = 2.5: PRINT x; x(3); LEN(x(3))` runs, and `CONST k = 1: k(1) = 2: PRINT k;
  k(1)`
- **THEN** they print ` 7  2.5  4 ` and ` 1  2 `

#### Scenario: ERASE of an implicit array
- **WHEN** `e(1) = 5: ERASE e: PRINT e(1); UBOUND(e)` runs, and `ERASE later: later(1) = 5` is compiled
- **THEN** the first prints ` 0  10 `; the second is a compile error

#### Scenario: DIM after an implicit use
- **WHEN** `a(1) = 5: DIM a(5)` is compiled
- **THEN** it is a compile error

#### Scenario: Another number of indexes
- **WHEN** `z(1) = 1: z(1, 2) = 1` is compiled
- **THEN** it is a compile error

#### Scenario: In a SUB
- **WHEN** a SUB that is not `STATIC` runs `x(2) = 5: PRINT x(2)` under a handler that prints `ERR` and resumes
  next, with no array `x` declared
- **THEN** the handler prints 9 twice

#### Scenario: In a SUB … STATIC
- **WHEN** `SUB st STATIC` with `x(2) = x(2) + 1: PRINT x(2); UBOUND(x)` is called twice
- **THEN** it prints ` 1  10 `, then ` 2  10 `

#### Scenario: A built-in's name without its suffix
- **WHEN** `chr(1) = 5: PRINT chr(1)` runs
- **THEN** it prints ` 5 `

#### Scenario: Names that are no arrays
- **WHEN** `_foo(1) = 2`, `to(1) = 5`, `cls(1) = 5`, `rnd(1) = 2`, or `f(3) = 5` with a FUNCTION `f` defined below,
  is compiled
- **THEN** each is a compile error that is not "not supported yet" (`verification\v24_x112`, `x119`, `x106`)

#### Scenario: The whole array of a name that is no array
- **WHEN** `x = 1: PRINT LEN(x())` runs, and `PRINT LEN(nothing())`
- **THEN** each prints ` 44 `: eleven `SINGLE` elements (`verification\v48_x04`, `x14`)

#### Scenario: The whole array of a name that is no array, in a SUB
- **WHEN** a SUB that is not `STATIC` runs `PRINT LEN(u()); u()` with no array `u` declared, under a handler
  that prints `ERR`
- **THEN** it prints ` 0  0 ` and the handler does not run (`verification\v55_b_implicit_whole`, `v55_x72`)

#### Scenario: An array named WIDTH
- **WHEN** a program has `WIDTH = 80`, `WIDTH (80) = 80` and `PRINT WIDTH(1)`
- **THEN** it compiles, as with the old compiler (`verification\v27_x51`)

### Requirement: Arrays in procedures
In a SUB or FUNCTION, `DIM` and `REDIM` of an array SHALL make a dynamic array that is new on every call and
released at the end of the call; running the same `DIM` twice in one call SHALL raise error 10. `DIM name()` and
`REDIM name()` with nothing between the parentheses, and `REDIM name [AS type]` without parentheses, SHALL declare
a scalar, as in the main module: a local variable of the procedure (one that lives for the program in a
`SUB … STATIC` or `FUNCTION … STATIC`, above its first `STATIC` statement), as `DIM name [AS type]` declares it.
The item may stand in a list with arrays (`REDIM a(n) AS LONG, i AS LONG`). Such a `REDIM` SHALL do nothing when it
runs: run again in one call it keeps the scalar's value. Below a use of the name, `REDIM name AS type` SHALL declare
a new scalar that the plain name names below it, and `REDIM name` without `AS` SHALL be a compile error, as `DIM`
(the old compiler: "Name already in use"; `verification\v48_x09`, `x10`). `REDIM SHARED` in a procedure SHALL be a
compile error with or without parentheses (`v48_x06`).

`STATIC name(bounds)` with constant bounds SHALL make an array that keeps its elements between calls. It SHALL be
a dynamic array that is allocated when the program starts, below `$DYNAMIC` or not: `ERASE` SHALL release it (a
use then raises error 9), a `REDIM` SHALL give it other bounds, and the next error that reaches a handler SHALL
allocate it again, empty, if it was released (the old compiler writes its allocation where the main module
starts, which a handler enters again). The old compiler writes the bounds into the main module's code as they
are. So a bound whose value is known when compiling (`2 + 3`, an expression of constants) SHALL be taken with
that value, and a bound that names a parameter or a local variable of the procedure SHALL be a compile error that
is not "not supported yet" (the old compiler's C++ build fails, `verification\v24_x121`). `LEN` of a string
literal SHALL be taken with its value (`verification\v55_x73`). A bound that names a shared main-module variable
or calls a FUNCTION SHALL stay "not supported yet": the old compiler reads it when its start code runs, at the
start and again when a handler is entered (`v55_x74`, `x75`). `STATIC name()` SHALL declare a dynamic array that lives
for the program and is not allocated: a `DIM` of it in the same procedure SHALL allocate it (raising error 10 on
a later call), a `REDIM` SHALL give it new elements and a `REDIM _PRESERVE` keep them; its number of dimensions
SHALL come from what names it first, a `DIM`, a `REDIM`, an index or `LBOUND`/`UBOUND`, and another number later
SHALL be a compile error.

In a `SUB … STATIC` or `FUNCTION … STATIC`, every variable and array first named **above the procedure's first
`STATIC` statement** SHALL live for the program: a scalar keeps its value, a `DIM` with constant bounds is a
static array, one with a bound that is not constant a dynamic array that lives for the program (its `DIM` raises
error 10 on a later call), and an array used without `DIM` a static one. Below that statement the procedure
SHALL be one like any other, as the old compiler has it (`DIVERGENCES-QB45.md` Q-010).

`SHARED name() [AS type]` SHALL name the main-module array of that name and type, declared or used without `DIM`,
which SHALL stand above the procedure in the file (a compile error otherwise; none is made). Below an array of
that name and type that the procedure has already, it SHALL change nothing. A main-module array declared with
`DIM SHARED`, `REDIM SHARED` or `COMMON SHARED` SHALL be visible in the procedures below its declaration in the
file; in a procedure above it, and for any other main-module array, the name SHALL be an implicit array of the
procedure. A `DIM` in a procedure of the name and type of a shared main-module array SHALL be an allocation of
that array, which raises error 10 when it runs; in a procedure with `STATIC` after its header, above its first
`STATIC` statement, a `DIM` with constant bounds of a static one SHALL be the compile error it is in the main
module.

`STATIC` of an array declared, used or listed already in the procedure, `DIM SHARED` and `REDIM SHARED` inside a
procedure, `SHARED name(bounds)` with bounds, and `STATIC` in the main module SHALL be compile errors. `STATIC
name(bounds)` beside a shared main-module array of that name and type SHALL name that array in the procedure. Its
allocation when the program starts SHALL be a second allocation of the shared array, after the array's own: a
static shared array then ends the program at the start with "Duplicate definition", before any `ON ERROR` runs,
as in the old compiler (`verification\v24_x272`, `v55_d_static_shared`).

#### Scenario: New on each call
- **WHEN** a SUB with `DIM t(5) AS LONG: PRINT t(1): t(1) = 7` is called twice
- **THEN** it prints ` 0 ` both times

#### Scenario: REDIM of a scalar in a SUB
- **WHEN** a SUB with a string parameter `s` holds `REDIM v(LEN(s)) AS LONG, i AS LONG`, then `FOR i = 1 TO
  UBOUND(v): v(i) = i * 100000: NEXT: PRINT v(UBOUND(v)); i`, and is called with `"abc"`
- **THEN** it prints ` 300000  4 ` (`i` is a LONG scalar of the SUB)

#### Scenario: REDIM of a scalar run twice
- **WHEN** a SUB runs `REDIM i AS LONG: PRINT i;: i = i + 5` twice in a loop, and is called twice
- **THEN** each call prints ` 0  5 ` (`verification\v48_b_redim_scalar`)

#### Scenario: STATIC array
- **WHEN** a SUB with `STATIC c(3) AS LONG: c(1) = c(1) + 1: PRINT c(1)` is called three times
- **THEN** it prints ` 1 `, ` 2 `, ` 3 `

#### Scenario: STATIC a() then DIM
- **WHEN** a SUB with `STATIC l() AS LONG`, then `DIM l(3) AS LONG`, then `l(1) = l(1) + 1: PRINT l(1)` is called
  twice under a handler that prints `ERR` and resumes next
- **THEN** it prints ` 1 `, then the handler prints 10 and ` 2 ` is printed

#### Scenario: SUB … STATIC
- **WHEN** `SUB k STATIC` with `DIM a(2) AS LONG: a(1) = a(1) + 1: PRINT a(1)` is called twice
- **THEN** it prints ` 1 `, then ` 2 `

#### Scenario: SHARED array
- **WHEN** the main module has `DIM g(3) AS LONG: g(2) = 8` and a SUB with `SHARED g() AS LONG` prints `g(2)` and sets
  it to 9
- **THEN** the SUB prints ` 8 ` and the main module then prints ` 9 `

#### Scenario: Main array not visible
- **WHEN** the main module has `DIM h(3) AS LONG: h(1) = 5` and a SUB without `SHARED` prints `h(1)` under a
  handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 (the SUB's `h` is an implicit array of its own, which nothing allocates), and
  `h(1)` in the main module is still 5

#### Scenario: Shared array declared below the SUB
- **WHEN** a SUB that prints `a(1)` stands above `DIM SHARED a(3) AS LONG` in the file and is called after `a(1) =
  1`, under the same handler
- **THEN** the handler prints 9; the same SUB placed below the `DIM SHARED` prints ` 1 `

#### Scenario: STATIC a() then REDIM
- **WHEN** a SUB with `STATIC r() AS LONG: REDIM r(3) AS LONG: r(1) = r(1) + 1: PRINT r(1)` is called twice, and
  one with `REDIM _PRESERVE` in place of `REDIM`
- **THEN** the first prints ` 1 ` twice, the second ` 1 `, then ` 2 `

#### Scenario: Recursion
- **WHEN** a FUNCTION with `DIM a(n) AS LONG: a(n) = n` calls itself with `n - 1` down to 1 and adds `a(n)` after
  the call returns
- **THEN** each level still holds its own `a(n)` (the result for 3 is 6)

#### Scenario: Rejected declarations
- **WHEN** a procedure holds `STATIC a(3)` twice, `STATIC a()` then `STATIC a(3)`, `a(1) = 5` then `STATIC a(3)`,
  `DIM SHARED a(3)`, `SHARED a(3)`, or `SHARED a()` for a main array declared below the procedure
- **THEN** each is a compile error

#### Scenario: STATIC with a bound that is not constant
- **WHEN** a SUB with a parameter `n` holds `STATIC v(n) AS LONG`, and one with `STATIC v(2 + 3) AS LONG: v(1) =
  v(1) + 1: PRINT v(1); UBOUND(v)` is called twice
- **THEN** the first is a compile error that is not "not supported yet" (`verification\v24_x121`); the second
  prints ` 1  5 `, then ` 2  5 ` (`v24_x280`)

#### Scenario: STATIC beside a shared array
- **WHEN** the main module has `ON ERROR GOTO h` and `DIM SHARED a(3) AS LONG`, and a SUB below holds `STATIC a(5)
  AS LONG`
- **THEN** the program ends when it starts with "Duplicate definition" on line 1, and the handler does not run
  (`verification\v24_x272`)

#### Scenario: ERASE of a STATIC array
- **WHEN** a SUB with `STATIC c(2) AS LONG: c(1) = c(1) + 1: PRINT c(1);: ERASE c: PRINT c(1);` is called twice
  under a handler that prints `ERR` and resumes next
- **THEN** each call prints ` 1 `, then the handler prints 9 (the array is released, and the error that reached
  the handler allocated it again, empty)

#### Scenario: Above and below the STATIC statement
- **WHEN** `SUB s STATIC` with `n = n + 1: PRINT n;`, then `STATIC x(2) AS LONG`, then `m = m + 1: PRINT m` is
  called three times
- **THEN** it prints ` 1  1 `, ` 2  1 `, ` 3  1 `

#### Scenario: DIM of a shared array in a SUB
- **WHEN** the main module has `DIM SHARED sa(3) AS LONG` and a SUB with `DIM sa(5) AS LONG: PRINT UBOUND(sa)` is
  called under the same handler
- **THEN** the handler prints 10 and the SUB prints ` 3 `

#### Scenario: SHARED below a local array
- **WHEN** the main module has `DIM a(3) AS LONG: a(1) = 7` and a SUB holds `DIM a(5) AS LONG: a(1) = 1: SHARED a()
  AS LONG: PRINT a(1); UBOUND(a)`
- **THEN** the SUB prints ` 1  5 ` and `a(1)` in the main module is still 7
