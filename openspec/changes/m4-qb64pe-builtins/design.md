# Design

## Context

For the motivation, the scope and what was found before this proposal, see `proposal.md`. For the behaviour, see
the delta specs. Where the parts stand in this compiler (2026-10-22):

- **Built-ins.** A function is compiled when it has a row in `sema\src\builtins.rs` `SUPPORTED`, and a statement
  when it has one in `STATEMENTS`. A `Rule::Plain` or `StmtRule::Plain` row is written as the call of its table
  entry with its template: optional slots with their `passed` flag, `{…|…}` choices as the numbers the old
  compiler passes (`_FULLSCREEN [mode][, _SMOOTH]`, `TIMER(n) ON|OFF|STOP`). An optional `$` (`mayhave`) is read
  by `check\expr.rs`. The coverage test (`crates\driver\tests\inputs.rs`) wants each supported name in a slice
  program and in a `typed` front-end test, and each name of `NEVER_RUN` in a call-site program.
- **The function `_CONSOLE`** is a row already. The statement `_CONSOLE {On|Off}` is the table's second entry of
  the name, `sub__console`.
- **Settings of the program.** The metacommands that set the whole program (`$CONSOLE`, `$SCREENHIDE`, …) are read
  by `check\mod.rs` `console_meta` into `ProgramSettings`. They reach `ir::Settings` (`ir\src\lib.rs`) through
  `lower.rs`, and `codegen-cpp` writes them (`decl.rs`). `dyninfo.txt` is one of the files `codegen-cpp\src\lib.rs`
  writes empty. Any other metacommand is "not supported yet" (`check\mod.rs`, the mark `metacommand …`).
- **Keys of the screen check.** `tools\screen\screen_state.cpp` `press` calls `keyboard_keydown(code)` and then
  `keyboard_keyup(code)`, and only once the `INKEY$` buffer is empty. The key names are a table of `key` codes
  (ASCII and `QBKC_…`). libqb's own key callback passes `VK + QBVK_LSHIFT` and the like for the modifier keys
  (`keyboard.cpp` 619), which is what `_KEYDOWN(100304)` asks about.

## Goals / Non-Goals

**Goals:**

- `qb64rust -z ..\QB64pe\source\qb64pe.bas` reports no "not supported yet". What else it reports is written down
  at the close.
- Each new built-in has what the coverage test wants, and each form a call-site program.

**Non-Goals:**

- `_SETBIT`, `_RESETBIT` and `_TOGGLEBIT`, the siblings of `_READBIT`: `qb64pe.bas` does not use them, so they stay
  "not supported yet". The unsupported scenario of `language/builtin-functions` names `_SETBIT` now.
- Building `qb64pe.bas` into an executable, and running it: tried once at the close and written down, not a goal
  (running the compiler built from it is in `SOMEDAY.md`).
- A test that resizes the window, drops a file on it, or answers a dialog.

## Decisions

### D1. Every built-in is a plain row

Each name gets a `Rule::Plain` row in `SUPPORTED` or a `StmtRule::Plain` row in `STATEMENTS` (`sema\src\
builtins.rs`), in comment blocks by group: keyboard, window and console, colours, system. No new rule is
expected. The plain path already writes optional slots, choices and `mayhave`. If `v53` shows a form the plain path
writes otherwise than the old compiler, or accepts where the old compiler gives an error, that form gets its check
in `check\builtins.rs`, as `CLEAR` did. Candidates: `_CONSOLE` as a bare statement, `_SCREENMOVE _MIDDLE` with
coordinates after it, and `_DROPPEDFILE` without `$` and with a string argument.

*Alternative:* one rule for the dialogs, which take strings in every slot. Rejected: the plain path already
converts each slot to its type, and the call-site check compares the call.

### D2. What runs and what never runs

