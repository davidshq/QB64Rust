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

- [ ] 1.1 Measure every form of every name of the proposal (design D1) with `qb64pe -z`, outside the repository:
  each argument left out in turn, each word of each template, one variable of each numeric type and a literal in
  one slot of each slot type, the result of each function in `PRINT`, `HEX$` and an operation, and `_UCHARPOS`
  with a LONG array, another array and a scalar. Keep the rejected forms and the forms the specs' scenarios name
  as programs `verification\v31_*` (`v31_x*` one form each), recorded with `--suite verification`. Verify: design
  D1 has one line per form in place of "the shape expected"; the `v31` paragraph of `study\00` §5 has the facts;
  each scenario of the three new specs names forms and values the old compiler does give, corrected if not; the
  reference clone's `git status --porcelain` is empty before and after.
- [ ] 1.2 Measure the make line (design D2): the old compiler's `internal\temp\compilelog.txt` for a program with
  `_LOADIMAGE`, one with `_LOADFONT`, one with `_SCREENIMAGE` and one with all three and `$CONSOLE:ONLY`, and what
  `_LOADFONT("cour.ttf", 16)`, `_MOUSEX` after a mouse step, and `_MOUSEWHEEL` after a scroll event of each sign
  give (a first version of the mouse steps, task 4.1, may be written for this). Verify: D2 and D4 say what was
  found; `study\00` §5 has it.
- [ ] 1.3 Record `v31*` in `CLAUDE.md`'s layout table, and tell the user of any form that needs more than a row
  and a slot kind (D1) or that stays "not supported yet", with the reason. Verify: the row names the programs;
  `check_repo.py --untracked` is clean.

## 2. Images and colours

- [ ] 2.1 The three runtime parts (design D2): `RuntimePart::ImageCodec`, `Font`, `ScreenImage`, through `sema`,
  the IR and its dump, and `make_flags` in the old compiler's order. Verify: a unit test of `make_flags` holds
  the four builds of task 1.2; an `ir` test shows the parts of a program with `_LOADIMAGE`.
- [ ] 2.2 The colour functions as rows (`_RGB`, `_RGBA`, `_RED`, `_GREEN`, `_BLUE`, `_ALPHA`, `_RED32`,
  `_GREEN32`, `_BLUE32`, `_ALPHA32`, `_HSB32`, `_HSBA32`, `_HUE32`, `_SATURATION32`, `_BRIGHTNESS32`), as task 1.1
  found them. Verify: a `typed` test pins each result type and conversion; a `check-fail` test holds their
  rejected forms of `v31_x*`; call-site programs for each form and argument type pass (`--glob`).
- [ ] 2.3 The image functions and statements as rows (`_LOADIMAGE`, `_COPYIMAGE`, `_SAVEIMAGE`, `_PUTIMAGE`,
  `_MAPTRIANGLE`, `_WIDTH`, `_HEIGHT`, `_PIXELSIZE`, `_CLEARCOLOR`, `_SETALPHA`, `_BLEND`, `_DONTBLEND`,
  `_PALETTECOLOR`, `_COPYPALETTE`, `_AUTODISPLAY`, `PMAP`), `_NEWIMAGE` and `_FREEIMAGE` off `NEVER_RUN`. Verify:
  `typed` and `ir` tests pin the functions and the statements' slots and masks (every form of `_PUTIMAGE`'s
  template); a `check-fail` test holds the rejected forms; call-site programs for each form pass.
- [ ] 2.4 `tools\screen\make_images.py` and the image files it writes (design D5), and screen programs recorded
  with `qb64pe.exe`: images made, copied, loaded from the data files, saved and loaded again, put scaled and
  mirrored, a triangle mapped, a clear colour, alpha and blending in 32 bits, a palette copied and set, the
  colour functions in a palette mode and in 32 bits, `PMAP`, `_AUTODISPLAY` after `_DISPLAY`. Verify: each
  passes with the new compiler and is on `slice.list`; the coverage test of tier 1 passes; `crates\README.md`
  names the built-ins.

## 3. Fonts

- [ ] 3.1 The font built-ins as rows (`_LOADFONT`, `_FONT`, `_FREEFONT`, `_FONTWIDTH`, `_FONTHEIGHT`,
  `_PRINTSTRING`, `_PRINTWIDTH`, `_PRINTMODE`, `_UFONTHEIGHT`, `_UPRINTWIDTH`, `_ULINESPACING`, `_UPRINTSTRING`,
  `_MAPUNICODE`), and `_UCHARPOS` with its LONG array (D1). Verify: `typed` and `ir` tests; a `check-fail` test
  holds the rejected forms and "Expected LONG array-name"; call-site programs for each form pass.
- [ ] 3.2 Screen programs recorded with `qb64pe.exe` (design D6): one with the built-in fonts 8, 14 and 16, the
  print modes and `_MAPUNICODE`; one with `cour.ttf` and `arial.ttf` loaded, freed, measured with the `_U…`
  functions and `_UCHARPOS`, and a font that is not there. Verify: both pass with the new compiler and are on
  `slice.list`; the coverage test passes; `crates\README.md` names the built-ins.

## 4. The mouse

- [ ] 4.1 Mouse steps in `tools\screen\screen_state.cpp` (design D4), documented in `tools\screen\README.md`.
  Verify: a program of the old compiler and one of the new read a click and a wheel step as the spec's scenario
  says; a keys file with `press 9` ends the program with exit code 125 and names the line (a command-line test in
  `cli.rs`, `#[ignore]` with the other tests that build); a keys file without mouse steps gives the same states as
  before (the 54 screen programs pass).
- [ ] 4.2 The mouse functions and `_MOUSEMOVE` as rows. Verify: `typed` and `ir` tests; a `check-fail` test for
  the rejected forms; call-site programs pass.
- [ ] 4.3 Screen programs with keys files of mouse steps, recorded with `qb64pe.exe`: the queue read event by
  event, the buttons, the wheel, the movement, a position in `SCREEN 0` and in `SCREEN 12`, `_MOUSEHIDDEN` and
  `_MOUSEDISABLED`. Verify: each passes with the new compiler and is on `slice.list`; the coverage test passes;
  `crates\README.md` names the built-ins.

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
