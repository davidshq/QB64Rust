# Proposal

## Why

This is step 2 of M3's order of work (`DECISIONS.md` 2026-10-12): what Civil War Strategy, the real-program target
of M3, needs to build, apart from sound. After `m3-window-output` the game has 135 marked errors in 15 messages
(`qb64rust --report unsupported tests\programs`, 2026-10-12):

| Uses | Not supported yet | In this change |
|---|---|---|
| 33, 31, 1 | `SOUND`, `PLAY`, `BEEP` | no: step 3 |
| 22 | `TAB` | yes |
| 19, 8 | graphics `PUT`, `GET` | yes |
| 8, 4 | `DEF SEG`, `BLOAD` (with `VARSEG` and `VARPTR` on the same lines) | yes |
| 7 | a label with the name of a SUB (`topbar`, `ships`, `cannon`) | yes |
| 1 | `_FULLSCREEN` | yes |
| 1 | the metacommand `$EXEICON` | no: with the step that builds the game |

Two screen programs of the corpus wait on the same things: `g06_message_box` (graphics `GET` and `PUT`) and
`m32_newimage_32` (`_RGB32`, `_RGBA32`). No program cut from the game loads one of its ten `*.VGA` pictures yet,
because the corpus runner gives a program no data files (`SOMEDAY.md`).

Measured before this proposal (`verification\v28_*`, 83 programs, and the old compiler's C++ read with `qb64pe -z`
outside the repository; the facts go into `study\00` §5 in task 1.1):

- **`TAB` and `SPC` are functions that give a string.** `PRINT TAB(5); "a"` prints the string `func_tab(5)` as it
  prints any string item, to the console, the window or a file; `t$ = TAB(5)` and `LEN(SPC(3))` compile too. No
  special code in `PRINT`.
- **`DEF SEG`, `BLOAD`, `BSAVE`, `POKE`, `PEEK`, `_FULLSCREEN`, `_RGB32`, `_RGBA32` are plain calls** of their rows
  and templates.
- **Graphics `GET` and `PUT` are plain calls with one argument of a new kind**: the bytes of an array from an
  element to its end (`byte_element(address, size, temporary)`), with a number for `STEP`, `_CLIP`, the action
  word and a passed colour.
- **`VARPTR` and `VARSEG` are written by the compiler and change where their variable lives.** `VARPTR(a(i))` is
  the element's index times its size, `VARSEG(a(i))` the array's address in libqb's block of conventional memory.
  A variable or array named in either is allocated in that block instead of the ordinary one: that is what makes
  `DEF SEG = VARSEG(a(1)): BLOAD f$, VARPTR(a(1))` load into the array. The game names three static arrays of the
  main module this way (`mtn`, `graphic`, `graft`).
- **A line `name:` with the name of a SUB is a call of the SUB, unless a jump names it.** `topbar:` in the game's
  main module calls `SUB topbar`. Where any `GOTO`, `GOSUB`, `RESTORE` or `ON ERROR GOTO` of the program names the
  word (`GOTO ships`, `GOTO cannon`), every such line of the program is a label instead, and `CALL ships` still
  calls the SUB. A SUB that needs an argument, and a FUNCTION, is always a label.

## What Changes

- **Labels with the name of a SUB or FUNCTION** follow the old compiler's rule above; the two marks "a label with
  the name of a SUB …" go. A label with the name of a built-in that is not compiled yet stays marked.
