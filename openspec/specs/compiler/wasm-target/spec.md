# compiler/wasm-target Specification

## Purpose
The WebAssembly target of `qb64rust`: what a build with `--target wasm` writes, which programs it takes, what
it rejects, and whose behavior a wasm program is held to.

## Requirements

### Requirement: A wasm build
A build with `--target wasm` SHALL compile the program's C++ and libqb with the Emscripten SDK for `wasm32`. For a
`$CONSOLE:ONLY` program it SHALL write two files: `<name>.js` and `<name>.wasm`. `node <name>.js` SHALL run the
program. Its standard input, standard output, exit status, command line and files SHALL be those of the Node
process, and its current folder the folder of `<name>.js`. A program without `$CONSOLE:ONLY` SHALL be built for a
browser page ("A window program runs in a page").

#### Scenario: Hello
- **WHEN** `qb64rust -x --target wasm hello.bas` builds a `$CONSOLE:ONLY` program that prints `Hello`
- **THEN** `hello.js` and `hello.wasm` are in the output folder `build`, and `node build/hello.js` prints `Hello` and
  exits with status 0

#### Scenario: A file beside the program
- **WHEN** a wasm program opens `data.txt` for input and the file is in the folder of its `.js` file
- **THEN** it reads the file's lines, whatever folder `node` was started from

#### Scenario: The exit status
- **WHEN** a wasm program ends with `SYSTEM 3`
- **THEN** the Node process exits with status 3

### Requirement: What the target does not have is rejected
A program that uses a word the wasm target does not have SHALL be rejected at compile time, at the word's
line, with "`<word>` is not available on the wasm target", and never built with a routine that does nothing.
The words SHALL be those of the table `WASM_UNAVAILABLE` (`crates\sema\src\target.rs`), which the
reference's chapter on the target lists: shell and network words, prebuilt libraries, `$DEBUG`, `_CLIPBOARD$`,
and in a window program `_WINDOWHANDLE`, the dialogs and `SUB _GL`.

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

#### Scenario: _WINDOWHANDLE
- **WHEN** a window program with `h& = _WINDOWHANDLE` on line 2 is compiled with `--target wasm`
- **THEN** the exit status is 1 and the error at line 2 is "`_WINDOWHANDLE` is not available on the wasm target"

#### Scenario: SUB _GL
- **WHEN** a window program has `SUB _GL` on line 2
- **THEN** the error at line 2 is "`SUB _GL` is not available on the wasm target"

#### Scenario: A console program keeps them
- **WHEN** a `$CONSOLE:ONLY` program has `SUB _GL` and `h& = _WINDOWHANDLE`
- **THEN** it builds for Node as before: it never shows a window

### Requirement: Parts of libqb the target does not build yet
A program that uses a built-in whose part of libqb the wasm build leaves out SHALL be rejected at compile time,
at the built-in's line, with "`<word>` is not available on the wasm target yet: libqb's `<part>` part is not
built for it". The part is the built-in table's `dependency`: `DEPENDENCY_MINIAUDIO` (audio) in a
`$CONSOLE:ONLY` program, `DEPENDENCY_DEVICEINPUT` (device-input, `ON STRIG`) or `DEPENDENCY_SCREENIMAGE`
(screen-image). A page SHALL play sound through Web Audio.

#### Scenario: A new built-in of a part
- **WHEN** a built-in is added to the table with one of these dependencies
- **THEN** a wasm build rejects it without another change

#### Scenario: SOUND
- **WHEN** a `$CONSOLE:ONLY` program with `SOUND 440, 1` on line 2 is compiled with `--target wasm`
- **THEN** the exit status is 1 and the error at line 2 names `SOUND`, says "yet" and names the audio part

#### Scenario: PLAY in a window program
- **WHEN** a window program with `PLAY "C"` is compiled with `--target wasm`
- **THEN** it builds, and its page plays the note

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

### Requirement: A window program runs in a page
A program without `$CONSOLE:ONLY` built with `--target wasm` SHALL be built for a browser: the build SHALL write
`<name>.html`, `<name>.js` and `<name>.wasm`. Opening the page from a host that sends the two headers of threads
(`Cross-Origin-Opener-Policy`, `Cross-Origin-Embedder-Policy`) SHALL run the program in the page's canvas, with
the keyboard and the mouse, and show the screen its desktop build shows. A `$CONSOLE:ONLY` build SHALL remove a
`<name>.html` left beside its `.js` file.

#### Scenario: A drawing
- **WHEN** `qb64rust -x --target wasm game.bas` builds `SCREEN 12: LINE (10, 10)-(100, 50), 4, BF: SYSTEM`
- **THEN** `game.html`, `game.js` and `game.wasm` are in the output folder `build`, and the page, run in headless
  Chrome, ends with exit status 0 and a screen state equal to the desktop build's

#### Scenario: Keys
- **WHEN** a page's program waits in `INKEY$` and a key is typed over its canvas
- **THEN** `INKEY$` returns the key as on the desktop

#### Scenario: A console program afterwards
- **WHEN** `game.bas` is changed to `$CONSOLE:ONLY` and built again with `--target wasm`
- **THEN** `game.html` no longer exists

### Requirement: A page's files
A wasm build of a window program without `-o` SHALL write the files that `[build]` `copy` copies into
`<name>.data` beside the page, which SHALL put them in the program's folder, at their paths in the output folder,
before it starts. The page SHALL keep what the
program writes there in the browser's storage across visits; a shipped file the program changed SHALL stay
changed until the page ships another version of it.

#### Scenario: A shipped file
- **WHEN** `qb64rust.toml` has `[build]` `copy = ["data"]` and the program reads `data/hello.txt`
- **THEN** the page's program reads the file, though no other file is beside the page

#### Scenario: A file kept across visits
- **WHEN** the program appends a line to `save.txt` and the page is opened three times in one browser
- **THEN** the third run counts three lines

#### Scenario: A new version of a shipped file
- **WHEN** the program changed its shipped `data/score.txt`, and the game is built again with another
  `data/score.txt`
- **THEN** the next visit reads the new version

#### Scenario: A build with -o
- **WHEN** the same program is built with `-o out/game.js`
- **THEN** no `out/game.data` is written, as nothing of `[build]` `copy` is copied

### Requirement: The page's script
The page SHALL keep a `Module` object set before its script, adding only what it lacks. A `$CONSOLE` program's
console output SHALL go to the page's `Module.print`, and its standard input SHALL come from `Module.stdin`.

#### Scenario: A test's Module
- **WHEN** a script run before the page sets `Module.print`
- **THEN** the program's console output goes to that function

### Requirement: libqb's messages in a page
In a page, libqb's own messages (an unhandled error's "Continue?", an alert) SHALL be the page's dialogs:
`alert` for a message, `confirm` for a question, whose OK is Yes. They SHALL also go to standard error, as on the
desktop.

#### Scenario: An unhandled error
- **WHEN** a page's program runs `ERROR 5` with no handler, and the question is answered No
- **THEN** the message is on standard error and the program ends with the screen as it was

### Requirement: Full screen in a page
`_FULLSCREEN` in a page SHALL ask the browser for the full screen of the page's canvas. A request made outside a
key press or a click SHALL be granted at the next one, as the browser allows.

#### Scenario: Leaving full screen
- **WHEN** a page's program runs `_FULLSCREEN _OFF` in full screen
- **THEN** the page leaves the browser's full screen
