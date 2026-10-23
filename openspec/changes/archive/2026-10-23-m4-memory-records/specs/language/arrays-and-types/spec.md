# Spec Delta

## MODIFIED Requirements

### Requirement: Member arrays
A `TYPE` member SHALL be declarable as an array with constant bounds, as `name(bounds) AS type` or in an
`AS type name(bounds), …` list, of any type a member may have and of one or more dimensions. A dimension without
`TO` SHALL start at the `OPTION BASE` in effect where the `TYPE` block stands; the literal 0 has no rule of its
own there (`z(0)` under base 1 is a compile error, as are bounds that are not constant, reversed or missing).
A bound written with numbers alone, also as an expression of numbers, SHALL be accepted; so SHALL one that names
a `CONST` above the block, a constant of the auto-included file or a function of the constant evaluator, with
the value of that expression, as in the old compiler (requirement "Member array bounds that name no constant").
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
element beside other targets of one `ERASE` or `REDIM` SHALL be cleared in its turn, and the statement SHALL go on
with the other targets after an index on the way raised an error, as in the old compiler
(`verification\v24_x312`). An element of a member array as a target of `INPUT`, `READ` or the `MID$` statement
SHALL be reported "not supported yet".

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
- **WHEN** `CONST top = 3` stands above a `TYPE t1` block with `v(top) AS LONG` and `w(top * 2 - 1) AS LONG`, and
  `DIM t AS t1: PRINT UBOUND(t.v); UBOUND(t.w); LEN(t)` runs (`verification\v24_x196`)
- **THEN** it prints ` 3  5  40 `

#### Scenario: Member arrays of elements in one ERASE or REDIM
- **WHEN** the program `verification\v24_x312_member_erase_base` runs `ERASE a(1).s, a(2).v: ERASE a(3).s` and
  `REDIM a(2).v(0 TO 3), a(2).s(0 TO 1)`, among the other forms of that program
- **THEN** it prints what the old compiler's program printed

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

### Requirement: LEN of a whole array
`LEN(name())` of an array, and `LEN` of a whole member array (`LEN(v.m())`, `LEN(a(i).m())`), SHALL give the size in
bytes of all its elements, as `_MEM` of it gives: the number of elements times the element's size. A dynamic array
that is not allocated SHALL give 0. `LEN(name())` of a name that is no array SHALL be the size of the array used
without `DIM` that it makes (requirement "Implicit arrays"). `LEN` of a whole member array of variable-length
strings SHALL give the old compiler's number, 8 bytes per element (`verification\v48_x30`). A whole dynamic
member array SHALL be reported "not supported yet".

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

#### Scenario: A whole member array of strings
- **WHEN** a `TYPE t` has `s(3) AS STRING`, and `DIM x AS t: PRINT LEN(x.s())` runs (`verification\v48_x30`), and
  the other bounds and dimensions of `verification\v56_g1_len_string_member_arrays`
- **THEN** it prints ` 32 `, and `v56_g1` prints what the old compiler's program printed

### Requirement: Member array bounds that name no constant
A bound of a member array in a `TYPE` block that names no `CONST` standing above the block SHALL be the compile
error "TYPE member array bounds must be numeric constant", as the old compiler gives it (measured,
`verification\v51_x01`–`x15`). A bound that names a `CONST` above the block, a constant of the auto-included file
or a function of the constant evaluator (`_PI`, `ABS(…)`) SHALL be taken with its value, rounded as any bound is
(requirement "Member arrays").

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
- **WHEN** `CONST top = 3` stands above a `TYPE` block with the member `v(top) AS LONG` (`verification\v24_x196`),
  or `v(top%)` (`v51_x09`)
- **THEN** it compiles with no diagnostic

#### Scenario: An auto-included constant or a function as the bound
- **WHEN** a `TYPE t1` block has `v(_PI) AS LONG`, `v(ABS(-3)) AS LONG` or, below `CONST top = 3`, `v(ABS(top)) AS
  LONG`, and `DIM t AS t1: PRINT UBOUND(t.v)` runs (`verification\v51_x10`, `x11`, `x14`)
- **THEN** each prints ` 3 `
