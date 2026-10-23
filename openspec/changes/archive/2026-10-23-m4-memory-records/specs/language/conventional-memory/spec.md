# Spec Delta

## MODIFIED Requirements

### Requirement: What VARPTR and VARSEG accept
An argument of `VARPTR` or `VARSEG` that is not a variable or an array element (a literal, an expression, a
function call) SHALL be a compile error. `VARPTR` and `VARSEG` of a variable, array or parameter of a SUB or
FUNCTION SHALL be reported "not supported yet", and so SHALL those of a variable, member or element of a `TYPE`
that holds a `STRING`, and `VARPTR` and `VARSEG` of a whole array `x()` that is not of numbers. `VARPTR` of a
`_BIT` array SHALL be the old compiler's error "VARPTR cannot reference _BIT type arrays", and `VARPTR` of a whole
`TYPE` variable a compile error (the old compiler fails there, `verification\v56_t02`). Every other variable,
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
- **WHEN** a program has `PRINT VARPTR(x.v)` for `x` of a `TYPE` with a `STRING` member
- **THEN** the only diagnostic is "not supported yet" (the old compiler's program crashes at the first use of the
  string, `verification\v56_a5`)

## ADDED Requirements

### Requirement: Variables in conventional memory
A variable of the main module that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names anywhere in the program SHALL
live in libqb's conventional memory, in the old compiler's order and sizes (with every `_BIT` scalar of the main
module, named or not): `VARPTR` SHALL give the offset the old compiler's program gives, and `VARSEG` its segment
(80 for a variable). `PEEK` and `POKE` through them SHALL reach the variable's bytes, and a string SHALL keep its
descriptor there through every assignment.

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

### Requirement: SADD
`SADD(s$)` of a variable-length string variable or element, or of `s$()` (its first element), SHALL give the
offset of its characters in conventional memory. `SADD` of anything else SHALL be a compile error, with the old
compiler's wording where it has one.

#### Scenario: SADD as the string grows
- **WHEN** the program `verification\v56_b1` prints `SADD` of a string before and after it grows, and of an
  element of an array of strings
- **THEN** it prints what the old compiler's program printed

#### Scenario: SADD of a fixed-length string or a literal
- **WHEN** a program has `l = SADD(fx)` for a `STRING * 4`, or `l = SADD("a")`
- **THEN** it is a compile error with the old compiler's wording, "SADD only accepts variable-length string
  variables", as a note (`verification\v28_x55`, `x56`)

#### Scenario: SADD of a number or a TYPE
- **WHEN** a program has `l = SADD(i)` for an INTEGER, or `l = SADD(r)` for a `TYPE` variable
- **THEN** it is a compile error (`verification\v56_x03`, `x09`)

### Requirement: VARPTR$
`VARPTR$(v)` of a number, a `_BIT`, a string, a fixed-length string, a `TYPE` variable, a member, an element of an
array of strings or `s$()` SHALL give the old compiler's string: a type byte, then an offset (for a fixed-length
string its descriptor's, for a member its variable's). `VARPTR$` of an element of an array that is not of
variable-length strings SHALL be the old compiler's error.

#### Scenario: The bytes of VARPTR$
- **WHEN** the programs `verification\v56_b1`–`b5`, `t04` and `t05` print `ASC` of each byte of `VARPTR$` of each
  type
- **THEN** each prints what the old compiler's program printed

#### Scenario: VARPTR$ of a number array
- **WHEN** a program has `t = VARPTR$(a(1))` for an INTEGER array
- **THEN** it is a compile error with the note "VARPTR$ only accepts variable-length string arrays"
  (`verification\v28_x57`)

### Requirement: VARPTR and VARSEG in HEX$, OCT$ and _BIN$
`VARPTR`, `VARSEG`, `SADD` or `_SCREENEXISTS` as the direct argument of `HEX$`, `OCT$` or `_BIN$` SHALL compile,
passed to the runtime as the old compiler passes it (the width 0), and SHALL give the old compiler's string.

#### Scenario: HEX$ of VARPTR
- **WHEN** `DIM a(1 TO 10) AS INTEGER: PRINT HEX$(VARPTR(a(9))); OCT$(VARSEG(a(1)) - VARSEG(a(1)))` runs
- **THEN** it prints `10` and `0`

#### Scenario: The forms measured
- **WHEN** the program `verification\v56_c_radix_width` runs
- **THEN** it prints what the old compiler's program printed (`HEX$(_SCREENEXISTS)` is empty where the value is -1)
