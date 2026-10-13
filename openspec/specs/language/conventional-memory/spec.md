# language/conventional-memory Specification

## Purpose
TBD - created by archiving change m3-get-put-bload. Update Purpose after archive.

## Requirements

### Requirement: VARPTR and VARSEG of an array of the main module
For an array `a` of numbers declared in the main module, static or dynamic, named from any body, `VARPTR(a(i))`
SHALL be the element's index from the array's first element times the size of an element, with the bounds check
of an array read, and `VARSEG(a(i))` the segment of the array's data in libqb's conventional memory, the same for
every element. Both SHALL be `_INTEGER64` values to `PRINT` and to operators, as in the old compiler.

#### Scenario: Offsets
- **WHEN** `DIM a(1 TO 10) AS INTEGER, c(0 TO 4) AS LONG: PRINT VARPTR(a(1)); VARPTR(a(2)); VARPTR(c(3))` runs
- **THEN** it prints ` 0  2  12 `

#### Scenario: Two dimensions
- **WHEN** `DIM m(1 TO 3, 1 TO 4) AS INTEGER: PRINT VARPTR(m(2, 1)); VARPTR(m(1, 2))` runs
- **THEN** it prints ` 2  6 `, as with the old compiler

### Requirement: What VARPTR and VARSEG accept
An argument of `VARPTR` or `VARSEG` that is not a variable or an array element (a literal, an expression, a
function call) SHALL be a compile error. `VARPTR` and `VARSEG` of anything but an array of numbers of the main
module (a scalar, a string, an array or variable of a procedure, a parameter, an array of a user type, of strings
or of `_BIT`, a member), `SADD` and `VARPTR$` SHALL be reported "not supported yet".

#### Scenario: Not a variable
- **WHEN** a program has `l& = VARPTR(5)` or `l& = VARSEG(i% + 1)`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: A scalar
- **WHEN** a program has `l& = VARPTR(i%)`
- **THEN** the only diagnostic is "not supported yet" and names `VARPTR` of a variable that is no array element

### Requirement: Arrays in conventional memory
An array named in a `VARPTR` or `VARSEG` anywhere in the program SHALL be allocated in libqb's conventional memory
as the old compiler allocates it: a static one in the order the arrays are declared, each rounded up to 16 bytes,
with error 257 when it is larger than 65,536 bytes or the memory is used up; a dynamic one at each allocation.
Every other use of the array (reads, stores, `ERASE`, `REDIM`, `LBOUND`, passing it to a procedure) SHALL behave
as for any array of its kind.

#### Scenario: Arrays lie one after the other
- **WHEN** `DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER: PRINT VARSEG(b(1)) - VARSEG(a(1)); VARSEG(a(1)) =
  VARSEG(a(7))` runs
- **THEN** it prints ` 2 -1 `, as with the old compiler

#### Scenario: A dynamic array
- **WHEN** an array listed in `COMMON` and then given bounds by `DIM`, or made by `REDIM`, is named in `VARSEG`,
  erased, redimmed with `_PRESERVE` and passed to a SUB that redims it
- **THEN** `PEEK` through `DEF SEG = VARSEG(…)` reads its elements after each, as with the old compiler

#### Scenario: An array used before the line that names it
- **WHEN** a program stores into `a(3)`, and a later line, or a procedure, has `VARSEG(a(1))`
- **THEN** the array is in conventional memory from its `DIM` on, and the program prints what the old compiler's
  program prints

### Requirement: DEF SEG, PEEK and POKE
`DEF SEG = n` SHALL set the segment `PEEK`, `POKE`, `BLOAD` and `BSAVE` use, and `DEF SEG` alone SHALL set it back
to the default, each as the runtime call the old compiler writes; `POKE offset, value` SHALL store one byte and
`PEEK(offset)` SHALL give one, a LONG. Each numeric argument SHALL be converted as a LONG slot is. `DEF SEG` with a
value and no `=`, with `=` and no value, or with a string SHALL be a compile error.

#### Scenario: Bytes of an array
- **WHEN** `DIM a(1 TO 10) AS INTEGER`, then `DEF SEG = VARSEG(a(1)): POKE VARPTR(a(1)), 65: POKE VARPTR(a(1)) +
  1, 1: a(4) = &H1234: PRINT a(1); PEEK(VARPTR(a(4))); PEEK(VARPTR(a(4)) + 1)` runs
- **THEN** it prints ` 321  52  18 `

#### Scenario: A value that does not fit a byte
- **WHEN** `POKE VARPTR(a(5)), 300: PRINT a(5)` runs after `DEF SEG = VARSEG(a(1))`
- **THEN** it prints ` 44 `, as with the old compiler

#### Scenario: A rejected form
- **WHEN** a program has `DEF SEG 5`
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: BLOAD and BSAVE
`BSAVE file$, offset, length` SHALL write the block at the offset in the current segment to a file, and `BLOAD
file$[, offset]` SHALL read such a file back to the offset, or to the offset it was saved from when none is given,
each as the runtime call the old compiler writes. A file that does not exist SHALL raise error 53.

#### Scenario: Saved and loaded into another array
- **WHEN** ten elements of an INTEGER array `a` are saved with `BSAVE "t.tmp", VARPTR(a(1)), 20` under `DEF SEG =
  VARSEG(a(1))` and loaded with `BLOAD "t.tmp", VARPTR(b(1))` under `DEF SEG = VARSEG(b(1))`
- **THEN** `b` holds the ten values of `a`, and the file is 27 bytes long

#### Scenario: Loaded in a procedure into a shared array
- **WHEN** a SUB runs `DEF SEG = VARSEG(g(1)): BLOAD "t.tmp", VARPTR(g(1)): DEF SEG` for an array `g` of the main
  module declared `DIM SHARED`
- **THEN** `g` holds the saved values after the call

#### Scenario: No such file
- **WHEN** `BLOAD "missing.tmp", VARPTR(b(1))` runs under a handler with `RESUME NEXT`
- **THEN** error 53 is raised and the program goes on

#### Scenario: A picture of Civil War Strategy
- **WHEN** a screen program loads the game's `MTN.VGA` into `DIM mtn(1 TO 1564)` as the game does and draws it
  with `PUT (380, 30), mtn, PSET`
- **THEN** the state of its screen is the one recorded with the old compiler
