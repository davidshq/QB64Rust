# Proposal

## Why

M4 is done when `qb64pe.bas` compiles with no diagnostic (`ROADMAP.md`). It is item 2 of `STATUS.md`'s "Next",
taken up after the merge of pull request 12 (user, 2026-10-22). The demand report on
`..\QB64pe\source\qb64pe.bas` (2026-10-22) has no other kind of mark left: one program, "only 'not supported
yet' errors". It names 25 built-ins and the metacommand `$RESIZE`. A scan of the sources for every name of the
built-in table that `sema` has no row for (strings and comments left out) finds one more, `_COMPILETIME$`. It
stands on the line of `_COMPILEDATE$` (`ide\ide_methods.bas` 5538), and the report names only the first mark of a
line.

Found before this proposal, from the table (`tools\builtins\builtins.json`), `qb64pe.bas`, libqb and the uses in
the old compiler's sources:

- **Every name is a row of the table**, with a libqb call and no `dependency`. The dialogs are in libqb's `gui`
  part (`parts\gui`, tinyfiledialogs), which the `Makefile` links into every program. So no new runtime part is
  needed.
- **Plain functions**: `_KEYHIT`, `_KEYDOWN(code)`, `_RESIZE`, `_RESIZEWIDTH`, `_RESIZEHEIGHT`, `_DIR$(name$)`,
  `_SHELLHIDE(cmd$)` (an `_INTEGER64`), `_COMMANDCOUNT`, `_COMPILEDATE$`, `_COMPILETIME$`, `_READBIT(a, b)` (an
  inline function of `bitops.h`), `_EXIT`, `_CONTROLCHR`, `_ACCEPTFILEDROP`, `_TOTALDROPPEDFILES`. Functions with
  an optional slot (`[?]`): `_DEFAULTCOLOR[(h)]`, `_BACKGROUNDCOLOR[(h)]`, `_DROPPEDFILE[$][(i)]` (its `$` is
  optional, `mayhave`). Functions with several optional slots: `_MESSAGEBOX`, `_OPENFILEDIALOG$`,
  `_SAVEFILEDIALOG$`, `_SELECTFOLDERDIALOG$`.
- **Statements by template**: `_KEYCLEAR [n]`, `_RESIZE [{ON|OFF}][, {_STRETCH|_SMOOTH}]`, `_SCREENMOVE
  {_MIDDLE | x, y}`, `_CONTROLCHR {ON|OFF}`, `_CONSOLE {ON|OFF}` (the function `_CONSOLE` is already compiled),
  `_ACCEPTFILEDROP [{ON|OFF}]`, `_FINISHDROP`, `_MESSAGEBOX [t$][, m$][, icon$]`. The parser already reads
  `{…|…}` choices in any template (`crates\builtins\src\template.rs`).
- **`_COMPILEDATE$` and `_COMPILETIME$`** need nothing in libqb. `codegen-cpp` already writes `func__compdate`
  and `func__comptime` into `main.txt` (`lib.rs`), as `qb64pe.bas` 17596 does.
- **`$RESIZE:ON|OFF|STRETCH|SMOOTH`** (`qb64pe.bas` 3626) sets two settings of the program. They are written to
  `dyninfo.txt` as `ScreenResize=1;` and `ScreenResizeScale=n;` (13101). We already write that file, empty.
- **What a test can see.** A key of the screen check's keys file goes down and up at once
  (`tools\screen\screen_state.cpp`, `press`), so no test can see `_KEYDOWN` true for a held key. No test can resize
  the window, drop a file on it, or answer a dialog. `_DIR$("desktop")` and its kin give the user's own folders,
  which no recorded output may hold (`CLAUDE.md` rule 2).

## What Changes

- **Keyboard**: `_KEYHIT`, `_KEYDOWN` and `_KEYCLEAR` compiled and run, with the old compiler's results, in
  screen programs fed by keys files. The keys file gains `hold <key>` and `lift <key>`: a key that stays down
  until it is lifted, with the modifier keys among the names (Shift, Ctrl, Alt).
- **The window and the console**: `$RESIZE` and the `_RESIZE` statement and functions, `_RESIZEWIDTH`,
  `_RESIZEHEIGHT`, the `_CONSOLE ON|OFF` statement, `_EXIT` and `_CONTROLCHR` (statement and function), compiled
  and run where a test can see what they do. The drop functions (`_ACCEPTFILEDROP`, `_TOTALDROPPEDFILES`,
  `_DROPPEDFILE$`, `_FINISHDROP`) are run with nothing dropped. `_SCREENMOVE` and the dialogs (`_MESSAGEBOX` as
  statement and function, `_OPENFILEDIALOG$`, `_SAVEFILEDIALOG$`, `_SELECTFOLDERDIALOG$`) are compiled and never
  run, as the window built-ins before them.
- **The screen's colours**: `_DEFAULTCOLOR` and `_BACKGROUNDCOLOR`, with and without an image handle.
- **The system**: `_DIR$`, `_SHELLHIDE`, `_COMMANDCOUNT`, `_COMPILEDATE$`, `_COMPILETIME$`, `_READBIT`.
- **Tests**: a measurement family `v53` in `verification\`, slice and screen programs, front-end tests, and a
  call-site program for each form.
- **`qb64pe.bas`** then compiles with no "not supported yet" mark. Whatever else its compile then reports is
  recorded in the change's close, and not fixed in this change unless it is a small slip of what it builds.

## Capabilities

### New Capabilities

- `language/keyboard`: the keyboard functions beside `INKEY$`: `_KEYHIT`, `_KEYDOWN`, `_KEYCLEAR`.

### Modified Capabilities

- `language/builtin-functions`: "Supported built-in functions" adds the functions of this change, and its
  scenario of an unsupported built-in names another function. New requirement: "The system's folders, commands
  and build" (`_DIR$`, `_SHELLHIDE`, `_COMMANDCOUNT`, `_COMPILEDATE$`, `_COMPILETIME$`, `_READBIT`).
- `language/builtin-statements`: "Supported built-in statements" adds the statements of this change.
- `language/console-and-window`: new requirements "$RESIZE", "_RESIZE and the size it asks for", "The console shown and hidden",
  "_EXIT", "Dialogs and the window's place are compiled" and "Files dropped on the window".
- `language/screen-output`: new requirements "Control characters" (`_CONTROLCHR`) and "The default colours"
  (`_DEFAULTCOLOR`, `_BACKGROUNDCOLOR`).
- `testing/screen-state`: "Keys for a program with a window" gains `hold` and `lift`.

## Impact

- `sema`: `builtins.rs` (rows in `SUPPORTED` and `STATEMENTS`; the new `NEVER_RUN` names), `check\mod.rs` (the
  metacommand `$RESIZE` and its two settings).
- `ir` and `codegen-cpp`: the two settings carried to `dyninfo.txt`. Every built-in is a plain call.
- `tools\screen\screen_state.cpp` and its `README.md`: the steps `hold` and `lift`. The coverage check of Civil
  War Strategy's scripts runs at the close (working rule 9).
- No new dependency, no libqb change.
