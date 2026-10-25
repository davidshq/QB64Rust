# Spec Delta

## MODIFIED Requirements

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
is not "not supported yet" (the old compiler's C++ build fails, `verification\v24_x121`); one that names an array
of the procedure (a parameter array, a local or `STATIC` one) SHALL be "not supported yet" (not measured). `LEN` of a string
literal SHALL be taken with its value (`verification\v55_x73`). A bound that names a shared main-module variable
or calls a FUNCTION SHALL be read when the start code allocates the array, as in the old compiler: when the
program starts (a shared variable has its first value then, 0 for a number; what a FUNCTION there prints SHALL be
lost), also for a procedure that is never called, and again each time an error reaches a handler while the array
is released, with the values of that moment; while the array is allocated the bound SHALL not be read, so a
FUNCTION in it does not run (`verification\v55_x74`, `x75`, `v60_a1`–`a7`). `STATIC name()` SHALL declare a dynamic array that lives
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

In a procedure, `REDIM name(bounds)` (also `_PRESERVE`, `AS type`, an item of a list) of a shared main-module
array whose bound is the first place in the procedure that names an array used without `DIM` SHALL make the name
and type an array of the procedure in the whole procedure, above that `REDIM` too, as the old compiler does: a
dynamic array that is new on every call and released at its end, also in a `SUB … STATIC`, which nothing
allocates until a `REDIM` runs. The shared array SHALL be left as it was and other procedures SHALL name it
(`verification\v55_x84`, `v24_x268` case 5, `v60_b1`–`b10`). When the array used without `DIM` was named above the
`REDIM` in the procedure, the `REDIM` SHALL give the shared array its bounds (`v60_b7`). In a procedure with
`STATIC` after its header, an array used without `DIM` that a bound of `REDIM` names first SHALL be a dynamic
array that nothing allocates, as below `$DYNAMIC` (`v60_b5`).

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

#### Scenario: STATIC with a bound that names an array of the procedure
- **WHEN** a SUB with a parameter array `pa()` holds `STATIC e(UBOUND(pa)) AS LONG`, one holds `STATIC sb(5) AS
  LONG: STATIC g(UBOUND(sb)) AS LONG`, and one `DIM lc(3) AS LONG: STATIC h(lc(1)) AS LONG`
- **THEN** each is "not supported yet"; a SUB with `SHARED sa() AS LONG: STATIC v(UBOUND(sa)) AS LONG` reads the
  shared array's bound when the program starts

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

#### Scenario: STATIC with a bound read when the program starts
- **WHEN** a SUB with `STATIC v(n) AS LONG: v(1) = v(1) + 1: PRINT v(1); UBOUND(v)` reads a shared `n` set to 4
  in the main module before the call, and another with `STATIC v(f) AS LONG` for a FUNCTION `f` that prints `"f"`
  and returns 6 is called twice (`verification\v55_x74`, `x75`)
- **THEN** the first ends with "Subscript out of range" (the array is `v(0 TO 0)`), and the second prints ` 1  6 `,
  then ` 2  6 `, and no `f`

#### Scenario: REDIM of a shared array with a new array in its bound
- **WHEN** the main module has `REDIM SHARED sd(1)`, then `REDIM sd(ASC("") + 6)` in a SUB, then a SUB with `REDIM
  sd(lc(1) + 4): PRINT UBOUND(sd)` is called twice, under a handler that prints `ERR` and resumes next, and the
  main module prints `UBOUND(sd)` after each (`verification\v55_x84`)
- **THEN** each call of the second SUB prints the handler's 9, then ` 4 `, and the main module prints ` 6 `

#### Scenario: The procedure's array above the REDIM
- **WHEN** the main module has `REDIM SHARED sd(1): sd(1) = 11`, and a SUB with `PRINT UBOUND(sd); sd(1)`, then
  `REDIM sd(lc(1) + 4)`, then `PRINT UBOUND(sd); sd(1): sd(1) = 22` is called twice under the same handler
  (`verification\v60_b1`)
- **THEN** each call prints the handler's 9 above the `REDIM` and ` 4  0 ` below it, and the main module still
  has `UBOUND(sd)` 1 and `sd(1)` 11

#### Scenario: STATIC with a FUNCTION bound under handlers
- **WHEN** a SUB holds `STATIC v(f) AS LONG` for a FUNCTION `f` that counts its calls, the program enters a
  handler twice, then the SUB runs `ERASE v` and the program enters the handler again (`verification\v60_a2`)
- **THEN** `f` has run once after the first two handled errors and twice after the third
