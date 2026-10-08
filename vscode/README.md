# QB64 for VS Code (qb64rust)

Edit, check, format, build and run QB64 programs in VS Code with the installed QB64pe compiler. Milestone M1 of the
QB64Rust project made the extension a thin client around the existing compiler; with the new compiler's binary
(`qb64rust`) it also runs a language server for syntax errors as you type, the outline, folding and go to
definition. The language server will take over checking and formatting later.

## Features

- **Language `qb64rust`** ("QB64") for `.bas`, `.bi` and `.bm` files, with syntax highlighting for comments
  (`'` and `REM`), strings, numbers (including `&H`, `&O`, `&B` and type suffixes), metacommands (`$CONSOLE`,
  `'$INCLUDE:`), line numbers and labels, keywords and built-in functions.
- **Comments, brackets and folding**: Toggle Line Comment uses `'`; `SUB`, `FUNCTION`, `TYPE` and
  `DECLARE LIBRARY` blocks fold (with the language server, every block does; see below).
- **CP437 by default.** QB64 programs store box-drawing and other extended characters as CP437 bytes, so files
  of this language open and save as `cp437` unless you choose another encoding.
- **Problems on save** (source `qb64pe`). Each save runs `qb64pe -z` (C++ generation only, no build, well under
  two seconds) and shows the compiler's error or warnings at the right file and line, including errors inside
  `$INCLUDE` files.
  The old compiler reports one error at a time and no column; the whole line is marked. These diagnostics are
  cleared when you edit the file.
- **Format Document** uses the compiler's own formatter (`qb64pe -y`), which lays code out the way the QB64pe IDE
  does: indentation, spacing and keyword case (`PRINT` → `Print`). Unsaved changes are formatted; the file on disk
  is not touched. If the program has an error, the document is left unchanged and the error is shown.
  Format on save is off unless you turn on `editor.formatOnSave`.
- **Build, Run, Build and Run** (Command Palette, category "QB64", or the run button in the editor title). Build
  writes the executable next to the source (`hello.bas` → `hello.exe`) and shows the compiler output in the
  "QB64" output channel. Run starts the program in a terminal named "QB64" in the source folder.
- **Tasks** of type `qb64rust`, for `tasks.json`:

  ```json
  { "type": "qb64rust", "action": "build", "file": "src/game.bas", "label": "build game" }
  ```

  `action` is `build` or `run`; without `file`, the active editor's file is used.

## Language server (qb64rust)

When the `qb64rust` binary is found (setting `qb64rust.path`, or `qb64rust` on `PATH`), the extension starts
`qb64rust lsp` and gets from it, from the new compiler's parser:

- **Syntax errors as you type** (source `qb64rust`), without saving, at the exact column, also in `$INCLUDE` files
  (an included file that is open is read from the editor, unsaved changes included; one that is not open is read
  from disk when the including file changes, so a change made to it outside the editor shows at the next edit of
  the including file). Only syntax errors: whether
  the new compiler can compile a statement yet is not shown. After a save, the old compiler's message (source
  `qb64pe`) can stand beside the server's for the same error; the next edit removes the `qb64pe` one.
- **Outline** (and breadcrumbs, Go to Symbol in Editor): `SUB` and `FUNCTION` with the labels inside them,
  `TYPE` blocks with their members, labels of the main module.
- **Folding** of every block: `IF`, `FOR`, `DO`, `WHILE`, `SELECT CASE` and each `CASE`, `$IF` regions, and runs of
  three or more comment lines, besides `SUB`, `FUNCTION` and `TYPE`.
- **Go to Definition** of a procedure (from a call with or without `CALL`, a function call, a `DECLARE`) and of a
  label or line number (from `GOTO`, `GOSUB`, `RETURN`, `RESUME`, `ON … GOTO/GOSUB`, `ON ERROR GOTO`, `RESTORE`),
  across included files. Variables and constants are not covered yet.

The language status shows `qb64rust` when the server runs and `qb64rust: not found` (with a link to the setting)
when there is no binary; the extension then works as without it. Included files are looked up as the compiler
does: the including file's folder, then `qb64rust.includeRoot` (empty: the folder of the `qb64rust` binary; a relative path
is taken from the workspace folder). The
server runs only in a trusted workspace.

## Requirements

[QB64 Phoenix Edition](https://github.com/QB64-Phoenix-Edition/QB64pe) (tested with 4.7.0 on Windows). Set
`qb64rust.compilerPath` to `qb64pe.exe` (or to its folder), or put the QB64pe folder on `PATH`. The language
status (the `{}` next to "QB64" in the status bar) shows `qb64pe` when the compiler is found, and an error with an
"Open Setting" link when it is not.

The language server is optional: build `qb64rust` from this repository (`cargo build --release`, see
`crates\README.md`) and set `qb64rust.path` to `target\release\qb64rust.exe`.

Highlighting works without a compiler. Checking, formatting, building and running call the compiler, so they are
off in [untrusted workspaces](https://code.visualstudio.com/docs/editor/workspace-trust) and in virtual
workspaces (folders opened from a remote repository without a local copy, e.g. on vscode.dev).

## Settings

| Setting | Default | Meaning |
|---|---|---|
| `qb64rust.compilerPath` | `""` | Path to `qb64pe.exe` or its folder. Empty: look up `qb64pe` on `PATH`. |
| `qb64rust.checkOnSave` | `true` | Check the program when a file is saved. |
| `qb64rust.checkTimeoutSeconds` | `30` | Stop a check or format that takes longer. |
| `qb64rust.runArguments` | `""` | Arguments for Run (the program sees them as `COMMAND$`). |
| `qb64rust.noPrompt` | `false` | Run with `QB64PE_NOPROMPT=y`: untrapped runtime errors are printed instead of shown in a dialog. |
| `qb64rust.path` | `""` | Path to the `qb64rust` binary, which runs the language server. Empty: look up `qb64rust` on `PATH`. |
| `qb64rust.includeRoot` | `""` | Where the language server looks up `$INCLUDE` files after the including file's folder. Empty: the folder of the `qb64rust` binary. |

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

- All compiler runs share QB64pe's `internal\temp` folder, so they run one at a time: a check waits while a build
  is running.
- C++ compile errors (rare; usually a QB64pe bug) are not mapped to BASIC lines; see the "QB64" output channel and
  `internal\temp\compilelog.txt`.

Requires VS Code 1.100 or later.
