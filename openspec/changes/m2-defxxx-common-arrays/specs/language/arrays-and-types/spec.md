# Spec Delta

Every rule here is measured with `qb64pe.exe` before it is built (`verification\v09_dim.bas`, `v18_*`, `v24_*`);
a scenario whose outcome reads "equals the old compiler's" is pinned by the recorded output of its slice program.

## MODIFIED Requirements

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

### Requirement: LBOUND and UBOUND
`LBOUND(array[, dimension])` and `UBOUND(array[, dimension])` SHALL give the lower and upper bound of a dimension
of an array (the first without `dimension`), typed `_INTEGER64`. The dimension SHALL be rounded half to even; a
dimension outside 1 to the array's number of dimensions SHALL raise error 9 when it runs. The array SHALL be an
array variable, an array parameter or a member array (`x(0).n`). A name that is no array SHALL make an implicit
array (requirement "Implicit arrays"). Of a dynamic array that is not allocated both SHALL raise error 9.
`LBOUND(a())` SHALL be a compile error.

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
A `TYPE name … END TYPE` block in the main module SHALL define a user type whose members are of the numeric types
other than `_BIT`, of fixed-length strings, of `STRING`, or of user types defined in earlier blocks, each a single
value or an array (requirement "Member arrays"); the program's types SHALL
be known before its statements (a type may be used above its block). `DIM v AS name` SHALL declare a variable of
that type, in every storage class a scalar variable may have; every member starts at zero (NUL bytes for a
fixed-length string, an empty string for a `STRING`), a local one on each call. A member's size in the layout SHALL
be its type's (1 byte for
`_BYTE`, 8 for `_OFFSET`, `n` for `STRING * n`), without padding. A member array marked `_STATIC` or `_DYNAMIC`,
and a `TYPE` block inside a SUB or FUNCTION, SHALL be reported "not supported yet"; a member declared
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

## ADDED Requirements

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
a compile error.

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
`REDIM` SHALL be `_PRESERVE`, then `SHARED`; `REDIM SHARED _PRESERVE` SHALL be a compile error. `SHARED` SHALL
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
`REDIM name()` with nothing between the parentheses SHALL declare a scalar, as in the main module.

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
give it new bounds, and a use before either SHALL raise error 9. A block name (`COMMON /name/ …`) SHALL be a
compile error, as in the old compiler. `COMMON` inside a SUB or FUNCTION, and a second declaration of a `COMMON`
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
`place.name(index, …)` SHALL name an element, through any chain of members and elements, with the index and
by-reference rules of array elements. A store SHALL evaluate the value first, then the indexes from left to
right, and SHALL store nothing when an index is outside its bounds (error 9; the old compiler writes element 0 of
that dimension, `DIVERGENCES.md` D-004). The elements of a member array SHALL be part of the value of their
variable: counted by `LEN`, zero or empty when the variable is made, and each variable and each array element
SHALL have its own. The bounds of a member array SHALL NOT change: `REDIM` of one SHALL be accepted only with
constant bounds equal to the declared ones, written `lower TO upper` or with `AS` and the member's type, and
SHALL then set the elements of that one array to zero or empty; `REDIM _PRESERVE` so written SHALL change
nothing; `ERASE` of one SHALL set its elements to zero or empty. `LBOUND` and `UBOUND` SHALL take a member array,
also written with empty parentheses.

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
- **WHEN** `REDIM t.v(0 TO 9)`, `REDIM t.v(1 TO 4)`, `REDIM t.v(0 TO n)`, `REDIM t.v(3)` or `REDIM t.v(0 TO 3) AS
  INTEGER` is compiled for `v(0 TO 3) AS LONG`
- **THEN** each is a compile error

#### Scenario: ERASE of a member array
- **WHEN** `ERASE t.v` runs after the elements and the members beside the array were set
- **THEN** the elements are 0, `UBOUND(t.v)` is 3, and the other members keep their values

#### Scenario: Marked member
- **WHEN** a `TYPE` block has `v(0 TO 3) _DYNAMIC AS LONG`
- **THEN** the compiler reports it "not supported yet" (the old compiler compiles it only under
  `$UNSTABLE:TYPEFIELDS`)

### Requirement: Whole user-type values
`q = p` for two places of the same user type (variables, array elements, members of a user type) SHALL copy every
member, a `STRING` member as its own string and a member array element by element; a place assigned to itself
SHALL keep its value. Places of two different user types, also two with the same members, and a number SHALL be
compile errors. `SWAP` of two places of the same user type SHALL exchange their values, `STRING` members
included. An index outside its bounds on either side SHALL raise error 9, and element 0 of that array SHALL then
stand in for the element named, as in the old compiler (`a(9) = p` overwrites `a(0)`; decided 2026-10-11, listed
for review in `SOMEDAY.md`).

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
