# The wasm target

`--target wasm` builds a program for WebAssembly instead of the machine `qb64rust` runs on, and Node runs it. The
target is experimental: it builds console programs only, and it rejects at compile time what it does not have.

The pages of this area: [$CONSOLE](../entries/meta-console.md), [$IF](../entries/meta-if.md),
[_OS$](../entries/_os-str.md), [SHELL](../entries/shell.md), [_SHELLHIDE](../entries/_shellhide.md),
[RUN](../entries/run.md), [CHAIN](../entries/chain.md), [_OPENHOST](../entries/_openhost.md),
[_OPENCLIENT](../entries/_openclient.md), [_OPENCONNECTION](../entries/_openconnection.md),
[DECLARE LIBRARY](../entries/declare-library.md), [_OFFSET](../entries/_offset.md), [_MEM](../entries/_mem.md).

## Building and running

`qb64rust -x --target wasm game.bas` writes `game.js` and `game.wasm` into the folder `build`, beside the
program's `qb64rust.toml` or beside the source, and `node build/game.js` runs the program. `-o` names the `.js`
file and must end in `.js`; the `.wasm` file gets the same name beside it. The two may be moved together, and
renamed together. The program's current folder is the folder of its `.js` file, so a data file it opens by name
alone must be there: list it in `[build]` `copy` of `qb64rust.toml`, which copies it into `build` at each build.
<!-- src: compiler/cli "The output of a wasm build"; compiler/wasm-target "A wasm build"; crates\driver\tests\it\cli.rs -->
<!-- The output folder and `copy`: OpenSpec change build-output-folder; name spec compiler/cli "The output folder" here once it is archived. -->

A build for the wasm target needs the Emscripten SDK, release 7.0.0 and no other, installed with Emscripten's own
`emsdk` (`emsdk install 7.0.0` and `emsdk activate 7.0.0`). `--emsdk-root <dir>` or the environment variable
`QB64RUST_EMSDK_ROOT` names its folder, and `qb64rust toolchain path --target wasm` shows the one a build would
use. `qb64rust` downloads nothing for this target yet. Of the C++ toolchain it uses `make` only.
<!-- src: compiler/toolchain "The Emscripten SDK"; compiler/toolchain "The pinned Emscripten release"; read: crates\driver\src\toolchain.rs -->

The debug build, the coverage build (`--coverage`), the value-trace build (`--trace-values`), `qb64rust test` and
`qb64rust dap` are not available for this target. Checking with `-z --target wasm` reports the same errors as a
build.
<!-- src: compiler/cli "Options that a wasm build does not take"; crates\driver\tests\it\cli.rs -->

## Console programs only

A program must have `$CONSOLE:ONLY`. Any other program is an error on its first line: "the wasm target builds only
`$CONSOLE:ONLY` programs so far". A window, graphics and sound are later steps of the target.
<!-- src: compiler/wasm-target "Console programs only"; read: crates\sema\src\target.rs -->

## What the target does not have

A word the target does not have is a compile error at its line, "`<word>` is not available on the wasm target".
It is never built with a routine that does nothing. The words:
<!-- src: compiler/wasm-target "What the target does not have is rejected"; read: crates\sema\src\target.rs -->

| Word | Rejected |
|---|---|
| `SHELL` | as a statement and as a function, with or without `_HIDE` and `_DONTWAIT` |
| `_SHELLHIDE` | the function |
| `RUN` | with a file name; `RUN` alone and `RUN` of a label or a line number are kept |
| `CHAIN` | the statement |
| `_OPENHOST` | the function |
| `_OPENCLIENT` | the function |
| `_OPENCONNECTION` | the function |
| `DECLARE DYNAMIC LIBRARY` | the block |
| `DECLARE LIBRARY` | a block whose library is a prebuilt file (`.a`, `.o`, `.so`, `.lib`); a header alone is compiled with the program |
| `$DEBUG` | the metacommand, whose code talks to the IDE through libqb's network part |
| `_CLIPBOARD$` | the statement and the function: Node has no clipboard |
<!-- src: read: crates\sema\src\target.rs; crates\driver\tests\it\wasm_target.rs -->

