# Tasks

Order: group 1 first (the measurements), then 2 to 5 (design D8; they do not depend on one another), then 6.
Each of groups 2 to 5 ends with `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test` (tier 1) green and with its tests, lists and `crates\README.md` lines in place, so the change can
stop after any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools or
a script file; `python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done.
**Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob`, `--category`, or a list with `--pass-cache target\pass-cache`), each run's output sent to a log under
`target\`; tier 2, the run without folding and the full suites run once, in group 6. CI is not a step of this
change. A new measured fact goes into a `v31` paragraph of `study\00` §5 in the task that measures it. A
behaviour of the old compiler that looks questionable is implemented its way and listed in `SOMEDAY.md` ("QB64pe
behaviours to review"). A form that turns out not to parse today is fixed in `crates\syntax` in the task that
finds it, with a parser test.

## 1. Measurements before code

- [x] 1.1 Measure every form of every name of the proposal (design D1) with `qb64pe -z`, outside the repository:
  each argument left out in turn, each word of each template, one variable of each numeric type and a literal in
  one slot of each slot type, the result of each function in `PRINT`, `HEX$` and an operation, and `_UCHARPOS`
  with a LONG array, another array and a scalar. Keep the rejected forms and the forms the specs' scenarios name
  as programs `verification\v31_*` (`v31_x*` one form each), recorded with `--suite verification`. Verify: design
  D1 has one line per form in place of "the shape expected"; the `v31` paragraph of `study\00` §5 has the facts;
  each scenario of the three new specs names forms and values the old compiler does give, corrected if not; the
  reference clone's `git status --porcelain` is empty before and after.
  Found: 225 accepted forms (`v31_x01_accepted`), 108 rejected ones (`x02`–`x109`), four programs of values
  (`v31_a` colours, `v31_b` handles, sizes, clear colour, palette, blending, `PMAP`, `_AUTODISPLAY`, `v31_c`
  six formats saved and loaded again, `v31_d` fonts). Every name is the plain call of its row and template; none
  needs more than a row and a slot kind. `_UCHARPOS` also takes an `_UNSIGNED LONG` array and an element (the
  check is "a 32-bit integer array"), so the `language/fonts` requirement was corrected. Two behaviours of
  libqb that look questionable: `_WIDTH` of a number that was never a handle is the screen's width, no error;
  `_SAVEIMAGE` with such a number writes a file. The clone's status was empty before and after.
- [x] 1.2 Measure the make line (design D2): the old compiler's `internal\temp\compilelog.txt` for a program with
  `_LOADIMAGE`, one with `_LOADFONT`, one with `_SCREENIMAGE` and one with all three and `$CONSOLE:ONLY`, and what
  `_LOADFONT("cour.ttf", 16)`, `_MOUSEX` after a mouse step, and `_MOUSEWHEEL` after a scroll event of each sign
  give (a first version of the mouse steps, task 4.1, may be written for this). Verify: D2 and D4 say what was
  found; `study\00` §5 has it.
  Found: six builds (the four, a plain program and `$CONSOLE:ONLY`): the flags and their order are as D2 has
  them; `DEP_SCREENIMAGE=y` alone makes the `Makefile` build the image codec in too, and `DEP_FONT=y` only swaps
  the font stub for FreeType, so `make_flags` needs nothing beyond the three flags. `_LOADFONT("cour.ttf", 16)`
  gives a handle above 31 (`v31_d`). The mouse steps were written in `screen_state.cpp` for this (task 4.1 still
  documents and tests them): a move is two events in libqb's queue, a wheel turn two, a button one, so the
  screen-state spec's click is four events, corrected there; `wheel 1` gives `_MOUSEWHEEL` 1. The user's real
  cursor over the window reaches the queue, so no test prints the movement of a first move (D4).
- [x] 1.3 Record `v31*` in `CLAUDE.md`'s layout table, and tell the user of any form that needs more than a row
  and a slot kind (D1) or that stays "not supported yet", with the reason. Verify: the row names the programs;
  `check_repo.py --untracked` is clean.
  Found: no form needs more than a row and a slot kind, and none is planned to stay "not supported yet"; told
  the user so with the close of group 1 (2026-10-14).

## 2. Images and colours

- [x] 2.1 The three runtime parts (design D2): `RuntimePart::ImageCodec`, `Font`, `ScreenImage`, through `sema`,
  the IR and its dump, and `make_flags` in the old compiler's order. Verify: a unit test of `make_flags` holds
  the four builds of task 1.2; an `ir` test shows the parts of a program with `_LOADIMAGE`.
  Found: as designed. The parts follow the three older ones in the enum, so the dumps that name parts did not
  change; `make_flags_of_images_and_fonts` holds the four builds and the parts beside the printer and the icon;
  `images_ir` shows "runtime parts: image codec".
- [x] 2.2 The colour functions as rows (`_RGB`, `_RGBA`, `_RED`, `_GREEN`, `_BLUE`, `_ALPHA`, `_RED32`,
  `_GREEN32`, `_BLUE32`, `_ALPHA32`, `_HSB32`, `_HSBA32`, `_HUE32`, `_SATURATION32`, `_BRIGHTNESS32`), as task 1.1
  found them. Verify: a `typed` test pins each result type and conversion; a `check-fail` test holds their
  rejected forms of `v31_x*`; call-site programs for each form and argument type pass (`--glob`).
  Found: each a `Rule::Plain` row, nothing more (`images_typed`, `images_errors`). Of the call-site programs, the
  first run differed for forms the check documents it does not compare (`tools\callsite\README.md`, "What it does
  not compare"): an integer in a DOUBLE slot (`_HSB32(120, 50, 50)`: the IR writes `(double)`, the old compiler
  leaves it to C++), a literal the slot's conversion changes (`_RED32(&HFF102030)`), arithmetic on operands of
  another width or type (`_RGB(…) + 1`, `_HUE32(i) + d`). The programs were written as that README says (DOUBLE
  variables and float literals, literals that fit, no mixed arithmetic); the mixed forms run in `i07`.
- [x] 2.3 The image functions and statements as rows (`_LOADIMAGE`, `_COPYIMAGE`, `_SAVEIMAGE`, `_PUTIMAGE`,
  `_MAPTRIANGLE`, `_WIDTH`, `_HEIGHT`, `_PIXELSIZE`, `_CLEARCOLOR`, `_SETALPHA`, `_BLEND`, `_DONTBLEND`,
  `_PALETTECOLOR`, `_COPYPALETTE`, `_AUTODISPLAY`, `PMAP`), `_NEWIMAGE` and `_FREEIMAGE` off `NEVER_RUN`. Verify:
  `typed` and `ir` tests pin the functions and the statements' slots and masks (every form of `_PUTIMAGE`'s
  template); a `check-fail` test holds the rejected forms; call-site programs for each form pass.
  Found: each a plain row, but **one form needed more than a row**: `_LOADIMAGE(f$, , r$)`, a function's slot
  left empty, was "not supported yet: omitted arguments" for every function. `check\builtins.rs` now fills a
  function's slots by position when the list has a gap (`filled_slots`), where the template has the slot alone
  in brackets after a comma (`,[?]`) and an argument follows; any other gap keeps its mark (`INSTR(, a$, b$)`,
  template `[?],?,?`, was not measured: `member_access` still holds it, and since the review of group 2 a gap at the
  end, at the start and in a required slot, and a gap with too many arguments, the arity error). The same rule serves `_LOADFONT`,
  `_UPRINTWIDTH` and `_UCHARPOS` of group 3. The IR shows slots, not masks; the masks are the emitter's from the
  slots and are compared by the call-site check: 169 programs for groups 2.2 and 2.3, all equal.
  `images_errors` holds the 49 rejected forms of these names, none "not supported yet".
- [x] 2.4 `tools\screen\make_images.py` and the image files it writes (design D5), and screen programs recorded
  with `qb64pe.exe`: images made, copied, loaded from the data files, saved and loaded again, put scaled and
  mirrored, a triangle mapped, a clear colour, alpha and blending in 32 bits, a palette copied and set, the
  colour functions in a palette mode and in 32 bits, `PMAP`, `_AUTODISPLAY` after `_DISPLAY`. Verify: each
  passes with the new compiler and is on `slice.list`; the coverage test of tier 1 passes; `crates\README.md`
  names the built-ins.
  Found: eight programs `i01`–`i08` (`tests\corpus\screen\SOURCE.md`), each state looked at as a picture
  before it was trusted; all eight pass with the new compiler on their first run and are on `slice.list`; the
  slice list with `--pass-cache` passes 394 of 394. Two of libqb's behaviours were found while they were
  written (`study\00` §5): `_COPYIMAGE` of a 256-colour image to 32 bits is error 5 (as `v31_b` had), and
  `PRINT` into a 5 by 5 image then `_FREEIMAGE` of it is a heap corruption (exit code 0xC0000374). **Beyond the
  task as written**: three upstream programs now compile cleanly and pass, so tier 1 asked for them to be
  listed in this group, not in group 6: `colors/test`, `console_only/image` and `image/svg_test` (it runs from
  the clone with its `.svg` files, so D7's fallback was not needed) joined `tests\upstream\pass.list` (151);
  `image/test` went on `tests\known_clean_not_passing.list` (it is built with `-f:GenerateLicenseFile`, as
  `audio_out/test` is).

## 3. Fonts

- [x] 3.1 The font built-ins as rows (`_LOADFONT`, `_FONT`, `_FREEFONT`, `_FONTWIDTH`, `_FONTHEIGHT`,
  `_PRINTSTRING`, `_PRINTWIDTH`, `_PRINTMODE`, `_UFONTHEIGHT`, `_UPRINTWIDTH`, `_ULINESPACING`, `_UPRINTSTRING`,
  `_MAPUNICODE`), and `_UCHARPOS` with its LONG array (D1). Verify: `typed` and `ir` tests; a `check-fail` test
  holds the rejected forms and "Expected LONG array-name"; call-site programs for each form pass.
  Found: every name a plain row. **`_UCHARPOS`'s array needed one node more**: a function's argument was a value
  only, so the array's bytes, which statements pass as `StmtArg::Block`, are a new `ExprKind::Block { array,
  index }` (sema, the IR's facts, dump and validator with a unit test, the emitter through the existing
  `byte_element` code of graphics `GET`); `check\builtins.rs` `long_array_arg` gives "needs an array of LONG"
  for the name alone, a scalar, a literal and an array of another type. **One accepted form stays "not supported
  yet"**, against what task 1.3 told the user: `_UCHARPOS(t$, nd&())`, an array not declared, marked as the same
  form of graphics `GET` is (`SOMEDAY.md`, "Left marked by `m3-images-fonts-mouse`"). `fonts_typed`, `fonts_ir`,
  `fonts_errors` (35 rejected forms, none "not supported yet"), and `fonts_cpp` for the element forms. 84
  call-site programs were written; three with an element (`p(2)`, `p(i)`, `z(1, 2)`) differed only in the
  index's spelling (`1ll`, `(int64)`), which `tools\callsite\README.md` lists as not compared, and were dropped
  for `fonts_cpp`; one passed a SINGLE to a DOUBLE slot (also listed) and was rewritten; the 81 left are equal.
- [x] 3.2 Screen programs recorded with `qb64pe.exe` (design D6): one with the built-in fonts 8, 14 and 16, the
  print modes and `_MAPUNICODE`; one with `cour.ttf` and `arial.ttf` loaded, freed, measured with the `_U…`
  functions and `_UCHARPOS`, and a font that is not there. Verify: both pass with the new compiler and are on
  `slice.list`; the coverage test passes; `crates\README.md` names the built-ins.
  Found: `f01_builtin_fonts` and `f02_loaded_fonts` in `tests\corpus\screen\` (`SOURCE.md`), in 256 colours so
  text is not anti-aliased; each state looked at as a picture; both pass with the new compiler on their first
  run and are on `slice.list` (396); tier 1 with the coverage test passes. One behaviour of libqb found and
  measured by a program built outside the repository: `_PRINTSTRING STEP` without an image counts from another
  image's last point (`f01`'s "step" stands over "font 14"; `study\00` §5, `SOMEDAY.md`).

## 4. The mouse

- [x] 4.1 Mouse steps in `tools\screen\screen_state.cpp` (design D4), documented in `tools\screen\README.md`.
  Verify: a program of the old compiler and one of the new read a click and a wheel step as the spec's scenario
  says; a keys file with `press 9` ends the program with exit code 125 and names the line (a command-line test in
  `cli.rs`, `#[ignore]` with the other tests that build); a keys file without mouse steps gives the same states as
  before (the 54 screen programs pass).
  Found: the steps of task 1.2 stood; **one thing was added while task 4.3 recorded**: `_MOUSEX` and `_MOUSEY`
  map an event when they are read, by the frame libqb last drew, and a program reading right after `SCREEN 12`
  got -1 for every event (`study\00` §5, `SOMEDAY.md`). Each mouse step now waits until libqb has drawn a frame of
  the displayed page at its own size (`window_shows_screen`), with the time limit of `prompt` and exit code 124
  ("the window never showed the screen"). `u01_mouse_queue` prints the spec's click (four events at 40, 30) and
  the wheel with both compilers. `cli.rs`: `screen_state_mistake_in_a_mouse_step` (`press 9`, `mouse 10`,
  `release 0`, `wheel 0`: exit code 125, stderr names the file and the line; the keys file is read before libqb
  points stderr elsewhere, so the test reads it) and `screen_state_mouse_step_of_a_hidden_window` (`$SCREENHIDE`:
  124 and the line). The whole `screen` category, 66 programs, passes with the new compiler and the changed file
  (the 54 of before, `i01`–`i08`, `f01`, `f02`, `u01`, `u02`). The README documents the steps, the wait, the
  real cursor and the limits.
