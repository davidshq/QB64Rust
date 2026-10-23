## MODIFIED Requirements

### Requirement: User types
A `TYPE name … END TYPE` block SHALL define a user type whose members are of the numeric types
other than `_BIT`, of fixed-length strings, of `STRING`, or of user types defined in earlier blocks, each a single
value or an array (requirement "Member arrays"). The block may stand in the main module, in a SUB or FUNCTION, or
inside a block (`IF`, a loop, `SELECT CASE`); wherever it stands, the type SHALL be one of the whole program,
usable in the main module and in every procedure, as the old compiler's prepass defines it. The program's types
SHALL be known before its statements (a type may be used above its block, also above the procedure that holds
it). A second `TYPE` block of a name, wherever the two stand, SHALL be a compile error (the old compiler: "Name
already in use"; `verification\v48_x05`). `DIM v AS name` SHALL declare a variable of
that type, in every storage class a scalar variable may have; every member starts at zero (NUL bytes for a
fixed-length string, an empty string for a `STRING`), a local one on each call. A member's size in the layout SHALL
be its type's (1 byte for
`_BYTE`, 8 for `_OFFSET`, `n` for `STRING * n`), without padding. A member array marked `_STATIC` or `_DYNAMIC`
SHALL be reported "not supported yet"; a member declared
with a type suffix, and a `_BIT` or `_BIT * n` member, SHALL be compile errors (the old compiler: "Cannot use _BIT
inside user defined types"). A whole user-type value in an expression, a `PRINT`
item or stored into a scalar, and a number stored into a user-type variable, SHALL be compile errors; a whole
value SHALL be assignable to a place of the same type (requirement "Whole user-type values").

#### Scenario: Members of each type
- **WHEN** a type with an INTEGER, a LONG, an `_INTEGER64`, a SINGLE, a DOUBLE and a `_FLOAT` member is declared,
  each member assigned and printed
- **THEN** each value is printed as a variable of that type would be

#### Scenario: Members of the new types
- **WHEN** a type with a `_BYTE`, an `_UNSIGNED _BYTE`, an `_UNSIGNED INTEGER`, an `_UNSIGNED LONG`, an
  `_UNSIGNED _INTEGER64` and an `_OFFSET` member is declared, each member assigned -1 and printed, and `LEN` of a
  variable of the type printed
- **THEN** the output equals the old compiler's, and `LEN` is 24 (1 + 1 + 2 + 4 + 8 + 8)

#### Scenario: Nested type
- **WHEN** `TYPE inner: v AS LONG: END TYPE` (as a block) and `TYPE outer: i AS inner: END TYPE` are declared, `DIM
  o AS outer` and `o.i.v = 7` runs
- **THEN** `PRINT o.i.v` prints `7`

#### Scenario: Type used before its block
- **WHEN** `DIM p AS pt` stands above `TYPE pt … END TYPE`
- **THEN** the program compiles and `p` has the type's members

#### Scenario: Local user-type variable
- **WHEN** a SUB with `DIM p AS pt` prints `p.x` and then sets it to 5, and is called twice
- **THEN** it prints `0` both times

#### Scenario: Whole value in PRINT
- **WHEN** `PRINT p` is compiled for a user-type variable `p`
- **THEN** the compiler reports an error

#### Scenario: Bit member
- **WHEN** a `TYPE` block has the member `b AS _BIT * 4`
- **THEN** it is a compile error

#### Scenario: String member
- **WHEN** a type has `n AS STRING` and `v AS LONG`, and `p.n = "abc": p.n = p.n + "!": PRINT p.n; LEN(p.n)` runs
- **THEN** it prints `abc! 4 `

#### Scenario: String member of a local
- **WHEN** a SUB with `DIM p AS` that type prints `"[" + p.n + "]"`, sets `p.n = "x"`, and is called twice
- **THEN** it prints `[]` both times

#### Scenario: Type block inside a SUB
- **WHEN** a SUB holds `TYPE pt: x AS LONG: END TYPE` (as a block) and `DIM p AS pt: p.x = 5: PRINT p.x`, and is
  called
- **THEN** it prints ` 5 ` (`verification\v18_h_type_in_sub`)

#### Scenario: Type of a SUB used elsewhere
- **WHEN** a SUB holds the block `TYPE pt … END TYPE`, and the main module above the SUB has `DIM q AS pt: q.x = 3:
  PRINT q.x`, and another SUB below it has a parameter `p AS pt`
- **THEN** the program compiles, and the main module prints ` 3 ` (`verification\v48_c_type_anywhere`)

#### Scenario: Two TYPE blocks of one name
- **WHEN** two SUBs each hold a block `TYPE pt … END TYPE`
- **THEN** it is a compile error

#### Scenario: Type block inside an IF block
- **WHEN** the main module has `IF 1 THEN`, the block `TYPE pt … END TYPE`, then `END IF`, and below it `DIM p AS
  pt`
- **THEN** the program compiles, and `p` has the type's members

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
starts, which a handler enters again). With a bound that is not one number or one constant it SHALL be reported
"not supported yet" (the old compiler writes the bound into the main module's code: its C++ build fails for a
parameter or a local variable and works for `2 + 3`). `STATIC name()` SHALL declare a dynamic array that lives
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
name(bounds)` beside a shared main-module array of that name and type SHALL be reported "not supported yet" (the
old compiler's program ends when it starts).

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
- **WHEN** a SUB with a parameter `n` holds `STATIC v(n) AS LONG`, or one holds `STATIC v(2 + 3) AS LONG`
- **THEN** each is reported "not supported yet"

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

## ADDED Requirements

### Requirement: A scalar and an array of one name
Where one of a scalar and an array of one name is typed by `DIM … AS` and the other is not, or both are typed by
`AS` with two types, they SHALL be two variables, as in the old compiler. A name without a suffix that no
declaration made a scalar SHALL be a scalar of the default type for its first letter beside an array of that name
typed by `AS`. An array declared without `AS` beside a scalar typed by `AS` SHALL be an array of the default type,
which the plain name names.

#### Scenario: A plain scalar beside a typed array
- **WHEN** `DIM a(3) AS INTEGER: a = 1.5: a(1) = 7: PRINT a; a(1); LEN(a)` runs
- **THEN** it prints ` 1.5  7  4 ` (`a` is a SINGLE of its own)

#### Scenario: The default type under DEFLNG
- **WHEN** `DEFLNG A-Z: REDIM SHARED t(1 TO 8) AS INTEGER: t = 100000: t(1) = 77: PRINT t; t(1)` runs
- **THEN** it prints ` 100000  77 ` (upstream `arrays/t001_whole_array_core`)

#### Scenario: The plain scalar as a record
- **WHEN** `DIM a(1 TO 3) AS INTEGER` stands in the program and `PUT #1, 1, a` writes into a new binary file
- **THEN** the file holds 4 bytes, a SINGLE (`verification\v30_x67`)

