# Spec Delta

## Purpose

The `_MEM` type and the memory blocks it describes: how a program takes a block of a variable, an array, an image
or new memory, reads and writes bytes through it, frees it, and what happens when a block is no longer valid, each
as the old compiler's program does.

## ADDED Requirements

### Requirement: The _MEM type
`_MEM` SHALL be a type that is defined before the program's first line, wherever the old compiler accepts a user
type (`DIM`, `REDIM`, `SHARED`, `STATIC`, an array, a member, a parameter), with the members `OFFSET`, `SIZE`,
`TYPE`, `ELEMENTSIZE` (`_OFFSET`), `IMAGE` and `SOUND` (LONG). A whole value SHALL copy as other user types do.
A store to a member SHALL be a compile error, as in the old compiler.

#### Scenario: Members of a block of a LONG
- **WHEN** `DIM a AS LONG, m AS _MEM: m = _MEM(a): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.SOUND` runs
- **THEN** it prints ` 4  132  4 -1  0 `, as with the old compiler

#### Scenario: A store to a member
- **WHEN** `DIM m AS _MEM: m.SIZE = 4` is compiled
- **THEN** it is a compile error ("Cannot set read-only element of _MEM TYPE"), as in the old compiler

#### Scenario: Whole value copied
- **WHEN** `DIM a AS LONG, m AS _MEM, n AS _MEM: m = _MEM(a): n = m: PRINT n.OFFSET = m.OFFSET; n.SIZE` runs
- **THEN** it prints `-1  4 `

### Requirement: A block of a reference
`_MEM(reference)` SHALL give the block of a variable, a fixed-length string, an element (from it to the array's
end), a whole array `a()`, a member or a member array, and `_MEMELEMENT(reference)` the block of the one element
or value, with the old compiler's `SIZE`, `TYPE` and `ELEMENTSIZE`. A reference the old compiler rejects (a
variable-length string) SHALL be a compile error with its message.

#### Scenario: Whole array, element and element alone
- **WHEN** `DIM b(10) AS INTEGER` and the program prints `SIZE`, `TYPE` and `ELEMENTSIZE` of `_MEM(b())`, then
  `SIZE` of `_MEM(b(4))`, then `SIZE` of `_MEMELEMENT(b(4))`
- **THEN** it prints ` 22  65666  2 `, ` 14 ` and ` 2 `, as with the old compiler

#### Scenario: Fixed-length string
- **WHEN** `DIM s AS STRING * 6, m AS _MEM: m = _MEM(s): PRINT m.SIZE; m.TYPE` runs
- **THEN** it prints ` 6  512 `

#### Scenario: Variable-length string
- **WHEN** `DIM s AS STRING, m AS _MEM: m = _MEM(s)` is compiled
- **THEN** it is a compile error ("_MEM cannot reference variable-length strings")

### Requirement: A block of an address
`_MEM(offset, size)` SHALL give a block of `size` bytes at `offset`, which SHALL be an `_OFFSET` value; any other
type SHALL be a compile error, as in the old compiler ("Expected _MEM(_OFFSET-value,...)").

#### Scenario: Block at an address
- **WHEN** `DIM a AS LONG, m AS _MEM: a = 9: m = _MEM(_OFFSET(a), 4): PRINT _MEMGET(m, m.OFFSET, LONG); m.SIZE` runs
- **THEN** it prints ` 9  4 `, and its `TYPE` is 16384, as with the old compiler

#### Scenario: Address of another type
- **WHEN** `DIM m AS _MEM: m = _MEM(5, 4)` is compiled
- **THEN** it is a compile error ("Expected _MEM(_OFFSET-value,...)")

### Requirement: New memory, images and sounds
`_MEMNEW(bytes)` SHALL give a block of new memory and `_MEMIMAGE[(handle)]` the pixels of an image (the
destination when no handle is given), as libqb gives them; `_MEMSOUND` SHALL be compiled and never run by a test.
`_MEMEXISTS(block)` SHALL be -1 while the block is valid and 0 after it is freed.

