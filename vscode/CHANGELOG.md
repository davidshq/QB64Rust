# Changelog

## Unreleased

- **Fixes of QB64pe's behaviours, behind a switch.** A `qb64rust.toml` beside a program (or in a folder above it)
  turns named fixes on: `[fixes]` with `all = true`, and `<name> = false` to leave one off. Seven fixes so far:
  `RESUME NEXT` after an error in a block's condition goes on after the block, a `DIM` with no element is error 5,
  `STATIC` beside a shared dynamic array is the SUB's own, and four that make QB64pe's odd forms compile errors (a
  whole array `a()` as a value, `RESUME` or `RUN` in a SUB or FUNCTION) or service an error under `$CHECKING:OFF`
  in time. `qb64rust --list-fixes` lists them; `--fixes on|off` and `--fix <name>=on|off` set them for one build.
  A program with no `qb64rust.toml` builds as before. The language server reads the same file and checks the open
  programs again when it changes.
- **QB64: New Project** writes `main.bas` and a `qb64rust.toml` with every fix on into a folder you choose, and
  opens `main.bas`. It writes nothing into a folder that already holds either file.
- **Crashes and memory errors QB64pe has, fixed for every program**: a `DIM` with fewer than no elements, `STATIC`
  beside a shared array, a whole array used after `ERASE`, a `_MEM` block of an erased array, a `STRING * n`
  member passed to a C library, `RESUME` and `RUN` inside a SUB, and `_CAST` of a constant out of range
  (`QB64PE-DIVERGENCES.md`, D-040 to D-047).

## 0.3.0

- **QB64pe is no longer needed to build or debug.** The first build or debug session without a C++ toolchain asks
  to download one: llvm-mingw 23.1.2 from github.com/mstorsjo/llvm-mingw (190 MB, about 460 MB on disk, checked
  against its SHA-256), the toolchain QB64pe ships byte for byte. Nothing is downloaded unless you choose
  **Download**. The command **QB64: Install the C++ Toolchain** does the same on demand; the setting
  `qb64rust.toolchainPath` names a toolchain folder of your own. With QB64pe found, its toolchain is used as
  before. The language status shows the toolchain, or an install link when there is none. **Build with QB64pe** is
  offered only when QB64pe is found.
- **Debugging.** F5 on a `.bas` file, or **QB64: Debug**, builds the program with `qb64rust --debug` and debugs it
  by BASIC line: breakpoints (also in `$INCLUDE` files), stepping, pause, the call stack and variables by BASIC
  name, watch and hover, set value, data breakpoints, set next statement, and a stop at a runtime error. A debugger
  of type `qb64rust` for `launch.json` (`program`, `args`, `cwd`, `stopOnEntry`, `console`, `noPrompt`). Needs a
  build by `qb64rust` and the C++ toolchain, which carries the debugger (`lldb-dap`).
- **Formatting by the new compiler.** Format Document goes through the language server while it runs (unsaved
  included files too, no temporary file), else through `qb64rust -y`; `qb64pe -y` only when `qb64rust.buildWith`
  is `qb64pe` or there is no `qb64rust` binary. The layout is the same as QB64pe's. New: **Format Selection**, and
  the settings `qb64rust.format.autoIndent`, `indentSize`, `indentSubs`, `autoLayout` and `keywordCase`, which
  replace QB64pe's `config.ini` (no longer read when formatting).
- **Hover, Find All References, Rename Symbol and Completion** from the language server, answered from the new
  compiler's whole front end; Go to Definition now covers variables, constants, types and members too. A rename
  keeps each use's type suffix and is checked against the renamed program before it is made. Code in a `$IF`
  branch that is not compiled is not reached.
- **The compiler as a download of its own.** The release page also has `qb64rust-win-x64-<version>.zip`: the
  same `qb64rust.exe` and copy of libqb as in the extension, for a command line, a build script or another
  editor. Unpack it into a folder you can write to, run `qb64rust toolchain install`, and read its `MANUAL.md`. The
  compiler finds the `libqb` folder beside it. `qb64rust --version` prints its version, which is the extension's,
  and `qb64rust --help` prints its options.

## 0.2.0

The first release with the new compiler, `qb64rust`, inside the extension (Windows x64).

- **The compiler comes with the extension.** Live syntax errors, the outline, folding, go to definition and the
  check on save come from the bundled `qb64rust` without any setting. `qb64rust.path` still chooses another
  binary.
- **Build, Build and Run and the build task use `qb64rust`.** The new setting `qb64rust.buildWith` (`auto`,
  `qb64rust`, `qb64pe`) chooses the compiler; `auto`, the default, uses `qb64rust` when it is found. A build that
  fails only on forms marked "not supported yet" offers to build that program once with QB64pe.
- **QB64pe is still needed**: its C++ toolchain builds the program (tested with QB64pe 4.7.0). Formatting still
  uses QB64pe.
- **The first build after installing takes longer** (about 40 seconds for a console program): it builds the
  QB64pe runtime library once, in the extension's storage.
- **Changed:** with `qb64rust.includeRoot` empty, included files not found next to the including file are looked
  up in the QB64pe folder, as a build does, no longer in the folder of the `qb64rust` binary. Set
  `qb64rust.includeRoot` to keep the old folder.

## 0.1.0

Highlighting, check on save, formatting, build and run with QB64pe; the optional language server.