#### Scenario: An array without AS beside a typed scalar
- **WHEN** `DIM b AS STRING: b = "str": DIM b(2): b(1) = 1.25: PRINT b; b(1); LEN(b(1))` runs
- **THEN** it prints `str 1.25  4 ` (`verification\v48_a_scalar_array`)

#### Scenario: A typed array beside a scalar typed otherwise
- **WHEN** `DIM d AS STRING: DIM d(3) AS LONG: d = "dd": d(1) = 5: PRINT d; d(1); LEN(d); LEN(d(1))` runs
- **THEN** it prints `dd 5  2  4 ` (`verification\v48_a_scalar_array`)

### Requirement: The plain name between two arrays
Between two arrays of one name, the plain name SHALL name the one typed by `AS` once it exists in the scope,
whichever was declared first: a `DIM` without `AS` beside it SHALL declare another array, of the default type,
named with its suffix (`c!`), and a `REDIM` without `AS` SHALL give the typed array new bounds. In a procedure, an
array declared there without `AS` SHALL take the plain name from a typed main-module array shared with it.

#### Scenario: The typed array keeps the plain name
- **WHEN** a SUB with `DIM c(2) AS DOUBLE: c(1) = 1 / 3`, then `DIM c(3)`, then `c(1) = c(1) + 1: PRINT c(1);
  LEN(c(1)); UBOUND(c); UBOUND(c!)` is called
- **THEN** it prints ` 1.333333333333333  8  2  3 ` (`verification\v48_x17`; the same in the main module, `v48_a`)

#### Scenario: A local array without AS beside a shared typed one
- **WHEN** the main module has `DIM SHARED c(2) AS DOUBLE` and a SUB with `DIM c(3): c(1) = c(1) + 1: PRINT c(1);
  LEN(c(1)); UBOUND(c)` is called
