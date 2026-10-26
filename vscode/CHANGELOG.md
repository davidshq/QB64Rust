# Changelog

## Unreleased

- **Debugging.** F5 on a `.bas` file, or **QB64: Debug**, builds the program with `qb64rust --debug` and debugs it
  by BASIC line: breakpoints (also in `$INCLUDE` files), stepping, pause, the call stack and variables by BASIC
  name, watch and hover, set value, data breakpoints, set next statement, and a stop at a runtime error. A debugger
  of type `qb64rust` for `launch.json` (`program`, `args`, `cwd`, `stopOnEntry`, `console`, `noPrompt`). Needs a
  build by `qb64rust` and QB64pe's toolchain, which carries the debugger (`lldb-dap`).
- **Formatting by the new compiler.** Format Document goes through the language server while it runs (unsaved
  included files too, no temporary file), else through `qb64rust -y`; `qb64pe -y` only when `qb64rust.buildWith`
  is `qb64pe` or there is no `qb64rust` binary. The layout is the same as QB64pe's. New: **Format Selection**, and
  the settings `qb64rust.format.autoIndent`, `indentSize`, `indentSubs`, `autoLayout` and `keywordCase`, which
  replace QB64pe's `config.ini` (no longer read when formatting).
- **Hover, Find All References, Rename Symbol and Completion** from the language server, answered from the new
  compiler's whole front end; Go to Definition now covers variables, constants, types and members too. A rename
  keeps each use's type suffix and is checked against the renamed program before it is made. Code in a `$IF`
  branch that is not compiled is not reached.

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
