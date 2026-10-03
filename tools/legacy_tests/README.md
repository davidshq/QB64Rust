# Legacy test runner

`run_legacy_tests.py` runs the QB64pe test suites on Windows without bash. It re-implements
`tests/compile_tests.sh`, `tests/qbasic_tests.sh` and `tests/format_tests.sh` from the reference clone
(`..\..\..\QB64pe`). It drives any compiler that accepts the qb64pe command line, so it measures the old
compiler now and the new one later.

```
python tools\legacy_tests\run_legacy_tests.py                          # compile_tests, old compiler
python tools\legacy_tests\run_legacy_tests.py --suite format           # format_tests (-y), about 10 s
python tools\legacy_tests\run_legacy_tests.py --suite all              # compile + qbasic (compile only) + format
python tools\legacy_tests\run_legacy_tests.py --category print         # one category
python tools\legacy_tests\run_legacy_tests.py --qb64 path\to\new.exe   # another compiler
```

Results (executables, compiler output, run output, a JSON summary) go to `target\legacy-tests\` (ignored by
git). The summary is `results.json` for `--suite all`, `results-<suite>.json` for one whole suite, and
`results-partial.json` when `--category` or `--glob` narrows the run, so a partial run never overwrites a full one.

Known failures: `known_failures.txt` lists tests (`<suite>:<test>  <reason>`) whose failure is reported as
`KFAIL` and does not fail the run. Today only `http/read_example` (environment-dependent, see `baselines\`).
If a listed test passes, the runner prints a note to remove the entry.

Exit code: 0 if nothing failed (KFAIL allowed), 1 if a test failed, 2 for a usage error (compiler missing, unknown
category, no tests selected). An exception inside one test is recorded as a FAIL at stage `runner` and the run
continues.

Notes:
- Tests run one at a time: the compiler builds in the shared `QB64pe\internal\temp`, which the
  runner empties before each compile. The reference clone is written to only in that folder
  (git-ignored) and by the compiler's cached `.o` files.
- Programs run with `QB64PE_NOPROMPT` set (default `y`, or the test's `.noprompt` file), so runtime errors
  do not open dialogs. A fatal runtime error still exits with code 0; the output comparison catches it.
- Deliberate differences from the bash runners: `.err` and `.license` comparisons ignore CR characters;
  `.output` comparison treats CRLF as LF (bare CR still counts); each compile and run has a timeout, and on a
  timeout the whole process tree is killed (`taskkill /T`); known failures do not fail the run.
- Not re-implemented: `add_prefix_test.sh` (one converter test), `dist_tests.sh` and `run_c_tests.sh`
  (`study\05` §5.4–5.6).
