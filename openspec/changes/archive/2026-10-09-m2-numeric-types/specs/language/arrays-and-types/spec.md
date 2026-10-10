## MODIFIED Requirements

### Requirement: Static arrays
`DIM name(bounds) [AS type]` and `DIM SHARED name(bounds) [AS type]` in the main module, with bounds that are
constant expressions, SHALL declare an array of any numeric type except `_BIT`, of `STRING`, of a fixed-length
string or of a user type: each
dimension is `upper` (lower bound 0) or `lower TO upper`, each bound rounded half to even to a whole number; every
element starts at zero, as an empty string, or as NUL bytes for a fixed-length string. The `DIM` line SHALL do
nothing when it runs. An array SHALL be a name
plus a type, in a name space of its own beside scalar variables. An upper bound below its lower bound, and a second
`DIM` of the same array, SHALL be compile errors. Every other way to make an array (bounds that are not constant,
`DIM` of an array inside a SUB or FUNCTION, `REDIM`, `STATIC`, `OPTION BASE`, `$DYNAMIC`, `SHARED name()`, array
parameters, an array used without `DIM`, and an array of `_BIT` or `_BIT * n`) SHALL be reported "not supported
yet".

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

#### Scenario: Array in a SUB
- **WHEN** `DIM t(5)` stands inside a SUB
- **THEN** the compiler reports it "not supported yet"

#### Scenario: Array used without DIM
- **WHEN** `z(3) = 4` is compiled and no array `z` is declared
- **THEN** the compiler reports it "not supported yet"

#### Scenario: Unsigned elements
- **WHEN** `DIM u(2) AS _UNSIGNED INTEGER: u(1) = -1: PRINT u(1); u(2)` runs
- **THEN** it prints ` 65535  0 `

#### Scenario: Bit array
- **WHEN** `DIM f(7) AS _BIT` is compiled
- **THEN** the compiler reports it "not supported yet"

### Requirement: User types
A `TYPE name … END TYPE` block in the main module SHALL define a user type whose members are of the numeric types
other than `_BIT`, of fixed-length strings, or of user types defined in earlier blocks; the program's types SHALL
be known before its statements (a type may be used above its block). `DIM v AS name` SHALL declare a variable of
that type, in every storage class a scalar variable may have; every member starts at zero (NUL bytes for a
fixed-length string), a local one on each call. A member's size in the layout SHALL be its type's (1 byte for
`_BYTE`, 8 for `_OFFSET`, `n` for `STRING * n`), without padding. Members that are arrays, variable-length
`STRING`, and a `TYPE` block inside a SUB or FUNCTION, SHALL be reported "not supported yet"; a member declared
with a type suffix, and a `_BIT` or `_BIT * n` member, SHALL be compile errors (the old compiler: "Cannot use _BIT
inside user defined types"). A whole user-type value in an expression, a `PRINT`
item or stored into a scalar, and a number stored into a user-type variable, SHALL be compile errors; an
assignment of one user-type variable to another and a user-type argument or parameter SHALL be reported "not
supported yet".

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
