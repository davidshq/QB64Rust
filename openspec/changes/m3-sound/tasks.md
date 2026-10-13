# Tasks

Order: group 1 first (the rest of the measurements), then 2 to 5 in that order (design D8), then 6. Each of groups
2 to 5 ends with `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test`
(tier 1) green and with its tests, lists and `crates\README.md` lines in place, so the change can stop after any
group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done. **Test cadence**
(`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches (`--glob`,
`--category`, or a list with `--pass-cache target\pass-cache`); tier 2, the run without folding and the full suites
run once, in group 6. CI is not a step of this change. A new measured fact goes into the `v29` paragraph of
`study\00` §5 in the task that measures it. A behaviour of the old compiler that looks questionable is implemented
its way and listed in `SOMEDAY.md` ("QB64pe behaviours to review").

## 1. Measurements before code

- [ ] 1.1 Measure, before the design, what the proposal rests on: the C++ of each sound form and of `$EXEICON`
  (`qb64pe -z`, programs outside the repository), what a program can observe of sound, accepted and rejected
  forms one program each. Verify: `verification\` holds the programs with their `.compile.txt` and `.out.txt`.
  Found (2026-10-13): 110 programs: `v29_a_sound_queue`, `v29_b_sound_wait`, `v29_c_sound_errors`, `v29_x01`–`x86`
  (79 sound programs, 43 accepted and 36 rejected; numbers 33–39 not used), `v29_e01`–`e28` (`$EXEICON`: 14
  accepted, 13 rejected, and `e16`, which the front end accepts and whose build fails), with
  `v29_inc\` (a 16 by 16 icon made for it, a text file, an included file). Every sound form is a plain call;
  `$EXEICON` is `_ICON` with no argument where it stands, a resource file and `DEP_ICON_RC=y`. Sound held back
  by `SOUND WAIT` is not in `PLAY(0)`; in the background (`PLAY "MB"`) the queue reads equal in tenths of a
  second in two runs. The background is set per voice. A relative `$EXEICON` name without `./` is looked for in
  the old compiler's own folder. **Still to do in this task:** the `v29` paragraph of `study\00` §5, the row of
  `v29*` in `CLAUDE.md`'s layout table.
- [ ] 1.2 Measure what the design leaves open, one program each (`verification\v29_…`), the C++ outside the
  repository:
  - volume 0 (`SOUND f, d, 0` and `PLAY "V0"`): are the queue lengths of `v29_a` and the waits of `v29_b` the
    same, and does the volume of one statement hold for the next statement and for `PLAY` (design D6);
  - the queue read as the slice program will read it (`PLAY(0)` directly before and after each statement,
    lengths of whole tenths), with a dotted note of a whole number of tenths (`T60 L4 C.`), run twice alone
    and once beside seven other programs;
  - a voice of its own, directly: the time `SOUND 440, 18.2, , , , , 1` takes while voice 0 is in the
    background, and `PLAY(0)` before and after it;
  - the range of the waveform parameter (`v29_c` has one value, 2 with waveform 9, an error), and a pan
    position below -1;
  - which run-time errors of `v29_c` are raised on a machine without a sound device, read from libqb's source
    (the order of the checks and of the device's start in `audio.cpp`), since no such machine is at hand;
  - `$EXEICON` with an absolute name (a drive letter; one leading slash, to see that it is taken from the
    compiler's drive), tried outside the repository and not recorded (D4); a second `$EXEICON`
    inside an included file and one in a `$IF` branch that is not taken after a first (what counts as second);
  - the make flags of a program with `SOUND`, with `$EXEICON`, and with both, read from the old compiler's
    build line (`internal\temp\debug_win.bat` or its log), for the order of D3 and D5.
  Verify: the design's D3 to D6 say what was found in place of "task 1.2 measures"; the spec scenarios name
  forms and values the old compiler does give, corrected if not; the reference clone's `git status --porcelain`
  is empty before and after.
- [ ] 1.3 Put the design's three open questions to the user (silent tests and `BEEP` never run; a relative
  `$EXEICON` name without `./` stays marked; the game built by a test and started once by hand). Verify: the
  answers are a row of `DECISIONS.md`, and the proposal, the design and the groups below follow them.

## 2. SOUND, PLAY and BEEP compile (design D1 to D3)

- [ ] 2.1 `sema`: the rows `SOUND`, `PLAY`, `BEEP` in `STATEMENTS` and `PLAY` in `SUPPORTED`;
  `RuntimePart::Audio` for `DEPENDENCY_MINIAUDIO`, through the IR and its dump; `make_flags` writes
  `DEP_AUDIO_MINIAUDIO=y` in the old compiler's place. The list of built-ins no test runs is renamed and `BEEP`
  joins it (D6). `syntax`: `PLAY` leaves the parser's event words (D1), so `ON PLAY(n) GOSUB label` is
  `ON … GOSUB` and `PLAY ON`, `PLAY OFF`, `PLAY STOP` are errors; this task says whether any accepted program
  of the corpus, the upstream set or `tests\programs` had the old reading. Where a form of task 1.1 does not
  compile as measured, the fix goes into the template plan or the parser for every statement, and this task
  says which. Tests: unit tests of `make_flags` (sound; sound and a console; only `BEEP`) and of the build key;
  an `ir` front-end test (every accepted form of the three statements, the settings line with `audio`) and a
  `typed` one (`PLAY` bare, with each numeric type, in an operation, in `ON … GOSUB` with a label: the line
  number of `v29_x64` is not compiled yet); a `check-fail` test with the rejected forms of `v29_x*`: 34 of the
  36 each an error that is not "not supported yet", and `x78` and `x79` (`GOTO BEEP`, `GOTO SOUND`) with the
  mark they keep (D2). Verify: `cargo test` passes; no existing snapshot changes but those that held the marks
  (2.3 names them).
- [ ] 2.2 Call-site programs, one per accepted form of task 1.1 (`tests\callsite\sound_*.bas`, `play_*.bas`,
  `beep_*.bas`; from the 43 accepted programs, more where one of them holds several forms or types, `x64` with
  a label). Verify: `python tools\callsite\callsite.py`
  on these programs alone reports every one equal; a difference is fixed in 2.1's code, not in the tool.
- [ ] 2.3 The lines the rows touch elsewhere: `BEEP:` and `SOUND:` as statements, `PLAY:` an error, `GOTO BEEP`
  keeps its mark (D2), with a front-end test from `v29_x24`, `x65`, `x72`, `x78`–`x83`; the `SOMEDAY.md` entry
  that names `BEEP: x = 1` gets another example. The front-end tests that hold one of the three words today:
  `proc_errors.bas` and `stmt_calls_errors.bas` (a bare `BEEP` as the statement that is not supported: each
  names another one), `error_handling_errors.bas` (`beep: PRINT 1`, a label named like a built-in: another
  name), `stmt_name_colon_errors.bas` (`BEEP: l = 10`: another name), `template_forms.bas` (`PLAY "cde"`,
  `SOUND 440, 18`: parsed only, stays); the slice program `s24_proc_names` has a FUNCTION named `beep`, which
  must go on passing. `crates\README.md`: the three statements and the
  function, the audio part. Verify: `cargo test` passes; `qb64rust --report unsupported tests\programs` names
  only `$EXEICON`.

## 3. Sound that runs (design D6)

- [ ] 3.1 Slice programs, console, silent, recorded with `qb64pe.exe`: `s74_sound_queue` (what each statement
  adds to `PLAY(0)` in the background, in tenths: read directly before and after the statement, lengths of
  whole tenths only), `s75_sound_wait` (which statements hold the program, yes or no, one-sided limits: "over"
  for a statement that holds, "under" for one that returns), `s76_sound_errors` (error 5 and the accepted
  limits, after `SOUND WAIT`), each with `BEEP` or another
  form as code that is not reached where the coverage check wants it. Each is recorded twice and the two
  recordings are equal, and the three are run once with `--jobs 8` beside the slice programs of another
  category. Verify: the three pass with the new compiler
  (`run_legacy_tests.py --suite corpus --glob` for them) and are on `tests\corpus\slice.list`; `cargo test`'s
  coverage check passes for the four built-ins.
- [ ] 3.2 `g03_capitol` joins `tests\corpus\slice.list`. Verify: all 52 programs of `tests\corpus\screen` pass
  with the new compiler by the state of their screen (one run of that category); `tests\corpus\README.md` and
  `tests\known_clean_not_passing.list` say so.
- [ ] 3.3 The shrink-only lists regenerated; the upstream programs that now compile cleanly are run and join
  `tests\upstream\pass.list` or `tests\known_clean_not_passing.list` with the reason. Verify: `cargo test`
  passes with the lists as committed.

## 4. $EXEICON (design D4, D5)

- [ ] 4.1 `sema`: `$EXEICON` read as D4 says (the name, its three classes, the errors, the rest of the line
  dropped, the statement `_ICON` with no argument, the icon among the settings, through the IR and its dump
  without a folder). First find how the lexer cuts the forms of `v29_e09`, `e22` and `e24` and say so here.
  Tests: an `ir` front-end test (main module, inside a SUB, after `SYSTEM`, in an included file, in a branch
  that is not taken, words and a colon after the name, in a comment and after `REM` (`e14`: no statement, no
  icon), with an `_ICON` statement in the same program (`e26`)); a `check-fail` test with `v29_e05`–`e07`, `e13`, `e17`,
  `e19`, `e23`, `e27`, and a relative name without `./` and a name with one leading slash (both marked). Verify: `cargo test` passes.
- [ ] 4.2 The driver: the check that the file is there, for every command that reports diagnostics (an error at
  the line, the name as written); before `make` the copy `icon.ico` and `icon.rc` in the build's `temp`,
  `DEP_ICON_RC=y`, both files in the build key. Tests: unit tests of the flags and of the key (a changed icon
  gives another key); command-line tests of the missing file (no clone needed) and, `#[ignore]`d, of a program
  with a 16 by 16 icon whose executable holds the icon's image bytes, and of a text file as icon (the build
  fails). `DIVERGENCES.md`: the missing-file error names the file as written, not the old compiler's resolved
  folder or bare file name (D4). Verify: `cargo test` and `cargo test -p qb64rust-driver --test cli --
  --ignored` pass.