- **`TAB` and `SPC`**: two function rows; usable as `PRINT`, `PRINT #` and `WRITE` items and in expressions.
- **Conventional memory**: `VARPTR` and `VARSEG` of an element of a numeric array of the main module, static or
  dynamic (`DIM`, `DIM SHARED`, `COMMON`, `REDIM`; the dynamic case was added while the change was built,
  `design.md` D4), from any body; such an array is allocated in conventional memory as the old
  compiler allocates it. `DEF SEG`, `BLOAD`, `BSAVE`, `POKE` as statement rows, `PEEK` as a function row.
  `VARPTR` and `VARSEG` of anything else (a scalar, a string, a local array, a parameter, a member),
  `SADD` and `VARPTR$` stay "not supported yet" (task 1.2 lists each with the old compiler's code).
- **Graphics `GET` and `PUT`**: every form of the two templates with a numeric array, static or dynamic, of any
  body, whole or from an element. A string or user-type array, and a plain variable in the array's place, stay
  marked. File `GET` and `PUT` are not part of this change.
- **`_FULLSCREEN`** (statement and function), **`_RGB32`**, **`_RGBA32`**: rows, each form.
- **Data files beside a corpus program**: a folder `<name>.files\` is copied into the program's run folder before
  it is compiled, for either compiler; its bytes are part of what the pass cache compares.
- **More programs cut from the game** (`tools\screen\cut_game.py`), recorded with `qb64pe.exe`: the pictures it
  loads with `BLOAD` and draws with `PUT`, its `GET`/`PUT` animations, a report screen laid out with `TAB`.
  `g06_message_box` and `m32_newimage_32` join `tests\corpus\slice.list`.
- **Tests and lists**: `verification\v28_*`; a call-site program per form; front-end tests; console slice
  programs for `TAB`/`SPC`, for `BLOAD` with `PEEK` and `POKE`, and for the labels; the shrink-only lists
  regenerated.

## Capabilities

### New Capabilities

- `language/conventional-memory`: `DEF SEG`, `VARSEG`, `VARPTR`, `PEEK`, `POKE`, `BLOAD` and `BSAVE` on the
  arrays of a program: which variables live in conventional memory, the value of an offset and a segment, and what
  loading and saving a block does, as the old compiler's program behaves.

### Modified Capabilities

- `language/control-flow`: a line that is the name of a SUB and a colon (a call or a label, by the jumps of the
  program).
- `language/builtin-functions`: `TAB`, `SPC`, `PEEK`, `VARPTR`, `VARSEG`, `_RGB32`, `_RGBA32`, `_FULLSCREEN`
  join the list.
- `language/builtin-statements`: `DEF SEG`, `BLOAD`, `BSAVE`, `POKE`, graphics `GET` and `PUT`, `_FULLSCREEN`
  join the list.
- `language/screen-output`: graphics `GET` and `PUT`; `TAB` and `SPC` in text output.
- `testing/golden-corpus`: data files beside a program.

## Impact

- Changed: `crates\sema` (`check\flow.rs`: labels; `builtins.rs`: rows, the slot kind of an array's bytes, the
  choice between the graphics and the file form of `GET` and `PUT`; `check\builtins.rs`, `check\places.rs`:
  `VARPTR` and `VARSEG`, the mark on an array for conventional memory), `crates\ir` (that mark on an array; the
  two expressions), `crates\codegen-cpp` (`decl.rs`: the allocation of such an array; `builtins.rs`: the new
  argument and its temporary), `crates\syntax` only where a form of these statements does not parse today,
  `tools\legacy_tests\run_legacy_tests.py` with its tests, `tools\screen\cut_game.py`,
  `tests\corpus\slice.list`, the shrink-only lists; new `tests\callsite\*.bas`, `tests\frontend\*.bas`,
  `tests\corpus\slice\*`, `tests\corpus\screen\*` (with `*.files\` folders holding copies of the game's `*.VGA`
  files, MIT), `verification\v28_*`.
- No change to libqb or the reference clone; no change to `tools\screen\screen_state.cpp` is planned.
- `_FULLSCREEN` is compiled and compared by the call-site check, and not run by any test (it would take the
  desktop): it joins the window built-ins that are compiled and never run.
- Not in this change: `SOUND`, `PLAY`, `BEEP`, `$EXEICON`; file `GET` and `PUT`; `PALETTE USING`; `SADD`,
  `VARPTR$`, `CALL ABSOLUTE`, `INP`, `OUT`; conventional memory for anything but a numeric array of the
  main module; the other image built-ins (`_LOADIMAGE`, `_PUTIMAGE`, …); a label named like a built-in that is not
  compiled; Civil War Strategy as a whole (it still needs sound).
- `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `tests\programs\README.md`,
  `tools\legacy_tests\README.md`, `study\00` §5, `SOMEDAY.md` (what stays marked; questionable behaviours) and
  `CLAUDE.md`'s layout table updated when done.
