# QB64 for VS Code (qb64rust)

Edit, check, build and run QB64 programs in VS Code with `qb64rust`, the new QB64 compiler of the QB64Rust project,
which comes with the extension (Windows x64). It runs a language server for syntax errors as you type, the
outline, folding and go to definition; it checks a program on save and builds it. An installed QB64 Phoenix
Edition is still needed: its C++ toolchain builds the program, and its own formatter formats it.

## Features

- **Language `qb64rust`** ("QB64") for `.bas`, `.bi` and `.bm` files, with syntax highlighting for comments
  (`'` and `REM`), strings, numbers (including `&H`, `&O`, `&B` and type suffixes), metacommands (`$CONSOLE`,
  `'$INCLUDE:`), line numbers and labels, keywords and built-in functions.
- **Comments, brackets and folding**: Toggle Line Comment uses `'`; `SUB`, `FUNCTION`, `TYPE` and
  `DECLARE LIBRARY` blocks fold (with the language server, every block does; see below).
- **CP437 by default.** QB64 programs store box-drawing and other extended characters as CP437 bytes, so files
  of this language open and save as `cp437` unless you choose another encoding.
- **Problems on save**, and with the command **QB64: Check Program** (`qb64rust.check`). When the language server runs
  (see below), each save asks it to check the program with the new compiler's whole front end (source
  `qb64rust`): every error and warning in one run, each at its column, also in `$INCLUDE` files, with the old
  compiler's wording as a second line (`QB64pe: …`) where it is known. A check of a large program (Civil War
  Strategy, about 5,000 lines) takes under a tenth of a second. The result stands until your next edit, after
  which only syntax errors are shown again until the next save. The command checks the editor's text as it is,
  saved or not.
- Without the language server, each save runs `qb64pe -z` (source `qb64pe`; C++ generation only, no build, well
  under two seconds) and shows the compiler's error or warnings at the right file and line, including errors
  inside `$INCLUDE` files. The old compiler reports one error at a time and no column; the whole line is marked.
  These diagnostics are cleared when you edit the file.
- **Format Document** uses the compiler's own formatter (`qb64pe -y`), which lays code out the way the QB64pe IDE
  does: indentation, spacing and keyword case (`PRINT` → `Print`). Unsaved changes are formatted; the file on disk
  is not touched. If the program has an error, the document is left unchanged and the error is shown.
  Format on save is off unless you turn on `editor.formatOnSave`.
- **Build, Run, Build and Run** (Command Palette, category "QB64", or the run button in the editor title). Build
  writes the executable next to the source (`hello.bas` → `hello.exe`) and shows the compiler output in the
  "QB64" output channel. Run starts the program in a terminal named "QB64" in the source folder.
- **Builds with `qb64rust`** when it is found (`qb64rust.buildWith`, default `auto`). Its errors appear in Problems
  at their file, line and column (source `qb64rust`) and go when you edit the file. When a build fails only on
  forms marked "not supported yet" (parts of QB64 the new compiler does not have yet), the message offers **Build
  with QB64pe**, which builds that program once with the old compiler; the extension never switches by itself.
  Set `qb64rust.buildWith` to `qb64pe` to build with the old compiler always. **The first build after installing
  or updating the extension takes longer** (about 40 seconds for a console program): it builds QB64pe's runtime
  library once, in the extension's own storage, never in your QB64pe folder.
- **Tasks** of type `qb64rust`, for `tasks.json`:

  ```json
  { "type": "qb64rust", "action": "build", "file": "src/game.bas", "label": "build game" }
  ```

  `action` is `build` or `run`; without `file`, the active editor's file is used.

## Language server (qb64rust)

The extension starts `qb64rust lsp` from the binary it carries, or from `qb64rust.path` when that is set (and, in a
development copy without the bundled binary, from `qb64rust` on `PATH`), and gets from it, from the new compiler's
parser:

- **Syntax errors as you type** (source `qb64rust`), without saving, at the exact column, also in `$INCLUDE` files
  (an included file that is open is read from the editor, unsaved changes included; one that is not open is read
  from disk when the including file changes, so a change made to it outside the editor shows at the next edit of
  the including file). Only syntax errors as you type; type errors and the rest come with the check on save
  (above). After a build, the old compiler's message (source `qb64pe`) can stand beside the server's for the same
  error; the next edit removes the `qb64pe` one.
- **Outline** (and breadcrumbs, Go to Symbol in Editor): `SUB` and `FUNCTION` with the labels inside them,
  `TYPE` blocks with their members, labels of the main module.
