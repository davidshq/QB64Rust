# Spec Delta

## Purpose

The WebAssembly target of `qb64rust`: what a build with `--target wasm` writes, which programs it takes, what
it rejects, and whose behavior a wasm program is held to.

## ADDED Requirements

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

### Requirement: Pointer-sized types wait for their width
Until `_OFFSET` is 4 bytes on this target in every pass, a wasm build SHALL reject a program that uses the
type `_OFFSET`, `_UNSIGNED _OFFSET` or `_MEM`, a value of one, or a built-in that takes or returns one, with
"`<word>` is not available on the wasm target yet". So SHALL a `TYPE` member that is an array of
variable-length `STRING`, whose elements are pointers. No wasm program SHALL be built in which the compiler and
libqb disagree about a pointer's size.

#### Scenario: A value of the type without a declaration
- **WHEN** a program prints `_CAST(_OFFSET, f!)`, `_CV(_OFFSET, s$)` or a `DECLARE LIBRARY` function named
  `strlen~%&`, compiled with `--target wasm`
- **THEN** each is an error that names `_OFFSET` or `_UNSIGNED _OFFSET` and says "yet"

#### Scenario: A variable of the type
- **WHEN** a program with `DIM p AS _OFFSET` is compiled with `--target wasm`
- **THEN** the exit status is 1 and the error names `_OFFSET` and says "yet"

#### Scenario: A built-in that returns one
- **WHEN** a program with `m = _MEMNEW(16)` is compiled with `--target wasm`
- **THEN** the exit status is 1 and the error names `_MEMNEW`

#### Scenario: A member array of strings
- **WHEN** a program's `TYPE` has the member `a(0 TO 1) AS STRING` on line 6 and is compiled with `--target wasm`
- **THEN** the exit status is 1 and the error at line 6 names `STRING` and says "yet"; a single `STRING` member,
  a numeric member array and a numeric `_DYNAMIC` member are no error

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
written on Linux, byte for byte. A difference is allowed only where a requirement of this spec names it.

#### Scenario: Same fragments
- **WHEN** a `$CONSOLE:ONLY` program without `DECLARE LIBRARY` is compiled with `-z --target wasm` and with
  `-z` on Linux
- **THEN** the fragments are equal byte for byte
