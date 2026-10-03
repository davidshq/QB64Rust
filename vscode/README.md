# QB64 for VS Code (qb64rust)

Edit, check, format, build and run QB64 programs in VS Code with the installed QB64pe compiler. This is milestone
M1 of the QB64Rust project: a thin client around the existing compiler. A language server built on the new Rust
compiler will replace checking and formatting later.

## Features

- **Language `qb64rust`** ("QB64") for `.bas`, `.bi` and `.bm` files, with syntax highlighting for comments
  (`'` and `REM`), strings, numbers (including `&H`, `&O`, `&B` and type suffixes), metacommands (`$CONSOLE`,
  `'$INCLUDE:`), line numbers and labels, keywords and built-in functions.
- **Comments, brackets and folding**: Toggle Line Comment uses `'`; `SUB`, `FUNCTION`, `TYPE` and
  `DECLARE LIBRARY` blocks fold.
- **CP437 by default.** QB64 programs store box-drawing and other extended characters as CP437 bytes, so files
  of this language open and save as `cp437` unless you choose another encoding.
- **Problems on save.** Each save runs `qb64pe -z` (C++ generation only, no build, well under two seconds) and
  shows the compiler's error or warnings at the right file and line, including errors inside `$INCLUDE` files.
  The old compiler reports one error at a time and no column; the whole line is marked.
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

## Requirements

[QB64 Phoenix Edition](https://github.com/QB64-Phoenix-Edition/QB64pe) (tested with 4.7.0 on Windows). Set
`qb64rust.compilerPath` to `qb64pe.exe` (or to its folder), or put the QB64pe folder on `PATH`. The language
status (the `{}` next to "QB64" in the status bar) shows `qb64pe` when the compiler is found, and an error with an
"Open Setting" link when it is not.

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
