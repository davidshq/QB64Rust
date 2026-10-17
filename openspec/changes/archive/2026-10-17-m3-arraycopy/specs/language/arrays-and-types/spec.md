## ADDED Requirements

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

## MODIFIED Requirements

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
