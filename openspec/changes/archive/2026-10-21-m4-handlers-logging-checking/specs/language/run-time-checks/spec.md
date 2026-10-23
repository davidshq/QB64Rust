# Spec Delta

## Purpose

What `$CHECKING:OFF` and `$CHECKING:ON` change in a program: which of the checks the old compiler writes between
and inside statements are left out below OFF, measured with `qb64pe.exe` (`verification\v50_*`).

## ADDED Requirements

### Requirement: The CHECKING metacommand
`$CHECKING:OFF` and `$CHECKING:ON` SHALL be accepted on a line of their own anywhere in a program, its included
files and its procedures, inside a block too, in any letter case. Each SHALL hold for the lines after it in the order of the source
text, through `SUB`s, `FUNCTION`s and included files, until the next one. A program starts with checking on. Any
other form the old compiler rejects SHALL be a compile error.

#### Scenario: Mixed case
- **WHEN** a program has `$Checking:Off` and later `$Checking:On`
- **THEN** it compiles, and the lines between them are unchecked

#### Scenario: A procedure in source order
- **WHEN** a `FUNCTION` is defined between `$CHECKING:OFF` and `$CHECKING:ON` and called from the main module
  after `$CHECKING:ON`
- **THEN** the function's statements are unchecked and the main module's are checked

### Requirement: Statements without checks
A statement below `$CHECKING:OFF` SHALL run without the check for events and errors after it, at its label and
at its block header. An error it raises SHALL stay pending until a checked statement runs, and it SHALL then be
handled at that statement, as the old compiler handles it (`v50_c_checking`). An event that comes due SHALL wait
in the same way.

#### Scenario: An error handled after the unchecked lines
- **WHEN** `ON ERROR GOTO h` is active, `ERROR 5` stands below `$CHECKING:OFF` followed by `PRINT "a"`, and
  `$CHECKING:ON` is followed by `PRINT "b"`
- **THEN** the output is that of the old compiler's build (`v50`)

#### Scenario: Unchecked FUNCTION
- **WHEN** `source_ordering/checking_off` is run
- **THEN** it prints ` 12`

### Requirement: Indexes without checks
An index of an array, of a member array or of a dynamic member array in a statement below `$CHECKING:OFF` SHALL
NOT be checked against the array's bounds. Indexes within the bounds SHALL give the same elements as with
checking on. An index outside them is not defined, in either compiler, and no test SHALL depend on it.

#### Scenario: Element within the bounds
- **WHEN** `DIM a(1 TO 3)` is filled and read below `$CHECKING:OFF`
- **THEN** the values printed are those printed with checking on

### Requirement: Memory statements without checks
`_MEMGET` (function and statement), `_MEMPUT` and `_MEMFILL` below `$CHECKING:OFF` SHALL read, write and fill
the bytes at the offset given without checking the block. No error 300, 308 or 309 SHALL be raised. With a valid
block and region the result SHALL be that of checking on.

#### Scenario: A get through an offset of no block
- **WHEN** a `FUNCTION` below `$CHECKING:OFF` reads a LONG with `_MEMGET m, p, r&` from `p = _OFFSET(x&)`, and `m`
  was never set
- **THEN** it returns the value of `x&`, as in `source_ordering/checking_off`

#### Scenario: Image bytes saved
- **WHEN** `mem/mem_checking_off` is run
- **THEN** its output equals the old compiler's

### Requirement: What CHECKING leaves alone
`$CHECKING:OFF` SHALL NOT change the value of any expression, the tests of a block's condition, `_ARRAYCOPY`'s
own checks, `ON ERROR` or `RESUME`, beyond what the requirements above name. A `$CHECKING:OFF` in a program with
`$DEBUG` SHALL compile, as with the old compiler run with `-x`.

#### Scenario: ARRAYCOPY below OFF
- **WHEN** `verification\v41_x62_checking_off` is compiled and run
- **THEN** its output equals the recorded output

#### Scenario: DEBUG with CHECKING OFF
- **WHEN** `verification\v33_x54_debug_checking_off` is compiled and run
- **THEN** it prints `ok`