- [ ] 4.3 A slice program `s77_exeicon` (console; the line between two `PRINT`s; its icon in `s77_exeicon.files\`
  if the runner's data files reach the compiler's folder for the program, else the program and its icon are a
  command-line test only and this task says why), recorded with `qb64pe.exe`; a call-site program if
  `tools\callsite` can give a program a file beside it without a change larger than the test is worth (rule 8:
  say which cost applies). `crates\README.md`, `SOMEDAY.md` (a relative name without `./`; the language server
  does not check the file). Verify: the program passes with the new compiler; `python
  tools\repo_check\check_repo.py --untracked` is clean (no recorded path of the icon).

## 5. Civil War Strategy compiles and builds (design D7)

- [ ] 5.1 Tier 1: the set `programs/` of `driver\tests\inputs.rs` requires no diagnostic at all and is lowered,
  validated and emitted; an `#[ignore]`d test in `driver\tests\cli.rs` builds `CWSTRAT.BAS` and asserts the
  executable is there and holds the first image of `cws.ico`. Verify: `cargo test` passes; the `--ignored` test
  passes; `qb64rust --report unsupported tests\programs` reports nothing held back.
- [ ] 5.2 Once: `tools\callsite\callsite.py` on `CWSTRAT.BAS` against the old compiler's C++. Verify: every
  difference is fixed (with a test where the fix is in the compiler) or listed here with its line and reason,
  and the user is told of each that is left; this task says how long the comparison took and whether the game
  becomes a standing program of the call-site check.
- [ ] 5.3 Once, by hand: the built game started from a copy of its folder (outside the repository), left ten
  seconds, closed; the same with the old compiler's build. Verify: this task records that neither ended by
  itself or showed an error box; `tests\programs\README.md` has a "Measured" paragraph for this change (the
  report, the build time against the old compiler's 14 s, the call-site comparison) and says how to build and
  start the game.

## 6. Closing runs and documents

- [ ] 6.1 The closing runs, each once, without `--pass-cache` (`--jobs 8 --build-cache`): `cargo test`, the
  runner's tests, the tests of `tools\testtime`, the `--ignored` command-line tests, the call-site check, the
  slice list with and without folding (`QB64RUST_NO_FOLD=1`), the upstream and differential pass lists, the two
  full suites. Verify: every run passes or its failures are known failures; the numbers are in `STATUS.md`.
- [ ] 6.2 Documents: `STATUS.md` (step 3 done, the next step, the open items: the two leftovers of
  `m3-get-put-bload` taken out, what of CI is still unseen and that the `tier2` job now builds libqb's audio
  part and runs programs that need a sound device), `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`,
  `tests\programs\README.md`, `tools\screen\README.md` (sound stays outside the state; where it is checked),
  `study\00` §5 and its document map, `SOMEDAY.md`, `DIVERGENCES-QB45.md` (a `PLAY` string with `T31`, `T256`
  or `O7` raises no error, where QuickBASIC 4.5 takes a tempo of 32 to 255 and an octave of 0 to 6: each row
  with the source of its QB 4.5 column), `CLAUDE.md`'s layout table (`v29*`, the corpus and slice
  counts); the purpose of `openspec\specs\language\conventional-memory\spec.md` written in place of its
  placeholder. Verify: `openspec validate m3-sound --strict` passes; `python tools\repo_check\check_repo.py
  --untracked` is clean; the counts in the documents equal those of the lists.
- [ ] 6.3 `python tools\testtime\test_time.py`. Verify: its three files are named among those to stage, with
  the commit message for the change.

## Workflow follow-up

- Archive the change (`openspec archive m3-sound`) when the user has looked at it; the delta specs merge into
  `openspec\specs`, and the new main spec `language\sound` keeps its purpose.