Three parts of libqb are not built for this target yet. A built-in that needs one is the error "`<word>` is not
available on the wasm target yet: libqb's `<part>` part is not built for it". The part is the one the built-in
table names for it, so a new built-in of a part is covered too.
<!-- src: compiler/wasm-target "Parts of libqb the target does not build yet"; read: crates\sema\src\target.rs -->

| Part | Its words |
|---|---|
| audio | `SOUND`, `PLAY`, `_WAVE`, `_MEMSOUND`, `_MIDISOUNDBANK` and the `_SND` words |
| device-input | `STICK`, `STRIG`, `ON STRIG`, `_DEVICES` and the other words of game controllers and devices |
| screen-image | `_SCREENIMAGE`, `_CLIPBOARDIMAGE` |
<!-- src: read: crates\sema\src\target.rs; crates\driver\tests\it\wasm_target.rs -->

## A pointer is 4 bytes

A pointer has 4 bytes in WebAssembly and 8 on Windows and Linux. On this target `_OFFSET` and
`_UNSIGNED _OFFSET` take 4 bytes and hold 32-bit values, and `LEN` of one is 4.
<!-- src: compiler/wasm-target "A pointer is 4 bytes on this target"; verification\v73_a_type_layout -->

In a `TYPE`, an `_OFFSET` member and a variable-length `STRING` member take 4 bytes each. A `TYPE` of a LONG, an
`_OFFSET` and a `STRING * 2` is 10 bytes here and 14 on Windows. `_MEM` takes 40 bytes, where Windows and Linux
have 56.
<!-- src: compiler/wasm-target "Layouts with pointers on this target"; verification\v73_a_type_layout; verification\v73_b_mem; verification\v73_c_string_members -->

`PUT`, `GET`, `_MK$` and `_CV` write and read 4 bytes for an `_OFFSET`.
<!-- src: compiler/wasm-target "Reading and writing an _OFFSET on this target"; verification\v73_d_values; verification\v73_e_input -->

An operator with an `_OFFSET` is computed in 64 bits, as on Windows, and the result is cut to 32 bits when it is
stored in an `_OFFSET`. `HEX$` of an `_OFFSET` holding -1 is `FFFFFFFF`. An `_OFFSET` variable is passed by
reference to a parameter of another integer type only when that type is also 32 bits wide; to an `_INTEGER64`
parameter a copy is passed. An array parameter of `_OFFSET` takes a `LONG` array here, and an `_INTEGER64`
array is a compile error. A program that must run on both can test `$IF 32BIT`, or use `LEN` of an `_OFFSET`
variable in place of the number 8.
<!-- src: compiler/wasm-target "Arithmetic on an _OFFSET on this target"; compiler/wasm-target "An array for an _OFFSET array parameter on this target"; verification\v73_d_values; read: crates\driver\tests\it\wasm_target.rs -->

## The target's name in a program

On this target `$IF WASM` and `$IF 32BIT` are true, and `$IF WIN`, `WINDOWS`, `LINUX`, `MAC`, `MACOSX` and
`64BIT` false. On Windows and Linux `WASM` is not defined, so `$IF WASM` is false there. `_OS$` is
`[WASM][32BIT]`.
<!-- src: compiler/wasm-target "The target's name in a program"; read: crates\syntax\src\pp.rs:400-427; tools\libqb\edits\wasm-os-name.edit -->

## How a program behaves

A program's standard input, standard output, command line, exit status and files are those of the Node process.
Its current folder is the folder of its `.js` file, wherever `node` was started.
<!-- src: compiler/wasm-target "A wasm build"; crates\driver\tests\it\cli.rs -->

A program gives the output and exit status that QB64pe's Linux build gives: its lines end in a line feed, and
the console's comma zones are Linux's.
<!-- src: compiler/wasm-target "A wasm program behaves as on Linux"; openspec\changes\archive\2026-11-20-wasm-console\design.md -->
