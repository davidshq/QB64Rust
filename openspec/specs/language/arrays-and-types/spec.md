# language/arrays-and-types Specification

## Purpose
Static arrays of the main module, their elements, `LBOUND`/`UBOUND`, user types (`TYPE`) with numeric, string and
nested members, and member access, with the old compiler's observed behaviour. Every rule is measured with
`qb64pe.exe` (`verification\v18_*`); the scenarios are tests in `tests\corpus\slice\` or `tests\frontend\`.

## Requirements

### Requirement: Static arrays
`DIM name(bounds) [AS type]` and `DIM SHARED name(bounds) [AS type]` in the main module, with bounds that are each
one number or the name of one constant (in parentheses or not; a minus before a number is part of the number, also
before `&H5`),
SHALL declare an array of any numeric type, of `_BIT` and `_BIT * n` with or without
`_UNSIGNED`, of `STRING`, of a fixed-length string or of a user type: each
dimension is `upper` (lower bound 0, or the `OPTION BASE` in effect) or `lower TO upper`, each bound rounded half
to even to a whole number; every
element starts at zero, as an empty string, or as NUL bytes for a fixed-length string. The `DIM` line SHALL do
nothing when it runs. An array SHALL be a name
plus a type, in a name space of its own beside scalar variables. An upper bound below its lower bound, and a second
`DIM` with constant bounds of an array that is static, SHALL be compile errors. Such a `DIM` SHALL make a dynamic
array instead (requirement "Dynamic arrays") when `$DYNAMIC` is in effect or a bound is not constant: a bound
with an operator is not, whatever its value (`DIM a(2 + 3)`, `DIM a(k + 1)` and `DIM a(-k)` for a constant `k`
make dynamic arrays, as in the old compiler). An element
of a `_BIT` array SHALL hold what a `_BIT` variable of that width holds; `SWAP` of two such elements, `LEN` of
one and console `INPUT` into one SHALL be compile errors, such an element SHALL be passed to a procedure by
value, and `READ` and `INPUT #` SHALL store into one. `DIM name()` with
nothing between the parentheses SHALL declare the scalar variable `name` and no array, as the old compiler does.

#### Scenario: Bounds
- **WHEN** `DIM b(-2 TO 3, 5) AS LONG` is declared and `b(-2, 0)` and `b(3, 5)` are assigned and printed
- **THEN** both values are printed, and every other element is 0

#### Scenario: Float bounds
- **WHEN** `DIM a(2.5)` is declared
- **THEN** `PRINT LBOUND(a); UBOUND(a)` prints `0  2`

#### Scenario: Array and scalar of the same name
- **WHEN** `DIM a(3)`, `a = 5` and `a(1) = 2` run
- **THEN** `PRINT a; a(1)` prints `5  2`

#### Scenario: DIM run twice
- **WHEN** `DIM a(5) AS LONG` stands in a `FOR` body that sets `a(1) = a(1) + 1` and runs twice
- **THEN** `a(1)` is 2 after the loop

#### Scenario: Array skipped by GOTO
- **WHEN** `GOTO after`, then `DIM a(5) AS LONG`, then `after: a(3) = 7: PRINT a(3)` run
- **THEN** it prints ` 7 ` (a static array exists from the start of the program)

#### Scenario: Array in a SUB
- **WHEN** `DIM t(5)` stands inside a SUB that is not `STATIC`
- **THEN** `t` is a dynamic array, new on each call (requirement "Arrays in procedures")

#### Scenario: Array used without DIM
- **WHEN** `z(3) = 4` is compiled and no array `z` is declared
- **THEN** `z` is an implicit array (requirement "Implicit arrays")

#### Scenario: Unsigned elements
- **WHEN** `DIM u(2) AS _UNSIGNED INTEGER: u(1) = -1: PRINT u(1); u(2)` runs
- **THEN** it prints ` 65535  0 `

#### Scenario: Bit array
- **WHEN** `DIM b4(3) AS _BIT * 4: b4(0) = 7: b4(1) = 8: b4(3) = 15: PRINT b4(0); b4(1); b4(2); b4(3)` runs
- **THEN** it prints ` 7 -8  0 -1 `

#### Scenario: SWAP of bit elements
- **WHEN** `SWAP b4(0), b4(1)` is compiled for that array
- **THEN** it is a compile error

#### Scenario: DIM with empty parentheses
- **WHEN** `DIM a() AS LONG: a = 100000.5: PRINT a; LEN(a)` runs
- **THEN** it prints ` 100000  4 ` (`a` is a scalar; `a(1)` after it is an array used without `DIM`)

### Requirement: Array elements
`name(i, …)` SHALL name an element of an array visible at that point, with one index per dimension; each index
SHALL be converted to a 64-bit integer as for an assignment (a float rounded half to even). An index outside its
own dimension's bounds SHALL raise error 9, and a read with such an index SHALL give the value of the array's first
element. A wrong number of indexes and a string index SHALL be compile errors. An element SHALL be readable,
assignable, and passable by reference to a parameter of its exact type; in parentheses or to a parameter of another
type it SHALL be passed as a copy. An element SHALL NOT be a `FOR` variable (compile error).

#### Scenario: Index out of range
- **WHEN** `x(11) = 5` runs for `DIM x(10)` under an error handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 and no element of `x` changes

#### Scenario: Each dimension checked
- **WHEN** `PRINT m(1, 4)` runs for `DIM m(2, 3)` under the same handler
- **THEN** the handler prints 9

#### Scenario: Float index
- **WHEN** `x(i) = i * 10` for `i` from 0 to 10 and `PRINT x(1.5); x(2.5); x(-0.5)` runs
- **THEN** it prints `20  20  0`

#### Scenario: Read with a bad index
- **WHEN** `x(0) = 77: y = x(11)` runs under the same handler
- **THEN** the handler prints 9 and `y` is 77

#### Scenario: Element passed by reference
- **WHEN** `SUB bump (v AS LONG)` adds 1 to `v` and is called as `bump x(3)` for a LONG array `x`
- **THEN** `x(3)` is 1 higher afterwards; called as `bump (x(3))` it is unchanged

#### Scenario: Element with a bad index passed to a SUB
- **WHEN** `bump x(9)` runs for `DIM x(5) AS LONG` under the same handler
- **THEN** the handler prints 9 and `bump` does not run

#### Scenario: Wrong number of indexes
- **WHEN** `x(1, 2) = 5` is compiled for `DIM x(10)`
- **THEN** the compiler reports an error

### Requirement: LBOUND and UBOUND
`LBOUND(array[, dimension])` and `UBOUND(array[, dimension])` SHALL give the lower and upper bound of a dimension
of an array (the first without `dimension`), typed `_INTEGER64`. The dimension SHALL be rounded half to even; a
dimension outside 1 to the array's number of dimensions SHALL raise error 9 when it runs. The array SHALL be an
array variable, an array parameter or a member array (`x(0).n`). A name that is no array SHALL make an implicit
array (requirement "Implicit arrays"). Of a dynamic array that is not allocated both SHALL raise error 9.
`LBOUND(a())` SHALL be a compile error for an array variable; a member array MAY be written with the empty
parentheses, and its bounds are given without evaluating the indexes of the place it is part of (requirement
"Member arrays").

#### Scenario: Both bounds
- **WHEN** `PRINT LBOUND(b); UBOUND(b); LBOUND(b, 2); UBOUND(b, 2)` runs for `DIM b(-2 TO 3, 5)`
- **THEN** it prints `-2  3  0  5`

#### Scenario: Typed _INTEGER64
- **WHEN** `PRINT UBOUND(b) / 7` runs for the same array
- **THEN** it prints `.4285714285714285`

#### Scenario: Dimension out of range
- **WHEN** `PRINT LBOUND(b, 3)` runs for the same array under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9

#### Scenario: Name that is no array
- **WHEN** `PRINT LBOUND(q); UBOUND(q)` runs and no array `q` is declared
- **THEN** it prints ` 0  10 `

#### Scenario: Member array
- **WHEN** `PRINT LBOUND(x(0).n); UBOUND(x(0).n)` runs for a member declared `n(1 TO 4) AS LONG`
- **THEN** it prints ` 1  4 `

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

### Requirement: Member access and dotted names
`a.b` SHALL be a member access when a scalar variable `a` of a user type is visible at that point, and a plain
variable named `a.b` otherwise, also before a later `DIM a AS t` (measured, `verification/v16_m8_*`,
`v18_h_dotted_before_dim`); when an array `a` of a user type is visible and no such scalar, `a.b` SHALL be a compile
error. A member that its type does not have, a member written with another type's suffix, and a member of an
element of a numeric array SHALL be compile errors. `a(i).m` SHALL be the member `m` of element `a(i)`. A member SHALL
be readable, assignable, and passable by reference to a parameter of its exact type. Blanks around the dot after a
plain name SHALL be dropped: `a . b` SHALL be `a.b` in every one of these uses (`verification/v19_dot_blanks_*`).

#### Scenario: Dotted name without a TYPE
- **WHEN** `a.b = 3: a = 4: PRINT a.b; a` runs and no user-type variable `a` exists
- **THEN** it prints `3  4`

#### Scenario: Dotted name before the DIM
- **WHEN** `a.b = 3: PRINT a.b` runs, then `DIM a AS t` (a type with a LONG member `b`), then `PRINT a.b`
- **THEN** it prints `3`, then `0`

#### Scenario: Unknown member
- **WHEN** `a.c = 6` is compiled for a variable `a` of a type without a member `c`
- **THEN** the compiler reports an error

#### Scenario: Member of an element
- **WHEN** `a(1).b = 8` runs for `DIM a(2) AS t`
- **THEN** `PRINT a(1).b; a(2).b` prints `8  0`

#### Scenario: Member passed by reference
- **WHEN** `bump p.x` is called for a LONG member `x`
- **THEN** `p.x` is 1 higher afterwards

#### Scenario: Blanks around the dot of a member
- **WHEN** `a . s = 5: PRINT a . s: PRINT a.s` runs for `DIM a AS t` (a type with a LONG member `s`)
- **THEN** it prints ` 5 ` twice (`v19_dot_blanks_type`)

#### Scenario: Blanks around the dot of a plain dotted name
- **WHEN** `a.b = 3: PRINT a . b` runs and no user-type variable `a` exists
- **THEN** it prints ` 3 ` (`v19_dot_blanks_plain`)

#### Scenario: ERASE of a member array with blanks
- **WHEN** `ERASE a . S, b` runs for a member array `s(1 TO 3) AS LONG` of `a` and an array `b`
- **THEN** both are cleared, as with the old compiler (`v19_dot_blanks_erase`, `arrays/04_float_family`)

#### Scenario: An element of a member array with blanks
- **WHEN** `a . s(2) = 7: PRINT a.s(2)` runs for that member array
- **THEN** it prints ` 7 ` (`v47_c_dot_blanks`, which also has `LBOUND(a . s)`, `a . w . v`, `p . q . r` and a
  member passed by reference)

#### Scenario: A suffix before the dot
- **WHEN** a program has `s$ . x = "hi"`
- **THEN** the compiler reports an error that is not "not supported yet", as the old compiler does (`v47_x29`)

