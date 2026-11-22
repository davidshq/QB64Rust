## ADDED Requirements

### Requirement: A pointer is 4 bytes on this target
On the wasm target a pointer SHALL be 4 bytes, the size libqb has on `wasm32`; on Windows and Linux it SHALL stay
8. `_OFFSET` and `_UNSIGNED _OFFSET` SHALL take 4 bytes and hold 32-bit values. If the compiler and libqb
disagree about a pointer's size, the C++ build SHALL fail.

#### Scenario: LEN of an _OFFSET
- **WHEN** a `$CONSOLE:ONLY` program with `DIM p AS _OFFSET` prints `LEN(p)`, built with `--target wasm`
- **THEN** it prints ` 4 `; built for Windows or Linux it prints ` 8 `

### Requirement: Arithmetic on an _OFFSET on this target
On the wasm target, an operator with an `_OFFSET` operand SHALL keep the result type of
`language/numeric-semantics`. It SHALL be computed in 64 bits and cut to 32 when stored in an `_OFFSET`. `HEX$`,
`OCT$` and `BIN$` of an `_OFFSET` SHALL give its 32 bits. An `_OFFSET` variable SHALL be passed by reference to
a parameter of another integer type only when that type is 32 bits wide.

#### Scenario: An unsigned _OFFSET of -1
- **WHEN** a program has `DIM u AS _UNSIGNED _OFFSET: u = -1: PRINT u; HEX$(u)`, built with `--target wasm`
- **THEN** it prints ` 4294967295 FFFFFFFF`

### Requirement: Layouts with pointers on this target
On the wasm target, a `TYPE`'s `_OFFSET` member, variable-length `STRING` member and dynamic member array slot
SHALL each take 4 bytes, also in `LEN` of a member array of `STRING` and in `_MEM` of a value. A type that owns
dynamic member arrays SHALL align them to 4 bytes. `_MEM` SHALL take 40 bytes, libqb's block on `wasm32`.
Where `language/arrays-and-types`, `language/mem-blocks` or `language/numeric-semantics` give 8 bytes for one
of these, that is the Windows and Linux number.

#### Scenario: A TYPE with an _OFFSET and a STRING
- **WHEN** a program has `TYPE t: a AS LONG: p AS _OFFSET: s AS STRING * 2: END TYPE` (as a block) and prints
  `LEN` of a value of it, built with `--target wasm`
- **THEN** it prints ` 10 `; for Windows or Linux ` 14 `

#### Scenario: _MEM
- **WHEN** a program with `DIM m AS _MEM` prints `LEN(m)`, built with `--target wasm`
- **THEN** it prints ` 40 `

#### Scenario: A block of a LONG
- **WHEN** a program has `DIM a AS LONG: m = _MEM(a): _MEMPUT m, m.OFFSET, 7 AS LONG: PRINT a; m.SIZE`, built
  with `--target wasm`
- **THEN** it prints ` 7  4 `

#### Scenario: A member array of strings
- **WHEN** a program's `TYPE` has the member `a(0 TO 1) AS STRING`, and it stores into both elements and prints
  them, built with `--target wasm`
- **THEN** it builds and prints the two strings

### Requirement: An array for an _OFFSET array parameter on this target
On the wasm target, an array passed to an array parameter of `_OFFSET` SHALL have integer elements of 4 bytes:
a `LONG` array is accepted and an `_INTEGER64` array is a compile error, the reverse of Windows and Linux.

#### Scenario: An _INTEGER64 array for an _OFFSET array parameter
- **WHEN** a program passes `q()`, of `DIM q(3) AS _INTEGER64`, to `SUB f (a() AS _OFFSET)`, checked with
  `--target wasm`
- **THEN** it is the error that the array cannot be passed to an array parameter of type `_OFFSET`; for
  Windows or Linux it builds

### Requirement: Reading and writing an _OFFSET on this target
On the wasm target, `PUT` and `GET`, `_MK$(_OFFSET, …)` and `_CV(_OFFSET, …)` SHALL write and read 4 bytes for an
`_OFFSET`. `INPUT` and `INPUT #` into one SHALL store 4 bytes. `_MEM` of an `_OFFSET` SHALL give the `TYPE` and
`ELEMENTSIZE` of a 4-byte offset.

#### Scenario: _MK$ of an _OFFSET
- **WHEN** a program prints `LEN(_MK$(_OFFSET, 5))`, built with `--target wasm`
- **THEN** it prints ` 4 `

## MODIFIED Requirements

### Requirement: The C++ is the same as for Linux
For a program the wasm target accepts, the C++ fragments written with `--target wasm` SHALL equal those
written on Linux, byte for byte. A difference is allowed only where a requirement of this spec names it. The
requirements on the pointer's size name one: a size, an offset, an alignment or a number of bits that comes from
a pointer's size, and the size checks that hold the pointer's size.

#### Scenario: Same fragments
- **WHEN** a `$CONSOLE:ONLY` program without `DECLARE LIBRARY` is compiled with `-z --target wasm` and with
  `-z` on Linux
- **THEN** the fragments are equal byte for byte, apart from the size checks, when the program has no
  `_OFFSET`, `_MEM`, `STRING` member or dynamic member array

#### Scenario: A program with an _OFFSET
- **WHEN** a program with `DIM p AS _OFFSET` is compiled with `-z --target wasm` and with `-z` on Linux
- **THEN** the fragments differ only in the sizes and offsets that come from the pointer's size

## REMOVED Requirements

### Requirement: Pointer-sized types wait for their width
**Reason**: `_OFFSET`, `_MEM` and a member array of `STRING` now have the target's pointer size in every pass
(requirements "A pointer is 4 bytes on this target" and "Layouts with pointers on this target"), so they are
built, not rejected.
**Migration**: None. Programs that were rejected with "`<word>` is not available on the wasm target yet" for
one of these build now. The other "yet" errors, for the parts of libqb the target does not build, stay.
