# Tasks

Order: group 1 first (measurements), then 2 (everything else needs a program with a window), then 3, 4 and 5 in
that order (each adds rows), then 6. Every group closes with `cargo fmt --check`, `cargo clippy --workspace
--all-targets -- -D warnings` and `cargo test` green and with its tests, lists and `crates\README.md` lines in
place, so the change can stop after any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done. "Tier 2 slice" below
is `python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list
tests\corpus\slice.list --jobs 8 --build-cache target\build-cache`, run also with `QB64RUST_NO_FOLD=1`. "The
call-site check" is `python tools\callsite\callsite.py`. A new measured fact goes into a `v27` paragraph of
`study\00` §5 in the task that measures it. A behaviour of the old compiler that looks questionable is implemented
its way and listed in `SOMEDAY.md` ("QB64pe behaviours to review").

## 1. Measurements before code

- [x] 1.1 Read the old compiler's C++ (`qb64pe -z`, programs outside the repository) for every form of every
  built-in of the proposal: each argument left out in turn, each choice (`STEP`, `B`, `BF`, `SCREEN`, `PRINT`,
  `#`, `LPRINT`, `_MANUALDISPLAY`), and one slot of each slot type with an INTEGER, a LONG, an `_INTEGER64`, a
  SINGLE, a DOUBLE and a literal; for each function also its result in `PRINT`, `HEX$` and an operation. Verify:
  design D2 holds a table with one line per form (call, conversions, mask, result types) in place of "expected",
  and says for each built-in whether it is a `Plain` row, needs a new slot type, or needs a rule of its own.
  Found: 455 forms compiled. Every statement is the plain call of its row and template, `LINE` too; no rule of
  its own is needed. `PAINT`'s fill is a number or a string with a cast written before it (a slot kind of its
  own). `_UNSIGNED LONG` is passed as a LONG slot is; SINGLE as `_FLOAT` is (as it is).
- [x] 1.2 Measure a program with no console (design D1): compile `PRINT "x"`, `SYSTEM` and the same with
  `$SCREENHIDE` with `qb64pe -x` outside the repository; note the two globals, the libqb variant and the link
  flags from the clone's `internal\temp\compilelog.txt` and `global.txt`; compare the C++ of `PRINT`, `INPUT`,
  `END` and an error check with the same program under `$CONSOLE:ONLY`. Verify: D1 holds the two rows beside the
  four of `m2-console-screenhide` D3 and says whether any statement's code differs by mode; the clone's `git
  status --porcelain` is the same before and after.
  Found: no make flag, the libqb variant of a `$CONSOLE` program with a window, `-mwindows`, `console=0`; no
  statement's code differs by mode. The clone's status was empty before and after.
- [x] 1.3 Measure the doubtful forms, one program each (`verification\v27_x01…`, recorded with `python
  tools\legacy_tests\run_legacy_tests.py --suite verification --glob "v27_x*.bas"`): forms a template seems to
  allow or forbid (`LINE (1, 2), 3`; `LINE -(5, 5)`; `LINE STEP(1, 1)-STEP(2, 2)`; `PSET STEP (1, 1)`; `LOCATE`
  with six arguments and with none; `COLOR` with none; `SCREEN` with none; `VIEW PRINT 2`; `PALETTE 1`;
  `INKEY$()` and `CSRLIN()`; `POINT` with three arguments; a string where a number is wanted and the other way
  round; each statement after `CALL`), and `INKEY$`, `CSRLIN` and `POS` under `$CONSOLE:ONLY`. Verify: each has a
  `.compile.txt` (and an `.out.txt` where it runs); `study\00` §5 says which forms are accepted; the spec
  scenarios "A rejected form" of `language/screen-output` name forms the old compiler does reject, corrected if
  not.
  Found: 51 programs (`v27_x01` accepted forms, `x02` after `CALL`, `x03`–`x50` one rejected form each,
  `v27_a_console_functions`). Both spec scenarios name forms the old compiler rejects (`x03`, `x04`). Under
  `$CONSOLE:ONLY` `INKEY$` is empty, `POS(0)` is 1 and `CSRLIN` does not move.