- **Folding** of every block: `IF`, `FOR`, `DO`, `WHILE`, `SELECT CASE` and each `CASE`, `$IF` regions, and runs of
  three or more comment lines, besides `SUB`, `FUNCTION` and `TYPE`.
- **Go to Definition** of a procedure (from a call with or without `CALL`, a function call, a `DECLARE`) and of a
  label or line number (from `GOTO`, `GOSUB`, `RETURN`, `RESUME`, `ON … GOTO/GOSUB`, `ON ERROR GOTO`, `RESTORE`),
  across included files. Variables and constants are not covered yet.

The language status shows `qb64rust (bundled)` (or `qb64rust` for a binary from the setting or `PATH`) when the
server runs, and `qb64rust: not found` (with a link to the setting) when there is no binary; the extension then
works as it did with QB64pe alone. Included files are looked up as a build looks them up: the including file's
folder, then `qb64rust.includeRoot`; when that is empty, the QB64pe folder, as QB64pe looks in its own folder (a
relative path is taken from the workspace folder). The server runs only in a trusted workspace.

## Install

Download `qb64rust-win32-x64-<version>.vsix` from the repository's
[releases](https://github.com/davidshq/QB64Rust/releases) and install it with **Extensions: Install from VSIX…**
or `code --install-extension qb64rust-win32-x64-<version>.vsix`.

## Requirements

[QB64 Phoenix Edition](https://github.com/QB64-Phoenix-Edition/QB64pe) (tested with 4.7.0 on Windows): its C++
toolchain builds programs, also those `qb64rust` compiles, and it formats them. Set `qb64rust.compilerPath` to
`qb64pe.exe` (or to its folder), or put the QB64pe folder on `PATH`. The language status (the `{}` next to "QB64"
in the status bar) shows `qb64pe` when it is found, with the compiler that builds in its detail line, and an error
with an "Open Setting" link when it is not.

The `qb64rust` binary comes with the extension. To use another one (a build of this repository: `cargo build
--release`, see `crates\README.md`), set `qb64rust.path` to it.

Highlighting works without a compiler. Checking, formatting, building and running call the compiler, so they are
off in [untrusted workspaces](https://code.visualstudio.com/docs/editor/workspace-trust) and in virtual
workspaces (folders opened from a remote repository without a local copy, e.g. on vscode.dev).

## Settings

| Setting | Default | Meaning |
|---|---|---|
| `qb64rust.compilerPath` | `""` | Path to `qb64pe.exe` or its folder. Empty: look up `qb64pe` on `PATH`. |
| `qb64rust.buildWith` | `"auto"` | The compiler of Build, Build and Run and the build task: `auto` (`qb64rust` when it is found, else QB64pe), `qb64rust`, or `qb64pe`. |
| `qb64rust.checkOnSave` | `true` | Check the program when a file is saved: with the language server when it runs, else with `qb64pe -z`. |
| `qb64rust.checkTimeoutSeconds` | `30` | Stop a check or format that takes longer (a check by the language server: stop waiting for it). |
| `qb64rust.runArguments` | `""` | Arguments for Run (the program sees them as `COMMAND$`). |
| `qb64rust.noPrompt` | `false` | Run with `QB64PE_NOPROMPT=y`: untrapped runtime errors are printed instead of shown in a dialog. |
| `qb64rust.path` | `""` | Path to the `qb64rust` binary, which runs the language server and builds. Empty: the bundled binary, else `qb64rust` on `PATH`. |
| `qb64rust.includeRoot` | `""` | Where the language server and a build with `qb64rust` look up `$INCLUDE` files after the including file's folder. Empty: the QB64pe folder, else the folder of the `qb64rust` binary. |

The encoding default can be changed per language:

```json
"[qb64rust]": { "files.encoding": "utf8" }
```

## Other BASIC extensions

This extension does not register the names `BASIC` or `QBasic`, so it can be installed next to other BASIC
extensions. If another extension also claims `.bas`, choose which one wins with `files.associations`:

```json
"files.associations": { "*.bas": "qb64rust", "*.bi": "qb64rust", "*.bm": "qb64rust" }
```

## Known limits

- Compiler runs go one at a time (QB64pe's share its `internal\temp` folder): a check waits while a build is
  running.
- C++ compile errors (rare; usually a compiler bug) are not mapped to BASIC lines; see the "QB64" output channel
  (with QB64pe, also `internal\temp\compilelog.txt`).
- The package is for Windows x64 only.

Requires VS Code 1.100 or later.