### Requirement: Dynamic arrays
An array SHALL be dynamic when the first declaration of it in the file is a `REDIM`, a `DIM` with a bound that is
not a constant expression, a `DIM` or `STATIC name(bounds)` while `$DYNAMIC` is in effect, a `DIM` in a procedure
that is not `STATIC`, or, while `$DYNAMIC` is in effect, a use without `DIM`. Its bounds SHALL be evaluated, and
its elements made (zero or empty), when the statement runs. `DIM` of a dynamic array that is allocated SHALL raise
error 10 and leave it unchanged, whether it is the statement that allocated it or another; a `DIM` with a bound
that is not constant of an array that is static SHALL raise error 10 likewise. A dimension with no element (its
upper bound one below its lower bound) SHALL raise error 5 and leave the array allocated with those bounds and no
element; a dimension whose upper bound is lower still, and a request the machine cannot hold, SHALL end the
program with "Out of memory", which no error handler receives. A bound whose evaluation raises an error SHALL NOT
stop the allocation. Using an element of a dynamic array that is not allocated, and `LBOUND` or `UBOUND` of it,
SHALL raise error 9. An array used without `DIM` while `$DYNAMIC` is in effect SHALL be allocated by the first
statement in the file that names it, where that statement evaluates the name: when the statement runs again the
allocation SHALL raise error 10 as an error of that statement, after what the statement did before (the earlier
items of a `PRINT`, `WRITE` or `READ`, the prompt of an `INPUT`) and in place of what it does after. One first
named in a bound of `REDIM` SHALL be dynamic wherever it stands and SHALL be allocated as that `REDIM` allocates
(anew on each run, or kept under `_PRESERVE`), raising no error 10. A dimension whose bounds are constants SHALL be checked when compiling as
in a static array, whatever the other dimensions are: a single negative bound, and `lower TO upper` in the wrong
order, SHALL be compile errors. A `DIM` or `REDIM` with another number of dimensions than the array has SHALL be
a compile error. A `DIM` or `REDIM` of several arrays SHALL be one statement: an error in the allocation of one
SHALL NOT keep the arrays after it in the list from being allocated, a second allocation that would raise error
10 SHALL raise nothing while that error is pending, and `RESUME NEXT` SHALL go on after the whole list.

#### Scenario: A constant dimension beside a variable one
- **WHEN** `DIM half(n TO 2, -1)` or `DIM part(n, 3 TO 1)` is compiled for a variable `n`
- **THEN** each is a compile error; `DIM none(n TO 2)` compiles, and with `n = 3` raises error 5 when it runs

#### Scenario: A second constant DIM below $DYNAMIC
- **WHEN** `DIM st(5) AS LONG` is followed, below a line `'$DYNAMIC`, by `DIM st(5) AS LONG: PRINT UBOUND(st)` under
  a handler that prints `ERR` and resumes next
- **THEN** it compiles, the handler prints 10, and ` 5 ` is printed

#### Scenario: Bound from a variable
- **WHEN** `n = 4: DIM d(n) AS LONG: d(4) = 9: PRINT UBOUND(d); d(4)` runs
- **THEN** it prints ` 4  9 `

#### Scenario: DIM twice at run time
- **WHEN** `n = 4: DIM d(n) AS LONG: d(1) = 5: DIM d(n + 1) AS LONG` runs under a handler that prints `ERR` and
  resumes next, then `PRINT UBOUND(d); d(1)`
- **THEN** the handler prints 10, then ` 4  5 ` is printed

#### Scenario: $DYNAMIC
- **WHEN** a line `'$DYNAMIC` stands above `DIM a(5) AS LONG`, followed by `REDIM a(8) AS LONG: PRINT UBOUND(a)`
- **THEN** it prints ` 8 ` (without the metacommand the `REDIM` raises error 10)

#### Scenario: No element
- **WHEN** `n = -1: DIM d(n) AS LONG` runs under the same handler, then `PRINT LBOUND(d); UBOUND(d)`, then `d(0) =
  1`
- **THEN** the handler prints 5, then ` 0 -1 ` is printed, then the handler prints 9

#### Scenario: Fewer than no element
- **WHEN** `n = -3: DIM d(n) AS LONG` runs under the same handler
- **THEN** the program ends with "Out of memory" and the handler does not run

#### Scenario: A bound with an operator
- **WHEN** `DIM a3(2 + 3) AS LONG: REDIM a3(20) AS LONG: PRINT UBOUND(a3)` runs, and `DIM a1(5) AS LONG: REDIM
  a1(20) AS LONG: PRINT UBOUND(a1)` under the same handler
- **THEN** the first prints ` 20 ` (a dynamic array); for the second the handler prints 10 and ` 5 ` is printed

#### Scenario: Two DIM statements
- **WHEN** `DIM a(5) AS LONG` is followed by `n = 3: DIM a(n) AS LONG: PRINT UBOUND(a)` under the same handler
- **THEN** the handler prints 10, then ` 5 ` is printed

#### Scenario: Several arrays in one DIM, run twice
- **WHEN** `DIM du(i + 1), dv(i + 3)` runs for `i` 1 and 2, with `ERASE dv` before the second run, under the same
  handler, and `PRINT UBOUND(du); UBOUND(dv)` follows
- **THEN** the handler prints 10 once and ` 2  5 ` is printed

#### Scenario: A bound that raises
- **WHEN** `DIM s(ASC("")) AS LONG: PRINT UBOUND(s)` runs under the same handler
- **THEN** the handler prints 5, then ` 0 ` is printed

#### Scenario: Used without DIM below $DYNAMIC
- **WHEN** below a line `'$DYNAMIC`, `im(3) = 4: PRINT UBOUND(im): REDIM im(20): PRINT UBOUND(im)` runs
- **THEN** it prints ` 10 `, then ` 20 `; with the first statement skipped by `GOTO`, `UBOUND(im)` raises error 9

#### Scenario: The first use run three times
- **WHEN** below a line `'$DYNAMIC`, `FOR i = 1 TO 3: im(3) = im(3) + 1: NEXT: PRINT im(3)` runs under a handler
  that prints `ERR` and resumes next
- **THEN** the handler prints 10 twice, then ` 1 ` is printed (the store of the second and third round is skipped)

#### Scenario: The first use in a later item
- **WHEN** below a line `'$DYNAMIC`, `FOR i = 1 TO 2: PRINT " x"; dp(1); " y";: NEXT` runs under such a handler
- **THEN** it prints ` x 0  y x`, then the handler prints 10 (the second round prints its first item, then the
  allocation raises)

#### Scenario: The first use in a bound of REDIM
- **WHEN** `FOR i = 1 TO 2: REDIM rb(rc(1) + i): PRINT UBOUND(rb);: rc(1) = 5: NEXT: REDIM rc(20): PRINT UBOUND(rc)`
  runs, with no `$DYNAMIC`
- **THEN** it prints ` 1  2  20 ` and raises no error (`rc` is dynamic and each round makes it anew)

### Requirement: REDIM
`REDIM [SHARED] name(bounds) [AS type]` SHALL declare the dynamic array if it does not exist and SHALL give it new
bounds and new elements (zero or empty) each time it runs. Without a type, `REDIM` of an array declared with `AS`
SHALL keep that type. `REDIM` of a static array SHALL raise error 10 and leave it unchanged. Another number of
dimensions than the array has, another type written with `AS` than the array was declared with, and a change
between `STRING` and `STRING * n` or of n, SHALL be compile errors. A dimension with no element, a lower upper
bound still and a bound that raises SHALL behave as for `DIM` (requirement "Dynamic arrays"), the old elements
being gone. `REDIM name()` and `REDIM name AS type` SHALL declare a scalar, as `DIM name()` does. The words after
`REDIM` SHALL be one of `_PRESERVE` and `_RETAIN`, then `SHARED`; `REDIM SHARED _PRESERVE`, `REDIM SHARED
_RETAIN`, both words together in either order, either word twice, `REDIM _RETAIN` with nothing after it and `DIM
_RETAIN` SHALL be compile errors. `SHARED` SHALL
share the array only in the statement that declares it: on a later `REDIM` of the array it SHALL change nothing,
as in the old compiler. `PRESERVE` and `RETAIN` without their underscore SHALL be names like any other (`REDIM
preserve(3)` declares an array of that name).

#### Scenario: SHARED on a later REDIM
- **WHEN** `REDIM ns(3) AS LONG`, then `REDIM SHARED ns(4) AS LONG: PRINT UBOUND(ns)` run
- **THEN** it prints ` 4 `, and `ns` is not visible in a SUB below

#### Scenario: New bounds, new elements
- **WHEN** `REDIM r(5) AS LONG: r(2) = 7: REDIM r(1 TO 20) AS LONG: PRINT LBOUND(r); UBOUND(r); r(2)` runs
- **THEN** it prints ` 1  20  0 `

#### Scenario: Type kept
- **WHEN** `REDIM x(5) AS LONG: REDIM x(10): x(1) = 100000: PRINT x(1)` runs
- **THEN** it prints ` 100000 `

#### Scenario: REDIM of a static array
- **WHEN** `DIM s(5) AS LONG: s(1) = 3: REDIM s(9) AS LONG` runs under a handler that prints `ERR` and resumes next,
  then `PRINT UBOUND(s); s(1)`
- **THEN** the handler prints 10, then ` 5  3 ` is printed

#### Scenario: Another number of dimensions
- **WHEN** `REDIM m(2, 3): REDIM m(4)` is compiled
- **THEN** it is a compile error

#### Scenario: Another type
- **WHEN** `REDIM x(5) AS LONG: REDIM x(5) AS INTEGER` is compiled
- **THEN** it is a compile error; `REDIM x&(7)` in its place names the same array and `REDIM x!(7)` another one

#### Scenario: REDIM of an array made by DIM
- **WHEN** `n = 3: DIM x(n) AS LONG: REDIM x(7): x(1) = 100000: PRINT UBOUND(x); x(1)` runs
- **THEN** it prints ` 7  100000 `

#### Scenario: The words after REDIM
- **WHEN** `REDIM a(3) AS LONG` is followed by `REDIM SHARED _RETAIN a(5) AS LONG`, by `REDIM _PRESERVE _RETAIN
  a(5) AS LONG`, by `REDIM _RETAIN _PRESERVE a(5) AS LONG` or by `REDIM _RETAIN _RETAIN a(5) AS LONG`, each in a
  program of its own, or a program holds `REDIM _RETAIN` alone or `DIM _RETAIN a(3) AS LONG`
- **THEN** each program is a compile error, and `REDIM _RETAIN SHARED a(5) AS LONG` in the main module is not

### Requirement: REDIM _PRESERVE
`REDIM _PRESERVE` SHALL keep the values of the elements by their flat position (first dimension fastest), as many
as the smaller of the old and the new number of elements; new elements SHALL be zero or empty. Of an array that
is not allocated it SHALL behave as `REDIM`.

#### Scenario: Grow and shrink
- **WHEN** `REDIM p(3) AS LONG`, `p(i) = i + 1` for 0 to 3, `REDIM _PRESERVE p(5) AS LONG: PRINT p(3); p(5)`, then
  `REDIM _PRESERVE p(1) AS LONG: PRINT UBOUND(p); p(1)` run
- **THEN** it prints ` 4  0 `, then ` 1  2 `

