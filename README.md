# QB64Rust

A new compiler, editor support and debugger for programs written for
[QB64 Phoenix Edition](https://github.com/QB64-Phoenix-Edition/QB64pe) 4.7.0, written in Rust. You can edit,
check, build, run and debug QB64 programs in VS Code, or compile them from a command prompt. QB64pe does not need to
be installed.

The goal: a QB64 programmer installs one thing on Windows or Linux and gets an editor, a compiler and a
debugger ([ROADMAP.md](docs/ROADMAP.md)).

## Features

- **Compiler** for the QB64pe 4.7.0 dialect, with QB64pe's command-line options
  ([MANUAL.md](docs/MANUAL.md#options)).
- **VS Code extension** that checks, builds and runs programs ([vscode/README.md](vscode/README.md)).
- **Language server**: errors as you type, outline, go to definition, rename and completion
  ([vscode/README.md](vscode/README.md#language-server-qb64rust)).
- **Formatter** that lays code out as QB64pe's IDE does, in VS Code or with `qb64rust -y`
  ([vscode/README.md](vscode/README.md#formatting)).
- **Debugger** by BASIC line: breakpoints, stepping, variables and the call stack
  ([vscode/README.md](vscode/README.md#debugging)).
- **Unit tests written in BASIC**, run by `qb64rust test` or VS Code's Test Explorer
  ([MANUAL.md](docs/MANUAL.md#testing-your-code), [vscode/README.md](vscode/README.md#tests)).
- **Coverage builds** with `qb64rust --coverage`, which count how often each statement runs
  ([MANUAL.md](docs/MANUAL.md#measuring-which-lines-run)).
- **C++ toolchain download** on the first build, or with `qb64rust toolchain install`.
- **Lists of every deliberate difference** from QB64pe ([QB64PE-DIVERGENCES.md](docs/QB64PE-DIVERGENCES.md)) and
  from QuickBASIC 4.5 ([DIVERGENCES-QB45.md](docs/DIVERGENCES-QB45.md)).

## Status

- **Windows x64 is released, as the VS Code extension. Linux x64 is built, and its first release is pending** (milestone M7 in [ROADMAP.md](docs/ROADMAP.md)). macOS is postponed (a row in [SOMEDAY.md](docs/SOMEDAY.md)).
- The compiler accepts the QB64pe 4.7.0 dialect. Its output is checked against the old compiler's on thousands of
  programs: the test corpus, QB64pe's own tests and the measured behaviour of each feature. Real programs such as
  Civil War Strategy, about 5,000 lines, compile and run the same as with QB64pe.
- Where this compiler deliberately differs from QB64pe, the difference is listed in
  [QB64PE-DIVERGENCES.md](docs/QB64PE-DIVERGENCES.md). Differences from QuickBASIC 4.5 are in
  [DIVERGENCES-QB45.md](docs/DIVERGENCES-QB45.md).
- Current state and next steps: [STATUS.md](docs/STATUS.md).

## Install

Both downloads are on the [releases](https://github.com/davidshq/QB64Rust/releases) page.

- **VS Code extension:** `qb64rust-win32-x64-<version>.vsix`. It includes the compiler, a language server
  (errors as you type, outline, go to definition, rename, completion), the formatter and the debugger. See
  [vscode/README.md](vscode/README.md).
- **Compiler on its own:** `qb64rust-win-x64-<version>.zip`, for a command prompt, a build script or another
  editor. See [MANUAL.md](docs/MANUAL.md).

Building a program needs a C++ toolchain (llvm-mingw, about 190 MB). The extension asks before downloading it on
the first build, and `qb64rust toolchain install` fetches it from a command prompt.

## How it works

`qb64rust` parses and checks the BASIC program itself, lowers it to its own intermediate form, and generates C++
that is linked against a copy of QB64pe's runtime library (`libqb/`). The compiler is one Rust crate per stage;
[crates/README.md](crates/README.md) describes them, how to build from source (`cargo build --release`) and how
the tests are run.

## Documents

| File | What it holds |
|---|---|
| [ROADMAP.md](docs/ROADMAP.md), [ROADMAP-COMPLETED.md](docs/ROADMAP-COMPLETED.md) | The project's purpose; milestones still open and done |
| [STATUS.md](docs/STATUS.md) | Where the project is now and what comes next |
| [DECISIONS.md](docs/DECISIONS.md) | Every project decision, dated |
| [GLOSSARY.md](docs/GLOSSARY.md) | Plain-language definitions of the compiler terms used here |
| [LAYOUT.md](docs/LAYOUT.md) | Where everything is in the repository |
| [UPSTREAM.md](docs/UPSTREAM.md) | Bugs in QB64pe found by this project, with their fixes |

## Licence

MIT ([LICENSE](LICENSE)). The folders `libqb/` and `tests/upstream/` hold copies of QB64 Phoenix Edition files under
their own licences (`libqb/COPYING.txt`, `libqb/licenses/`, `tests/upstream/SOURCE.md`).
