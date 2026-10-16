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

- [x] 1.1 Measure, before the design, what the proposal rests on: the C++ of each sound form and of `$EXEICON`
  (`qb64pe -z`, programs outside the repository), what a program can observe of sound, accepted and rejected
  forms one program each. Verify: `verification\` holds the programs with their `.compile.txt` and `.out.txt`.
  Found (2026-10-13): 110 programs: `v29_a_sound_queue`, `v29_b_sound_wait`, `v29_c_sound_errors`, `v29_x01`–`x86`
  (79 sound programs, 43 accepted and 36 rejected; numbers 33–39 not used), `v29_e01`–`e28` (`$EXEICON`: 14
  accepted, 13 rejected, and `e16`, which the front end accepts and whose build fails), with
  `v29_inc\` (a 16 by 16 icon made for it, a text file, an included file). Every sound form is a plain call;
  `$EXEICON` is `_ICON` with no argument where it stands, a resource file and `DEP_ICON_RC=y`. Sound held back
  by `SOUND WAIT` is not in `PLAY(0)`; in the background (`PLAY "MB"`) the queue reads equal in tenths of a
  second in two runs. The background is set per voice. A relative `$EXEICON` name without `./` is looked for in
  the old compiler's own folder. The `v29` paragraph of `study\00` §5 and the row of `v29*` in `CLAUDE.md`'s
  layout table are written (session 56).
- [x] 1.2 Measure what the design leaves open, one program each (`verification\v29_…`), the C++ outside the
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
  Found (2026-10-13, session 56): seven programs, `v29_d_volume_zero`, `v29_f_queue_slice`, `v29_g_voice`,
  `v29_h_sound_ranges`, `v29_e29`–`e31`; the clone's status empty before and after.
  - Volume 0 changes no queue length and no wait. That a volume holds for the next statement and for `PLAY`
    cannot be observed; read in `audio.cpp`: it is the voice's setting, shared by both statements.
  - `v29_f`, written as the slice program will be (21 statements), gave equal output in three recordings and
    in 24 runs made eight at a time (its executable started eight times at once, three rounds; heavier than
    seven other programs beside it). `T60 L4 C.` is 15 tenths.
  - Voice 1 with voice 0 in the background holds the program and adds nothing to `PLAY(0)`; `PLAY "", "MB"`
    puts it in the background.
  - **Not expected by the design:** a `PLAY` with more than one string never holds the program (a two-second
    note in a foreground voice returned at once), and it ends a `SOUND WAIT`. The first recording of `v29_h`
    showed it: a `PLAY` with four strings released what the program had queued. Design D6 and the spec
    (two scenarios) say so now.
  - The waveform parameter has a range (0 to 1) only in a statement that itself gives waveform 9; a pan
    position below -1, a waveform or a voice of -1 and `PLAY "V101"` are error 5. The spec's requirement and
    scenarios are corrected ("outside its range" was not what the old compiler does).
  - Without a device (read, `PlayWaveform` and `PlayMML`): no argument, frequency, duration and voice are
    checked on any machine; volume, pan, waveform, its parameter and every `PLAY` string are not (design,
    Risks).
  - `$EXEICON`: a name with a drive letter is found; one with one leading slash is found on the compiler's
    drive; a second one inside an included file is the error, one in a branch not taken is not; the error
    names a line of the main file (`e31`).
  - The make flags: the old compiler records its `make` line nowhere, so the order stays the source's
    (`qb64pe.bas` 13706–13721); its compile log shows what the flags do (design D3). A program with
    `$EXEICON` is compiled with `-DDEPENDENCY_ICON`, as D4 point 5 has it.
- [x] 1.3 Put the design's three open questions to the user (silent tests and `BEEP` never run; a relative
  `$EXEICON` name without `./` stays marked; the game built by a test and started once by hand). Verify: the
  answers are a row of `DECISIONS.md`, and the proposal, the design and the groups below follow them.
  Found (2026-10-13): all three answered with the default, so nothing below changes.

## 2. SOUND, PLAY and BEEP compile (design D1 to D3)

- [x] 2.1 `sema`: the rows `SOUND`, `PLAY`, `BEEP` in `STATEMENTS` and `PLAY` in `SUPPORTED`;
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
  Found (2026-10-13): the four rows compiled 76 of the 79 forms of `v29_x*` as measured with no other code.
  No accepted program of the corpus, the upstream set, the snippets, the differential set or `tests\programs`
  has `ON PLAY` or `PLAY ON|OFF|STOP` at the start of a statement. **The three that did not: `CALL SOUND(440,
  1)`, `CALL PLAY(t)` (marked) and `CALL PLAY(0)` (a mark where the old compiler has an error): a built-in
  statement with a template after `CALL`**, the gap three earlier changes left in `SOMEDAY.md`. Fixed in the
  parser for every statement, as D1 asks (`template_call` in `syntax\src\parser\template.rs`): the old
  compiler's rule, read in `qb64pe.bas` 10617–10627 and measured with twelve more programs (`v29_x87`–`x98`),
  is that after the name comes nothing or one pair of parentheses around all that follows, and what stands
  between them is read by the statement's template. So `CALL SOUND(440, 1 WAIT)`, `CALL SOUND(WAIT)`, `CALL
  SOUND` and `CALL LINE(-(3, 4))` are accepted, `CALL CLS()`, `CALL SOUND(440, 1) WAIT` and `CALL LOCATE 1, 2`
  are "Expected CALL sub-name (...)", and `CALL SOUND(440)` and `CALL PLAY` are the template's syntax error.
  The match is a `BuiltinStmt` that starts with `CALL` (`ast::BuiltinStmt::after_call`), so `sema` needed no
  change. **Beyond the sound statements this compiles what was marked before** (each form the old compiler
  accepts, each with a call-site program): the screen statements, `BLOAD`, graphics `PUT`, `_FULLSCREEN`, the
  window statements, `SHELL`, `RANDOMIZE` after `CALL`; and it makes real errors of `CALL PSET(1, 2)`, `CALL
  VIEW PRINT` and `CALL DEF SEG` (`v27_x14`, `x15`, `v28_x67`). Still marked after `CALL`: `SWAP`, `CLOSE`,
  `MID$` (rules of their own), graphics `GET`. Tests: `sound_ir.bas`, `sound_beep_only_ir.bas`,
  `sound_typed.bas`, `sound_errors.bas` (36 lines: 33 errors, and three jumps to `BEEP`, `SOUND` and `PLAY`
  that keep the mark, the third beside the real error of its `PLAY:` line), `call_template_ir.bas`,
  `call_template_errors.bas`; unit tests `make_flags_of_a_program_with_sound` and the build key. Existing
  snapshots that changed: the six of task 2.3 below, and four that held marks for `CALL` forms which now
  compile (`m3_step2_marked`, `shell_environ_errors`, `swap_mid_marked`, `window_errors`: their `CALL` lines
  moved to `call_template_ir.bas`).
- [x] 2.2 Call-site programs, one per accepted form of task 1.1 (`tests\callsite\sound_*.bas`, `play_*.bas`,
  `beep_*.bas`; from the 43 accepted programs, more where one of them holds several forms or types, `x64` with
  a label). Verify: `python tools\callsite\callsite.py`
  on these programs alone reports every one equal; a difference is fixed in 2.1's code, not in the tool.
  Found (2026-10-13): 110 programs, all equal (run on them alone, from a folder outside the repository): 38
  `sound_*`, 16 `play_*`, 11 `func_play_*`, 5 `beep_*`, and 40 `*_call*` for the other statements with a
  template after `CALL`. Seven more were written and taken out again, because the tool does not compare their frame,
  not because a call differed: the three inside a one-line `IF`, `ON PLAY(5) GOSUB done` (the frame of `ON …
  GOSUB`; its number is `qbr_float_to_long(func_play( 5 ,1))` in both, the new compiler with a `(float)` cast
  that the function's `float` parameter makes anyway), and three with an operator or a store whose cast to a
  float type the tool does not drop (`SOUND 37 + 50 * RND, .03`, `n = PLAY(1) + 1`, `sg = PLAY(0)`). Those
  forms are in the `ir` and `typed` tests and the slice programs. `tests\callsite` now has 920 programs.
- [x] 2.3 The lines the rows touch elsewhere: `BEEP:` and `SOUND:` as statements, `PLAY:` an error, `GOTO BEEP`
  keeps its mark (D2), with a front-end test from `v29_x24`, `x65`, `x72`, `x78`–`x83`; the `SOMEDAY.md` entry
  that names `BEEP: x = 1` gets another example. The front-end tests that hold one of the three words today:
  `proc_errors.bas` and `stmt_calls_errors.bas` (a bare `BEEP` as the statement that is not supported: each
  names another one), `error_handling_errors.bas` (`beep: PRINT 1`, a label named like a built-in: another
  name), `stmt_name_colon_errors.bas` (`BEEP: l = 10`: another name), `template_forms.bas` (`PLAY "cde"`,
  `SOUND 440, 18`: parsed only, stays); the slice program `s24_proc_names` has a FUNCTION named `beep`, which
  must go on passing. `crates\README.md`: the three statements and the
  function, the audio part. Verify: `cargo test` passes; `qb64rust --report unsupported tests\programs` names
  only `$EXEICON`.
  Found (2026-10-13): the rule of `CLS:` took `BEEP:` and `SOUND:` with no change. `FILES` stands where `BEEP`
  stood in the four front-end tests, in two command-line tests (`cli.rs`) and in `SOMEDAY.md`. The game's
  report names only `$EXEICON` (1 use). `crates\README.md` is written in task 6.2 with the other documents.

## 3. Sound that runs (design D6)

- [x] 3.1 Slice programs, console, silent, recorded with `qb64pe.exe`: `s74_sound_queue` (what each statement
  adds to `PLAY(0)` in the background, in tenths: read directly before and after the statement, lengths of
  whole tenths only), `s75_sound_wait` (which statements hold the program, yes or no, one-sided limits: "over"
  for a statement that holds, "under" for one that returns), `s76_sound_errors` (error 5 and the accepted
  limits, after `SOUND WAIT`), each with `BEEP` or another
  form as code that is not reached where the coverage check wants it. Each is recorded twice and the two
  recordings are equal, and the three are run once with `--jobs 8` beside the slice programs of another
  category. Verify: the three pass with the new compiler
  (`run_legacy_tests.py --suite corpus --glob` for them) and are on `tests\corpus\slice.list`; `cargo test`'s
  coverage check passes for the four built-ins.
  Found (2026-10-13): the three recorded on their first try (two equal runs each) and passed with the new
  compiler on their first run, alone and in two runs of the whole slice category with `--jobs 8` (83
  programs). `s74` has 47 statements over four voices, `SOUND WAIT` and `RESUME` among them; `s75` has 13
  lines and waits about seven seconds (the longest program of the slice group); `s76` has a part checked on
  any machine and a part that needs a sound device (design, Risks). Not taken from `v29_c`: `SOUND 440,
  65535`, the accepted upper limit of the duration, because libqb fills a buffer of an hour of samples for it
  (about 700 MB); the limit stays measured in `verification\`. Accepted limits are tried with a duration of 0,
  which queues nothing. `BEEP` stands in `s74` behind `IF PLAY(0) > 99`.
- [x] 3.2 `g03_capitol` joins `tests\corpus\slice.list`. Verify: all 52 programs of `tests\corpus\screen` pass
  with the new compiler by the state of their screen (one run of that category); `tests\corpus\README.md` and
  `tests\known_clean_not_passing.list` say so.
  Found (2026-10-13): 52 of 52 in one run of the category; `g03_capitol` left the recorded state on its first
  run. It was never on `tests\known_clean_not_passing.list` (it did not compile cleanly). `tests\corpus\
  README.md` is written in task 6.2.
- [x] 3.3 The shrink-only lists regenerated; the upstream programs that now compile cleanly are run and join
  `tests\upstream\pass.list` or `tests\known_clean_not_passing.list` with the reason. Verify: `cargo test`
  passes with the lists as committed.
  Found (2026-10-13): two upstream programs compile cleanly now, both `PLAY "O3"` and a `PRINT`:
  `console_only/audio_out` passes and joins `pass.list` (141); `audio_out/test` is a test of the generated
  licence file and joins `known_clean_not_passing.list` with that reason. No other list changed.

## 4. $EXEICON (design D4, D5)

- [x] 4.1 `sema`: `$EXEICON` read as D4 says (the name, its three classes, the errors, the rest of the line
  dropped, the statement `_ICON` with no argument, the icon among the settings, through the IR and its dump
  without a folder). First find how the lexer cuts the forms of `v29_e09`, `e22` and `e24` and say so here.
  Tests: an `ir` front-end test (main module, inside a SUB, after `SYSTEM`, in an included file, in a branch
  that is not taken, words and a colon after the name, in a comment and after `REM` (`e14`: no statement, no
  icon), with an `_ICON` statement in the same program (`e26`)); a `check-fail` test with `v29_e05`–`e07`, `e13`, `e17`,
  `e19`, `e23`, `e27`, and a relative name without `./` and a name with one leading slash (both marked). Verify: `cargo test` passes.
  Found (2026-10-13): the lexer makes one token of a metacommand's whole line, to the line end, so the words,
  the comment and the colon with a statement of `e09`, `e22` and `e24` are inside the token and never
  statements; `sema` reads the name from the token's text (`exe_icon_meta`, `check\mod.rs`). All 31 programs
  of `v29_e*` get the old compiler's verdict from the front end but two that the front end cannot give: `e04`
  (the missing file, task 4.2) and `e16` (the build). A second `$EXEICON` names a line of the main file, the
  `$INCLUDE` line for a first one inside an included file, as the old compiler does (`main_line`). Tests:
  `exeicon_ir.bas`, `exeicon_sub_ir.bas`, `exeicon_include_ir.bas` (with `inc\exeicon.bi` and the icon
  `inc\exeicon.ico`), `exeicon_errors.bas` (13 diagnostics: 4 without a name, 3 marked names, 4 second lines,
  one of them in the included file, after a colon, after a tab).
- [x] 4.2 The driver: the check that the file is there, for every command that reports diagnostics (an error at
  the line, the name as written); before `make` the copy `icon.ico` and `icon.rc` in the build's `temp`,
  `DEP_ICON_RC=y`, both files in the build key. Tests: unit tests of the flags and of the key (a changed icon
  gives another key); command-line tests of the missing file (no clone needed) and, `#[ignore]`d, of a program
  with a 16 by 16 icon whose executable holds the icon's image bytes, and of a text file as icon (the build
  fails). `DIVERGENCES.md`: the missing-file error names the file as written, not the old compiler's resolved
  folder or bare file name (D4). Verify: `cargo test` and `cargo test -p qb64rust-driver --test cli --
  --ignored` pass.
  Found (2026-10-13): `check_exe_icon` and `exe_icon_path` in `driver\src\lib.rs`, called after the front end
  by the compile, `-z`, `--dump` and `--report unsupported` paths and by the front-end tests; `write_icon` in
  `build.rs`. A build without an icon keeps the key it had (the two files are hashed only when they are
  there), so no cached executable was lost. Tests: `make_flags_of_a_program_with_an_exe_icon`,
  `build_key_follows_the_icon`; `exe_icon_file_is_missing` (five commands, no clone needed),
  `exe_icon_is_in_the_executable` (also: after the icon changes the executable holds the new image, and a
  program without the line holds neither) and `exe_icon_that_is_no_icon_fails_the_build` (`#[ignore]`d);
  `exeicon_missing_errors.bas`. `DIVERGENCES.md` D-015. **In tier 1 the check is made for the real programs
  only** (`inputs.rs`): the upstream copy holds no binary files (`icon_relative/test` names `./qb64pe.ico`,
  which lies in the clone), and a corpus program's icon lies in its `.files` folder until the runner copies it.
  That upstream test compiles cleanly now, passes when run from the clone, and joined `pass.list` (142).