#### Scenario: Strings
- **WHEN** `REDIM t(1) AS STRING: t(1) = "keep": REDIM _PRESERVE t(4) AS STRING: PRINT t(1); LEN(t(4))` runs
- **THEN** it prints `keep 0 `

#### Scenario: Two dimensions keep positions, not coordinates
- **WHEN** a `REDIM g(1, 1) AS LONG` array holding 1 to 4 in flat order is given `REDIM _PRESERVE g(2, 1) AS LONG`
  and all six elements are printed
- **THEN** `g(0, 0)`, `g(1, 0)`, `g(2, 0)` are 1, 2, 3 and `g(0, 1)`, `g(1, 1)`, `g(2, 1)` are 4, 0, 0 (the four
  values in the first four flat positions)

#### Scenario: Another lower bound
- **WHEN** a `REDIM b(3) AS LONG` array holding 1 to 4 is given `REDIM _PRESERVE b(10 TO 15) AS LONG`
- **THEN** `b(10)` to `b(15)` are 1, 2, 3, 4, 0, 0

#### Scenario: Static array
- **WHEN** `DIM s(5) AS LONG: s(1) = 3: REDIM _PRESERVE s(9) AS LONG` runs under a handler that prints `ERR` and
  resumes next, then `PRINT UBOUND(s); s(1)`
- **THEN** the handler prints 10, then ` 5  3 ` is printed

### Requirement: REDIM _RETAIN
`REDIM _RETAIN` SHALL give the array new bounds and SHALL keep each element whose indexes lie within both the old
and the new bounds of every dimension, at the same indexes; every other element SHALL be zero or empty. In all
else it SHALL behave as `REDIM _PRESERVE`, for every kind of element and every kind of array, and on a member
array it SHALL change nothing.

#### Scenario: One dimension moved
- **WHEN** `REDIM a(1 TO 5) AS LONG` holds 10, 20, 30, 40, 50, then `REDIM _RETAIN a(3 TO 7) AS LONG` runs and
  `a(3)` to `a(7)` are printed
- **THEN** it prints ` 30  40  50  0  0 `

#### Scenario: Grown and shrunk
- **WHEN** that array is then given `REDIM _RETAIN a(0 TO 9) AS LONG`, and after it `REDIM _RETAIN a(4 TO 5) AS
  LONG`
- **THEN** `a(0)` to `a(9)` are 0, 0, 0, 30, 40, 50, 0, 0, 0, 0, then `a(4)` and `a(5)` are 40 and 50

#### Scenario: Two dimensions keep coordinates
- **WHEN** `REDIM m(1 TO 3, 1 TO 4) AS INTEGER` holds `10 * i + j` at `m(i, j)`, then `REDIM _RETAIN m(2 TO 5, 0
  TO 3) AS INTEGER` runs
- **THEN** `m(2, 1)` to `m(2, 3)` are 21, 22, 23, `m(3, 1)` to `m(3, 3)` are 31, 32, 33, and the other elements
  are 0

#### Scenario: Ranges that do not meet
- **WHEN** an array `a(4 TO 5) AS LONG` holding 40 and 50 is given `REDIM _RETAIN a(100 TO 102) AS LONG`
- **THEN** its three elements are 0

#### Scenario: Strings, fixed-length strings and bits
- **WHEN** `s(1 TO 4) AS STRING` holding `A` to `D` is given `REDIM _RETAIN s(3 TO 6) AS STRING`, `x(1 TO 3) AS
  STRING * 3` holding `aaa`, `bbb`, `ccc` is given `REDIM _RETAIN x(2 TO 5) AS STRING * 3`, and `p(0 TO 9) AS
  _UNSIGNED _BIT * 5` holding `3 * i` is given `REDIM _RETAIN p(5 TO 14) AS _UNSIGNED _BIT * 5`
- **THEN** `s(3)` to `s(6)` are `C`, `D` and two empty strings, `x(2)` to `x(5)` are `bbb`, `ccc` and two strings
  of three NUL bytes, and `p(5)` to `p(14)` are 15, 18, 21, 24, 27 and five zeros

#### Scenario: User types
- **WHEN** an array of a user type with a `STRING` member, a `STRING * 3` member and a `LONG` is given
  `REDIM _RETAIN` with bounds that move it by one, and another of a user type with a member array is given
  `REDIM _RETAIN` with a second dimension moved by one
- **THEN** the elements at kept indexes have every member as before, member arrays included, and the others are
  zero or empty

#### Scenario: Not allocated
- **WHEN** `REDIM _RETAIN f(2 TO 4) AS LONG` is the first statement of `f`, and later, after `ERASE f`, `REDIM
  _RETAIN f(1 TO 5) AS LONG` runs
- **THEN** both give an array of zeros with those bounds

#### Scenario: Static array
- **WHEN** `DIM st(5) AS LONG: st(1) = 3: REDIM _RETAIN st(9) AS LONG` runs under a handler that prints `ERR` and
  resumes next, then `PRINT UBOUND(st); st(1)`
- **THEN** the handler prints 10, then ` 5  3 ` is printed

#### Scenario: In a procedure
- **WHEN** a SUB with a parameter `p() AS LONG` does `REDIM _RETAIN p(0 TO 5) AS LONG` and is called for an array
  `w(1 TO 3)` holding 1, 2, 3
- **THEN** `w(0)` to `w(5)` are 0, 1, 2, 3, 0, 0 after the call

#### Scenario: Member array
- **WHEN** a type has `v(1 TO 3) AS LONG`, `DIM x AS` that type`: x.v(2) = 5: REDIM _RETAIN x.v(1 TO 3) AS LONG:
  PRINT x.v(2)` runs
- **THEN** it prints ` 5 `

### Requirement: ERASE
`ERASE name[, name…]` of a static array SHALL set every element to zero, an empty string or NUL bytes and keep its
bounds. Of a dynamic array it SHALL release the elements, after which the array is not allocated and may be
allocated again with `DIM` or `REDIM`; of a dynamic array that is not allocated it SHALL do nothing. Anything that
is no declared array SHALL be a compile error: a scalar, a name never used (no array is made for it), `name()`,
an element, a suffix of another type than the array's. A scalar of the array's name SHALL NOT change.

#### Scenario: Static array
- **WHEN** `DIM a(5) AS LONG: a(2) = 9: ERASE a: PRINT UBOUND(a); a(2)` runs
- **THEN** it prints ` 5  0 `

#### Scenario: Dynamic array
- **WHEN** `REDIM d(5) AS LONG: ERASE d: PRINT UBOUND(d)` runs under a handler that prints `ERR` and resumes next,
  then `REDIM d(2) AS LONG: PRINT UBOUND(d)`
- **THEN** the handler prints 9, then ` 2 ` is printed

#### Scenario: Array of a user type with a string member
- **WHEN** `ERASE` runs on a static array of a type with a `STRING` member whose members were set
- **THEN** every numeric member is 0 and every string member is empty

### Requirement: OPTION BASE
`OPTION BASE 1` SHALL make 1 the lower bound of every dimension declared below it in the file without `TO`, in
arrays (declared, used without `DIM`, static and dynamic) and in member arrays; `OPTION BASE 0` SHALL restore 0.
The statement MAY stand anywhere, any number of times, and SHALL apply by its place in the file, not by whether it
runs: one inside a SUB or FUNCTION applies to the lines below the procedure too. The word after `BASE` SHALL be
`0` or `1` in plain digits (zeros before it allowed: `01`), anything else (`2`, `1.0`, `1&`, `&H1`, a name) being a
compile error. A dimension written as the number 0 in plain digits SHALL stay
`0 TO 0`; any other constant bound that is 0 (`0&`, `0.0`, `(0)`, `0 + 0`, a `CONST`) SHALL make, under base 1, a
static array without elements, for which the program SHALL raise error 5 when it starts. Bounds written with `TO`
SHALL NOT change.

#### Scenario: Base 1
- **WHEN** `OPTION BASE 1: DIM a(3): DIM b(2 TO 4): PRINT LBOUND(a); UBOUND(a); LBOUND(b)` runs
- **THEN** it prints ` 1  3  2 `

#### Scenario: DIM a(0) under base 1
- **WHEN** `OPTION BASE 1: DIM z(0): PRINT LBOUND(z); UBOUND(z)` runs
- **THEN** it prints ` 0  0 `

#### Scenario: Only later arrays
- **WHEN** `DIM a(3): OPTION BASE 1: DIM b(3): PRINT LBOUND(a); LBOUND(b)` runs
- **THEN** it prints ` 0  1 `

#### Scenario: The literal 0 per dimension
- **WHEN** `OPTION BASE 1: DIM z2(0, 3): PRINT LBOUND(z2, 1); UBOUND(z2, 1); LBOUND(z2, 2); UBOUND(z2, 2)` runs
- **THEN** it prints ` 0  0  1  3 `

#### Scenario: A constant that is 0 but not written 0
- **WHEN** `OPTION BASE 1: CONST zero = 0: DIM a(zero): PRINT LBOUND(a)` is compiled and run
- **THEN** it compiles, and the program ends with "Illegal function call" before printing (the array is static,
  1 to 0, and the error is raised when the program starts)

#### Scenario: An expression that is 0
- **WHEN** `PRINT "first line"`, `OPTION BASE 1`, `DIM a(0 + 0)`, `PRINT LBOUND(a); UBOUND(a)` run under a handler
  that prints `ERR` and resumes next, set before them
- **THEN** `first line` is printed, the handler prints 5, then ` 1  0 ` is printed (the array is dynamic and the
  error is raised at the `DIM`)

#### Scenario: A statement that does not run
- **WHEN** `GOTO skip`, `OPTION BASE 1`, `skip: DIM e(3): PRINT LBOUND(e)` stand on three lines
- **THEN** it prints ` 1 `

#### Scenario: Another number
- **WHEN** `OPTION BASE 2`, `OPTION BASE` or `OPTION BASE n` is compiled
- **THEN** each is a compile error

### Requirement: Implicit arrays
`name(index, …)` where no array of that name and type is visible, the name with its suffix may be a variable, no
SUB or FUNCTION of that name exists and the spelling calls no built-in function SHALL make an array of the scope
it stands in, with the bounds `OPTION BASE` to 10 in each of as many dimensions as indexes are written;
`LBOUND(name)` and `UBOUND(name)` of such a name SHALL make one of one dimension. In the main module, and in a
`SUB … STATIC` or `FUNCTION … STATIC`, the array SHALL be static (dynamic in the main module while `$DYNAMIC` is
in effect, requirement "Dynamic arrays"). In any other procedure, and in every procedure while `$DYNAMIC` is in
effect, it SHALL be a local dynamic array that is not allocated, so that every use raises error 9 until a `DIM`
or `REDIM` of it in the procedure runs, as in the old compiler. Under `OPTION _EXPLICITARRAY` or `OPTION
_EXPLICIT`, wherever in the program the statement stands, an implicit array SHALL be a compile error instead. A
scalar variable of the name, also one typed with `AS`, and a `CONST` of the name SHALL NOT keep the name from
being an array and SHALL NOT give it their type. `ERASE` of an implicit array SHALL clear a static one and
release a dynamic one; above its first use in the file it SHALL be a compile error. A later `DIM` of a static
one, and a use with another number of indexes, SHALL be compile errors. A name that may not be a variable (one leading underscore, a keyword, a built-in statement)
and a store to a spelling that is a FUNCTION or a built-in function SHALL be compile errors.

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
- **THEN** each is a compile error

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

