# Spec Delta

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
`_BYTE`, 8 for `_OFFSET`, `n` for `STRING * n`), without padding. A member SHALL take any name the old
compiler takes, keywords and the names of built-ins among them (`Time`, `Left`, `Case`), and a name it rejects
(`PRINT`, `END`, a name with a single leading underscore) SHALL be a compile error with its wording
(`verification\v61_m_*`). A member array marked `_STATIC` or `_DYNAMIC`
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

#### Scenario: A member named like a built-in
- **WHEN** a type has the members `Time AS INTEGER` and `x AS INTEGER`, and `DIM o AS t: o.Time = 4: o.x = 2:
  PRINT o.Time; o.x; TIME$ <> ""` runs
- **THEN** it prints ` 4  2 -1 `, as with the old compiler

#### Scenario: A name the old compiler rejects
- **WHEN** a type has the member `print AS LONG`
- **THEN** it is a compile error that is not "not supported yet"