- **THEN** it prints ` 1  4  3 ` (`verification\v48_x20`: the SUB's plain `c` is its SINGLE array)

### Requirement: An array listed with a type
An array listed by `STATIC name() AS type` SHALL exist only once a `DIM` or `REDIM` of its type, or a use, reaches
it (the old compiler's list, `qb64pe.bas` 9260): a `DIM` or `REDIM` without `AS` before that SHALL declare an array
of the default type, which the plain name names. An array without `AS` beside one listed by `COMMON name() AS type`
is not measured and SHALL be reported "not supported yet".

#### Scenario: An array without AS beside a listed one
- **WHEN** a SUB with `STATIC a() AS DOUBLE`, then `DIM a(3)`, then `a(1) = a(1) + 1: PRINT a(1); LEN(a(1))` is
  called
- **THEN** it prints ` 1  4 ` (`verification\v24_x126`: the listed DOUBLE array was never reached, and `a(1)` names
  the SINGLE array)

### Requirement: LEN of a whole array
`LEN(name())` of an array, and `LEN` of a whole member array (`LEN(v.m())`, `LEN(a(i).m())`), SHALL give the size in
bytes of all its elements, as `_MEM` of it gives: the number of elements times the element's size. A dynamic array
that is not allocated SHALL give 0. A whole dynamic member array, and `LEN(name())` of a name that is no array
(the old compiler makes it an array used without `DIM`, `verification\v48_x04`, `x14`), SHALL be reported "not
supported yet".

#### Scenario: Numbers and dimensions
- **WHEN** `DIM a(3 TO 7) AS INTEGER, b(1, 2) AS DOUBLE: PRINT LEN(a()); LEN(a(3)); LEN(b())` runs
- **THEN** it prints ` 10  2  48 ` (`verification\v48_d_len_whole`)

#### Scenario: An array parameter and a shared array
- **WHEN** a SUB with the parameter `x() AS INTEGER` prints `LEN(x())`, called with an array of 5 INTEGERs, and a
  SUB prints `LEN(sg())` of `DIM SHARED sg(9) AS LONG`
- **THEN** they print ` 10 ` and ` 40 `

#### Scenario: Fixed-length strings and user types
- **WHEN** `TYPE num_t: a AS LONG: b AS LONG: END TYPE` (as a block), `DIM na(-3 TO 2) AS num_t, f(4) AS STRING
  * 3` and `PRINT LEN(na()); LEN(f())` run
- **THEN** it prints ` 48  15 ` (upstream `types/udt_array`)

#### Scenario: A dynamic array
- **WHEN** `n = 7: REDIM d(3 TO n) AS INTEGER: PRINT LEN(d()): ERASE d: PRINT LEN(d())` runs
- **THEN** it prints ` 10 `, then ` 0 `

#### Scenario: A whole member array
- **WHEN** `TYPE test: a(1 TO 50) AS LONG: END TYPE` (as a block), `DIM parent(1) AS test` and `PRINT
  LEN(parent(0).a()); LEN(parent()); LEN(parent(0).a(1))` run
- **THEN** it prints ` 200  400  4 ` (upstream `arrays/test_mem_vs_len`)

#### Scenario: In an expression
- **WHEN** `DIM f(3) AS LONG: PRINT LEN(f()) / 4; LEN(f()) \ 4; LEN(f()) MOD 3` runs
- **THEN** it prints ` 4  4  1 `

### Requirement: Arrays LEN does not take
`LEN` of a whole array of variable-length strings, of a user type with a `STRING` member, and of a `_BIT` array
SHALL be compile errors, as in the old compiler ("Cannot pass array of variable-length strings", "UDT must have
fixed size", "Variable/element cannot be _BIT aligned"; `verification\v48_x01`–`x03`).

#### Scenario: Strings and bits
- **WHEN** `DIM s(3) AS STRING: PRINT LEN(s())` is compiled, or `DIM b(3) AS _BIT: PRINT LEN(b())`
- **THEN** each is a compile error

#### Scenario: A user type with a STRING member
- **WHEN** a `TYPE ts` has the members `n AS STRING` and `v AS LONG`, and `DIM a(3) AS ts: PRINT LEN(a())` is
  compiled
- **THEN** it is a compile error
