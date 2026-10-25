# Tasks

Standing rules: CLAUDE.md. What this change does differently: the first full report (group 3) decides how many
programs groups 4 and 5 write, so the delta specs and design D5 and D6 may change after it, before any program is
recorded; every recording with `qb64pe.exe` (screen programs and `v61`) is batched in one recording in 4.1.

## 1. The names in the record (D1, D2)

- [x] 1.1 `sema::builtins::OWN_NODE` (moved from `inputs.rs`); find where `sema` keeps the built-in a span
  resolved to (design, risk of a user name), and the driver's `names` callback by D1 on it or on the table.
  `Coverage::names`, `Counters` keep the union per entry and the innermost rule (`coverage.rs`). Verify: unit
  tests in `codegen-cpp` or `driver` of the spec's two scenarios and of an `IF` block whose body names a built-in
  its condition does not.
- [x] 1.2 The record's version 2 (`Counters::table`); `coverage.py` reads 1 and 2, with a test of each in
  `test_coverage.py`; `tools\coverage\README.md`'s format block. Verify: `python -m unittest discover -s
  tools/coverage -p "test_*.py"`; a CLI test that builds a two-line program with `--coverage` and reads `LEFT$` and
  `LEN` in its record; `coverage.py --check` passes.

## 2. The runner, the list and the report (D3, D4)

- [x] 2.1 `run_legacy_tests.py`: `--coverage` for `--suite verification` and `--suite compile`; its test in
  `test_run_legacy_tests.py`; `tools\legacy_tests\README.md`'s flag line. Verify: the runner's tests; a
  `--suite verification --glob "v13_*" --coverage <file>` run writes one record per run.
- [x] 2.2 `tests\never_run.list` from `NEVER_RUN` with the reasons of D4; `NEVER_RUN` removed from `sema`, its
  doc links pointed to the file; `inputs.rs` reads the file and checks each name is a built-in `sema` compiles
  (the delta's new scenario) and each reason is of the set. Verify: tier 1; a test copy of the list with
  `_NOSUCH` fails with the name.
- [x] 2.3 `tools\coverage\builtin_runs.py` (report and `--check`) and `test_builtin_runs.py` on records
  written by the test, one per scenario of `testing/coverage`'s new requirements; `tools\coverage\README.md`
  section. Verify: `python -m unittest discover -s tools/coverage -p "test_*.py"`.

## 3. The first report

- [x] 3.1 The three full suites with `--coverage target\coverage\builtins.txt` (`--jobs 8 --build-cache
  target\build-cache`), then `builtin_runs.py target\coverage\builtins.txt > target\coverage\builtins-report.txt`.
  Write the names not run beyond the 40 into D6 as a table (name, the programs that hold it, its home by D6) and
  confirm or move each row of D5; the delta specs follow if a rule changes. Verify: `openspec validate
  m4-coverage-review --strict` passes; every name the report gives as not run is in D5 or D6's table.

## 4. Tests for the names that can run (D5, D6)

- [x] 4.1 The screen programs of D5 (`tests\corpus\screen\w*`) and the `v61` family of D6 (none needed: the
  report found no name beyond the 40), written and recorded with `qb64pe.exe` in one batch (screen programs two
  equal runs). Verify: each has its recording; `git -C ..\QB64pe status` is clean.
- [x] 4.2 The new programs pass with the new compiler (`--glob` on them); the screen programs join `slice.list`.
  Each name they run leaves `tests\never_run.list`, and so does a statement-and-function name of which one use
  runs (the list is by name, D5). Verify: tier 1;
  `builtin_runs.py --check` on the counts of 3.1 plus a `--coverage` run of the new programs passes.
- [x] 4.3 `DECISIONS.md`: one row for the change (the list moved to a file, D5's split, the cadence of D7);
  `SOMEDAY.md`: the rows that say a name of this change is compiled and never run are narrowed to the list's
  names. Verify: no row of `SOMEDAY.md` calls a name never run that the report gives as run.

## 5. The cadence (D7)

- [x] 5.1 `CLAUDE.md` working rule 9's merge set and `tools\legacy_tests\README.md` ("The runs at a merge into
  `main`"): the full suites with `--coverage`, then `builtin_runs.py --check`. Verify: the two texts name the same
  commands; `tools\repo_check\check_repo.py` passes.

## 6. Closing

- [x] 6.1 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes (the coverage check of Civil War Strategy runs: `codegen-cpp` and
  `tools\coverage\` changed). Then the counts of the test READMEs by `tools\legacy_tests\counts.py`, and
  `STATUS.md`'s "Next": step 6 done, M4 waits on the merge into `main` and the user's word. Then
  `python tools\testtime\test_time.py`. Verify: `--check` passes.

## 7. After the review (the pragmatic engineer's items 1, 2 and 4)

- [x] 7.1 `builtin_runs.py --check` fails on a built-in of `sema`'s tables that no record names, the tables read
  from `builtins.rs`'s text; tier 1's `builtin_tables_are_one_row_per_line` holds that text to the form; a line
  of spaces in the list is a problem in both readers; design D1 says what was built, D3 the new check. Verify:
  tier 1 with fmt and clippy; `python -m unittest discover -s tools/coverage -p "test_*.py"`;
  `builtin_runs.py --check` on the counts of 3.1 and 4.2 passes. Tier 2 is not repeated: no compiler code,
  recording or list changed.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
- Then the full set at a merge into `main`, with D7's coverage runs for the first time, and the pull request.
