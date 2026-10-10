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
no arguments, `QB64PE_NOPROMPT=y` (or the contents of `<name>.noprompt`) and a 60 s timeout. The folder is
deleted when the test passes and kept when it fails.

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
- `<name>.stdin`: the program's standard input is a copy of this file, byte for byte, when checking and when
  recording, with either compiler (on Windows `press_any_key.py` passes it on as its second argument). Without
  the file a program runs as before (no input). Such a program must end with `SYSTEM`: with standard input from a
  file, `END`'s "Press any key to continue" never returns.
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

### Verification programs (`--suite verification`)

Records the programs of `verification\` (`--verification-root` for another folder) with the old compiler; it
compares nothing. `verification\run.sh [name ...]` starts it, so both spellings do the same.

```
python tools\legacy_tests\run_legacy_tests.py --suite verification v01_numeric v02_errors   # these two
python tools\legacy_tests\run_legacy_tests.py --suite verification --glob "v22_*.bas"       # a group
python tools\legacy_tests\run_legacy_tests.py --suite verification                          # all of them
```

- Each program is compiled where it is (`qb64pe -x -q -m`, working folder `verification\`), so its included
  files are found; `<name>.exe` is written beside it (ignored by git) and runs there with a 60 s timeout.
- Written: `<name>.compile.txt` (the compiler's output, then `exit=<code>`) and `<name>.out.txt` (the program's
  stdout and stderr, `(timed out after 60 s)` after a timeout, then `exit=<code>`; or `(not compiled)`). The
  `.out.txt` text is printed too. A file is reported as `recorded new` or `warning: ... changed`, and left
  untouched when it differs only by CRLF against LF: `.gitattributes` marks both kinds `text=auto`, so git
  stores text with LF on every machine. A file git takes for binary (a NUL, a CR without LF; six today) is
  stored and compared byte for byte.
- Exit code: 0 when every program was recorded, whatever it did (a rejected or crashing program is a result);
  1 when a compile timed out and nothing was recorded for it. `run.sh` passes the code on and needs `python`
  on the path.
- The folder's full path, which the compiler prints when it names an included file, is written as
  `<verification>` in both files (with either slash and in any letter case), so no local path is recorded.
  Program names and `--glob` cannot be combined.
- `<name>.stdin` and `<name>.noprompt` work as in the corpus. Unlike the corpus there is no `press_any_key.py`:
  a program that reaches `END` on Windows waits for a key and is recorded as timed out, so end with `SYSTEM`.
- At the timeout the program gets Ctrl+Break and 5 s before it is killed, so what it printed last (a prompt
  without a line end) is in the file, as it was under the shell's `timeout`.
- `exit=` holds the code as Git Bash reports it, since the files were first recorded by a shell script: the low
  byte of an ordinary code, 124 for a timeout, 139 for an access violation, 127 for most other crashes
  (`bash_exit_code` in the runner).
- `qb64pe` only, one program at a time, in the reference clone: these files are recorded truth, like
  `--record`. `--suite all` does not include it. A run of named programs or with `--glob` writes
  `results-partial.json`; a full one `results-verification.json`.

Results (executables, compiler output, run output, a JSON summary) go to `target\legacy-tests\` (ignored by
git). The summary is `results.json` for `--suite all`, `results-<suite>.json` for one whole suite, and
`results-partial.json` when `--category`, `--glob`, `--list` or program names narrow the run, so a partial run never
overwrites a full one.

Known failures: `known_failures.txt` lists tests (`<suite>:<test>  <reason>`) whose failure is reported as
`KFAIL` and does not fail the run. Today only `http/read_example` (environment-dependent, see `baselines\`).
If a listed test passes, the runner prints a note to remove the entry.

Exit code: 0 if nothing failed (KFAIL allowed), 1 if a test failed (or, with `--jobs` and `qb64pe`, passed only
when run again alone, or the copies for `--jobs` could not be made), 2 for a usage error (compiler missing, unknown category, no tests selected), 3 when
another run holds the results folder, the clone or the copies (Notes; `--wait` waits instead). Usage errors are
reported before any lock is taken, so `--wait` never waits for a command that cannot run. An exception inside
one test is recorded as a FAIL at stage `runner` and the run continues.

Notes:
- By default tests run one at a time: `qb64pe` builds in the shared `QB64pe\internal\temp`, which the
  runner empties before each compile. The reference clone is written to only in that folder
  (git-ignored) and by the compiler's cached `.o` files.
- **One run at a time per results folder.** A corpus program is built and run in
  `<results>\corpus\<its key>`, which the runner removes first, so two runs there would delete each other's
  folders and run each other's executables; the other suites write each test's files into their folder under
  names made from the test's key. Every run takes a lock on `in-use.lock` in each suite's results folder
  (`target\legacy-tests\<suite>`, `corpus-cpp-opt` with `--cpp-opt`) before anything else; a second run of
  the same suite stops at once with exit code 3, or waits with `--wait`. Runs of different suites, or with
  their own `--results`, do not meet here. The summary file is not locked: two narrowed runs with one
  `--results` both write `results-partial.json` when they end, and the later one stays. Give one of them its
  own `--results` when both summaries are wanted.
- **One run at a time builds in the clone** (`clone_lock.py`). This runner (so `verification\run.sh` and
  tier 2 too) and `tools\callsite\callsite.py` take a lock on `target\qb64pe-clone\in-use.lock` first; a
  second one started meanwhile stops at once with exit code 3, or with `--wait` prints that it is waiting and
  starts when the other has ended. The system drops the lock when the run ends, however it ends.
  - With `qb64pe`: two runs would empty and read each other's `internal\temp`.
  - With `qb64rust` (any `--jobs`): its C++ and executable are outside the clone, but its `make` runs in the
    clone, reads `internal\temp` and builds the libqb objects the clone lacks.
  - The one exception is `--jobs` with `qb64pe` (next point): it builds in its copies and can run beside
    another run that uses another results folder.
  - Not covered: `qb64rust` started without this runner (by hand, or `cargo test -p qb64rust-driver --test
    cli -- --ignored`).
  - Not covered: runs from two checkouts (or git worktrees) of this repo that use one clone. The lock file
    is under each checkout's own `target\` (the clone is a read-only reference and gets no lock file). Use
    one checkout per clone.
- **`--jobs N` with `qb64pe`** (corpus suite only, not with `--record`) runs N programs at a time. The old
  compiler's answer is what gets recorded as truth, so nothing is shared between workers: each one compiles
  with a private copy of the compiler, `target\qb64pe-copies\w1` … `wN` (`--copies-dir` for another place).
  - A copy is `qb64pe.exe`, the `Makefile`, `internal\` and `settings\`: about 800 MB, 737 MB of it the C++
    compiler in `internal\c\c_compiler`. It takes nothing that a build writes (`internal\temp*`, the libqb
    objects, `qbx<N>.cpp`, `qbx*.o`, `parts\core\gl_header_for_parsing\temp`): such a file may change or
    vanish while another run builds in the clone. Each copy builds its own libqb objects with its first
    programs, so the first run after the copies are made takes about 3 minutes longer.
  - A copy compiles one program at a time, with its `internal\temp` emptied first: the same steps as the
    clone with one job.
  - A copy is made again when the clone's `qb64pe.exe`, `Makefile` or a file of `internal\c` differs (by size
    or time) from what the copy was made from, or when a file of `settings\` differs by content
    (`config.ini` holds options a compile follows, such as `OptimizeCppProgram` and `ExtraCppFlags`). Files
    a build writes in `internal\c` do not count. `cargo clean` removes the copies with the rest of `target\`;
    the next run makes them again.
  - A program that fails runs again alone in the clone, after the others. Both outcomes are printed
    (`[again]`). A program that passes only alone is counted as passed, gets a `note:` at the end, and makes
    the exit code 1: the copies gave another answer than the clone, so find out why before trusting the
    parallel run.
  - `--record` and one job use the clone itself.
  - One run at a time per folder of copies: the run holds a lock on `in-use.lock` there, and a second run
    started meanwhile stops at once with exit code 3 (`--wait` waits instead; its own `--copies-dir` and
    `--results` let both run). A `--jobs` run can go on beside a one-job run of another suite, of
    `--cpp-opt`, or with its own `--results` (not beside one in the same results folder: see above). It takes
    the clone's lock only to run a failed program again, and waits for it then.
  - Measured 2026-10-10 (16 logical processors, 305 programs): five runs with `--jobs 8`, each equal test by
    test to a run with one job on the same day (300 pass, the 5 known failures). One job: 11 min 32 s.
    `--jobs 8`: 3 min 44 s to 3 min 47 s; 6 min 37 s for the run that made the copies and built their libqb.
    Each C++ build takes about twice as long beside seven others, so eight workers give a factor of three.
  - Why not several instances in one clone: `qb64pe` gives a second instance its own `internal\temp2` and
    `qbx2.cpp`, but all instances share the libqb objects under `internal\c` and one generated header
    (`gl_helper_code.h`). Not tried (`DECISIONS.md` 2026-10-10).
- Programs run with `QB64PE_NOPROMPT` set (default `y`, or the test's `.noprompt` file, in both suites), so
  runtime errors do not open dialogs. A fatal runtime error still exits with code 0; the output comparison catches it.
- Deliberate differences from the bash runners: `.err` and `.license` comparisons ignore CR characters;
  `.output` comparison treats CRLF as LF (bare CR still counts); each compile and run has a timeout, and on a
  timeout the whole process tree is killed (`taskkill /T`); known failures do not fail the run.
- Not re-implemented: `add_prefix_test.sh` (one converter test), `dist_tests.sh` and `run_c_tests.sh`
  (`study\05` §5.4–5.6).
