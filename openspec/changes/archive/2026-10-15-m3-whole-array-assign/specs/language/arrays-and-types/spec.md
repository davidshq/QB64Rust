## ADDED Requirements

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
have the same lower bound and the same number of elements; if not, it SHALL raise error 9 and copy nothing. A static
array after `ERASE` SHALL count as allocated. A target with other bounds SHALL NOT be given the source's.

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
source's, strings and nested owner types included, for a `DIM` and a `REDIM` target alike.

#### Scenario: Members with their own bounds
- **WHEN** `dst() = src()` runs where `src(1).v` was given the bounds 3 to 5 and `src(1).v(4) = 44`, then
  `src(1).v(4) = -1`
- **THEN** `LBOUND(dst(1).v)`, `UBOUND(dst(1).v)` and `dst(1).v(4)` are 3, 5 and 44

#### Scenario: Other bounds copy nothing
- **WHEN** `e() = s()` runs for `s(1 TO 2)` and `e(1 TO 3)` of an owner type, under a handler that prints `ERR`
- **THEN** the handler prints 9 and every element of `e` keeps its values (the old compiler copies three elements,
  reading past `s`, `DIVERGENCES.md` D-022)

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
parentheses (except as an array argument and as a side of a whole-array assignment, requirement "Whole-array
assignment of member arrays") or with another number of indexes than its dimensions SHALL be a compile error,
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
