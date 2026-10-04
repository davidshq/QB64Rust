# Legacy test runner

`run_legacy_tests.py` runs the QB64pe test suites on Windows without bash. It re-implements
`tests/compile_tests.sh`, `tests/qbasic_tests.sh` and `tests/format_tests.sh` from the reference clone
(`..\..\..\QB64pe`). It drives any compiler that accepts the qb64pe command line, so it measures the old
compiler now and the new one later.

```
python tools\legacy_tests\run_legacy_tests.py                          # compile_tests, old compiler
python tools\legacy_tests\run_legacy_tests.py --suite format           # format_tests (-y), about 10 s
python tools\legacy_tests\run_legacy_tests.py --suite all              # compile + qbasic (compile only) + format + corpus
python tools\legacy_tests\run_legacy_tests.py --category print         # one category
python tools\legacy_tests\run_legacy_tests.py --qb64 path\to\new.exe   # another compiler
python tools\legacy_tests\run_legacy_tests.py --suite corpus           # golden corpus (tests\corpus of this repo)
```

### Golden corpus (`--suite corpus`)

Checks a compiler against `tests\corpus\<group>\` (`--corpus-root` for another folder; `--category <group>`,
`--glob` and `--list` narrow the run). Each `<name>.bas` has exactly one of `<name>.output` (compile, run, compare the
merged stdout and stderr), `<name>.err` (the compile must fail with this output) or `<name>.norun` (compile only,
never run; the file says why). Unlike the compile suite, each program is copied into a fresh folder
`target\legacy-tests\corpus\<group>-<name>\`, compiled there with `-q -m -x` only (no `-O2`), and run there with
no arguments, `QB64PE_NOPROMPT=y` and a 60 s timeout. The folder is deleted when the test passes and kept when it
fails.

```
python tools\legacy_tests\run_legacy_tests.py --suite corpus --record                  # write .output / .err
python tools\legacy_tests\run_legacy_tests.py --suite corpus --record --glob "0*.bas"  # record a few
python tools\legacy_tests\run_legacy_tests.py --suite corpus --cpp-opt                 # -O2 build, same files
```

- `--record` compiles each program once, runs it twice and writes `<name>.output` (or `<name>.err` if the compile
  fails) only if both runs agree. A program whose runs differ is reported as non-deterministic and nothing is
  written; give it a `.normalize` sidecar. A changed expected file is reported as `warning: ... changed`.
- `<name>.normalize`: one rule per line, a Python regular expression, a tab, a replacement (`#` lines are
  comments), applied in order to each output line before comparing and before recording. Keep rules anchored and
  narrow, e.g. `^timer: *[0-9.]+ *$` → `timer: <TIMER>` (`PRINT` puts a space after a number).
- A program listed in `known_failures.txt` that has no expected file (it hangs or loops forever, so nothing can
  be recorded) is compiled but never run, and reported as KFAIL `not run`. To try it again, remove the entry and
  record it.
- `--list <file>` runs only the programs named in the file: one `<group>/<name>` per line (no `.bas`), `#`
  starts a comment. A name with no program is an error. Combines with `--category` and `--glob` (both must match).
  `tests\corpus\slice.list` names the programs the new compiler's first slice must pass.
- `--cpp-opt` adds `-f:OptimizeCppProgram=true`; its failures list the programs whose behaviour depends on the
  C++ optimiser (`tests\corpus\README.md`). It cannot be combined with `--record`.
- On Windows, `END` in a console program waits for a key from the console ("Press any key to continue"), which
  the null device never sends. The runner therefore starts each program through `press_any_key.py`, which gives
  it a console of its own with no window and keeps pressing Shift until it exits. Every recorded `.output` of a
  program that reaches `END` ends with that line. Only for programs that do not read the keyboard.
- Results: `results-corpus.json`, `results-corpus-record.json`, `results-corpus-cpp-opt.json` (the `-O2` run keeps
  its files in `corpus-cpp-opt\`); a narrowed run writes `results-partial.json`. Known failures are listed as
  `corpus:<group>/<name>`.

Results (executables, compiler output, run output, a JSON summary) go to `target\legacy-tests\` (ignored by
git). The summary is `results.json` for `--suite all`, `results-<suite>.json` for one whole suite, and
`results-partial.json` when `--category`, `--glob` or `--list` narrows the run, so a partial run never overwrites a full one.

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
- Programs run with `QB64PE_NOPROMPT` set (default `y`, or the test's `.noprompt` file; always `y` in the corpus), so runtime errors
  do not open dialogs. A fatal runtime error still exits with code 0; the output comparison catches it.
- Deliberate differences from the bash runners: `.err` and `.license` comparisons ignore CR characters;
  `.output` comparison treats CRLF as LF (bare CR still counts); each compile and run has a timeout, and on a
  timeout the whole process tree is killed (`taskkill /T`); known failures do not fail the run.
- Not re-implemented: `add_prefix_test.sh` (one converter test), `dist_tests.sh` and `run_c_tests.sh`
  (`study\05` §5.4–5.6).
