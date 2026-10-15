# Spec Delta

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
parentheses or with another number of indexes than its dimensions SHALL be a compile error, as an index after a
member that is no array SHALL be. A store SHALL evaluate a numeric value before the indexes and a string value
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

## ADDED Requirements

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
