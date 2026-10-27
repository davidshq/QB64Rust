# QB64 for VS Code (qb64rust)

Edit, check, build, run and debug QB64 programs in VS Code with `qb64rust`, the new QB64 compiler of the QB64Rust
project, which comes with the extension (Windows x64). It runs a language server for syntax errors as you type,
the outline, folding and go to definition; it checks a program on save, builds it, and debugs it by BASIC line.
QB64 Phoenix Edition is optional: building and debugging need a C++ toolchain, which the first build offers to
download (190 MB, from llvm-mingw's releases on GitHub; nothing is downloaded unless you agree), or QB64pe's when
QB64pe is installed.

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
- **Format Document** and **Format Selection** lay code out the way the QB64pe IDE does: indentation, spacing and
  keyword case (`PRINT` → `Print`); see [Formatting](#formatting).
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
- **Debugging** by BASIC line, with the program built by `qb64rust` (below).

## Formatting

**Format Document** lays the program out as `qb64pe -y` (and the QB64pe IDE) does: the indent of each block,
spaces around operators and after commas, keyword case, and names spelled as they are declared (also in an
`$INCLUDE` file). Unsaved changes are formatted; the file on disk is not touched. If the program has an error, the
document is left unchanged, the error is shown as a message and in Problems. Format on save is off unless you turn
on `editor.formatOnSave`.

Which formatter runs:

- **The language server**, while it runs: the editor's text, unsaved included files too, with no compiler process
  and no temporary file.
- **`qb64rust -y`**, when the server is off (an untrusted workspace, or it failed to start) and the `qb64rust`
  binary is found, unless `qb64rust.buildWith` is `qb64pe`.
- **`qb64pe -y`**, when `qb64rust.buildWith` is `qb64pe` or there is no `qb64rust` binary.

A formatter that reports an error is not followed by another. The compilers format a temporary copy of the editor's
text beside the file (so that relative includes resolve) and remove it at once.

**Format Selection** changes only the lines the selection touches, each laid out as in the whole formatted
document, so a line inside a `FOR` gets the loop's indent.

The layout comes from the settings `qb64rust.format.*` (below), given to whichever formatter runs; the defaults are
QB64pe's own. QB64pe's `config.ini` (the IDE's layout options) is no longer read: set the same options here. In a
workspace of several folders the language server uses the window's settings, not a folder's own.

If the language server stops by itself, formatting goes on with `qb64rust -y` (or `qb64pe -y`) until it is
started again.

## Debugging

Press **F5** on a `.bas` file, or run **QB64: Debug** (`qb64rust.debug`; also in the editor title's run menu). The
program is saved and built with `qb64rust --debug`, as Build would build it (same output channel, errors in
Problems), and then runs under the debugger; a build that fails starts no session. Click in the gutter of a `.bas`,
`.bi` or `.bm` line to set a breakpoint.

What the debugger shows (the adapter is `qb64rust dap`, in front of the `lldb-dap` of the C++ toolchain):

- **Breakpoints** by line, also in `$INCLUDE` files, with conditions, hit counts and log points; **step over, into
  and out** by BASIC line; **pause** a running program, also a one-line loop; **set next statement** within the
  current `SUB`, `FUNCTION` or the main module.
- **Call stack** by BASIC name (`SUB show`, `FUNCTION twice&`, "Main module").
- **Variables**: "Locals" (the procedure's parameters, locals, statics and result) and "Main module", by BASIC name
  and type; strings, arrays (by index within their current bounds) and `TYPE` variables expand. Hover a name, add
  it to Watch, or change its value. In Watch and hover, `,h`, `,b` and `,o` after a name show it in hex, binary or
  octal (`total,h`).
- **Data breakpoints** (Break on Value Change) on a variable, element or member of up to 8 bytes.
- **"BASIC runtime errors"** in the Breakpoints view stops at the line that raises an error, before an `ON ERROR`
  handler runs, with the error's number and text.

A `launch.json` configuration (**Run: Add Configuration… → QB64**):

```json
{ "type": "qb64rust", "request": "launch", "name": "Debug QB64 program", "program": "${file}" }
```

| Attribute | Default | Meaning |
|---|---|---|
| `program` | the active file | The main `.bas` file (built with `--debug` first), or the executable of an earlier debug build (launched as it is). |
| `args` | `qb64rust.runArguments`, split into words | The program's arguments (`COMMAND$`). |
| `cwd` | the program's folder | The working folder. |
| `stopOnEntry` | `false` | Stop at the first BASIC line. |
| `console` | `integratedTerminal` | Where a `$CONSOLE` program reads and writes: `integratedTerminal` (its `INPUT` works there), `internalConsole` (the Debug Console: output only), `externalTerminal`. |
| `noPrompt` | `qb64rust.noPrompt` | Set `QB64PE_NOPROMPT=y`. |

Debugging needs a build by `qb64rust`: with `qb64rust.buildWith` set to `qb64pe`, or without the `qb64rust` binary,
it is refused with a message naming the setting. A debug build replaces the program's executable (never
optimised; it otherwise behaves the same); the next Build makes the usual one again. The debugger comes from the
C++ toolchain (`bin\lldb-dap.exe`): the installed one, QB64pe's, or the folder of `qb64rust.toolchainPath`; without
one, the session offers it as a build does and starts only once it is installed.

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
  across included files; and of variables, constants, types and members, from the whole front end (see
  [Language features](#language-features)).

The language status shows `qb64rust (bundled)` (or `qb64rust` for a binary from the setting or `PATH`) when the
server runs, and `qb64rust: not found` (with a link to the setting) when there is no binary; the extension then
works as it did with QB64pe alone. Included files are looked up as a build looks them up: the including file's
folder, then `qb64rust.includeRoot`; when that is empty, the QB64pe folder, as QB64pe looks in its own folder (a
relative path is taken from the workspace folder). The server runs only in a trusted workspace.

## Language features

With the language server running, these come from the new compiler's whole front end, the same name table a build
uses, so a use it shows is one the compiler resolved. The front end runs once per text of the program, at the first
such request after an edit (about a twentieth of a second on a program of 5,000 lines).

- **Hover** on a name: a variable's type and where it lives (main module, shared, local to or a parameter of a
  procedure), a constant's type and value, a procedure's header, a type's members, a member's type, a label's body.
  On a built-in, its arguments and result from the built-in table. No help text is shown.
- **Find All References** of a variable, constant, procedure, type, member or label, in every file of the program;
  a FUNCTION's include the assignments to its result.
  Two variables that differ by suffix (`a$`, `a%`), or that are local to different procedures, are kept apart.
- **Rename Symbol** renames every use, in every file of the program, and keeps the type suffix written at each use
  (give the new name without one). A FUNCTION and its result variable are renamed together. Before the edit is
  made, the renamed program is checked: it must have the same names, uses and types. The rename is refused, with
  the reason, when the program has an error, when the new name is not a name, is a reserved word or a built-in's
  name, or is already a name a use can see, when a use lies in one of QB64pe's own files, and for a line number,
  one of QB64pe's constants, or a procedure of a `DECLARE LIBRARY` block. It may refuse some renames the compiler
  would take.
- **Completion** of the names seen at the cursor (a procedure's own locals and parameters, or the main module's,
  with the shared ones), procedures, constants, types, labels, built-ins and reserved words; after `p.` for a
  variable of a user type, that type's members. Right after an edit it offers the names found at the last run of the front end.

**Code in a `$IF` branch that is not compiled has no names**: references miss it and a rename leaves it unchanged
(a message says so when the program has such a branch).

## Install

Download `qb64rust-win32-x64-<version>.vsix` from the repository's
[releases](https://github.com/davidshq/QB64Rust/releases) and install it with **Extensions: Install from VSIX…**
or `code --install-extension qb64rust-win32-x64-<version>.vsix`.

Nothing else is needed to edit and check programs. **The first build or debug session asks to download the C++
toolchain** that builds and debugs them: llvm-mingw 23.1.2 (release `20260922`) from
[github.com/mstorsjo/llvm-mingw](https://github.com/mstorsjo/llvm-mingw/releases), a 190 MB download, about 460 MB
on disk in `%LOCALAPPDATA%\qb64rust\toolchain`. Only **Download** fetches it (the download is checked against its
SHA-256); **Not Now** builds nothing and asks again next time. **QB64: Install the C++ Toolchain** does the same at
any time, and `qb64rust toolchain install` from a terminal too. With QB64pe installed and found, its own toolchain
is used and nothing is asked. To use a toolchain folder of your own, set `qb64rust.toolchainPath` to it.

## Requirements

Windows x64. [QB64 Phoenix Edition](https://github.com/QB64-Phoenix-Edition/QB64pe) is optional (tested with 4.7.0):
when it is found, `qb64rust` builds with its toolchain and looks for `DECLARE LIBRARY` files in its folder, and it
is needed to build with QB64pe (`qb64rust.buildWith` set to `qb64pe`, or **Build with QB64pe**) and to format
without `qb64rust`. Set `qb64rust.compilerPath` to `qb64pe.exe` (or to its folder), or put the QB64pe folder on
`PATH`. The language status (the `{}` next to "QB64" in the status bar) shows the compiler that builds: `qb64rust`
with its toolchain in the detail line, "C++ toolchain not installed" with an install link when there is none yet;
`qb64pe` when QB64pe builds, and an error with an "Open Setting" link when it is not found.

The `qb64rust` binary comes with the extension. To use another one (a build of this repository: `cargo build
--release`, see `crates\README.md`), set `qb64rust.path` to it.

Highlighting works without a compiler. Checking, formatting, building, running and debugging call the compiler, so they are
off in [untrusted workspaces](https://code.visualstudio.com/docs/editor/workspace-trust) and in virtual
workspaces (folders opened from a remote repository without a local copy, e.g. on vscode.dev).

## Settings

| Setting | Default | Meaning |
|---|---|---|
| `qb64rust.compilerPath` | `""` | Path to `qb64pe.exe` or its folder. Empty: look up `qb64pe` on `PATH`. |
| `qb64rust.toolchainPath` | `""` | A C++ toolchain folder for `qb64rust` (llvm-mingw's layout: `bin\c++.exe`, `bin\lldb-dap.exe`). Empty: QB64pe's when it is found, else the installed one. A machine setting: a workspace cannot set it. |
| `qb64rust.buildWith` | `"auto"` | The compiler of Build, Build and Run and the build task: `auto` (`qb64rust` when it is found, else QB64pe), `qb64rust`, or `qb64pe`. |
| `qb64rust.checkOnSave` | `true` | Check the program when a file is saved: with the language server when it runs, else with `qb64pe -z`. |
| `qb64rust.checkTimeoutSeconds` | `30` | Stop a check or format that takes longer (a check by the language server: stop waiting for it). |
| `qb64rust.runArguments` | `""` | Arguments for Run (the program sees them as `COMMAND$`). |
| `qb64rust.noPrompt` | `false` | Run with `QB64PE_NOPROMPT=y`: untrapped runtime errors are printed instead of shown in a dialog. |
| `qb64rust.path` | `""` | Path to the `qb64rust` binary, which runs the language server and builds. Empty: the bundled binary, else `qb64rust` on `PATH`. |
| `qb64rust.includeRoot` | `""` | Where the language server and a build with `qb64rust` look up `$INCLUDE` files after the including file's folder. Empty: the QB64pe folder, else the folder of the `qb64rust` binary. |
| `qb64rust.format.autoIndent` | `true` | Indent each block. |
| `qb64rust.format.indentSize` | `4` | Spaces a level of indent (1 to 64). |
| `qb64rust.format.indentSubs` | `true` | Indent the body of a `SUB` or `FUNCTION`. |
| `qb64rust.format.autoLayout` | `true` | Space each statement and spell its keywords and names; off, only the indent changes. |
| `qb64rust.format.keywordCase` | `"camel"` | Keyword case: `camel` (`Print`, `End If`), `upper` (`PRINT`) or `lower` (`print`). |

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