- [x] 4.3 A slice program `s77_exeicon` (console; the line between two `PRINT`s; its icon in `s77_exeicon.files\`
  if the runner's data files reach the compiler's folder for the program, else the program and its icon are a
  command-line test only and this task says why), recorded with `qb64pe.exe`; a call-site program if
  `tools\callsite` can give a program a file beside it without a change larger than the test is worth (rule 8:
  say which cost applies). `crates\README.md`, `SOMEDAY.md` (a relative name without `./`; the language server
  does not check the file). Verify: the program passes with the new compiler; `python
  tools\repo_check\check_repo.py --untracked` is clean (no recorded path of the icon).
  Found (2026-10-13): the runner copies a program's `.files` folder beside the program before either compiler
  reads it, so `s77_exeicon` is a slice program with its icon in `s77_exeicon.files\`; recorded and passing on
  its first run. The call-site program is there too (`exeicon_line`, its icon in `exeicon_line.files\`): the
  tool copied only the program, and six lines in `callsite.py` copy the folder as the corpus runner does. The
  cost that stays is those lines and a paragraph of the tool's `README.md`; for it the call of the `$EXEICON`
  line is compared with the old compiler's (`sub__icon(NULL,NULL,0)`, equal). `crates\README.md` and
  `SOMEDAY.md` are written in task 6.2.

## 5. Civil War Strategy compiles and builds (design D7)

- [x] 5.1 Tier 1: the set `programs/` of `driver\tests\inputs.rs` requires no diagnostic at all and is lowered,
  validated and emitted; an `#[ignore]`d test in `driver\tests\cli.rs` builds `CWSTRAT.BAS` and asserts the
  executable is there and holds the first image of `cws.ico`. Verify: `cargo test` passes; the `--ignored` test
  passes; `qb64rust --report unsupported tests\programs` reports nothing held back.
  Found (2026-10-13): `real_programs_have_no_diagnostics` in `inputs.rs` (no diagnostic, a warning included;
  the icon found; the IR valid; the C++ emitted) and `civil_war_strategy_builds` in `cli.rs`. The game needed
  nothing beyond groups 2 and 4: it compiled with no diagnostic and built on its first try, in 7 s as the
  test. The report: 1 program, 1 without errors.
- [x] 5.2 Once: `tools\callsite\callsite.py` on `CWSTRAT.BAS` against the old compiler's C++. Verify: every
  difference is fixed (with a test where the fix is in the compiler) or listed here with its line and reason,
  and the user is told of each that is left; this task says how long the comparison took and whether the game
  becomes a standing program of the call-site check.
  Found (2026-10-13): the tool as it is compares one statement of one program, so the comparison was made by a
  script outside the repository that uses the tool's normaliser on every source line (`qb64pe -z` 5.3 s,
  `qb64rust -z` 0.4 s, the comparison itself a few seconds). Of the 4,332 source lines either compiler writes
  C++ for: 1,619 equal token for token; 1,475 with equal calls (libqb's and the program's procedures, with
  their arguments) and another frame or an explicit cast; 1,159 that differ only in parentheses; 79 that
  differ otherwise. **No difference was found that needs a fix, so none is left for the user.** The 79, by
  kind: 48 the number of a temporary on a line that has two (`&(pass1= 9 )` against `&(pass2= 9 )` in a
  one-line `IF`; lines 2118, 4540, 4791 and others); 17 declarations the compilers write in different places (`DIM
  SHARED` of static arrays, lines 106 to 113; `DIM` in a SUB, 1503, 3305, 3474; the copy of a string
  argument at a SUB's header, 2917 and three more); 11 the old compiler's `CHAIN` data on the `COMMON` lines
  and line 1, which the new compiler does not write; and one each: the end of the main module after the
  last `RETURN` (830), a clean-up after `ON ERROR GOTO 0` (277), `"Raze \077"` for `"Raze ?"` (2638, the same
  bytes). What the comparison does not
  see: arithmetic between the calls on the lines that are not equal token for token. **Not made a standing
  check:** the run time would allow it (ten seconds), but it needs a second mode of the tool with these
  classes written down as accepted, which is code to keep in step with every change of the emitter's frames;
  the build test and stage 3 (the game's screens) are the standing checks.
- [x] 5.3 Once, by hand: the built game started from a copy of its folder (outside the repository), left ten
  seconds, closed; the same with the old compiler's build. Verify: this task records that neither ended by
  itself or showed an error box; `tests\programs\README.md` has a "Measured" paragraph for this change (the
  report, the build time against the old compiler's 14 s, the call-site comparison) and says how to build and
  start the game.
  Found (2026-10-13): built in two copies of the folder outside the repository (new compiler 3.6 s, old
  compiler 6.9 s) and each started, left ten seconds and closed: neither ended by itself, each had one visible
  window, titled "Civil War Strategy Game", and no second window (an error box would be one). What the
  screens showed and what was heard was not compared. `tests\programs\README.md` has "Measured (2026-10-13,
  after `m3-sound`)" and "Building and starting the game".

## 6. Closing runs and documents

- [x] 6.1 The closing runs, each once, without `--pass-cache` (`--jobs 8 --build-cache`): `cargo test`, the
  runner's tests, the tests of `tools\testtime`, the `--ignored` command-line tests, the call-site check, the
  slice list with and without folding (`QB64RUST_NO_FOLD=1`), the upstream and differential pass lists, the two
  full suites. Verify: every run passes or its failures are known failures; the numbers are in `STATUS.md`.
  Found (2026-10-13, session 56), each run once: `cargo test` 297 pass (29 s); the runner's tests 68; the tests
  of `tools\testtime` 36; the `--ignored` command-line tests 16 of 16 (10 s); the call-site check 921 of 921
  (46 s); the slice list 362 of 362 (269 s) and without folding 362 of 362 (218 s); the upstream pass list
  142 of 142 (83 s); the differential pass list 59 of 59 (49 s); the full corpus 381 pass of 399, 12 rejected,
  1 `.err` program with only marked errors, 5 known failures (54 s); the full upstream suite 142 pass of 404,
  none wrong (13 s). **Not repeated after the closing runs, and why that is safe:** one call-site program
  added afterwards (`get_call`, `CALL GET(…)`) was compared alone and is equal; the edits after the runs are
  documents, a spec delta and that program, none of which a test of tier 2 reads.
- [x] 6.2 Documents: `STATUS.md` (step 3 done, the next step, the open items: the two leftovers of
  `m3-get-put-bload` taken out, what of CI is still unseen and that the `tier2` job now builds libqb's audio
  part and runs programs that need a sound device), `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`,
  `tests\programs\README.md`, `tools\screen\README.md` (sound stays outside the state; where it is checked),
  `study\00` §5 and its document map, `SOMEDAY.md`, `DIVERGENCES-QB45.md` (a `PLAY` string with `T31`, `T256`
  or `O7` raises no error, where QuickBASIC 4.5 takes a tempo of 32 to 255 and an octave of 0 to 6: each row
  with the source of its QB 4.5 column), `CLAUDE.md`'s layout table (`v29*`, the corpus and slice
  counts); the purpose of `openspec\specs\language\conventional-memory\spec.md` written in place of its
  placeholder. Verify: `openspec validate m3-sound --strict` passes; `python tools\repo_check\check_repo.py
  --untracked` is clean; the counts in the documents equal those of the lists.
  Found (2026-10-13): written. Beyond the list: the delta of `language/builtin-statements` has a requirement
  for a statement with a template after `CALL`; `tests\upstream\README.md` (142 of 279); `tools\callsite\
  README.md` (the `.files` folder); `DIVERGENCES-QB45.md` has Q-011 (the tempo and the octave) and Q-012 (`ON
  PLAY`, which QuickBASIC has as an event); the count of `.output` files of the slice group in
  `tests\corpus\README.md` was stale (75) and is 84.
- [x] 6.3 `python tools\testtime\test_time.py`. Verify: its three files are named among those to stage, with
  the commit message for the change.
  Found (2026-10-13): run at the close (187 sessions and 3,528 runs on record); `runs.csv`, `sessions.csv` and
  `REPORT.md` named to the user with the other files and the commit message.

## Review of the change (2026-10-13, after its close)

Found by a review of the staged change and fixed, each with a test:

- A `$EXEICON` name with a byte outside ASCII was never found: the name shown in messages (`\xNN`) was used as
  the file's name. It is taken as UTF-8 now, as an included file's name is (`cli.rs`, `exe_icon_name_forms`). The
  old compiler's reading of such bytes is not measured.
- The rest of a `./` name is added to the source file's folder as text (`qb64pe.bas` 3853; measured,
  `verification\v29_e32`), where `Path::join` left the folder for a rest that starts with a slash. A drive with
  no slash after it (`C:game.ico`) is "not supported yet" with the other relative names, not absolute.
