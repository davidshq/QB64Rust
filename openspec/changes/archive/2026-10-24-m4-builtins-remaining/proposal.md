# Proposal

## Why

This is step 4b of the plan for the rest of M4 (user, `DECISIONS.md` 2026-10-22 and 2026-10-23): **the built-ins
that no program uses yet, and `$ERRORLOCATION` and `$USELIBRARY`**. After it, every name of the old compiler's
built-in table and every metacommand it knows compiles here, or gives the old compiler's error, so "not
supported yet" is left only for the forms the plan's later steps take (`SUB _GL`, step 5) and the 9 programs steps
2 and 3 left marked.

A probe of every entry of `tools\builtins\builtins.json` with `qb64rust -z` (2026-10-24, on `m4-finish`) marks 51
forms of 47 names "not supported yet":

- **Math:** `_ASINH`, `_ACOSH`, `_ATANH`, `_D2G`, `_G2D`, `_R2G`, `_G2R` (the `CONST` evaluator already knows the
  last four; an expression does not).
- **Bits:** `_SETBIT`, `_RESETBIT`, `_TOGGLEBIT`.
- **Microsoft Binary Format:** `MKSMBF$`, `MKDMBF$`, `CVSMBF`, `CVDMBF`.
- **Ports and the printer:** `INP`, `OUT`, `WAIT`, `LPOS`.
- **The lock keys:** `_CAPSLOCK`, `_NUMLOCK`, `_SCROLLLOCK`, each as a function and as a statement.
- **The console:** `_CINP`, `_CONSOLEINPUT`, `_CONSOLETITLE`, `_CONSOLEFONT`, `_CONSOLECURSOR`.
- **The window and its display:** `_SCALEDWIDTH`, `_SCALEDHEIGHT`, `_GLRENDER`, `_DISPLAYORDER`,
  `_ALLOWFULLSCREEN`, `_SMOOTH`, `_FPS`, `_DESKTOPREFRESHRATE`, `_DISPLAY` as a function, `_BLINK` as a function
  and as a statement.
- **The desktop and dialogs:** `_SCREENCLICK`, `_SCREENPRINT`, `_NOTIFYPOPUP`, `_COLORCHOOSERDIALOG`,
  `_INPUTBOX$`.
- **The system:** `_COMPILERVERSION$`, `_ENVIRONCOUNT`, `_UPTIME`, `FILES`, and the statements `DATE$ = …` and
  `TIME$ = …`.

These count with the metacommands `$ERRORLOCATION` and `$USELIBRARY`, and with the first argument of `INSTR`
left out (`verification\v57_o07`, which the old compiler compiles). The "about 55" of the plan was this list
before step 4a moved the old compiler's stubs (`FRE`, `TRON`, …) to its error.

## What Changes

- **A measurement family `verification\v58_*` comes first.** It records each name's value or effect, its
  argument conversions and the forms the old compiler rejects (too many or too few arguments, a string where a
  number goes, a statement form of a function). The code follows the recordings.
- **Each of the 51 forms compiles** with the libqb call the old compiler writes, held to it by a program of the
  call-site check for each form.
- **The forms that act outside the program are compiled, never run**, as `_SCREENMOVE` and the dialogs are
  (`DECISIONS.md` 2026-10-22): `_SCREENCLICK` and `_SCREENPRINT` (they click and type on the user's desktop),
  `_NOTIFYPOPUP`, `_COLORCHOOSERDIALOG` and `_INPUTBOX$` (each waits for a person or shows on the desktop), the
  statements `_CAPSLOCK`, `_NUMLOCK` and `_SCROLLLOCK` (they change the machine's keyboard state) and
  `_CONSOLEFONT` (it changes the console's font).
- **`INSTR(, a$, b$)`** compiles as the old compiler compiles it.
- **`$ERRORLOCATION:ON` and `:OFF`**: `ON` writes the old compiler's `error_track_line(…)` before every statement,
  so that a critical error names its line as the old compiler's program does; a second setting that differs is
  the old compiler's error.
- **`$USELIBRARY:'author/library'`** in full: the descriptor `libraries\descriptors\<author/library>.ini` under the
  compiler root, its three files (`IncAtTop`, `IncAfterMain`, `IncAtBottom`) included where the old compiler
  includes them, in its order, and its errors for a malformed line, a missing descriptor and a missing file
  (user, 2026-10-24).
- **The success path of `$USELIBRARY` is measured with `qb64pe.exe` in a copy of the release under `target\`**
  that has a `libraries\` folder of test libraries, since the 4.7.0 release ships none and the clone is
  read-only (user, 2026-10-24). The runner records a family with that copy as its QB64pe root.
- **The scenarios that name a construct "not supported yet" as their example** (`_SETBIT`, `FILES`,
  `$ERRORLOCATION:ON`) take one this change leaves marked, and a tier 1 test holds every entry of the built-in
  table to compiling without a mark.
- The programs of `v58` that pass join `tests\verification\pass.list`; `v57_o07` leaves `known.list`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `language/builtin-functions`: the math, bit, MBF and system functions; `LPOS`; the first argument of `INSTR`
  left out; every function of the table compiles.
- `language/keyboard`: the lock keys as functions and statements.
- `language/console-and-window`: the console statements and functions, the window and display statements and
  functions, the desktop statements and the dialogs `_COLORCHOOSERDIALOG` and `_INPUTBOX$`.
- `language/screen-output`: `_BLINK` and the function `_DISPLAY`.
- `language/conventional-memory`: the ports `INP`, `OUT` and `WAIT`.
- `language/file-io`: `FILES`.
- `language/builtin-statements`: `DATE$ = …` and `TIME$ = …`.
- `language/error-handling`: `$ERRORLOCATION` and the line a critical error names.
- `language/external-libraries`: `$USELIBRARY`.
- `compiler/pipeline`: `$ERRORLOCATION` and `$USELIBRARY` leave the metacommands not built yet; the example of a
  statement not compiled yet.
- `testing/verification-suite`: a family recorded with a copy of the release that has a `libraries\` folder.

## Impact

- `crates\sema`: rows of the built-in table (`builtins.rs`), the statements' rules (`check\builtins.rs`,
  `check\io.rs`), the metacommand checker (`check\mod.rs`), `INSTR`'s omitted argument.
- `crates\syntax`: `$USELIBRARY`'s included files at the top, after the main module and at the bottom
  (`program.rs`, the rounds of `parse_round`), and the descriptor through the loader.
- `crates\ir`, `crates\codegen-cpp`: `error_track_line` before each statement under `$ERRORLOCATION:ON`; the
  statements' calls.
- `crates\driver`: the loader reads the descriptor under the compiler root.
- `tools\legacy_tests\`: recording a family with another QB64pe root, and the copy it is made from.
- Tests: `verification\v58_*` with its recordings and its test libraries, the call-site programs, the front-end
  tests, the tier 1 test over the built-in table, `tests\verification\pass.list` and `known.list`, and
  `crates\base\src\old_wording.tsv`, regenerated.