#### Scenario: New memory freed
- **WHEN** `DIM n AS _MEM: n = _MEMNEW(16): PRINT _MEMEXISTS(n); n.SIZE; n.TYPE: _MEMFREE n: PRINT _MEMEXISTS(n)`
  runs
- **THEN** it prints `-1  16  16384 ` and then ` 0 `

#### Scenario: Pixels of an image
- **WHEN** a program makes a 4 by 2 image with `_NEWIMAGE(4, 2, 32)`, `PSET`s one pixel and reads it with
  `_MEMGET(_MEMIMAGE(img), …, _UNSIGNED LONG)` at its offset
- **THEN** it prints the colour it set (`FF0A141E` for `_RGB32(10, 20, 30)`), `SIZE` is 32 and `IMAGE` is the
  image's handle, as with the old compiler

### Requirement: Reading and writing through a block
`_MEMGET(block, offset, type)`, the statement `_MEMGET block, offset, variable`, `_MEMPUT block, offset,
value [AS type]`, `_MEMFILL block, offset, bytes, value [AS type]` and `_MEMCOPY block, offset, bytes TO block,
offset` SHALL read, write, fill and copy bytes as the old compiler's program does, for every numeric type, a
fixed-length string and a user type it takes. A string, `_BIT` or user type where a number is needed SHALL be a
compile error, as in the old compiler.

#### Scenario: Put and get
- **WHEN** `DIM a AS LONG, m AS _MEM: m = _MEM(a): _MEMPUT m, m.OFFSET, 7 AS LONG: PRINT a; _MEMGET(m,
  m.OFFSET, LONG)` runs
- **THEN** it prints ` 7  7 `

#### Scenario: Fill and copy
- **WHEN** `DIM b(10) AS INTEGER, a AS LONG`, `a = 7`, `_MEMFILL` of all of `b()` with `3 AS INTEGER`, then
  `_MEMCOPY` of 2 bytes of `_MEM(a)` to byte 4 of `_MEM(b())`, and `PRINT b(0); b(10); b(2); b(3)` runs
- **THEN** it prints ` 3  3  7  3 `

#### Scenario: Put a string as a number
- **WHEN** `DIM m AS _MEM: m = _MEMNEW(4): _MEMPUT m, m.OFFSET, "ab" AS LONG` is compiled
- **THEN** it is a compile error ("Illegal string-number conversion"), as in the old compiler

### Requirement: A block that is no longer valid
A block SHALL stop being valid when it is freed, when the SUB or FUNCTION whose variable it describes returns,
and when its array is redimensioned or erased. Reading, writing or freeing through it, or outside its region,
SHALL be the old compiler's runtime error (300 to 313), which ends the program with libqb's message and is not
trapped by `ON ERROR`, as in the old compiler.

#### Scenario: Write after free
- **WHEN** a program with `ON ERROR GOTO h` frees a block from `_MEMNEW` and then `_MEMPUT`s into it
- **THEN** it stops with "Memory has been freed" and the handler does not run, as with the old compiler

#### Scenario: Write outside the region
- **WHEN** `DIM a AS LONG, m AS _MEM: m = _MEM(a): _MEMPUT m, m.OFFSET + 2, 1 AS LONG` runs
- **THEN** it stops with "Memory region out of range"

#### Scenario: Block of an erased array
- **WHEN** `REDIM d(5) AS LONG`, `m = _MEM(d())`, `ERASE d`, then `PRINT _MEMEXISTS(m)` runs
- **THEN** it prints `-1`, as the old compiler's program does (`_MEMEXISTS` does not look at the lock's id;
  listed for review)

### Requirement: The address of a reference
`_OFFSET(reference)` SHALL give the address of a variable, an element or an array as an `_UNSIGNED _OFFSET`; a
value that is not a reference SHALL be a compile error, as SHALL a `_BIT` array, as in the old compiler.

#### Scenario: Address equals the block's offset
- **WHEN** `DIM a AS LONG, m AS _MEM: m = _MEM(a): PRINT m.OFFSET = _OFFSET(a)` runs
- **THEN** it prints `-1`

#### Scenario: Not a reference
- **WHEN** `PRINT _OFFSET(5)` is compiled
- **THEN** it is a compile error ("_OFFSET expects the name of a variable/array")
