# Tasks

Standing rules: CLAUDE.md. What this change does differently: D-012 (groups 1 and 2) comes before the test
command and is closed in the same change; the recordings of `qb64pe.exe` are not re-recorded (design D2).

## 1. D-012 in libqb

- [x] 1.1 `DECISIONS.md`: a row for D-012's value (1), for the runner's rule without sidecars (design D2), and for
  the test command's protocol (design D4), naming `openspec\changes\basic-unit-tests\design.md`. Verify: the rows
  are there.
- [x] 1.2 The edit `tools\libqb\edits\d012-error-exit-code.edit` (design D1): `exit_code = 1` before `end()` in
  `fix_error`'s "no" path, `libqb_exit(1)` in `exit_after_fatal_error`, and the trace of `D-012` from libqb.
  Also `libqb\SOURCE.md`'s list, `copy_libqb.py` run, and an `UPSTREAM.md` section. Verify: `python
  tools\libqb\copy_libqb.py --check` passes, and the probe of 2026-10-29 (`ERROR 5` under `QB64PE_NOPROMPT=y`)
  exits with 1.
- [x] 1.3 CLI tests in `crates\driver\tests\cli.rs` for the scenarios of `language/error-handling` "The exit code
  of a program ended by an error": no handler, `_ASSERT` (critical), `continue`, a trapped error, the trace. Update
  `an_error_that_ends_the_program_traces_nothing` and every other CLI test that expects success or exit 0 after
  an untrapped error (found by `cargo test -p qb64rust-driver --test cli`). Verify: `cargo test -p
  qb64rust-driver --test cli` passes, the run logged to `target\t-cli.log`.
- [x] 1.4 `QB64PE-DIVERGENCES.md` D-012: the new value, "Traced" in the trace paragraph, its tests in the last
  column; `vscode\CHANGELOG.md`'s unreleased section has a line on the exit code. Verify: the row names the CLI
  tests of 1.3.

## 2. The runner's D-012 rule

- [x] 2.1 `tools\legacy_tests\run_legacy_tests.py` (design D2): in the corpus run, the compile suite and the
  verification compare, a trace with `D-012` turns a wanted 0 into 1, and `D-012` needs no sidecar. A record run
  with `qb64pe.exe` is unchanged. Tests in `test_run_legacy_tests.py` for the four scenarios of
  `testing/golden-corpus` "The exit code of D-012"; `tools\legacy_tests\README.md` gets the rule's paragraph.
  Verify: `python -m pytest tools\legacy_tests\test_run_legacy_tests.py` passes.
- [x] 2.2 The programs D-012 reaches pass. These are the 133 verification programs ending in `Runtime error`,
  the corpus and upstream programs that print one (`--glob` by the list a script under `target\` makes from
  `grep -l "Runtime error"`), and `noprompt\noprompt-y`. Verify: each run passes or `KFAIL`s as before this
  change, and none fails at the exit code.
- [x] 2.3 Checkpoint before the test command (pragmatic engineer on the proposal, 2026-10-29): D-012 is checked on
  its own, so that a failure at the close cannot come from either half. Run `cargo test -p qb64rust-driver --test
  cli`, the runner's tests and 2.2's programs once more, if anything changed since they passed, and a slice list
  run with `--pass-cache target\pass-cache`. Verify: all pass, logged under `target\`. Group 3 starts only after.

## 3. The compile path shared

- [x] 3.1 Move the `-x` compile of `run()` in `crates\driver\src\main.rs` into `crates\driver\src\compile.rs`
  (`compile_exe`, design D3), taking the main file's path, its bytes and its loader. `run()` calls it. Code is
  moved, not changed. If `run()` will not move without changes to how it parses options or prints, stop and tell
  the user, with the fallback: spawn `qb64rust -x` on an entry point in a temp folder, which loses `$EMBED` and
  `DECLARE LIBRARY` paths relative to the test file. Verify: `cargo test -p qb64rust-driver` passes, logged to
  `target\t-driver.log`.

## 4. The test command

- [x] 4.1 The library `crates\driver\src\unit_tests\qbtest.bi` and `qbtest.bm` (spec "The assertion library",
  design D4): explicit types and `qbtest_` names throughout, and the result file opened and closed at each write.
  Verify: built through 4.3's entry point in 4.5's tests, including the `DEFSTR A-Z` + `OPTION _EXPLICIT` scenario.
- [x] 4.2 Discovery `unit_tests\discover.rs` (design D5), with unit tests for the scenarios of "Test files and
  test SUBs": names in any case, parameters, `FUNCTION`, a SUB of an included file, a comment, a string; plus the
  file search of a folder (`*_test.bas`, any case). Verify: `cargo test -p qb64rust-driver --lib unit_tests`
  passes.
- [x] 4.3 The entry point `unit_tests\entry.rs` and `unit_tests\loader.rs` (`TestLoader`, the virtual main file
  beside the test file, design D3 and D4). Unit test: the generated text of a fixture is pinned. Verify: `cargo
  test -p qb64rust-driver --lib unit_tests` passes.
- [x] 4.4 Running and judging: `unit_tests\run.rs` and `judge.rs` (design D4, D6; a timeout kills the test's
  process only). `judge.rs` gets unit tests for each row of the order in D4, from result file, stderr and exit
  code given as data. Verify: `cargo test -p qb64rust-driver --lib unit_tests` passes.
- [x] 4.5 The subcommand `qb64rust test` in `main.rs` (`--list`, `--filter`, `--jobs`, `--timeout`, `--format`,
  the build options of spec "Build options of a test run"), `unit_tests\report.rs` (text and JSON lines, design
  D6), and `json_string` moved to `crates\base`. The usage text names `test`. CLI tests in a new
  `crates\driver\tests\unit_tests.rs`, one per scenario of `compiler/unit-tests`, on fixtures under
  `tests\unit_tests\`. The test file and its sidecars are in `tests\unit_tests\README.md`. Also measure which
  image functions fail under the entry point's `$CONSOLE:ONLY` (`_NEWIMAGE`, `_DEST` and `_SOURCE` of an image,
  `PSET` and `_RGB32` into it, `_FREEIMAGE`), with a fixture test per function. Write what fails into design.md's
  "Graphics" non-goal, and add a CLI test that pins one such ERROR. Verify: `cargo test -p qb64rust-driver --test
  unit_tests` passes, logged to `target\t-unit.log`.
- [x] 4.6 The check against the old compiler (design D8): an `#[ignore]`d test in `unit_tests.rs` that builds
  4.5's fixture with `qb64pe.exe` and compares the result-file lines. Verify: `cargo test -p qb64rust-driver
  --test unit_tests -- --ignored` passes with the clone present.
- [x] 4.7 `SOMEDAY.md` rows for the Non-Goals of the design (a program's SUBs without its main module, graphics
  tests, debugging a test, coverage per test, a pass cache of test builds, killing a timed-out test's children
  on Windows and Linux). `crates\README.md`'s test commands name `qb64rust test`. `LAYOUT.md` rows for
  `crates\driver\src\unit_tests\` and `tests\unit_tests\`. Verify: the rows are there.

## 5. Test Explorer

- [x] 5.1 `vscode\src\compiler\testOutput.ts` (JSON lines to results, filters from items) with unit tests in
  `vscode\test\unit\testOutput.test.ts`. Verify: `npm run test:unit` in `vscode\` passes.
- [x] 5.2 `vscode\src\testing.ts` and its wiring in `extension.ts` (design D7): discovery, refresh on save,
  create and delete, the run profile, cancel, the missing-binary message. Verify: lint passes and an integration
  test in `vscode\test\integration\suite` lists and runs a fixture with one passing and one failing test
  (skipped without `QB64RUST_TEST_QB64RUST`, as the others are).
- [x] 5.3 `vscode\README.md` gets a section on tests, and `vscode\CHANGELOG.md`'s unreleased section gets a
  line. Verify: the section names `*_test.bas`, a test SUB, the assertions, and that tests are for console-only
  code, with 4.5's list of image functions that fail.

## 6. Closing

- [x] 6.1 `openspec validate basic-unit-tests --strict` passes. Verify: it passes.
- [ ] 6.2 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then `--check`
  passes. `STATUS.md`'s "Next" gets the change. Verify: `--check` passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