`NEVER_RUN` gains `_SCREENMOVE` (it moves a window on the user's desktop, as `_SCREENX` and its kin are never run),
`_MESSAGEBOX`, `_OPENFILEDIALOG$`, `_SAVEFILEDIALOG$` and `_SELECTFOLDERDIALOG$` (each waits for a person). Every
other name runs in a slice or screen program:

- **Keyboard** (`language/keyboard`): screen programs fed by keys files, with `hold` and `lift` (D4).
- **`_RESIZE`, `_RESIZEWIDTH`, `_RESIZEHEIGHT`, `$RESIZE`**: a screen program that prints the three with no
  resize. If `v53` shows that the values depend on the machine (a resize event when the window opens, a size
  from the desktop), the functions go on `NEVER_RUN` instead, with that reason beside them. *Decided by task 2.1:*
  `c15_resize` printed `0 0 0` at the start, after half a second and after `_RESIZE OFF`, the same in both runs of
  its recording, so the functions run.
- **`_CONSOLE ON|OFF`**: a `$CONSOLE` slice program. Under the runner the console is a pipe, so no window is
  shown or hidden. libqb only shows or hides the console's window when the console is the program's own
  (`console.cpp`, `sub__console`, `console_active`); output still goes there while it is off (`v53_a_window`
  printed `b` after `_CONSOLE OFF`), so the slice program sees the output unchanged.
- **`_EXIT`**, **`_CONTROLCHR`**, **`_DEFAULTCOLOR`**, **`_BACKGROUNDCOLOR`**, the drop built-ins with nothing
  dropped, and the system functions: slice programs, screen programs where the result is on the screen.

### D3. `$RESIZE` is a setting carried to `dyninfo.txt`

`check\mod.rs` reads `$RESIZE:ON|OFF|STRETCH|SMOOTH` beside `console_meta`, into two fields of `ProgramSettings`:
`resize` (bool) and `resize_scale` (0, 1, 2), as `qb64pe.bas` 3626–3645 sets `ResizeOn` and `ResizeScale`. The last
one in the file counts. They go to `ir::Settings` (`lower.rs`), and `codegen-cpp` writes `ScreenResize=1;` and
`ScreenResizeScale=n;` into `dyninfo.txt` when they are set, as 13101–13106 does (`lib.rs`, where the file is
written empty today). The other forms are measured in `v53_x70`–`x81`: the word in any case, in a
`$CONSOLE:ONLY` program and in a SUB is accepted; `$RESIZE` alone, another word, a blank before or after the colon,
`:ON:ON`, and anything after the word (a comment too) are "Syntax error". None is "not supported yet".

*Alternative:* the two lines written in `decl.rs` beside the other settings. Rejected: libqb reads `dyninfo.txt`
for them, and `qb64pe.bas` writes them there.

### D4. `hold` and `lift` in the keys file

`screen_state.cpp` gains two steps, `Step::Hold` and `Step::Lift`, parsed beside `key`. `hold <name>` calls
`keyboard_keydown(code)` alone and `lift <name>` calls `keyboard_keyup(code)`. The name table gains `lshift`,
`rshift`, `lctrl`, `rctrl`, `lalt` and `ralt` with the codes libqb's callback passes (`VK + QBVK_LSHIFT`, …,
`keyboard.cpp` 619). A `hold` of a key that writes to the `INKEY$` buffer waits for that buffer to be empty first,
as `press` does. A modifier writes nothing there and does not wait. A key still held when the program exits is
left down (libqb ends with the process). The parser's errors for a bad name are those of `key` (exit 125).
`tools\screen\README.md` gets the two rows of the keys-file table.

*Alternative:* `key <name> down <ms>`, a press held for a time. Rejected: a program that waits for a key to be held
would race the time, and `hold … prompt … lift` orders it with the program's own announcements.

### D5. No user's path in a recorded output

`_DIR$("desktop")`, `"documents"`, `"fonts"` and their kin are the user's or the machine's folders. The programs of
`v53` and the slice program print only what holds on any Windows machine: the last character, whether the folder
exists (`_DIREXISTS`), and whether two names give the same folder. They never print the path. `CLAUDE.md` rule 2
covers recorded outputs, and `tools\repo_check` checks them before staging. The same goes for `_COMPILEDATE$` and
`_COMPILETIME$`, which change at each build: their length and separators are printed, never their value.

