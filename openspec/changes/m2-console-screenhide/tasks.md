# Tasks

Order: group 1 first (the last measurements), then 2 (everything else builds on it), then 3 to 6 in any order
(3 and 4 add rows; 5 and 6 stand alone), then 7. Every group closes with `cargo fmt --check`, `cargo clippy` and
`cargo test` green and with its tests, lists and `crates\README.md` lines in place, so the change can stop after
any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done. "Tier 2 slice" below
is `python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list
tests\corpus\slice.list`, run also with `QB64RUST_NO_FOLD=1`. "The call-site check" is `python
tools\callsite\callsite.py`. A new measured fact goes into the `v26` paragraph of `study\00` §5 in the task that
measures it.

Already measured before this proposal (2026-10-12): `verification\v26_a`–`v26_h2` and `v26_x01`–`x39`, 51
programs, recorded with `qb64pe.exe`.

## 1. Measurements before code

- [ ] 1.1 Measure the two line rules of design D2 for other metacommands: `verification\v26_x40…`, one program
  each, for a tab before `$IF`, `$LET`, `$INCLUDEONCE`, `$CONSOLE:ONLY` and `$SCREENHIDE`, and for each of them
  but `$SCREENHIDE` (measured, `v26_x36`) after a statement and a colon. Record with `python
  tools\legacy_tests\run_legacy_tests.py --suite verification --glob "v26_x*.bas"`. Verify: a `.compile.txt` and
  an `.out.txt` exist for each; `study\00` §5 says for which metacommands each rule holds; design D2 and its open
  question are rewritten to match.
- [ ] 1.2 Read the old compiler's make line for four programs (`$CONSOLE:ONLY`; `$CONSOLE`; both; `$CONSOLE` with
  `_ICON`): find where `qb64pe -x` leaves it (the clone's git-ignored `internal\temp`: its `compilelog.txt` holds the
  link line and the name of the libqb variant, seen for `v26_x38`; or its `-q`-less output)
  and note the `DEP_*` flags of each. Verify: the four flag sets are written into design D3, replacing "read in
  task 1.2", and D3's mapping is corrected if a flag differs from it.
- [ ] 1.3 Read the old compiler's C++ (`qb64pe -z`) for one call of each of the 20 window built-ins and of
  `_DEST`, `_SOURCE`, `_CONSOLE`, `_ECHO`, in every form of its template. Note per built-in: the call, the
  conversion of each argument, the result type as the generated code holds it, and whether an existing rule of
  `sema\builtins.rs` fits. Verify: design D6 holds a table with one line per built-in, and its three "oddities"
  (`_SCREENEXISTS`, `_WINDOWHANDLE`, the parenthesised pairs) each have an answer.
