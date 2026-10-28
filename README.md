# QB64Rust

A new compiler, editor support and debugger for programs written for
[QB64 Phoenix Edition](https://github.com/QB64-Phoenix-Edition/QB64pe) 4.7.0, written in Rust. You can edit,
check, build, run and debug QB64 programs in VS Code, or compile them from a command prompt. QB64pe does not need to
be installed.

The goal: a QB64 programmer installs one thing on Windows, Linux or macOS and gets an editor, a compiler and a
debugger ([ROADMAP.md](ROADMAP.md)).

## Status

- **Windows x64 only for now.** Linux, then macOS, are planned (milestones M7 and M8 in [ROADMAP.md](ROADMAP.md)).
- The compiler accepts the QB64pe 4.7.0 dialect. Its output is checked against the old compiler's on thousands of
  programs: the test corpus, QB64pe's own tests and the measured behaviour of each feature. Real programs such as
  Civil War Strategy, about 5,000 lines, compile and run the same as with QB64pe.
- Where this compiler deliberately differs from QB64pe, the difference is listed in
  [QB64PE-DIVERGENCES.md](QB64PE-DIVERGENCES.md). Differences from QuickBASIC 4.5 are in
  [DIVERGENCES-QB45.md](DIVERGENCES-QB45.md).
- Current state and next steps: [STATUS.md](STATUS.md).

## Install

Both downloads are on the [releases](https://github.com/davidshq/QB64Rust/releases) page.

- **VS Code extension:** `qb64rust-win32-x64-<version>.vsix`. It includes the compiler, a language server
  (errors as you type, outline, go to definition, rename, completion), the formatter and the debugger. See
  [vscode/README.md](vscode/README.md).
- **Compiler on its own:** `qb64rust-win-x64-<version>.zip`, for a command prompt, a build script or another
  editor. See [MANUAL.md](MANUAL.md).

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
| [ROADMAP.md](ROADMAP.md), [ROADMAP-COMPLETED.md](ROADMAP-COMPLETED.md) | The project's purpose; milestones still open and done |
| [STATUS.md](STATUS.md) | Where the project is now and what comes next |
| [DECISIONS.md](DECISIONS.md) | Every project decision, dated |
| [GLOSSARY.md](GLOSSARY.md) | Plain-language definitions of the compiler terms used here |
| [LAYOUT.md](LAYOUT.md) | Where everything is in the repository |
| [UPSTREAM.md](UPSTREAM.md) | Bugs in QB64pe found by this project, with their fixes |

## Licence

MIT ([LICENSE](LICENSE)). The folders `libqb/` and `tests/upstream/` hold copies of QB64 Phoenix Edition files under
their own licences (`libqb/COPYING.txt`, `libqb/licenses/`, `tests/upstream/SOURCE.md`).