### Requirement: COMMON
`COMMON name [AS type], …` in the main module SHALL declare each variable as `DIM` does, and `COMMON SHARED` as
`DIM SHARED` does, wherever it stands in the main module. `COMMON name()` SHALL declare a dynamic array that is
not allocated, shared after `COMMON SHARED`; what is written between its parentheses SHALL be ignored. A `DIM` of
that name and type below it SHALL allocate the array when it runs (error 10 if it is allocated), a `REDIM` SHALL
give it new bounds, and a use before either SHALL raise error 9. Its number of dimensions SHALL be that of the
first index, `DIM` or `REDIM` that names it; `LBOUND` and `UBOUND` of it SHALL NOT fix that number. A block name
(`COMMON /name/ …`) SHALL be a compile error, as in the old compiler. `COMMON` inside a SUB or FUNCTION, and a second declaration of a `COMMON`
scalar, SHALL be compile errors. `CHAIN` SHALL be reported "not supported yet".

#### Scenario: COMMON SHARED scalar
- **WHEN** `COMMON SHARED total AS LONG` stands in the main module, `total = 5` runs, and a SUB prints `total`
- **THEN** the SUB prints ` 5 `

#### Scenario: COMMON SHARED array
- **WHEN** `COMMON SHARED cs() AS LONG`, then `DIM cs(3) AS LONG: cs(2) = 6`, and a SUB prints `cs(2)`
- **THEN** the SUB prints ` 6 `

#### Scenario: COMMON alone is not shared
- **WHEN** `COMMON v AS LONG: v = 5` stands in the main module and a SUB without `SHARED` prints `v`
- **THEN** the SUB prints ` 0 `

#### Scenario: COMMON in a SUB
- **WHEN** `COMMON x` stands inside a SUB
- **THEN** it is a compile error

#### Scenario: COMMON array is dynamic
- **WHEN** `COMMON SHARED a() AS LONG: DIM a(3) AS LONG: a(1) = 5: REDIM a(9) AS LONG: PRINT UBOUND(a); a(1)` runs
- **THEN** it prints ` 9  0 `

#### Scenario: COMMON array never given bounds
- **WHEN** `COMMON a() AS LONG: a(1) = 1` runs under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9

#### Scenario: Bounds read before the DIM
- **WHEN** `COMMON c(): PRINT UBOUND(c);: DIM c(2, 5): c(1, 2) = 8: PRINT UBOUND(c, 2); c(1, 2)` runs under the
  same handler
- **THEN** it compiles, the handler prints 9, and ` 5  8 ` is printed

#### Scenario: Another type is another array
- **WHEN** `COMMON SHARED a()`, then `DIM a(3) AS LONG: a(2) = 6`, and a SUB prints `a(2)` under the same handler
- **THEN** the handler prints 9 (the LONG array is not the shared one)

#### Scenario: Block name
- **WHEN** `COMMON /blk/ b1 AS LONG` is compiled
- **THEN** it is a compile error

### Requirement: DIM AS type lists
`DIM AS type name, name(bounds), …` (also after `SHARED`, and in `REDIM`, `STATIC` and `COMMON`) SHALL declare
every name of the list with that type. A type suffix on a name of such a list SHALL be a compile error.

#### Scenario: Scalars and an array
- **WHEN** `DIM AS LONG a, b(3): a = 100000: b(3) = a: PRINT a; b(3)` runs
- **THEN** it prints ` 100000  100000 `

#### Scenario: Suffix in the list
- **WHEN** `DIM AS LONG a%` is compiled
- **THEN** it is a compile error

