# Spec Delta

## MODIFIED Requirements

### Requirement: What a variable reads and writes
`PUT` SHALL write and `GET` SHALL read the bytes of the variable: a numeric variable its type's size, a
fixed-length string its length, a `TYPE` variable or member its size, an array element its own, a whole array
`a()` the bytes of all its elements from the first. A variable-length string SHALL be written with its
characters and read for as many characters as it holds.
A whole array of a type with dynamic member arrays (`language/arrays-and-types`, "Dynamic member arrays") SHALL
be written and read in the type's declared layout: each element as the type's members one after the other with
no padding, each dynamic member array as its elements at its declared bounds. A dynamic member array whose bounds
are not its declared ones SHALL raise error 9: `PUT` then writes nothing; `GET` reads the file and stores the
other members and elements, and leaves that member as it was (the old compiler writes the declared bytes into
the member's smaller block, `DIVERGENCES.md`). A type with a `STRING` member anywhere in it SHALL be a compile
error there ("UDT must have fixed size"), as SHALL be one value of such a type (requirement "Dynamic member
arrays"). On a file open `FOR RANDOM` the whole array SHALL be one record, as any whole array is.

#### Scenario: Numbers of each size in a binary file
- **WHEN** a program writes an INTEGER, a LONG, a DOUBLE and a `_BYTE` with `PUT #1, , v` to a file open `FOR
  BINARY`, and prints `LOF(1)`
- **THEN** it prints 15, and reading them back with `GET` gives the values written

#### Scenario: A whole array
- **WHEN** `DIM a(1 TO 4) AS LONG`, filled, is written with `PUT #1, , a()` and read into another array with `GET
  #1, 1, b()`
- **THEN** the file is 16 bytes long and `b` holds the four values

#### Scenario: A TYPE variable
- **WHEN** a variable of a `TYPE` with a LONG and a `STRING * 6` member is written with `PUT` and read back into
  another with `GET`
- **THEN** the file is 10 bytes long and both members come back

#### Scenario: An empty string reads nothing
- **WHEN** `s$ = "": GET #1, 1, s$` runs on a binary file that holds `abcdef`, then `s$ = SPACE$(3): GET #1, 1,
  s$`
- **THEN** `s$` is empty after the first and `abc` after the second

#### Scenario: A whole array with dynamic members
- **WHEN** `REDIM p(0 TO 1)` of a type with `a AS INTEGER`, `v(1) _DYNAMIC AS LONG`, `b AS INTEGER`, each member
  at its declared bounds and set, is written with `PUT #1, 1, p()`
- **THEN** the file is 24 bytes, each element's `a`, two LONGs and `b`; `GET #1, 1, q()` into another such array
  gives back every value

#### Scenario: A member with other bounds
- **WHEN** `p(0).v` has bounds 0 to 2 and `PUT #1, 1, p()` runs under a handler, and later `q(0).v` has bounds 5
  to 5 and `GET #1, 1, q()` runs
- **THEN** the `PUT` raises error 9 and the file stays empty; the `GET` raises error 9, `q(0).a`, `q(0).b` and
  `q(1)` are read, and `q(0).v` keeps its bounds and element

#### Scenario: Strings
- **WHEN** `PUT #1, 1, p()` is compiled for a type with `v(1) _DYNAMIC AS STRING`
- **THEN** it is a compile error
