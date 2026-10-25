# Spec Delta

## MODIFIED Requirements

### Requirement: What VARPTR and VARSEG accept
An argument of `VARPTR` or `VARSEG` that is not a variable or an array element (a literal, an expression, a
function call) SHALL be a compile error. `VARPTR` and `VARSEG` of a variable, array or parameter of a SUB or
FUNCTION SHALL be reported "not supported yet", and so SHALL `VARPTR` and `VARSEG` of a whole array `x()` that
is not of numbers. `VARPTR` of a
`_BIT` array SHALL be the old compiler's error "VARPTR cannot reference _BIT type arrays", and `VARPTR` of a whole
`TYPE` variable a compile error (the old compiler fails there, `verification\v56_t02`). `VARPTR` of the `STRING`
member of an array element and `VARPTR$` of a `STRING` member SHALL be compile errors (the old compiler's C++
build fails, `v60_d7`, `d8`). Every other variable,
element and member of the main module SHALL be taken (requirement "Variables in conventional memory"), `_BIT`
scalars included. A name alone that is an array of strings, of fixed-length strings or of a `TYPE` SHALL name the
scalar of that name, as in the old compiler.

#### Scenario: Not a variable
- **WHEN** a program has `l& = VARPTR(5)` or `l& = VARSEG(i% + 1)`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: A scalar
- **WHEN** a program of the main module has `l& = VARPTR(i%)`, `VARSEG(d#)`, `VARPTR(t$)`, `VARPTR(fx)` for a
  `STRING * 8`, `VARPTR(r(1))` for an array of a `TYPE`, `VARPTR(one.y)` for a member, or `VARSEG(one)`
- **THEN** it compiles with no diagnostic (`verification\v28_x01_accepted`, `v56_t01`, `t03`)

#### Scenario: A whole TYPE variable
- **WHEN** a program has `l& = VARPTR(one)` for a `TYPE` variable `one`
- **THEN** it is a compile error that is not "not supported yet"

#### Scenario: A variable of a SUB
- **WHEN** a SUB has `DIM k AS INTEGER: PRINT VARPTR(k)`
- **THEN** the only diagnostic is "not supported yet"

#### Scenario: A TYPE that holds a string
- **WHEN** a program has `PRINT VARPTR(x.v)` or `VARPTR(x.n)` for `x` of a `TYPE` with a `STRING` member `n`
- **THEN** it compiles with no diagnostic

#### Scenario: A STRING member where the old build fails
- **WHEN** a program has `VARPTR(e(2).n)` for an array `e` of that `TYPE`, or `VARPTR$(x.n)`
- **THEN** it is a compile error that is not "not supported yet"

### Requirement: Variables in conventional memory
A variable of the main module that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names anywhere in the program SHALL
live in libqb's conventional memory, in the old compiler's order and sizes (with every `_BIT` scalar of the main
module, named or not): `VARPTR` SHALL give the offset the old compiler's program gives, and `VARSEG` its segment
(80 for a variable). `PEEK` and `POKE` through them SHALL reach the variable's bytes, and a string SHALL keep its
descriptor there through every assignment. A `TYPE` variable or array element that holds a `STRING` (a member,
or a member of a member) SHALL take the old compiler's place and size there, 8 bytes for each `STRING` member;
its strings SHALL be made as for a `TYPE` variable anywhere else, empty, where the old compiler's program leaves
them out and crashes at the first use of one (`DIVERGENCES.md` D-037).

#### Scenario: Offsets of scalars
- **WHEN** the programs `verification\v56_a1`–`a3` print `VARPTR` and `VARSEG` of scalars of every type, strings,
  fixed-length strings and `_BIT` variables, declared in several orders
- **THEN** each prints what the old compiler's program printed

#### Scenario: Bytes through PEEK and POKE
- **WHEN** `DIM i AS INTEGER: i = 258: DEF SEG = VARSEG(i): PRINT PEEK(VARPTR(i)); PEEK(VARPTR(i) + 1): POKE
  VARPTR(i), 3: PRINT i` runs
- **THEN** it prints ` 2  1 `, then ` 259 `

#### Scenario: A string keeps its place
- **WHEN** a string named by `VARPTR` is assigned, grown, swapped, changed by `MID$`, `READ` and `LSET`, and passed
  to a SUB, and its `VARPTR`, `SADD` and descriptor printed after each (`verification\v56_a4`)
- **THEN** each value is the old compiler's

#### Scenario: Elements and members
- **WHEN** the programs `verification\v56_a6`, `a7`, `t01`, `t06`–`t09` print `VARPTR` and `VARSEG` of elements of
  arrays of a `TYPE`, of strings and of fixed-length strings, of members through member arrays and nested types,
  and of an index outside its bounds
- **THEN** each prints what the old compiler's program printed: the element's offset and the member's for a
  `TYPE`, the element's offset for fixed-length strings, the descriptor's offset for strings, error 9 for an index
  outside

#### Scenario: A TYPE that holds a string, in conventional memory
- **WHEN** the programs `verification\v56_x08`, `x26` and `x27` print `VARPTR` of a `STRING` member, of a `LONG`
  member beside it and of a member of an array element of such a `TYPE`
- **THEN** each prints what the old compiler's program printed

#### Scenario: Its string used
- **WHEN** `verification\v56_a5` prints `VARPTR(r.v); VARSEG(r)`, sets `r.v = 5`, prints it, then sets `r.n = "abc"`
  and prints `r.n; LEN(r.n)`
- **THEN** it prints the old compiler's first two lines, then `abc 3`, and ends normally (D-037: the old
  compiler's program crashes there)