### Requirement: Member arrays
A `TYPE` member SHALL be declarable as an array with constant bounds, as `name(bounds) AS type` or in an
`AS type name(bounds), …` list, of any type a member may have and of one or more dimensions. A dimension without
`TO` SHALL start at the `OPTION BASE` in effect where the `TYPE` block stands; the literal 0 has no rule of its
own there (`z(0)` under base 1 is a compile error, as are bounds that are not constant, reversed or missing).
A bound written with numbers alone, also as an expression of numbers, SHALL be accepted; one that names a
`CONST`, which the old compiler accepts, SHALL be reported "not supported yet" (the blocks are read before the
program's constants).
Below `$UNSTABLE:TYPEFIELDS` (`language/unstable-features`) a member array SHALL take the marker `_STATIC` or
`_DYNAMIC` before its name, after its name or bounds, or after its type, and in an `AS type` list after the
bounds of a name or before it, where it marks that name alone. `_STATIC` SHALL change nothing: the member array
is the one this requirement describes. `_DYNAMIC` SHALL make it a dynamic member array (requirement "Dynamic
member arrays"), to which the rest of this requirement does not apply. A marker without the flag, both markers,
one marker twice, a marker on a member that is no array, a marker between `AS` and the type and a marker before
an `AS type` list SHALL be compile errors.
`place.name(index, …)` SHALL name an element, through any chain of members and elements, with the index and
by-reference rules of array elements; a member array named without an index, with nothing between its
parentheses (except as an array argument, as a side of a whole-array assignment, requirement "Whole-array
assignment of member arrays", and as a side of `_ARRAYCOPY`, requirement "_ARRAYCOPY element types and places") or with another number of indexes than its dimensions SHALL be a compile error,
as an index after a member that is no array SHALL be. A store SHALL evaluate a numeric value before the indexes and a string value
after them (as the old compiler's program does), the indexes from left to
right, and SHALL store nothing when an index is outside its bounds (error 9; the old compiler writes element 0 of
that dimension, `DIVERGENCES.md` D-004). The elements of a member array SHALL be part of the value of their
variable: counted by `LEN`, zero or empty when the variable is made, and each variable and each array element
SHALL have its own. The bounds of a member array SHALL NOT change: `REDIM` of one SHALL be accepted only with
constant bounds equal to the declared ones, written `lower TO upper` or with `AS` and the member's type (the
upper bound alone then starts at the `OPTION BASE` in effect at the `REDIM`), with or without `SHARED`, and
SHALL then set the elements of that one array to zero or empty; `REDIM _PRESERVE` so written SHALL change
nothing and SHALL NOT evaluate the indexes of the place; `ERASE` of one, also written with empty parentheses,
SHALL set its elements to zero or empty. `ERASE` and `REDIM` SHALL find the place the member array is part of
once and SHALL NOT be guarded: an index outside its bounds on the way SHALL raise error 9, and the member array
of the element at position 0 of that dimension SHALL then be cleared, as in the old compiler. `LBOUND` and
`UBOUND` SHALL take a member array, also written with empty parentheses; they and `LEN` SHALL NOT evaluate the
indexes of the place it is part of, and an index outside its bounds there SHALL raise nothing. A whole member
array written with empty parentheses SHALL be an argument for an array parameter of its element type: the
procedure works on its elements, with the member's bounds, as on a static array (`ERASE` clears, `REDIM` raises
error 10); an index outside its bounds on the way SHALL raise error 9 and the procedure SHALL NOT be entered.
`SWAP` SHALL take an element of a member array as an argument by reference takes it. A member array of an array
element beside other targets of one `ERASE` or `REDIM`, and an element of a member array as a target of `INPUT`,
`READ` or the `MID$` statement, SHALL be reported "not supported yet".

#### Scenario: Element of a member array
- **WHEN** a type has `v(0 TO 3) AS LONG`, and `DIM t AS` that type`: t.v(2) = 7: PRINT t.v(2); t.v(3)` runs
- **THEN** it prints ` 7  0 `

#### Scenario: Nested chain
- **WHEN** `TYPE inner` has `AS INTEGER q(2), r`, `TYPE outer` has `AS inner n(1), m`, and for `REDIM x(0) AS outer`
  the statements `x(0).n(1).q(2) = 77: x(0).m.r = 88: PRINT x(0).n(1).q(2); x(0).m.r` run
- **THEN** it prints ` 77  88 `

#### Scenario: Each element has its own
- **WHEN** `DIM a(1) AS` a type with `v(3) AS LONG`, and `a(0).v(1) = 5: PRINT a(0).v(1); a(1).v(1)` runs
- **THEN** it prints ` 5  0 `

#### Scenario: Index out of range
- **WHEN** `t.v(4) = 1` runs for `v(0 TO 3) AS LONG` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9

#### Scenario: Index outside in a store
- **WHEN** `t.v(0) = 100: t.v(9) = 5` runs under the same handler, then `PRINT t.v(0)`
- **THEN** the handler prints 9 and ` 100 ` is printed (the old compiler prints ` 5 `, D-004)

#### Scenario: Value before the indexes
- **WHEN** `a(one(1)).v(one(2)) = one(3)` runs, where FUNCTION `one` prints its argument and returns 1
- **THEN** ` 3  1  2 ` is printed

#### Scenario: A string after the indexes
- **WHEN** `z$ = "before": u.s(setz(1)) = z$` runs for a member `s(0 TO 1) AS STRING`, where FUNCTION `setz`
  stores `"after"` into the shared `z$` and returns its argument
- **THEN** `u.s(1)` is `after`

#### Scenario: Bounds without finding the place
- **WHEN** `PRINT UBOUND(a(one(1)).v); LBOUND(a(9).v)` runs for `DIM a(2)` of a type with `v(0 TO 3) AS LONG`,
  under a handler that prints `ERR`
- **THEN** it prints ` 3  0 `: `one` is not called and no error is raised

#### Scenario: Whole member array as an argument
- **WHEN** `fill t.v()` runs for `SUB fill (b() AS LONG)`, which stores `i * 2 + 1` into every `b(i)` from
  `LBOUND(b)` to `UBOUND(b)`, and `t.v(0)` and `t.v(3)` are printed
- **THEN** it prints ` 1  7 `, and the members beside the array keep their values; `fill a(9).v()` raises error 9
  and `fill` is not entered

#### Scenario: ERASE with an index outside
- **WHEN** `ERASE a(9).v` runs for `DIM a(2)` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 and the elements of `a(0).v` are 0, as with the old compiler

#### Scenario: Bound named by a constant
- **WHEN** a `TYPE` block has `v(top) AS LONG` below `CONST top = 3`
- **THEN** the compiler reports it "not supported yet" (the old compiler compiles it)

#### Scenario: Forms that are compile errors
- **WHEN** `t.v = 5`, `PRINT t.v()`, `t.v(1, 2) = 5`, `t.v("a") = 5`, `PRINT t.id(1)`, `PRINT LBOUND(t.id)` or
  `fill t.v` is compiled for `id AS LONG` and `v(0 TO 3) AS LONG`
- **THEN** each is a compile error

#### Scenario: LEN counts the elements
- **WHEN** a type has `id AS LONG`, `v(0 TO 3) AS LONG` and `tail AS INTEGER`, and `PRINT LEN(t)` runs for a
  variable of it
- **THEN** it prints ` 22 `

#### Scenario: OPTION BASE at the TYPE block
- **WHEN** `OPTION BASE 1` stands above a block with `v(3) AS LONG`, and in another program below it
- **THEN** `LBOUND(t.v)` is 1 in the first and 0 in the second

#### Scenario: REDIM of a member array
- **WHEN** `item(0).Values(i)` is set for 0 to 3, then `REDIM item(0).Values(0 TO 3)` runs and the four elements are
  printed
- **THEN** all four are 0, and the elements of `item(1).Values` are unchanged

#### Scenario: REDIM with other bounds
- **WHEN** `REDIM t.v(0 TO 9)`, `REDIM t.v(1 TO 4)`, `REDIM t.v(0 TO n)`, `REDIM t.v(3)`, `REDIM t.v(0 TO 3) AS
  INTEGER`, `REDIM AS LONG t.v(0 TO 3)` or, below `OPTION BASE 1`, `REDIM t.v(3) AS LONG` is compiled for
  `v(0 TO 3) AS LONG`
- **THEN** each is a compile error

#### Scenario: ERASE of a member array
- **WHEN** `ERASE t.v` runs after the elements and the members beside the array were set
- **THEN** the elements are 0, `UBOUND(t.v)` is 3, and the other members keep their values

#### Scenario: A _STATIC member
- **WHEN** below `$UNSTABLE:TYPEFIELDS` a type has `id AS INTEGER`, `v(1) _STATIC AS LONG` and `n AS INTEGER`,
  and `PRINT LEN(x)` runs for a variable of it; and in another program `REDIM x.v(0 TO 5) AS LONG` is compiled
- **THEN** the first prints ` 12 ` and the second is a compile error, as for an unmarked member

#### Scenario: Positions of the marker
- **WHEN** below the flag a member is written `_DYNAMIC v(1) AS LONG`, `v(1) _DYNAMIC AS LONG`, `v(1) AS LONG
  _DYNAMIC`, `AS LONG _DYNAMIC v(1), w(1)` or `AS LONG v(1) _DYNAMIC, w(1)`
- **THEN** each compiles; in the two lists `v` is a dynamic member array and `w` is not (`REDIM x.w(0 TO 5) AS
  LONG` is a compile error)

#### Scenario: Marked member
- **WHEN** a `TYPE` block has `v(0 TO 3) _DYNAMIC AS LONG` and no `$UNSTABLE:TYPEFIELDS` stands above it
- **THEN** it is a compile error that names the flag, as in the old compiler

#### Scenario: Marker errors
- **WHEN** below the flag a type has `v(1) _DYNAMIC _STATIC AS LONG`, `v(1) _DYNAMIC _DYNAMIC AS LONG`, `n
  _DYNAMIC AS LONG`, `n AS LONG _STATIC`, `AS _DYNAMIC LONG v(1)` or `_STATIC AS LONG v(1), w(2)`
- **THEN** each is a compile error

### Requirement: Whole user-type values
`q = p` for two places of the same user type (variables, array elements, members of a user type) SHALL copy every
member, a `STRING` member as its own string and a member array element by element; a place assigned to itself
SHALL keep its value. Places of two different user types, also two with the same members, and a number SHALL be
compile errors. `SWAP` of two places of the same user type SHALL exchange their values, `STRING` members
included. An index outside its bounds on either side SHALL raise error 9, and element 0 of that array SHALL then
stand in for the element named, as in the old compiler (`a(9) = p` overwrites `a(0)`; decided 2026-10-11, listed
for review in `SOMEDAY.md`). The indexes of the target SHALL be evaluated before those of the source. For a type
with a `STRING` member, in itself or in a member of a user type, both SHALL be evaluated again for every part of
the value, as in the old compiler: a `STRING` member is a part, a member of a user type that holds a `STRING`
gives its own parts, any other member is one part (measured, `verification\v24_x306`, `x307`; listed for review
in `SOMEDAY.md`). Each element of a `STRING` member array SHALL be a part, each element of a member array of a
user type that holds a `STRING` SHALL give its own parts, and any other member array SHALL be one part, all its
elements together (measured, `v24_x310`). A dynamic member array SHALL be copied deep (requirement "Dynamic
member arrays"); for an owner type the indexes SHALL be evaluated as many times, and in the order, the old
compiler's copy evaluates them (measured, `verification\v35_x56`; listed for review in `SOMEDAY.md`).

#### Scenario: Indexes evaluated per part
- **WHEN** `a(one(1)) = b(one(2))` runs, where FUNCTION `one` prints its argument and returns 1, for a type of a
  LONG and an INTEGER, and for a type of a LONG, a `STRING` and a `STRING * 2`
- **THEN** ` 1  2 ` is printed for the first and ` 1  2  1  2  1  2 ` for the second

#### Scenario: Indexes of an owner type
- **WHEN** the same runs for a type of a LONG, a `_DYNAMIC` LONG member array and a LONG, and for a type of a LONG,
  a `STRING`, a `_DYNAMIC` LONG member array and a `_DYNAMIC` `STRING` member array
- **THEN** the old compiler's sequences are printed: ` 1  2  1  2  1  2  1  2 ` and ` 1  2  1  2  1  2  1  2  1
  1  2  1 `

#### Scenario: Index outside
- **WHEN** `a(0).x = 100: p.x = 6: a(9) = p` runs for `DIM a(3)` under a handler that prints `ERR` and resumes
  next, then `PRINT a(0).x`
- **THEN** the handler prints 9 and ` 6 ` is printed

#### Scenario: Copy
- **WHEN** `p.x = 5: p.n = "a": q = p: p.x = 6: p.n = "b": PRINT q.x; q.n` runs for two variables of a type with a
  LONG `x` and a `STRING` `n`
- **THEN** it prints ` 5 a`

#### Scenario: Element to variable
- **WHEN** `a(2).x = 9: q = a(2): PRINT q.x` runs for an array `a` of the same type
- **THEN** it prints ` 9 `

#### Scenario: Different types
- **WHEN** `q = r` is compiled for variables of two different user types
- **THEN** it is a compile error

#### Scenario: Member array copied
- **WHEN** `p.v(0) = 10: p.v(3) = 13: q = p: p.v(3) = 0: PRINT q.v(0); q.v(3)` runs for a type with `v(0 TO 3) AS
  LONG`
- **THEN** it prints ` 10  13 `

#### Scenario: SWAP with string members
- **WHEN** `p.n = "p": q.n = "q": SWAP p, q: PRINT p.n; q.n` runs
- **THEN** it prints `qp`

#### Scenario: SWAP of owner values
- **WHEN** `REDIM x.v(3 TO 4) AS LONG: x.v(4) = 4: SWAP x, y` runs for two owner values
- **THEN** `x.v` has bounds 0 to 1, `y.v` has bounds 3 to 4 and `y.v(4)` is 4

### Requirement: Dynamic member arrays
A member array marked `_DYNAMIC` SHALL be an array of its own in each value of its type: each variable, each
array element and each copy SHALL have its own bounds and elements. A value of a type that has such a member, in
itself or in a member of a user type, an owner value, SHALL be made with each such member at its declared bounds
(under the `OPTION BASE` of the `TYPE` block, a `CONST` allowed in a bound) with zero or empty elements, and SHALL
release them when it goes: a local variable of a procedure at each return (a new one on each call), the elements
of an array when it is erased or given new elements. Every kind of element SHALL be allowed: every numeric type
and `_OFFSET`, `STRING * n`, `STRING`, a user type, also one with strings or with dynamic member arrays of its own.

`REDIM place.name(bounds) [AS type]` SHALL give the member array of that one value new bounds and new elements as
`REDIM` gives a dynamic array (requirements "REDIM", "REDIM _PRESERVE", "REDIM _RETAIN"), also with another
number of dimensions, `SHARED`, a list of targets, a type suffix on the member's name; it SHALL need `lower TO
upper` in the first dimension or `AS` the member's type, and another type after `AS` SHALL be a compile error.
`ERASE place.name` SHALL release its elements; it is then not allocated, and every use of it (an element,
`LBOUND`, `UBOUND`) SHALL raise error 9 until a `REDIM` gives it bounds again. An element outside the bounds, an
index with another number of dimensions than the member has now and a dimension it does not have in `LBOUND` or
`UBOUND` SHALL raise error 9. A `REDIM` whose bounds are reversed at run time, or too large for memory, SHALL end
the program with "Out of memory", as for an array (the old compiler gives a member too large an overflowed size
and goes on, `DIVERGENCES.md`). Every use of the member, `LBOUND` and `UBOUND` included, SHALL find the place it
is part of, evaluating its indexes once. An index outside its bounds on the way SHALL raise error 9, and the
element at position 0 of that dimension SHALL then stand in, for `REDIM`, `ERASE`, a read, `LBOUND` and
`UBOUND`, as in the old compiler; a store SHALL then store nothing (the old compiler stores into the member of
element 0; as `DIVERGENCES.md` D-004).

An element SHALL be a place like an element of a dynamic array: in stores, as an argument by reference, in
`SWAP`, `READ`, `INPUT`, `INPUT #`, `LINE INPUT #` and the `MID$` statement. The member written with empty
parentheses SHALL be an argument for an array parameter of its element type, passed as a dynamic array: the
procedure sees its bounds and a `REDIM` or `ERASE` there changes the member. A parameter of the owner type and an
array of it as an argument SHALL be passed by reference, so a `REDIM` of the member in the procedure is seen by
the caller. A `STATIC` local of an owner type SHALL keep its members between calls.

A whole-value copy SHALL copy each such member deep: the target then has the bounds and elements of the source
and no change to either is seen in the other. `SWAP` of two owner values SHALL exchange them, members included.
`REDIM` of an array of an owner type SHALL give the elements it keeps (`_PRESERVE`, `_RETAIN`) their members as
they were and the new elements members at their declared bounds; plain `REDIM` SHALL give every element new
members at their declared bounds. `ERASE` of a dynamic array of an owner type SHALL release the members with the
elements; `ERASE` of a static one SHALL release them and leave each member not allocated until a `REDIM` of it,
as in the old compiler (listed for review in `SOMEDAY.md`).

An owner value SHALL be laid out in memory with each member that is a dynamic member array, and each member of an
owner type, at a multiple of 8 bytes, and with its size rounded up to a multiple of 8; `_MEM` of an owner value
SHALL give that size. A type with no dynamic member array, also with `_STATIC` members, SHALL keep its layout.
`LEN` of an owner value, `_OFFSET` of it, `COMMON` of a variable of an owner type and `PUT` or `GET` of one owner
value SHALL be compile errors, as in the old compiler. A whole member array with no index SHALL be a compile
error where a value is expected, as for any array.

#### Scenario: First bounds and REDIM
- **WHEN** below the flag a type has `a AS INTEGER`, `v(2) _DYNAMIC AS LONG` and `b AS INTEGER`, and for `DIM x`
  `PRINT LBOUND(x.v); UBOUND(x.v); x.v(2)` runs, then `REDIM x.v(1 TO 5) AS LONG` and the bounds are printed
- **THEN** it prints ` 0  2  0 ` then ` 1  5 `, and `x.a` and `x.b` keep their values

#### Scenario: _PRESERVE and _RETAIN of a member
- **WHEN** `x.v(1 TO 5)` holds 21 to 25, then `REDIM _PRESERVE x.v(3 TO 5) AS LONG` and `REDIM _RETAIN x.v(4 TO
  8) AS LONG` run
- **THEN** after the first `x.v(3 TO 5)` holds 21 22 23, after the second `x.v(4 TO 8)` holds 22 23 0 0 0

#### Scenario: ERASE of a member
- **WHEN** `ERASE x.v` runs, then `UBOUND(x.v)` and `x.v(1)` are read under a handler that prints `ERR`, then
  `REDIM x.v(0 TO 1) AS LONG: x.v(1) = 5` runs
- **THEN** the handler prints 9 twice, and the member then has bounds 0 to 1 and `x.v(1)` is 5

#### Scenario: Each element has its own
- **WHEN** `REDIM p(0 TO 2)` of a type with `v(1) _DYNAMIC AS LONG`, and `REDIM p(i).v(0 TO i + 1) AS LONG` for
  each `i`
- **THEN** `UBOUND(p(0).v)`, `UBOUND(p(1).v)`, `UBOUND(p(2).v)` are 1, 2 and 3

#### Scenario: Deep copy
- **WHEN** `y = x` runs for owner values, then `y.v(5) = 999`
- **THEN** `x.v(5)` keeps its value and `y.v` has the bounds `x.v` had

#### Scenario: REDIM of the parent
- **WHEN** `REDIM _PRESERVE p(0 TO 3)` runs for the array of the scenario above, then plain `REDIM p(0 TO 1)`
- **THEN** after the first `p(0)` to `p(2)` keep their members and `p(3).v` has bounds 0 to 1; after the second
  every element's `v` has bounds 0 to 1 and zero elements

#### Scenario: ERASE of a static parent
- **WHEN** `DIM s(1)` of the type, `ERASE s`, then `LBOUND(s(1).v)` under a handler, then `REDIM s(1).v(2 TO 3)
  AS LONG`
- **THEN** the handler prints 9, and after the `REDIM` `s(1).v` has bounds 2 to 3

#### Scenario: Every kind of element
- **WHEN** a type has dynamic member arrays of `_BYTE`, `_UNSIGNED _INTEGER64`, `_FLOAT`, `_OFFSET`, `STRING *
  3`, `STRING`, a type with a `STRING` and a type with a dynamic member array, each set, copied with `y = x`, the
  source changed, and `_RETAIN` and `_PRESERVE` applied to the strings and types
- **THEN** each element prints the value stored, the copy keeps the values before the change, and the kept
  elements keep theirs (the measured output of `verification\v35_c_dyn_kinds`)

#### Scenario: In procedures
- **WHEN** a SUB has a local of the type and is called twice, another a `STATIC` local, another `REDIM`s a
  parameter's member, another `REDIM`s the member passed as `x.v()` to `a() AS LONG`
- **THEN** the local starts at its declared bounds on each call, the `STATIC` local keeps its member, and the
  caller sees both `REDIM`s (the measured output of `verification\v35_d_dyn_places`)

#### Scenario: Statements on an element
- **WHEN** `READ x.v(0), x.v(1)`, `SWAP x.v(0), x.v(1)`, `MID$(z.w(0), 2, 3) = "EYY"`, `INPUT #1, x.v(1)` and
  `LINE INPUT #1, z.w(1)` run
- **THEN** each stores into the element (the measured output of `verification\v35_e_dyn_statements`)

#### Scenario: Index errors
- **WHEN** `x.v(5)` is read and `x.v(-1) = 3` stored for bounds 0 to 1, `UBOUND(x.v, 2)` is read, and `x.v(1)` is
  read after `REDIM x.v(0 TO 1, 0 TO 1) AS LONG`, each under a handler
- **THEN** each raises error 9

#### Scenario: The place on the way
- **WHEN** for `REDIM p(2)` of the type, under a handler that prints `ERR` and resumes next, `REDIM p(9).v(4 TO 6)
  AS LONG`, then `p(9).v(5) = 55`, then `PRINT UBOUND(p(one(1)).v)` run, where FUNCTION `one` prints its argument
  and returns 1
- **THEN** the handler prints 9 twice, `p(0).v` has bounds 4 to 6 and `p(0).v(5)` is 0, and ` 1  1 ` is printed
  (`one` is called, then `UBOUND(p(1).v)`)

#### Scenario: Layout
- **WHEN** a type `t` has `a AS INTEGER`, `v(1) _DYNAMIC AS LONG`, `b AS INTEGER`, a type `o` has `c AS _BYTE`, `i
  AS t`, `d AS _BYTE`, and the `SIZE` of `_MEM` of a variable of each and the offsets of their members are printed
- **THEN** `t` is 24 bytes with `b` at 16, `o` is 40 bytes with `i` at 8 and `d` at 32, and two elements of an
  array of `t` are 24 bytes apart

#### Scenario: Forms that are compile errors
- **WHEN** for an owner type `LEN(x)`, `_OFFSET(x)`, `COMMON SHARED x AS t`, `PUT #1, 1, x`, `GET #1, 1, p(0)`,
  `REDIM x.v(5)`, `REDIM x.v(0 TO 5) AS INTEGER` or `PRINT x.v` is compiled
- **THEN** each is a compile error; `LEN(x.v(0))` compiles and is 4

### Requirement: Whole-array assignment
`target() = source()` (also after `LET`, the source in parentheses) SHALL copy every element of the source array
into the target array, where each side is an array variable or a member array written with empty parentheses. The
copy SHALL be deep: a `STRING` element, a `STRING` member and a dynamic member array of a user-type element are the
target's own afterwards. The target SHALL keep its bounds, and the same array on both sides SHALL change nothing.

#### Scenario: Numbers
- **WHEN** `DIM s(1 TO 3) AS LONG, d(1 TO 3) AS LONG`, `s(i) = i * 10` for each `i`, then `d() = s(): s(2) = 5`
  runs and `d(2)` is printed
- **THEN** it prints ` 20 `

#### Scenario: Strings are copied
- **WHEN** `d() = s()` runs for two `STRING` arrays of the same bounds, then `s(1) = "changed"`
- **THEN** `d(1)` keeps the string `s(1)` had at the copy

#### Scenario: User type with strings and member arrays
- **WHEN** `dh() = sh()` runs for arrays of a type with `v(2) AS INTEGER` and `w(1) AS STRING`, then
  `sh(1).w(1) = "shallow"`
- **THEN** `dh(1).w(1)` keeps its copied value and `dh(0).v(2)` is the source's

#### Scenario: Itself
- **WHEN** `r() = r()` runs
- **THEN** the elements of `r` keep their values

#### Scenario: Every place of an array
- **WHEN** one side is `SHARED`, local, `STATIC`, an array parameter, `COMMON SHARED`, `$DYNAMIC` or used without
  `DIM`, in the main module, a SUB or a FUNCTION
- **THEN** the copy is made as for two arrays of the main module (`verification` `v36_c`)

### Requirement: Whole-array assignment at run time
Before copying, a whole-array assignment SHALL check that both arrays are allocated and that in every dimension they
have the same lower bound and the same number of elements; if not, it SHALL raise error 9 and copy nothing, except
for two array variables of an owner type with other bounds (requirement "Whole-array assignment of owner types").
A static array after `ERASE` SHALL count as allocated. A target with other bounds SHALL NOT be given the source's.

#### Scenario: Other bounds
- **WHEN** `b() = a()` runs for `a(0 TO 5)` and `b(1 TO 6)`, also `c(0 TO 4)`, `d(0 TO 6)`, and two arrays of two
  dimensions whose second differs, under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 for each and the target's elements keep their values

#### Scenario: Not allocated
- **WHEN** the target or the source is a dynamic array after `ERASE`, or a `$DYNAMIC` array after `ERASE`
- **THEN** error 9 is raised and nothing is copied

#### Scenario: A static array after ERASE
- **WHEN** `ERASE st: st() = a()` runs for two static arrays of the same bounds
- **THEN** no error is raised and `st` holds the elements of `a`

### Requirement: Whole-array assignment checks
A whole-array assignment SHALL be a compile error, as in the old compiler, when the element types differ
(kind, width, sign, `STRING * n` length, user type), when the numbers of dimensions differ, and when one side is
`name()` and the other is not a whole array (a scalar, an element, an expression, a literal, a FUNCTION, a constant,
an undeclared name). A scalar or element target with a whole array as its value SHALL be a compile error too.

#### Scenario: Types that differ
- **WHEN** `a() = b()` is compiled for `LONG` and `INTEGER`, `LONG` and `_UNSIGNED LONG`, `SINGLE` and `DOUBLE`,
  `_BIT * 2` and `_BIT * 3`, `_OFFSET` and `_INTEGER64`, `STRING` and `STRING * 4`, two user types of one layout
- **THEN** each is a compile error that names the two element types (the old compiler: "Cannot assign arrays of
  different types")

#### Scenario: Sizes and dimensions
- **WHEN** `a() = b()` is compiled for `STRING * 4` and `STRING * 8`, and for one and two dimensions
- **THEN** each is a compile error (the old compiler: "Cannot assign fixed-length STRING arrays with different
  element sizes", "Cannot assign arrays with different dimensions")

#### Scenario: Not a whole array on one side
- **WHEN** `arrA() = arrB`, `arrA() = arrB(1)`, `arrA = arrB()`, `arrA(1) = arrB()`, `values() = scalarVal`,
  `scalarVal = values()`, `a() = b() + 1` or `a() = 5` is compiled for arrays `arrA`, `arrB`, `values`, `a`, `b` and
  a scalar `scalarVal`
- **THEN** each is a compile error at the side that is not a whole array, or at the statement when neither side
  can be taken alone (the old compiler: "Expected array index after" the name, or "Whole-array assignment requires
  empty parentheses on both sides")

#### Scenario: Suffixes on both names
- **WHEN** `b%() = a%()` is compiled
- **THEN** it is reported "not supported yet" (the old compiler copies element 0 into element 0)

### Requirement: Whole-array assignment of member arrays
A member array written with empty parentheses (`x.v()`, `e(1).v()`, a `_DYNAMIC` member) SHALL be a side of a
whole-array assignment with an array variable or another member array, with the same checks. The target's place
SHALL be found before the source's, each once. If an index on the way raises an error, nothing SHALL be copied (the
old compiler copies strings through the member of element 0, `DIVERGENCES.md` D-004).

#### Scenario: Member to member, member to array and back
- **WHEN** `y.v() = x.v(): a() = x.v(): x.v() = a()` runs for a type with `v(1 TO 3) AS LONG` and `a(1 TO 3) AS
  LONG`, also with `STRING` and user-type member arrays
- **THEN** each copy is made as for two arrays

#### Scenario: Bounds of a member that differ
- **WHEN** `b() = x.v()` runs for `b(0 TO 2)` and `v(1 TO 3)` under a handler that prints `ERR`
- **THEN** the handler prints 9 and `b` keeps its elements

#### Scenario: Index outside on the way
- **WHEN** `e(i).v() = a(): e(i).w() = s()` runs with `i` outside the bounds of `e`, for a `LONG` and a `STRING`
  member array, under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 twice and no member of any element of `e` changes

#### Scenario: Order of the places
- **WHEN** `e(f(2)).v() = e(f(1)).v()` runs, where FUNCTION `f` prints its argument
- **THEN** ` 2 ` is printed before ` 1 `

#### Scenario: Dynamic member arrays
- **WHEN** below `$UNSTABLE:TYPEFIELDS` `q.v() = p.v()` runs for a `_DYNAMIC` member, first with other bounds, then
  after `REDIM q.v` to the bounds of `p.v`
- **THEN** the first raises error 9 and leaves `q.v` with its bounds, the second copies the elements

### Requirement: Whole-array assignment of owner types
Below `$UNSTABLE:TYPEFIELDS` a whole-array assignment of arrays of a type with dynamic member arrays SHALL copy each
element as a whole-value copy does: each dynamic member of the target then has the bounds and elements of the
source's, strings and nested owner types included, for a `DIM` and a `REDIM` target alike. For two array
variables with other bounds or shape it SHALL raise error 9 and then, as the old compiler does, copy the target's
number of elements, element `i` of the target from element `i` of the source in order, the target keeping its
bounds, when the source has at least that many elements; when it has fewer, or either is not allocated, nothing
SHALL be copied (the old compiler reads past the source's block, `DIVERGENCES.md` D-022).

#### Scenario: Members with their own bounds
- **WHEN** `dst() = src()` runs where `src(1).v` was given the bounds 3 to 5 and `src(1).v(4) = 44`, then
  `src(1).v(4) = -1`
- **THEN** `LBOUND(dst(1).v)`, `UBOUND(dst(1).v)` and `dst(1).v(4)` are 3, 5 and 44

#### Scenario: Other bounds, as many or fewer elements in the target
- **WHEN** `d() = s()` runs for `s(1 TO 2)` and `d(0 TO 1)`, also `d0(0 TO 0)`, `r(5 TO 6)` given by `REDIM`, and a
  `n(0 TO 1, 0 TO 1)` from an `m(1 TO 4, 0 TO 0)`, under a handler that prints `ERR`
- **THEN** the handler prints 9 for each, and each target holds the source's first elements in order with its own
  bounds (`verification\v38_d`)

#### Scenario: Other bounds copy nothing
- **WHEN** `e() = s()` runs for `s(1 TO 2)` and `e(1 TO 3)` of an owner type, under a handler that prints `ERR`
- **THEN** the handler prints 9 and every element of `e` keeps its values (the old compiler copies three elements,
  reading past `s`, `DIVERGENCES.md` D-022)

### Requirement: _ARRAYCOPY
`_ARRAYCOPY source TO destination` SHALL copy elements of one array into another in one of six forms: `s() TO
d()`, `s(lo TO hi) TO d(start)`, `s() TO d(start, …)`, `s(lo TO hi) TO d()`, `s(lo TO hi, …) TO d(start, …)` and
`s(lo TO hi, …) TO d()`, with one range or start per dimension. Elements SHALL be copied as if read before any is
written, so the two may overlap or be one array. `STRING` elements SHALL be the destination's own afterwards.

#### Scenario: A range to a start
- **WHEN** `DIM s(3 TO 8) AS LONG, d(-2 TO 12) AS LONG`, `s(i) = i * 10` for each `i`, then `_ARRAYCOPY s(4 TO 6)
  TO d(-1)` runs
- **THEN** `d(-1)`, `d(0)` and `d(1)` are 40, 50 and 60 and every other element of `d` keeps its value
  (`verification\v41_a`)

#### Scenario: A range to the whole destination keeps its indexes
- **WHEN** `_ARRAYCOPY s(4 TO 6) TO d()` runs for the same arrays
- **THEN** `d(4)`, `d(5)` and `d(6)` are 40, 50 and 60, and nothing else changes

#### Scenario: A whole one-dimensional source
- **WHEN** `_ARRAYCOPY s() TO d()` runs, then `_ARRAYCOPY s() TO d(0)` after `d` was set to -1
- **THEN** the first sets `d(3)` to `d(8)`, the second `d(0)` to `d(5)`, to 30 … 80

#### Scenario: Overlap
- **WHEN** `_ARRAYCOPY s(3 TO 6) TO s(5)` runs for `s(i) = i * 10`, and in another run `_ARRAYCOPY s(5 TO 8) TO
  s(3)`
- **THEN** `s` holds 30 40 30 40 50 60, and 50 60 70 80 70 80

#### Scenario: Several dimensions
- **WHEN** `_ARRAYCOPY big(1 TO 2, 2 TO 4) TO small(1, 0)` runs for `big(i, j) = i * 10 + j`, and `_ARRAYCOPY m()
  TO big(1, 2)` for `m(1 TO 2, 0 TO 2)`
- **THEN** `small(1, 0)` to `small(2, 2)` hold 12 13 14 and 22 23 24, and `big(1, 2)` to `big(2, 4)` hold the
  elements of `m` in their own rows and columns; the elements around keep their values (`v41_c`)

#### Scenario: Strings are copied
- **WHEN** `_ARRAYCOPY vs1(2 TO 3) TO vs2(1)` runs for two `STRING` arrays, then `vs1(2) = "changed"`
- **THEN** `vs2(1)` keeps the string `vs1(2)` had at the copy

### Requirement: _ARRAYCOPY element types and places
Both sides SHALL have one element type: a number type, `_OFFSET`, `_UNSIGNED _OFFSET`, `STRING * n` of one length
or `STRING`. Each side SHALL be an array variable of any place (SHARED, local, STATIC, a parameter, COMMON,
`$DYNAMIC`, used without `DIM`) or a member array, through any chain of members and elements, `_DYNAMIC` members
included. A `_FLOAT` array SHALL be copied element by element (the old compiler copies the wrong elements,
`DIVERGENCES.md` D-025).

#### Scenario: Every type
- **WHEN** a range of two elements is copied to a start for arrays of each number type, `_OFFSET`, `STRING * 3`
  and `STRING`
- **THEN** each destination holds the two values at the start and its other elements are unchanged, as the old
  compiler's program prints (`v41_b`), but for `_FLOAT`

#### Scenario: _FLOAT
- **WHEN** `_ARRAYCOPY g1(2 TO 3) TO g3(0)` runs for `_FLOAT` arrays with `g1(i) = i + 0.25`
- **THEN** `g3(0)` and `g3(1)` are 2.25 and 3.25 (the old compiler gives 3.25 and 4.25, `v41_i`)

#### Scenario: Member arrays
- **WHEN** `_ARRAYCOPY src() TO tt(2).b(5)`, `_ARRAYCOPY v.b(2 TO 4) TO oo.inner.b()` and `_ARRAYCOPY v.s(1 TO 2)
  TO tt(1).s(0)` run for member arrays of `LONG`, nested members and `STRING * 3` members of a plain `TYPE`
- **THEN** each copies as for two arrays, and the members beside the arrays keep their values (`v41_d`, `v41_k`)

#### Scenario: _DYNAMIC members
- **WHEN** below `$UNSTABLE:TYPEFIELDS` an array is copied to a `_DYNAMIC` member, after `REDIM` of the member to
  a start, a two-dimensional member's range to another's start, and `STRING` members to each other
- **THEN** each copies as for two arrays (`v41_e`)

### Requirement: _ARRAYCOPY evaluation
The places on the way to member arrays SHALL be found first, the source's then the destination's, then every
lower bound, then every upper bound, then the starts, each once and left to right. A bound or start SHALL be a
number, rounded to an integer as an index is. If finding a place raises an error, nothing SHALL be copied (the old
compiler copies through the member of the first element, `DIVERGENCES.md` D-004).

#### Scenario: Order
- **WHEN** `_ARRAYCOPY tt(f(1)).b(f(2) TO f(3)) TO tt(f(2)).b(f(1))` and `_ARRAYCOPY m(f(1) TO f(2), f(3) TO f(3))
  TO n(f(0), f(1))` run, where FUNCTION `f` prints its argument
- **THEN** they print 1 2 2 3 1, and 1 3 2 3 0 1 (`v41_h`)

#### Scenario: Rounding
- **WHEN** `_ARRAYCOPY a(1.5 TO 3.5) TO d(0.5)` runs for `a(i) = i`
- **THEN** `d(0)` to `d(2)` are 2, 3 and 4

#### Scenario: Index outside on the way
- **WHEN** `_ARRAYCOPY src() TO tt(3).b()` and `_ARRAYCOPY tt(0).b(1 TO 2) TO d(2)` run for `tt(1 TO 2)`, under a
  handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 for each, and no element of `tt` or `d` changes

### Requirement: _ARRAYCOPY at run time
Before copying, `_ARRAYCOPY` SHALL raise error 9 and copy nothing when an array is not allocated, a range's lower
bound is above its upper, a range or start lies outside its array, the elements do not fit in the destination
from the start, or a whole source of several dimensions has other bounds than the destination. A static array
after `ERASE` SHALL count as allocated.

#### Scenario: Outside
- **WHEN** each of `a(3 TO 2)`, `a(0 TO 2)`, `a(4 TO 6)` to `d(0)`, `a(1 TO 2)` to `d(-1)` and to `d(6)`,
  `a(1 TO 3)` to `d(4)` and `a()` to `d(2)` is copied for `a(1 TO 5)` and `d(0 TO 5)`, under a handler that prints
  `ERR` and resumes next
- **THEN** the handler prints 9 for each and `d` keeps its elements (`v41_g`)

#### Scenario: Not allocated
- **WHEN** the source or the destination is a dynamic array after `ERASE`, or a `$DYNAMIC` array after `ERASE`
- **THEN** error 9 is raised and nothing is copied; for a static array after `ERASE` the copy is made

#### Scenario: Whole copy of several dimensions
- **WHEN** `_ARRAYCOPY m() TO n()` runs for `m(1 TO 2, 1 TO 3)` and `n(1 TO 3, 1 TO 2)`, and for `k(0 TO 2, 1 TO 3)`
- **THEN** error 9 is raised for each

### Requirement: _ARRAYCOPY of a dynamic member with other dimensions
A `_DYNAMIC` member array that a `REDIM` gave another number of dimensions than it was declared with SHALL raise
error 9 where its place is found when it is a side of `_ARRAYCOPY`, and nothing SHALL be copied, as in the old
compiler.

#### Scenario: A dynamic member with another number of dimensions
- **WHEN** below `$UNSTABLE:TYPEFIELDS` a member declared `_DYNAMIC n(1 TO 4) AS LONG` is given `REDIM v.n(1 TO 2,
  1 TO 3) AS LONG`, then `_ARRAYCOPY v.n() TO d()`, `_ARRAYCOPY v.n(1 TO 2) TO d(5)` and `_ARRAYCOPY src() TO
  v.n()` run under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 for each and `d` keeps its elements (`verification\v41_l`)

### Requirement: _ARRAYCOPY checks
`_ARRAYCOPY` SHALL be a compile error, as in the old compiler, for a form the six do not cover, a side that is not
an array (a scalar, a FUNCTION, a constant, an undeclared or suffixed name), element types or `STRING * n`
lengths that differ, an element type it does not take, numbers of dimensions or of ranges and starts that do not
fit the form, a range without one `TO`, an empty range or start, and a bound that is a string.

#### Scenario: Forms
- **WHEN** `_ARRAYCOPY a() b()`, `a() TO b() TO c()`, `a TO b()`, `a(1) TO b()`, `a(1 TO 2) TO b(1 TO 2)` or `(a())
  TO b()` is compiled
- **THEN** each is a compile error (the old compiler: "Expected _ArrayCopy source(lower TO upper) TO …", or "Expected
  one TO between _ArrayCopy source and destination")

#### Scenario: Not an array
- **WHEN** a side is a scalar, an undeclared name, a FUNCTION, a `CONST`, a scalar member, or written with a type
  suffix (`a%()`, also for `DIM a(5) AS LONG` named `a&()`)
- **THEN** each is a compile error (the old compiler: "_ArrayCopy source must be an array", or "destination")

#### Scenario: Types
- **WHEN** the sides are `LONG` and `INTEGER`, `LONG` and `_UNSIGNED LONG`, `STRING` and `STRING * 4`, `STRING *
  4` and `STRING * 5`, `_OFFSET` and `_UNSIGNED _OFFSET`, or both of a user type, `_BIT`, `_BIT * 4` or `_MEM`
- **THEN** each is a compile error (the old compiler: "… requires arrays with the same element type", "… with the
  same element size", "… supports only byte-aligned numeric or fixed-length STRING arrays")

#### Scenario: Dimensions and counts
- **WHEN** `m(1 TO 2) TO n(0)` for two-dimensional arrays, `a() TO n()` and `a() TO n(0, 0)` for one and two,
  `m() TO n(0)`, `a(1 TO 2, 1 TO 2) TO b(0, 0)` for one dimension, a range count or start count other than the
  array's, `m(1 TO 2, 3)`, `m(1 TO 2 TO 3, 1 TO 2)`, `n(0, )` or `m(1 TO 2, )` is compiled
- **THEN** each is a compile error with the old compiler's reason

#### Scenario: Strings of several dimensions
- **WHEN** `sm(1 TO 2) TO sn(0)`, `s() TO sn()`, `sm(1 TO 2, 1 TO 2) TO s3(0, 0, 0)` or `sm() TO u(0)` is compiled
  for `STRING` arrays of the dimensions their names say
- **THEN** each is a compile error

#### Scenario: Parameters
- **WHEN** a SUB that is never called holds `_ARRAYCOPY x(1 TO 2, 1 TO 2) TO y(1, 1)` for array parameters `x` and
  `y`, and another SUB holds `_ARRAYCOPY x() TO y(2)` and is called with a two-dimensional `x` and a
  one-dimensional `y`
- **THEN** the first is a compile error, as nothing else gives the number of dimensions (the old compiler:
  "_ArrayCopy cannot determine the array dimensions"), and the second is one for the dimensions (`v41_x75`,
  `x76`); the same SUB called with two one-dimensional arrays compiles and copies

#### Scenario: A string bound
- **WHEN** `_ARRAYCOPY a("1" TO 2) TO b(0)` or `_ARRAYCOPY a(1 TO 2) TO b("0")` is compiled
- **THEN** each is a compile error

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

### Requirement: Member array bounds that name no constant
A bound of a member array in a `TYPE` block that names no `CONST` standing above the block SHALL be the compile
error "TYPE member array bounds must be numeric constant", as the old compiler gives it (measured,
`verification\v51_x01`–`x15`). A bound that names a `CONST` above the block, a constant of the auto-included file
or a function of the constant evaluator (`_PI`, `ABS(…)`) SHALL stay "not supported yet" (requirement "Member
arrays").

#### Scenario: A variable as the bound
- **WHEN** a program does `DIM AS _BYTE a`, `a = 1` and declares `array (a) AS LONG` in a `TYPE` block (upstream
  `arrays/39_err_member_must_be_const`)
- **THEN** it is the compile error "TYPE member array bounds must be numeric constant" at the bound, not "not
  supported yet"

#### Scenario: A variable assigned above the block
- **WHEN** `n = 3` stands above a `TYPE` block with the member `v(n) AS LONG` (`verification\v24_x195`)
- **THEN** it is the same compile error

#### Scenario: A CONST below the block or in a SUB
- **WHEN** a `TYPE` block of the main module has the member `v(top) AS LONG` and `CONST top = 3` stands below it,
  or in a SUB above or below it (`verification\v51_x03`, `x04`, `x13`)
- **THEN** it is the compile error "TYPE member array bounds must be numeric constant"

#### Scenario: A call in the bound
- **WHEN** the bound is `LEN(x)`, `LEN("abc")`, an array's element `a(1)` or a call of the program's FUNCTION
  `f(1)` (`verification\v51_x05`, `x06`, `x12`, `x15`)
- **THEN** it is the same compile error

#### Scenario: A CONST as the bound
- **WHEN** `CONST top = 3` stands above a `TYPE` block with the member `v(top) AS LONG` (`verification\v24_x196`)
- **THEN** the only diagnostic is "not supported yet" at the bound

### Requirement: Undefined member types
A member of a `TYPE` block declared `AS name`, where `name` is no type word of the old compiler and no `TYPE` of
an earlier block, SHALL be the compile error "Undefined type", and before the names of an `AS type` list the old
compiler's "Expected element-name AS type, AS type element-list, or END TYPE" (measured,
`verification\v51_x20`–`x27`). A name of an earlier block that was not entered SHALL stay "not supported yet".

#### Scenario: A member array of an undefined type
- **WHEN** a `TYPE` block declares `array (10) AS notdefined` and the program has no `TYPE notdefined` (upstream
  `arrays/40_err_undefined_type`)
- **THEN** it is the compile error "Undefined type" at the member, not "not supported yet"

#### Scenario: Near types and sizes
- **WHEN** a member is declared `AS _UNSIGNED nosuch`, `AS BYTE`, `AS _Bytef`, `AS Stringfoobar * 2` or `AS
  INTEGER * 2` (`verification\v51_x22`–`x25`, `x27`)
- **THEN** it is the compile error "Undefined type"

#### Scenario: A type defined in an earlier block
- **WHEN** the member names a `TYPE` that the program defines in an earlier block
- **THEN** the member's type is checked as requirement "User types" says, and no "Undefined type" error is given

#### Scenario: A type defined in a later block
- **WHEN** the member names a `TYPE` that the program defines in a later block (`verification\v51_x21`)
- **THEN** it is the compile error "Undefined type"

### Requirement: LBOUND and UBOUND of an element
`LBOUND` and `UBOUND` whose array argument is an element of a member array with its indexes, as
`UBOUND(f(10).a(5), 1)`, SHALL be the compile error "Incorrect number of arguments", as the old compiler gives it.
Each other element form that the old compiler is measured to reject (in `verification\v51`) SHALL be its
measured error. A form that is not measured SHALL stay "not supported yet".

#### Scenario: UBOUND of a member array element
- **WHEN** `PRINT UBOUND(f(10).a(5), 1)` is compiled for `DIM f(15) AS e`, where `e` has the member `a(1 TO 50,
  2 TO 20) AS _BYTE` (upstream `arrays/42_err_solved_bug_Incorrect_nr_of_args`)
- **THEN** it is the compile error "Incorrect number of arguments", not "not supported yet"

#### Scenario: LBOUND of a member array element
- **WHEN** `PRINT LBOUND(f(10).a(5), 1)` is compiled for the same array (upstream
  `arrays/43_err_solved_bug_Incorrect_nr_of_args`)
- **THEN** it is the same compile error

#### Scenario: Other elements and values
- **WHEN** the argument is an element of an array of a `TYPE` (`LBOUND(f(1))`), an element of another array, a
  call of the program's FUNCTION, a literal or `a + 1` (`verification\v51_x28`–`x31`, `x33`, `x39`)
- **THEN** it is the compile error "Expected ." for the first and "Syntax error" for the others

#### Scenario: The member array itself
- **WHEN** `PRINT LBOUND(f(10).a, 2); UBOUND(f(10).a, 2)` runs for the same array
- **THEN** it prints ` 2  20 `, as requirement "LBOUND and UBOUND" says

### Requirement: Unknown type names
In an `AS` clause of `DIM`, `REDIM`, `STATIC` or `COMMON`, a name that is no type word of the old compiler (a
word without its underscore, as `BYTE`, is none) and no `TYPE` of the program SHALL be the compile error "Unknown
type", and so SHALL `* n` after a word other than `STRING` and `_BIT`; after `_UNSIGNED` each SHALL be "Type
cannot be _UNSIGNED" (measured, `verification\v51_x60`–`x88`). A name of a `TYPE` block that was not entered
SHALL stay "not supported yet".

#### Scenario: A misspelt numeric type
- **WHEN** a program does `DIM b AS _Bytef`, or `STATIC b AS _Bytef` in a SUB (upstream
  `types/dim_bad_typename_byte`, `types/static_bad_typename_byte`)
- **THEN** it is the compile error "Unknown type", not "not supported yet"

#### Scenario: A size after another word
- **WHEN** a program does `DIM b AS INTEGER * 2`, or `DIM b AS _UNSIGNED INTEGER * 2` (`verification\v51_x65`,
  `x84`)
- **THEN** it is the compile error "Unknown type", and "Type cannot be _UNSIGNED"

#### Scenario: A type that does not exist in a DIM AS list
- **WHEN** a program does `DIM AS nosuch a` and has no `TYPE nosuch` (`verification\v24_x232`)
- **THEN** it is the compile error "Unknown type"

#### Scenario: A type defined below its use
- **WHEN** `DIM v AS pt` stands above the program's `TYPE pt` block
- **THEN** the program compiles as requirement "User types" says, and no "Unknown type" error is given

### Requirement: Words glued to STRING
In an `AS` clause of `DIM`, `REDIM`, `STATIC` or `COMMON`, a word that starts with `STRING` and is not `STRING`
SHALL be the compile error "Expected STRING * number/constant", with `* n` or without and after `_UNSIGNED`
(measured, `verification\v51_x67`, `x70`, `x83`), unless a `TYPE` of the program has that name: the old compiler
looks the user types up before the `STRING` prefix, so `TYPE StringList` is a type in every `AS` clause.

#### Scenario: A word glued to STRING
- **WHEN** a program does `DIM s AS Stringfoobar * 2`, or `STATIC s AS Stringfoobar * 2` in a SUB (upstream
  `types/dim_bad_typename_string`, `types/static_bad_typename_string`)
- **THEN** it is the compile error "Expected STRING * number/constant"

#### Scenario: Without a size
- **WHEN** a program does `DIM s AS Stringfoobar` (`verification\v51_x70`)
- **THEN** it is the same compile error

#### Scenario: A TYPE whose name starts with STRING
- **WHEN** a program defines `TYPE StringList` and declares `DIM a AS StringList`, `REDIM c(1 TO 2) AS
  StringList` or a parameter `x AS StringList` (`tests\frontend\type_string_prefix_ok.bas`)
- **THEN** each is the user type, and no error is given

### Requirement: Unknown type names in SHARED, _DEFINE and parameters
A name that is no type in `SHARED` or `_DEFINE` SHALL be the compile error "Cannot convert type (…) to symbol",
and in a parameter of a SUB, a FUNCTION or a `DECLARE LIBRARY` procedure, an array parameter too, "Illegal
SUB/FUNCTION parameter" (measured, `verification\v51_x61`, `x63`, `x69`, `x74`, `x82`, `x87`, `x89`). A form
not measured in these places SHALL stay "not supported yet".

#### Scenario: A misspelt type in SHARED and in a parameter
- **WHEN** a SUB has `SHARED b AS _Bytef`, or the parameter `(b AS _Bytef)` (`verification\v51_x61`, `x63`)
- **THEN** it is the compile error "Cannot convert type (_BYTEF) to symbol", and "Illegal SUB/FUNCTION
  parameter"
