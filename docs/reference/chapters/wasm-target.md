# The wasm target

`--target wasm` builds a program for WebAssembly instead of the machine `qb64rust` runs on, and Node runs it. The
target is experimental: it builds console programs only, and it rejects at compile time what it does not have.

The pages of this area: [$CONSOLE](../entries/meta-console.md), [$IF](../entries/meta-if.md),
[_OS$](../entries/_os-str.md), [SHELL](../entries/shell.md), [_SHELLHIDE](../entries/_shellhide.md),
[RUN](../entries/run.md), [CHAIN](../entries/chain.md), [_OPENHOST](../entries/_openhost.md),
[_OPENCLIENT](../entries/_openclient.md), [_OPENCONNECTION](../entries/_openconnection.md),
[DECLARE LIBRARY](../entries/declare-library.md), [_OFFSET](../entries/_offset.md), [_MEM](../entries/_mem.md).

## Building and running

`qb64rust -x --target wasm game.bas` writes `game.js` and `game.wasm` beside the source, and `node game.js` runs
the program. `-o` names the `.js` file and must end in `.js`; the `.wasm` file gets the same name beside it. The
two may be moved together, and renamed together.
<!-- src: compiler/cli "The output of a wasm build"; compiler/wasm-target "A wasm build"; crates\driver\tests\it\cli.rs -->

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

`_OFFSET`, `_UNSIGNED _OFFSET` and `_MEM` are not available yet: a variable, parameter, result or member of one
of these types, and a built-in that takes or gives one (such as `_MEMNEW` or `_WINDOWHANDLE`), is the error
"`<word>` is not available on the wasm target yet". A pointer has 4 bytes in WebAssembly, and the compiler still
gives `_OFFSET` 8. So is a value of one of these types without a declaration, such as `_CAST(_OFFSET, x)` or a
`DECLARE LIBRARY` function named `strlen~%&`. For the same reason a `TYPE` member that is an array of
variable-length `STRING`, fixed or `_DYNAMIC`, is not available yet. A single `STRING` member is built.
<!-- src: compiler/wasm-target "Pointer-sized types wait for their width"; read: crates\sema\src\target.rs; crates\driver\tests\it\wasm_target.rs -->

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