- The copy of a read-only icon was read-only, so the next build into a kept build folder failed; the icon files
  of an earlier build entered the build key of a program without `$EXEICON` (`build.rs`,
  `icon_files_of_a_kept_build_folder`).
- `tools\callsite\callsite.py` left each program's `<name>.files` in the work folder all programs share, and a
  glob without `.bas` took that folder for a program.
- `call_stmt` held a branch `template_call` had made unreachable; `CALL CLS()` was told to add the parentheses it
  has (now "expected something between the parentheses"); the IR's `exe_icon` was a name nobody read (now a
  flag).
- By a second review, of these fixes: a source file named with a drive and no folder (`C:game.bas`) got a
  separator after the drive (`lib.rs`, `beside` and its test); a name's byte that is no UTF-8 is shown as
  written (`ExeIcon::shown`); the call-site tool's self-test covers a program's files and the glob.
- Not changed: `BuiltinStmt::after_call` counts words where it could read the first one (it needs the source
  text, which the tree's accessors do not have; `sema` skips a statement with a parse error, so no reader sees
  the difference); the missing-icon check stays a call of the driver's (design D4).
- Runs: `cargo test`, fmt, clippy; the command-line tests that build (16); the call-site check for `exeicon*`
  and `*_call*` (62 programs, all equal) and its self-test; the slice program `s77_exeicon`. Not repeated: the
  rest of the slice list, the pass lists and the two full suites. No generated C++ changed (every `ir` and
  `cpp` snapshot is as it was), and what changed is where an icon is looked for and copied, which `s77_exeicon`,
  the game's build and `exeicon_line` run.

## Workflow follow-up

- Archive the change (`openspec archive m3-sound`) when the user has looked at it; the delta specs merge into
  `openspec\specs`, and the new main spec `language\sound` keeps its purpose.
