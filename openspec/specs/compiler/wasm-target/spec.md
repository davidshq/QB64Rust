# compiler/wasm-target Specification

## Purpose
The WebAssembly target of `qb64rust`: what a build with `--target wasm` writes, which programs it takes, what
it rejects, and whose behavior a wasm program is held to.

## Requirements

### Requirement: A wasm build
A build with `--target wasm` SHALL compile the program's C++ and libqb with the Emscripten SDK for `wasm32` and
write two files: `<name>.js` and `<name>.wasm`. `node <name>.js` SHALL run the program. Its standard input,
standard output, exit status, command line and files SHALL be those of the Node process, and its current
folder the folder of `<name>.js`.

#### Scenario: Hello
- **WHEN** `qb64rust -x --target wasm hello.bas` builds a `$CONSOLE:ONLY` program that prints `Hello`
- **THEN** `hello.js` and `hello.wasm` are beside the source, and `node hello.js` prints `Hello` and exits
  with status 0

#### Scenario: A file beside the program
- **WHEN** a wasm program opens `data.txt` for input and the file is in the folder of its `.js` file
- **THEN** it reads the file's lines, whatever folder `node` was started from

#### Scenario: The exit status
- **WHEN** a wasm program ends with `SYSTEM 3`
- **THEN** the Node process exits with status 3

### Requirement: Console programs only
The wasm target SHALL build only a program with `$CONSOLE:ONLY`. Any other program SHALL be rejected at
compile time with an error that names the target and `$CONSOLE:ONLY`, and nothing SHALL be written.

#### Scenario: A window program
- **WHEN** `qb64rust -x --target wasm game.bas` runs on a program without `$CONSOLE:ONLY`
- **THEN** the exit status is 1, the error says the wasm target builds only `$CONSOLE:ONLY` programs so far,
  and no `game.js` exists

### Requirement: What the target does not have is rejected
A program that uses a word the wasm target does not have SHALL be rejected at compile time, at the word's
line, with "`<word>` is not available on the wasm target", and never built with a routine that does nothing.
The words: `SHELL`, `_SHELLHIDE`, `RUN` with a file, `CHAIN`, `_OPENHOST`, `_OPENCLIENT`, `_OPENCONNECTION`,
`DECLARE DYNAMIC LIBRARY`, a `DECLARE LIBRARY` of a prebuilt library, `$DEBUG` and `_CLIPBOARD$`.

#### Scenario: SHELL
- **WHEN** a program with `SHELL "dir"` on line 4 is compiled with `--target wasm`
- **THEN** the exit status is 1 and the error at line 4 is "`SHELL` is not available on the wasm target"

#### Scenario: The same program for the host
- **WHEN** the same program is compiled without `--target`
- **THEN** it builds as before

#### Scenario: A header-only library
- **WHEN** a program's `DECLARE LIBRARY "helper"` resolves to `helper.h` alone
- **THEN** the wasm build compiles the header with the program

#### Scenario: Checking only
- **WHEN** the program with `SHELL` is checked with `qb64rust -z --target wasm`
- **THEN** the same error is reported and the exit status is 1

#### Scenario: $DEBUG
- **WHEN** a program with `$DEBUG` on line 2 is compiled with `--target wasm`
- **THEN** the exit status is 1 and the error at line 2 is "`$DEBUG` is not available on the wasm target"

### Requirement: Parts of libqb the target does not build yet
A program that uses a built-in whose part of libqb the wasm build leaves out SHALL be rejected at compile time,
at the built-in's line, with "`<word>` is not available on the wasm target yet: libqb's `<part>` part is not
built for it". The part is the built-in table's `dependency`: `DEPENDENCY_MINIAUDIO` (audio),
`DEPENDENCY_DEVICEINPUT` (device-input, and `ON STRIG`) or `DEPENDENCY_SCREENIMAGE` (screen-image).

#### Scenario: A new built-in of a part
- **WHEN** a built-in is added to the table with one of these dependencies
- **THEN** a wasm build rejects it without another change

#### Scenario: SOUND
- **WHEN** a program with `SOUND 440, 1` on line 2 is compiled with `--target wasm`
- **THEN** the exit status is 1 and the error at line 2 names `SOUND`, says "yet" and names the audio part

#### Scenario: ON STRIG
- **WHEN** a program with `ON STRIG(0) GOSUB h` on line 3 is compiled with `--target wasm`
- **THEN** the error at line 3 names `ON STRIG` and the device-input part

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

### Requirement: The target's name in a program
On the wasm target, `$IF WASM` SHALL be true and `$IF WIN`, `$IF WINDOWS`, `$IF LINUX`, `$IF MAC` and
`$IF MACOSX` false; `$IF 32BIT` SHALL be true and `$IF 64BIT` false. `_OS$` SHALL be `[WASM][32BIT]`. On the
other targets `$IF WASM` SHALL be false.

#### Scenario: A branch for the target
- **WHEN** a program prints `wasm` inside `$IF WASM THEN` and `other` inside `$ELSE`, and is built with
  `--target wasm`
- **THEN** it prints `wasm`; built for Windows or Linux it prints `other`

#### Scenario: _OS$
- **WHEN** a wasm program prints `_OS$`
- **THEN** it prints `[WASM][32BIT]`

### Requirement: A wasm program behaves as on Linux
A wasm program SHALL give the output and exit status that QB64pe's Linux build gives for the same program,
apart from the rows of `DIVERGENCES-QB64PE.md` that name the wasm target. Where the result comes from the C
library or from `_FLOAT` arithmetic, the wasm target SHALL give a well-defined value, and the row SHALL say
which.

#### Scenario: Comma zones
- **WHEN** a wasm program prints `1, 2`
- **THEN** the items are separated as on the Linux console (D-011)

#### Scenario: A _FLOAT result
- **WHEN** a wasm program prints a `_FLOAT` value whose last digits differ from the x87 result
- **THEN** the value is the one the target's row of `DIVERGENCES-QB64PE.md` defines, and the program is on
  `known.wasm.list` with the reason `float`

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