- [x] 1.4 Write and record the new screen programs of design D4 (`tests\corpus\screen\d01_print_window` and the
  `d…` programs per statement family, the functions, the runtime errors, pacing with a `.keys` file where a key
  is read), with `python tools\legacy_tests\run_legacy_tests.py --suite corpus --record --glob "screen/d*"`.
  Verify: each has a `.screen` recorded from two equal runs and an empty `.output`; `tests\corpus\screen\SOURCE.md`
  lists them; `python tools\screen\state_to_png.py` of two graphics ones shows what the program draws; the full
  corpus run with the old compiler still passes for `screen/`.
  Found: 16 programs, `d01`–`d16` (the command is `--category screen --glob "d*.bas"`: `--glob` matches the file
  name only). All 11 graphics states were looked at as pictures; four programs were rewritten because their last
  statements hid the others' work (a flood fill that leaked, text printed over the pixels `POINT` reads, a
  `CLS` or a mode change at the end). `SLEEP` with no time cannot be ended by the keys file once a key is in
  the buffer: `d15_pacing` empties the buffer and gives `SLEEP` a time (`tools\screen\README.md`). The old
  compiler passes all 45 programs of `screen\`.
- [x] 1.5 Put the order of work proposed for M3 (`proposal.md`, "Why") to the user. Verify: the answer is a row of
  `DECISIONS.md`, and `STATUS.md` "Next" names the order and this change as its current step.
  Found: the user accepted the order as proposed (2026-10-12).

## 2. A program with a window as its output (design D1, D6)

- [x] 2.1 `sema`: remove the mark for a program without a console; follow task 1.2 where a flag, a global or a
  statement's code differs. Tests: the front-end tests that pinned the mark (`no_console`, `no_console_untaken`
  and the others whose snapshot holds its text) become `ir` or `cpp` tests of the settings of such a program
  (window, no console; with `$SCREENHIDE`: hidden); a unit test in `driver\build.rs` for its make flags. Verify:
  `cargo test` passes with the changed snapshots reviewed: only the mark's line goes, and no program gains an
  error.
  Found: nothing but the mark had to change. `no_console` and `no_console_untaken` are `ir` tests (`Settings:
  window`, `Settings: window, hidden at the start`); `flags_nothing_set_errors` is `check-ok`;
  `echo_no_console` keeps its three `_ECHO` errors.
- [x] 2.2 Run `d01_print_window` and every program of the 29 that now compiles with the new compiler; add those
  that pass to `tests\corpus\slice.list`. Verify: tier 2 slice passes for them with and without folding; a
  program that compiles and does not leave the recorded state is a bug to fix before the group closes, not a
  list entry.
  Found: three programs compile, `c07_input_window`, `c08_end_prompt` and `d01_print_window`, and all three
  leave the recorded state on their first run, with and without folding.
- [x] 2.3 Regenerate the shrink-only lists (`QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test
  inputs`) and settle every corpus and upstream program that now compiles cleanly (design D6): run each with both
  compilers; those whose recorded output matches join their pass list, the others go into
  `tests\known_clean_not_passing.list` with the reason. Verify: `cargo test` passes; the diff of the lists holds
  no program that gained an error; the tier 2 upstream and differential pass lists pass.
  Found: the three shrink-only lists did not change. One upstream program became clean, `basic/test` (`print
  "Hello, World!"`, `SYSTEM`): it is a test of the generated licence file (`-f:GenerateLicenseFile`, which the
  driver does not write), so it is in `tests\known_clean_not_passing.list` with that reason; the old compiler
  passes it. The other upstream programs without a console still carry other marks. Upstream 140 of 140,
  differential 59 of 59, slice 300 of 300.
- [x] 2.4 Documents of the group: `crates\README.md` (a program without a console; screen programs in tier 2 and
  that their windows show), `tests\corpus\README.md` (screen programs on `slice.list`), `tools\screen\README.md`
  if a line of it changed. Verify: every command written there runs as written.
  Found: `tools\screen\README.md` gained the rule for `SLEEP` with a keys file (task 1.4).
- [x] 2.5 Record what is not yet known about the screen programs on GitHub's Windows runner, for the next time CI
  is looked at. (Reworded 2026-10-12: it was "after the user's push, read the `tier2` job"; a change no longer
  waits on CI, `CLAUDE.md` working rule 9, and the `tier2` job runs by hand and weekly.) Verify: `STATUS.md`
  "Open items" names the question and the fallback of the design's first risk.
  Found: recorded there. Whether the runner gives a screen program a shown window is still not known: the job
  has to be started by hand on this branch to see it. If the programs fail there and pass here, the fallback
  stands: a `screen.list` run locally and not by the job, with the coverage test reading both lists.

## 3. Text-screen statements and functions (design D2, D3, D5)

- [x] 3.1 The slot types task 1.1 found (`_UNSIGNED LONG`, SINGLE, and the `_UNSIGNED LONG` return type), in
  `sema\builtins.rs`, the checker and the emitter, each converted as measured. Tests: a unit test per slot type;
  they are exercised by the rows of 3.2 and 4.1. Verify: `cargo test -p qb64rust-sema` passes; no existing `cpp`
  snapshot changes.
  Found: two new slot kinds, `Slot::ULong` (`_UNSIGNED LONG`: a store into that type) and `Slot::ColorOrTile`
  (the fill of `PAINT`: a number or a string, with the old compiler's cast in the emitter); SINGLE maps to the
  existing `Slot::Float` (passed as it is); `table_ty` knows `_UNSIGNED LONG`. Unit test `screen_slot_kinds`,
  `cpp` test `screen_calls_cpp`. No existing `cpp` snapshot changed.
- [x] 3.2 Rows for `SCREEN`, `CLS`, `COLOR`, `LOCATE`, `WIDTH`, `VIEW PRINT`, each form task 1.1 listed, with a
  rule of its own where it found one; forms left marked are named in `SOMEDAY.md`. Tests: a call-site program per
  form and per argument type of one slot; an `ir` front-end test per statement; a `check-fail` test with the
  rejected forms of task 1.3. Verify: the call-site check reports every new program equal; `cargo test` passes
  (the coverage test included, once 3.4 has listed the screen programs).
  Found: no rule of its own was needed and no form stays marked; the rows of groups 3, 4 and 5 came in together.
  `ir` test `screen_text_ir`, `check-fail` test `screen_text_errors` (every rejected form of `v27_x03`–`x50`
  that belongs here is an error that is not "not supported yet"). **A hole found on the way:** `cls(1) = 5`
  compiled as `CLS` with a comparison; measured for 44 statement names (`v27_x51`–`x57`), a statement that
  starts `name (…) =` is an assignment to the old compiler and rejected, `WIDTH` alone excepted. `sema` now
  rejects it for every built-in statement (`not_an_assignment`, test `stmt_name_assigned`); statements of
  earlier changes had the same hole (`SEEK (1) = 2, 3`). Four older tests that used `CLS` or `LOCATE` as their
  example of a statement that is not compiled now use `BEEP`.
- [x] 3.3 Rows for the functions `INKEY$`, `CSRLIN`, `POS`, `SCREEN`. Tests: a call-site program per form; a
  `typed` front-end test per function with its held and believed type; the rejected forms in the `check-fail`
  test of 3.2; one line of an existing console slice program for `CSRLIN`, `POS` and `INKEY$` under
  `$CONSOLE:ONLY` if task 1.3 found their values stable there. Verify: as 3.2.
  Found: all four are `Rule::Plain` rows (`POINT` too, with the rows of group 4). `typed` test
  `screen_functions_typed`. In a console program `POS(0)` and a difference of two `CSRLIN` are stable, so a new
  slice program, `s69_console_screen_functions`, prints them and `LEN(INKEY$)` (recorded; it passes), and
  `237_csrlin_pos` of the corpus, which waited on the two, passes and is listed. Left marked: an argument left
  out of a function call (`SCREEN(1, , 1)`: an error in the old compiler, "omitted arguments" here).
- [x] 3.4 Run the screen programs in reach (`c01`, `c05` to `c11`, `m00`, the `d…` programs of text statements and
  functions); add those that pass to `slice.list`. Verify: tier 2 slice passes with and without folding; every
  statement and function of this group runs in at least one listed screen program (the coverage test passes).
  Found: with the rows of all three groups in, 42 of the 45 screen programs compile and all 42 leave the
  recorded state on their first run, with and without folding; none needed a fix. They are on `slice.list`
  with `189_width` (342 programs). The coverage test passes.
- [x] 3.5 Review of the group, inline, by the panel's roles: the new code against tasks 1.1 and 1.3, each row's
  forms against its call-site programs, what stayed marked. Verify: each finding is fixed or listed in
  `SOMEDAY.md`; `crates\README.md` says what the compiler supports now; `qb64rust --report unsupported
  tests\corpus\screen` is noted here.
  Found, by role (the extra role is a runtime engineer, for what libqb's entries take: its prototypes were read
  for every call, `study\00` §5):
  - *Compiler and languages:* the `name (…) =` hole of 3.2, fixed. The cast of `PAINT`'s fill binds to the
    first operand in the old compiler's C++; shown to give the same 32 bits as a cast of the whole (design D2,
    "As built"), so the emitter writes the whole.
  - *Test:* the call-site check reported 20 of the 308 programs written first as different; none was a
    different call. Rule 5 of the tool now also drops the `u` suffix of a whole-argument literal (generic,
    with self-test pairs); seven programs the check cannot compare were taken out and their cases put into
    the screen programs `d08` and `d12` (`tools\callsite\README.md`). 663 of 663 equal.
  - *QB64:* `k = CSRLIN$` is a variable in the old compiler and stays marked here by the older rule for a
    reserved word with a suffix; `WIDTH (80) = 80` is an array there and marked here.
  - *Runtime:* libqb takes `uint32` for the colours the table types LONG (`COLOR`, `LINE`, `PSET`,
    `PALETTE`) and `float` or `double` for every coordinate; an integer or a float passed as it is converts
    in C++ as the old compiler's argument does. Nothing was assumed from the table's types alone.
  - *Rust:* `Slot` is matched exhaustively in the checker, the validator and the emitter; no wildcard arm.
  - *Pragmatic:* the 301 call-site programs make the call-site check longer (the time is in `STATUS.md`); that
    is machine time in tier 2 and CI, accepted for a check of every form.
  `qb64rust --report unsupported tests\corpus\screen` after the group: 45 programs, 42 without errors; `PLAY`
  1 program (5 uses), `_RGB32` 1 (2), `GET` 1, `PUT` 1, `_RGBA32` 1.

## 4. Drawing statements (design D2 to D4)

- [x] 4.1 Rows for `PSET`, `PRESET`, `LINE`, `CIRCLE`, `PAINT`, `DRAW`, `VIEW`, `WINDOW`, `PALETTE`, `PCOPY` and
  the function `POINT`, each form task 1.1 listed, with a rule of its own where it found one (`LINE` and `PAINT`
  are the likely ones). Tests: a call-site program per form and per argument type of one slot of each slot type;
  an `ir` front-end test per statement, a `typed` test for `POINT`; the rejected forms of task 1.3 in a
  `check-fail` test. Verify: the call-site check reports every program equal; `cargo test` passes once 4.2 has
  listed the screen programs.
  Found: `LINE` is a `Plain` row; `PAINT` is one too, with the slot kind of 3.1 for its fill. `ir` test
  `screen_draw_ir`, `check-fail` test `screen_draw_errors`, `POINT` in `screen_functions_typed`. 301 call-site
  programs for the three groups, all equal.
- [x] 4.2 Run the screen programs in reach (`m01` to `m13`, `m32`, `m256`, `c02` to `c04`, `g01`, `g02`, `g04`,
  `g05`, the `d…` drawing programs); add those that pass to `slice.list`. Verify: tier 2 slice passes with and
  without folding; a program that compiles and leaves another state is fixed before the group closes.
  Found: all pass (the 42 of task 3.4) but `m32_newimage_32`, which does not compile: it names `_RGB32` and
  `_RGBA32`, image built-ins this change leaves out. So 26 of the first 29 pass, not the 27 the design counted.
- [x] 4.3 Civil War Strategy after the group: `qb64rust --report unsupported tests\programs`. Verify:
  `tests\programs\README.md` has a dated "Measured" paragraph with what the game still needs, in order of uses.
  Found: 135 marked errors in 15 messages (1,709 in 33 before): `SOUND` 33, `PLAY` 31, `TAB` 22, `PUT` 19,
  `DEF SEG` 8, `GET` 8, labels named like a SUB 7, `BLOAD` 4, `BEEP`, `_FULLSCREEN`, `$EXEICON`.
- [x] 4.4 Review of the group, as 3.5. Verify: findings fixed or listed; `crates\README.md` updated.
  Found, by role:
  - *QB64:* the game's report still showed "the label `CLS:`" ten times: a line that starts `CLS :` was read
    as a label and marked. Measured for 17 statements (`v27_x58`–`x63`): a statement's name and a colon at the
    start of a line is the statement with no argument, never a label, and an error when the statement needs an
    argument. Built (`bare_stmt`; tests `stmt_name_colon_ir`, `stmt_name_colon_errors`); it holds for the
    statements of earlier changes too (`CLOSE:`, `RANDOMIZE:`). A jump to such a name stays marked.
  - *Compiler and languages:* `PALETTE USING a(0)`, which the old compiler reads by hand and this change
    leaves out, got the parser's "matches none of its forms": a false error for an accepted line, there since
    `m2-parser-breadth` and unseen because no program in hand has it. Now marked (`v27_x64`, test
    `palette_using_marked`). The other built-ins of the change have no hand-written form (`qb64pe.bas` read
    for each name).
  - *Test:* four of the `d…` programs had to be rewritten after their states were looked at as pictures
    (task 1.4); the states of all 11 graphics programs were looked at before they were trusted.
  - *Runtime, Rust, pragmatic:* nothing beyond 3.5.

## 5. Pacing, and what the 29 still name

- [x] 5.1 Rows for `_LIMIT`, `_DELAY`, `SLEEP`, `_DISPLAY`. Tests: call-site programs per form and argument type;
  an `ir` test each; the pacing screen program of task 1.4 on `slice.list`. Verify: the call-site check and tier
  2 slice pass; that program takes under two seconds to run.
  Found: four `Plain` rows; `ir` test `screen_pacing_ir`. `_LIMIT` and `_DELAY` take a `double`, so their
  call-site programs pass a DOUBLE variable, a float literal and an expression of them (the check does not
  compare another type there); the other types are in the `ir` test and `d15_pacing` (`_LIMIT 60`, `_DELAY
  .01`). `d15_pacing` is listed and takes 0.8 s with the build cache, compile and run together.
- [x] 5.2 For each of the 29 screen programs not yet on `slice.list`: name what holds it back (`qb64rust --report
  unsupported tests\corpus\screen`). A built-in of this proposal's list that a program still needs in another
  form is brought in; anything outside the list (`PLAY`, graphics `GET`/`PUT`) stays marked. Verify:
  `tests\corpus\README.md` says, per program not listed, what it waits on; every other screen program is on
  `slice.list` and passes.
  Found: three are not listed, and none waits on a built-in of this change: `g03_capitol` (`PLAY`),
  `g06_message_box` (graphics `GET` and `PUT`), `m32_newimage_32` (`_RGB32`, `_RGBA32`). The other 42 are
  listed and pass. `_RGB32` and `_RGBA32` are two plain functions and would be a small addition; the proposal
  puts the image built-ins outside this change, so they are named for the user and not brought in.
- [x] 5.3 `SOMEDAY.md`: "Left marked by `m3-window-output`", one row per form left marked with the reason and
  the measurement; "QB64pe behaviours to review" gains what the measurements found. Verify: every "not supported
  yet" message the new rows can give has a row.
  Found: eight rows under "Left marked", five under "QB64pe behaviours to review". The marks the new code can
  give: a statement with a template after `CALL` (named by the statement), "omitted arguments" in a function
  call, "an array named `WIDTH`", `` `PALETTE USING` ``, a jump to a statement's name; each has a row.

## 6. Closing runs and documents

- [x] 6.1 The full runs, release build, `--jobs 8 --build-cache`: tier 1, the runner's own tests, the call-site
  check, tier 2 slice (with and without folding), the upstream and differential pass lists, the full corpus and
  the full upstream suite with the new compiler. Verify: none wrong at run time; the numbers, and how much longer
  tier 2 takes with the screen programs, are in `STATUS.md` "Numbers at the last full runs".
  Found: tier 1 292 tests, 28 s; the runner's tests 56; call-site 663 of 663 (5 min 44 s); slice 342 of 342
  without folding and 341 with it, about 2 min each (under a minute before); upstream pass list 140, differential
  59; full corpus 360 pass of 384, 17 rejected, 1 `.err` only marked, 5 known failures, 1 wrong state; full
  upstream as before (140 pass, 225 rejected, 38 only marked, 1 known failure). **The one wrong state was the
  test's:** `m10_screen_10` failed in two of the three parallel runs because libqb rewrites four palette entries
  of `SCREEN 10` from the clock at each refresh. The user chose to fix the state file (`DECISIONS.md`);
  after the fix the 45 screen programs pass with the old compiler and the 42 that compile with the new one, run
  in parallel. Two small changes came after the full runs and were checked by tier 1 and that screen run only,
  not by the full set again (user, 2026-10-12: fewer repeated runs): the state file's fix and the mark for
  `PALETTE USING`, which no corpus or upstream program has.
- [x] 6.2 Documents: `STATUS.md` (where we are, next, the screen-program and demand rows), `ROADMAP.md` (M3's
  boxes: the drawing statements as far as done), `study\00` (§2's M3 row, §5 complete for `v27`, the document
  map if a study was added), `CLAUDE.md`'s layout table (`tests\corpus\`, `verification\`), `DECISIONS.md` for
  anything decided on the way. Verify: `python tools\repo_check\check_repo.py --untracked` is clean; `openspec
  validate m3-window-output --strict` passes.
- [x] 6.3 Leave CI as an item to look at, not a step of the change. (Reworded 2026-10-12: it was "after the
  user's push, all three workflows green"; `CLAUDE.md` working rule 9.) Verify: `STATUS.md` "Open items" says
  the change is not yet seen on GitHub and what to look for.
  Found: recorded there. Nothing of this change has run on GitHub; the workflow file itself changed too (the
  `tier2` job by hand and weekly), which shows the first time the workflow is started.