### D6. The measurement family `v53`

The `verification\` suite runs console programs only. What needs a window and keys is a screen program of
`tests\corpus\screen\`, recorded by the corpus runner with `qb64pe.exe`. Both sets are recorded before the code
(task 2.1), after D4 (the keys of the screen programs need `hold` and `lift`):

- **`verification\v53_*`** (`--suite verification`, one line in `verification\README.md`): `v53_a_window`
  (`$RESIZE` forms, `_CONSOLE OFF`/`ON` in a `$CONSOLE` program, `_EXIT`, the drop built-ins with nothing
  dropped), `v53_b_colours` (`_DEFAULTCOLOR` and `_BACKGROUNDCOLOR` of `_NEWIMAGE` images of each depth and of
  pages, error 258), `v53_c_system` (`_DIR$` (D5), `_SHELLHIDE` exit codes and an empty command (error 5),
  `_COMMANDCOUNT`, the shapes of `_COMPILEDATE$`/`_COMPILETIME$`, `_READBIT` on each type and bit 63), `v53_d_keys`
  (`_KEYDOWN(0)` and `_KEYCLEAR 4` errors, `_KEYHIT` with no key), and `v53_x…` (the wrong forms of each name:
  arity, a string where a number goes, `_CONSOLE` bare, `_RESIZE ON, ON`, `_KEYCLEAR "a"`, `_DROPPEDFILE` with a
  string, `_MESSAGEBOX` with six arguments, each statement used as a function and the other way round, and the
  `$RESIZE` forms of D3).
- **`tests\corpus\screen\`**, in the `k` series of `k01_on_key` (keys) and the `c` series of the text-screen
  programs, each with the next free number: `k…_keyhit` (`_KEYHIT` codes of letters, special keys and a modifier),
  `k…_keydown` (`hold`/`lift`), `k…_keyclear` (each argument), `c…_resize` (`$RESIZE:ON` and the three functions,
  D2's fallback), `c…_controlchr` (the characters 0 to 31 with `_CONTROLCHR` off and on), `c…_default_colours`
  (after `COLOR` in `SCREEN 0`, `12` and a 32-bit screen).

Where a recording differs from a delta spec, the spec is changed in task 2.1.

### D7. `qb64pe.bas` after the marks

At the close, `qb64rust -z ..\QB64pe\source\qb64pe.bas` runs once more. If it is clean, the exit criterion's
compile half is met and `ROADMAP.md` says so at the merge. If other errors appear (they could be hidden behind the
100-error limit or behind the marks), each is written in the change's `tasks.md` and `STATUS.md`'s "Next". A
small slip in what this change builds is fixed here. Anything else becomes the next change.

## Risks / Trade-offs

- [A screen program that reads `_KEYHIT` but not `INKEY$` leaves its `INKEY$` buffer full, and `press` waits on
  that buffer forever] → the keyboard screen programs empty it (`_KEYCLEAR 1` or `INKEY$`) after each key, and the
  prompt's timeout (exit 124) names the place if one does not.
- [`_RESIZE`'s values depend on the window manager] → D2's fallback to `NEVER_RUN`, decided from two recordings of
  the resize screen program.
- [`_CONSOLE OFF` behaves otherwise when the console is the program's own window than when it is a pipe] → the
  test sees only the pipe, where output goes on. The other case is libqb's `ShowWindow`, which the call-site check
  covers by its call.
- [`_SHELLHIDE` starts `cmd`, as `SHELL` does in `g12_game.files`, which hangs late in long runs ("Still to
  watch")] → its programs run a command that ends at once (`cmd /c exit 3`). If they hang the same way, that is
  the same environment cause, and they are marked like `g12_game.files`.
- [`screen_state.cpp` changes] → it is linked into every screen program, so the coverage check of Civil War
  Strategy's scripts and the screen programs of the slice list run at the close (`close.py` decides).