- [ ] 1.4 Measure the flags once more where D7 assumes: `v26_x…` programs for `$IF _EXPLICIT_` between two `OPTION
  _EXPLICIT` lines, `$ASSERTS:CONSOLE` with `$IF _CONSOLE_` above and below it (compile only if the program is
  otherwise rejected), and a program that needs three rounds (a `$CONSOLE` inside `$IF _EXPLICIT_`, with `OPTION
  _EXPLICIT` on the last line). Each program that runs prints where it is recorded: under `$CONSOLE:ONLY`, after
  `_DEST _CONSOLE`, or with `_ECHO`. (D7's rule for a round that is not the last and a `$CONSOLE:ONLY` in its
  own flag's branch are measured: `v26_x39`, `x38`.) Verify: each is recorded; D7 says what each showed; a
  second corner where the old compiler's passes disagree, if found, is added to D7 and to the `compiler/pipeline`
  delta as a marked case.

## 2. The console mode, the metacommands and the build (design D1–D4)

- [ ] 2.1 `sema`: the four metacommands matched exactly, each rejected form an error of the program (D2); the
  mode recorded (console, window, hidden; D1); "select the console" pushed at every `$CONSOLE:ONLY`; the mark
  reworded to "programs without `$CONSOLE` or `$CONSOLE:ONLY`". Tests: a `check-fail` front-end test with every
  rejected form of `v26_x09`–`x14`, `x23`, `x29`–`x32`, `x35`–`x37` (and the forms of 1.1 that hold), a
  `check-ok` test with the accepted forms of `x01`, `x04`, `x08`, `x15`. Verify: `cargo test` passes with the
  new snapshots reviewed; each rejected form's diagnostic is not marked "not supported yet".
- [ ] 2.2 IR: `Program::settings` (console, window, window hidden, runtime parts), `lower`, `validate`, the dump's
  `settings` line (D3). Tests: an `ir` front-end test for `$CONSOLE` with `$SCREENHIDE`, one for `$CONSOLE:ONLY`,
  one for both metacommands, one for `$SCREENHIDE` followed by `$SCREENSHOW`. Verify: the four snapshots show the
  settings the `language/console-and-window` delta names; existing `ir` snapshots change by that one line only.
- [ ] 2.3 Emitter and driver: the two globals written from the settings; the make flags from the settings in
  place of the fixed `DEP_CONSOLE_ONLY=y`; the flags hashed into the build key and its version string changed
  (D3, D4). Tests: a `cpp` front-end test for the globals in each of the four cases of 2.2; a unit test in
  `build.rs` that the key differs between two settings and that the flags of each case equal those noted in
  task 1.2. Verify: `cargo test -p qb64rust-driver` passes, with `-- --ignored` too (the tests that build an
  executable).
- [ ] 2.4 Record two slice programs with `qb64pe.exe` and add them to `slice.list`. `s62_console_only_forms`, a
  program without a window: `PRINT` above `$CONSOLE:ONLY`, `$CONSOLE:ONLY` and `$CONSOLE` in one program, the
  metacommands as last lines and in a SUB (`_DEST` comes in group 3, so `$CONSOLE:ONLY` a second time after
  `_DEST 0` goes into `s64`). `s63_console_window`, the first program linked with the windowed libqb: `$CONSOLE`
  and `$SCREENHIDE`, a `PRINT` that goes to the hidden window, `SYSTEM`; its recorded output is empty and its
  exit code 0. Verify: tier 2 slice passes for both with and without folding; `git status --porcelain` in the
  reference clone is the same before and after the first build of the windowed libqb.
- [ ] 2.5 Regenerate the lists and snapshots the reworded mark changes (`QB64RUST_UPDATE_LISTS=1 cargo test`, then
  review the diff: only the wording and the programs that lose a mark may change). Update `crates\README.md`
  (what the compiler supports; the build's flags), `tests\corpus\README.md` (no `PRINT` with a comma in a program
  with plain `$CONSOLE`). Verify: `cargo test` passes; the list diff holds no program that gained an error.
- [ ] 2.6 After the user pushes group 2: read the `tier2` job of `rust.yml` on GitHub. Verify: it is green, and
  `STATUS.md` records how long the job took before and after; or, if a program with a window does not start
  there, the fallback of design D9 is put to the user before group 3's windowed programs go onto a list.

## 3. `_DEST`, `_SOURCE`, `_CONSOLE`, `_ECHO` (design D5)

- [ ] 3.1 Rows for the statements `_DEST`, `_SOURCE`, `_ECHO` and the functions `_DEST`, `_SOURCE`, `_CONSOLE`,
  with the rules task 1.3 found; `_ECHO`'s compile error in a program with no console, checked after the walk.
  Call-site programs in `tests\callsite\`: one per form and per argument type class (`_DEST` with a literal, a
  LONG, an INTEGER, a SINGLE, a DOUBLE, an `_INTEGER64`, an expression; `_ECHO` with a literal, a variable, an
  expression). Tests: `typed` for the three functions, `ir` for the three statements, `check-fail` for `_DEST`
  with no argument, `_DEST "a"`, `_CONSOLE = 1` and `_ECHO` without a console. Verify: the call-site check
  reports every new program equal; `cargo test` passes, the built-in coverage test included once 3.2 is done.
- [ ] 3.2 Record slice programs with `qb64pe.exe` and add them to `slice.list`: `s64_dest_console` (the scenarios
  of "_DEST and _SOURCE" and "_ECHO" in the delta: switching, the handles, a SUB and a FUNCTION, error 258, every
  numeric type as a handle, and in a second program of the same name with suffix `b` `$CONSOLE:ONLY` a second
  time after `_DEST 0`; no `PRINT` with a comma) and `s65_console_input` with a `.stdin` file (`INPUT` and
  `LINE INPUT` after `_DEST _CONSOLE`, with and without `_SOURCE _CONSOLE`). Verify: tier 2 slice passes for both
  with and without folding; their outputs equal `v26_b`, `v26_c`, `v26_d1`, `v26_g` where they repeat them.
- [ ] 3.3 Run the two upstream programs that need nothing else (`data\test`, `keyboard\devices_windows`) with
  `--suite compile --qb64 target\release\qb64rust.exe` and add those that pass to `tests\upstream\pass.list`;
  regenerate the shrink-only lists. Update `crates\README.md` and `tests\upstream\README.md` (progress "x of
  279"). Verify: the pass list passes run by itself; the progress number in the README equals the list's length.

## 4. Window built-ins, compiled and never run (design D6)

- [ ] 4.1 Rows for the ten statements and ten functions of `proposal.md`, with the rules of task 1.3; the
  `WINDOW_BUILTINS` list; the template matching extended for parenthesised pairs if 1.3 showed it is needed; the
  icon runtime part collected for `_ICON` (D3). Call-site programs: one per form of each template and per
  argument type class. Tests: `typed` for each function, `ir` for each statement, a `check-fail` test for one
  wrong form of each special-format statement. Verify: the call-site check reports every new program equal;
  `cargo test` passes.
- [ ] 4.2 The coverage rule (`testing/compiler-tests` delta): the built-in coverage test reads `WINDOW_BUILTINS`
  and requires, for each, a slice program that names it and a call-site program; its own negative test (a name
  added to the list with no call-site program fails and is named). Verify: `cargo test` passes, and fails with
  the built-in's name when one call-site program is moved away (tried once by hand, then restored).
- [ ] 4.3 Record slice program `s66_window_builtins` with `qb64pe.exe` (the shape of the upstream `glut` tests:
  `$CONSOLE:ONLY`, a `PRINT`, `SYSTEM`, then `$CONSOLE`, `_DEST _CONSOLE` and every window built-in in every form)
  and add it to `slice.list`. Run the 19 upstream `glut` tests and add those that pass to
  `tests\upstream\pass.list`; regenerate the shrink-only lists; update the two READMEs. Verify: tier 2 slice
  passes for `s66` with and without folding; the pass list passes run by itself; each `glut` test that does not
  pass is named here with its reason.

## 5. Constants of the auto-included file (design D8)

- [ ] 5.1 `tools\builtins\extract_builtins.py`: per constant of `beforefirstline.bi`, its name with suffix and the
  text of its expression, from the Windows 64-bit branches; `builtins.json` regenerated; the generated table of
  `crates\builtins` carries the field. Tests: the extractor's output is unchanged for everything else (diff of
  `builtins.json` reviewed: only `auto_include` entries gain fields); a unit test in `crates\builtins` pins
  `_TRUE`, `_FALSE`, `_SIZE_OF_OFFSET` and one constant that names another. Verify: `cargo test -p
  qb64rust-builtins` passes.
- [ ] 5.2 `sema`: a constant of the file resolves on first use through the `CONST` evaluator, in file order, cached,
  global; one the evaluator cannot compute keeps its mark with the reason; the `$COLOR` names and the three
  functions keep theirs. Tests: `typed` for `_TRUE`, a constant with a suffix and one computed from others, used
  in the main module and in a SUB; `check-fail` for `_STR_CRLF` (marked) and for `CONST _TRUE = 1` (still the
  reserved-name error). Verify: `cargo test` passes; a program that names no such constant has an unchanged
  `typed` snapshot.
- [ ] 5.3 Record slice program `s67_auto_constants` with `qb64pe.exe`: it prints every constant 5.2 computes, and
  `LEN` of one per type suffix; add it to `slice.list`. Count again which corpus and upstream programs lost their
  last mark (`qb64rust --report unsupported`), run them and add those that pass to their pass lists; regenerate
  the shrink-only lists; update `crates\README.md`. Verify: tier 2 slice passes for `s67` with and without
  folding; a constant whose value differed is fixed or left marked and named here.

## 6. Precompiler flags (design D7)

- [ ] 6.1 `syntax`: `PpFlags` as an input of `parse`; the settings met reported in file order; the running value of
  `_CONSOLE_` and the marked corner; `parse_settled` with its cap; a round repeated only when a condition named a
  flag. The driver, the language server and the tests call `parse_settled`. Tests: unit tests of the state
  machine (set once; set to 2 then 1; a locked flag; the cap); front-end tests in the modes they need for every
  scenario of the "Precompiler flags" requirement (`v26_h_flag_console`, `v26_h2_flag_both`, `x26`, `x27`, `x28`,
  `x33`, `x38`, `x39`, and what 1.4 added). Verify: `cargo test` passes; the mutation test and the parse-gap list are
  unchanged; tier 1's time is within 10% of before (noted here).
- [ ] 6.2 `sema`: the "precompiler flag in `$IF`" mark is gone except for the corner; the program's console
  comes from the settled `_CONSOLE_` flag as well as from the walk (D1; a front-end `ir` test with the program
  of `v26_x26` shows a console in its `settings` line and no mark); `$ASSERTS` and `$DEBUG` stay
  marked as metacommands. Record slice program `s68_precompiler_flags` with `qb64pe.exe` (the scenarios above
  that run) and add it to `slice.list`. Run the seven upstream `precomp-flags` tests and add those that pass to
  `tests\upstream\pass.list`; regenerate the shrink-only lists; update the READMEs and the comment at the top of
  `syntax\pp.rs`. Verify: tier 2 slice passes for `s68` with and without folding; the pass list passes run by
  itself.

## 7. Integration

- [ ] 7.1 Full runs with the release build: `cargo test`; the call-site check; tier 2 slice with and without
  folding; the upstream pass list; the differential pass list; `--suite compile` and `--suite corpus` in full
  with the new compiler, to see that no program outside the lists changed its result. Verify: every run passes,
  and the numbers are written into `STATUS.md` "Numbers at the last full runs".
- [ ] 7.2 Documents: `STATUS.md` (step 11 done, what is next), `ROADMAP.md` (M2's last box ticked, and whether
  M2's exit criterion is met), `crates\README.md`, `study\00` §5 and its document map, `CLAUDE.md` (the
  `verification\` row, the corpus and slice counts of the layout table), `SOMEDAY.md` (what stayed marked:
  the constants that need `CHR$` or `EXP`, the corner of D7, window built-ins not run), `DECISIONS.md` (anything
  decided on the way). Verify: every count in the layout table equals the files on disk; `repo_check` is clean.
- [ ] 7.3 After the user pushes: read the three workflows on GitHub. Verify: all green; then the change is ready
  to archive (`/opsx:archive`), which syncs the deltas into `openspec\specs`.