- [x] 4.2 The mouse functions and `_MOUSEMOVE` as rows. Verify: `typed` and `ir` tests; a `check-fail` test for
  the rejected forms; call-site programs pass.
  Found: each a plain row. `_MOUSEMOVE` went on `NEVER_RUN` here, not in task 5.1: the coverage test wants a
  statement row on a slice program and a never-run name with call-site programs, so it stands below `u01`'s
  `SYSTEM`. `mouse_typed`, `mouse_ir`, `mouse_errors` (10 rejected forms, none "not supported yet"). 25
  call-site programs were written; `mouseinput_while` differed in the shape of the `WHILE` loop (labels against
  `while`), which is this compiler's way and not the call, and was dropped; the 24 left are equal.
- [x] 4.3 Screen programs with keys files of mouse steps, recorded with `qb64pe.exe`: the queue read event by
  event, the buttons, the wheel, the movement, a position in `SCREEN 0` and in `SCREEN 12`, `_MOUSEHIDDEN` and
  `_MOUSEDISABLED`. Verify: each passes with the new compiler and is on `slice.list`; the coverage test passes;
  `crates\README.md` names the built-ins.
  Found: `u01_mouse_queue` (`SCREEN 12`: 20 events, the three buttons, the wheel each way, a move with a button
  held) and `u02_mouse_text` (`SCREEN 0`: columns and rows, the corners, beyond the edges) in
  `tests\corpus\screen\` (`SOURCE.md`), each with a keys file; the program empties the queue, announces, and
  reads a key that the keys file gives after its mouse steps. **The first recording was wrong** (-1 for every
  position, and `u02` stopped at a `LOCATE` of -1) and was redone after the fix of task 4.1; each state was then
  looked at. Both pass with the new compiler on their first run and are on `slice.list` (398); tier 1 with the
  coverage test passes. `_MOUSEHIDDEN` and `_MOUSEDISABLED` are 0 in both (nothing hides or disables the cursor:
  `_MOUSEHIDE` stays on `NEVER_RUN`).

## 5. What no test runs

- [ ] 5.1 `_SCREENIMAGE`, `_CLIPBOARDIMAGE` (both forms), `_PRINTIMAGE`, `_DEPTHBUFFER` and `_MOUSEMOVE` on
  `NEVER_RUN`, each held in a slice program as code that is not reached, and the 3-D forms of `_MAPTRIANGLE` in
  one (design D3). Verify: call-site programs for each form pass; taking the `_SCREENIMAGE` programs out of
  `tests\callsite` makes `cargo test` fail and name it; an `ir` test shows the `ScreenImage` and `Printer` parts.

## 6. Upstream, lists, closing runs and documents

- [ ] 6.1 Upstream (design D7) and the lists: run the upstream programs that name the family; those that pass
  join `tests\upstream\pass.list`, one that fails for a missing data file only goes on
  `tests\known_clean_not_passing.list` with the reason; regenerate the shrink-only lists; run the demand report.
  Verify: tier 1 passes; `qb64rust --report unsupported` on the corpus holds nothing back and on upstream names
  no built-in of this change.
- [ ] 6.2 The closing runs, each once, with `--jobs 8 --build-cache` and without `--pass-cache`: tier 1 and the
  tests that build (`--ignored`), the runner's own tests, the call-site check, the slice list with and without
  folding (`QB64RUST_NO_FOLD=1`), the upstream and differential pass lists, the two full suites. Verify: every
  run passes, or a failure is fixed and only what the fix can affect is rerun, with a note of which runs were not
  repeated and why that is safe; `STATUS.md` has the numbers.
- [ ] 6.3 The documents: `STATUS.md` (step 6 done, `_MEM` next), `ROADMAP.md`, `crates\README.md`,
  `tests\corpus\README.md`, `tools\screen\README.md`, `study\00` §5, `SOMEDAY.md`, `DIVERGENCES-QB45.md` where
  needed, `CLAUDE.md`'s layout table, `design.md` "As built", and a `DECISIONS.md` row for any choice made while
  building. Verify: `check_repo.py --untracked` is clean; `openspec validate m3-images-fonts-mouse --strict`
  passes.
- [ ] 6.4 The record of test runs: `python tools\testtime\test_time.py`. Verify: its three files are named among
  those to stage, with a commit message for the change.

## Workflow follow-up

- Archive the change (`/opsx:archive`) after the user has staged and committed it; `language/images`,
  `language/fonts` and `language/mouse` become main specs.
