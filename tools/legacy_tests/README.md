# Legacy test runner

`run_legacy_tests.py` runs the QB64pe test suites on Windows without bash. It re-implements
`tests/compile_tests.sh` and `tests/qbasic_tests.sh` from the reference clone (`..\..\..\QB64pe`).
It drives any compiler that accepts the qb64pe command line, so it measures the old compiler now
and the new one later.

```
python tools\legacy_tests\run_legacy_tests.py                          # compile_tests, old compiler
python tools\legacy_tests\run_legacy_tests.py --suite all              # + qbasic_testcases (compile only)
python tools\legacy_tests\run_legacy_tests.py --category print         # one category
python tools\legacy_tests\run_legacy_tests.py --qb64 path\to\new.exe   # another compiler
```

Results (executables, compiler output, run output, `results.json`) go to `target\legacy-tests\`
(ignored by git).

Notes:
- Tests run one at a time: the compiler builds in the shared `QB64pe\internal\temp`, which the
  runner empties before each compile. The reference clone is written to only in that folder
  (git-ignored) and by the compiler's cached `.o` files.
- Deliberate differences from the bash runner: `.err` comparison ignores CR characters; `.output`
  comparison treats CRLF as LF (bare CR still counts); each compile and run has a timeout.
