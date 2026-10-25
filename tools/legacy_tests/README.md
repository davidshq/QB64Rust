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
- `<name>.exitcode` (beside an `.output`): the exit code the run must end with, in decimal as the process
  reports it (`END n`, `SYSTEM n`; `4294967295` for -1 on Windows); without it the code must be 0. `--record`
  writes it when the two runs end with the same code that is not 0, and removes it when the code is 0
  (`m3-asserts-debug-zlib`, design D7).
- `<name>.stdin`: the program's standard input is a copy of this file, byte for byte, when checking and when
  recording, with either compiler (on Windows `press_any_key.py` passes it on as its second argument). Without
  the file a program runs as before (no input). Such a program must end with `SYSTEM`: with standard input from a
  file, `END`'s "Press any key to continue" never returns.
- `<name>.screen`, beside an `.output`: a **screen program**, checked by the state of the screen it leaves as well
  as by its output (`tools\screen\README.md`). It is built with `tools\screen\screen_state.cpp` on the link line
  (`-f:ExtraLinkerFlags=`, with either compiler) and run with `QB64_SCREEN_STATE` naming
  `<results>\corpus\<key>-run<N>-screen.txt`. That file must equal `.screen` (CRLF as LF); a run that writes none
  fails at stage `screen`, and the state written stays in the results folder for a look
  (`tools\screen\state_to_png.py`). `<name>.keys`, when it exists, is copied to the results folder and given as
  `QB64_SCREEN_KEYS`: the keys the program reads. `--record` writes `.screen` only if both runs wrote a state and
  the two are equal. To add a screen program, create an empty `<name>.screen` and record it. A `.screen` beside an
  `.err` or `.norun` is an error in the test, as is a `.keys` without a `.screen`, and a screen program that does
  not compile is never recorded as an `.err`. The path of this repo must hold no blank (`make` splits the flag).
  On Windows a screen program runs on a desktop of its own that is never shown (`press_any_key.py
  --hidden-desktop`), so its window does not appear and the user's keys and mouse cannot reach it;
  `QB64RUST_SHOW_WINDOWS=1` shows the windows again (`tools\screen\README.md`, "Where the window goes").
- **Scripts of a screen program** (`cws-whole-game`, design D3): `<name>.<script>.keys` beside a screen program is a
  test of its own, `<group>/<name>.<script>`, judged by its own `<name>.<script>.screen`, `.output` and other
  sidecars as a screen program is, and run from `<name>.bas` with `<name>.files`. The tests of one program share
  one build in a run (`<results>\corpus\<group>-<name>-build\`). `--glob` matches the test name
  (`--glob "g12_game*.bas"` takes the program and all its scripts), and `--list`, `--record`, `--pass-cache` and
  `--check-list` name it as `screen/g12_game.union_time`. A script without its `.screen` fails at stage
  `expected`; `--record` writes it (no empty file needed).
- `--full-states <group>/<name>[.<script>]`: that test's `digest` steps write full states
  (`QB64_SCREEN_DIGEST_AS_STATE=1`) into its `-run1-screen.txt` in the results folder, to look at with
  `state_to_png.py` when a digest differs; the test then fails at stage `screen`, its states not compared.
- `--coverage <file>` (qb64rust, `--suite corpus`, `verification` or `compile`, not with `--record` or
  `--pass-cache`): each program is built with `--coverage`, and each run's counts are appended to `<file>`
  (`tools\coverage\README.md`). **At a merge into `main` the three full suites run with it**, each into its own
  file (`--coverage target\coverage\merge\corpus.txt`, `upstream.txt`, `verification.txt`), and
  `python tools\coverage\builtin_runs.py --check target\coverage\merge\*.txt` reads them: the built-ins no test
  runs must be those of `tests\never_run.list` (`CLAUDE.md` working rule 9; OpenSpec change
  `m4-coverage-review`, design D7), and the pass lists are judged from them (`--check-list`): the counters
  change no value. A run with `--full-states` cannot stand for a run of a pass list.
- `<name>.files`, a folder beside a program that runs: the data files it reads (a picture loaded with `BLOAD`).
  Everything in it, folders too, is copied into the program's folder before the compile and again before each
  run (the folder is emptied between the two runs of `--record`, and a run may have changed a file), with either
  compiler. Beside an `.err` or a `.norun` it is an error in the test. `.gitattributes` stores the files byte
  for byte.
- **The run-time trace of divergences** (`m3-divergence-trace`; corpus and compile suites): every run has
  `QB64RUST_DIVERGENCES` naming a fresh file `<results>\<key>-run<n>-divergences.txt`, where a program built by
  the new compiler writes `D-0nn` the first time it takes the path of that row of `DIVERGENCES.md` (the old
  compiler's programs write nothing). When checking, a row that `<name>.divergences` (one `D-0nn` per line, `#`
  starts a comment) does not list fails the program at the stage `divergences`; a listed row not taken does not.
  A corpus program's sidecar lies beside it; a compile-suite program's is
  `tests\upstream\divergences\<category>\<name>.divergences`, since its own folder is the clone's or the copy that
  `tools\upstream\copy_upstream_tests.py` remakes.
  A compare of the verification suite sets it too, with the sidecar
  `tests\verification\divergences\<name>.divergences`; its recording does not.
- A program listed in `known_failures.txt` that has no expected file (it hangs or loops forever, so nothing can
  be recorded) is compiled but never run, and reported as KFAIL `not run`. To try it again, remove the entry and
  record it.
- `--list <file>` runs only the programs named in the file: one `<group>/<name>` per line (no `.bas`), `#`
  starts a comment. A name with no program is an error. Combines with `--category` and `--glob` (both must match).
  `tests\corpus\slice.list` names the programs the new compiler's first slice must pass.
- `--pass-cache <folder>` (corpus suite, not with `--record`): a program that passed is remembered by a hash of
  its executable, its source and every file its run and comparison read (`.output`, `.screen`, `.keys`, `.stdin`,
  `.normalize`, `.exitcode`, `.noprompt`, `.divergences`, and each file of `.files` by its path and bytes). The next run with the option still compiles and builds it, and when the hash is
  the same reports `PASS` with "not run: passed before …" without running it; a last line says how many were
  not run. With `--build-cache`, which gives an unchanged program a byte-identical executable, an edit to the
  compiler then runs only the programs whose C++ it changed: the 342 of `slice.list` take 24 s when nothing
  changed, against 92 s (`--jobs 8`, 2026-10-12). A failure is never remembered. A change's close runs the slice
  and upstream pass lists with it; **the runs at a merge into `main` go without it** (`CLAUDE.md` working rule
  9): it cannot see a result that depends on time or load, which is how `m10_screen_10` was found. With
  `qb64pe` it does no harm and saves nothing (its builds differ byte for byte). The folder is safe to delete.
- `--cpp-opt` adds `-f:OptimizeCppProgram=true`; its failures list the programs whose behaviour depends on the
  C++ optimiser (`tests\corpus\README.md`). It cannot be combined with `--record`.
- On Windows, `END` in a console program waits for a key from the console ("Press any key to continue"), which
  the null device never sends. The runner therefore starts each program through `press_any_key.py`, which gives
  it a console of its own with no window and keeps pressing Shift until it exits. Every recorded `.output` of a
  program that reaches `END` ends with that line. Only for programs that do not read the keyboard.
- Results: `results-corpus.json`, `results-corpus-record.json`, `results-corpus-cpp-opt.json` (the `-O2` run keeps
  its files in `corpus-cpp-opt\`); a narrowed run writes `results-partial.json`. Known failures are listed as
  `corpus:<group>/<name>`.

### Tests of the runner

```
python -m unittest discover -s tools/legacy_tests -p "test_*.py"
```

`test_run_legacy_tests.py` (standard library only, under a second, no compiler and no clone; also in CI, job
`test` of `rust.yml`). The runner judges tier 2 and records every expected output, so the rules that decide what
"pass" means and what gets recorded are pinned there:

- the pure rules: `norm_output` (CRLF as LF, a bare CR counts, trailing line ends dropped), `norm_text`,
  `new_compiler_rejection` (a real error is a rejection; only marked errors, an internal compiler error or a
  failed C++ build are not), `.normalize` files, `bash_exit_code`, `write_recorded`, `write_expected`, the
  `--list` and known-failures files, what a copy of the clone leaves out;
- every outcome of one corpus program, with `run_proc` (through which every process is started) replaced by a
  stand-in for the compiler and for the program: equal and different output, each kind of `.err` result with the
  old and the new compiler, `.norun`, `.stdin`, `.normalize`, a known failure without an expected file, a failed
  or timed-out run, `--record` with equal runs, with differing runs and with a rejection, and the data files of
  `<name>.files` copied for a run and for a recording, and refused beside `.err` or `.norun`;
- `--pass-cache`: a program that passed is compiled and not run; another executable, source, expected file,
  input file, screen state or data file (changed, added, renamed, gone) runs it; a failure and a `--record` run
  are not remembered;
- every outcome of one screen program, the stand-in writing the state file the environment names: equal and
  different state, no state, a state left by an earlier run, the keys file passed on, `.screen` beside `.err` or
  `.norun`, `.keys` without `.screen`, and `--record` with equal states, differing states and a run without one;
- every rule of the verification compare (spec `testing/verification-suite`): a rejection by both, one with only
  marked errors, a build where the old compiler rejected, equal and different output, a different exit code, a
  timeout on both sides, nothing recorded, the folder mask, the support files and sidecars copied, the run out of
  sight without key presses, the verification folder left unwritten, a `.normalize` sidecar, a divergence not
  listed, a `v37` program without its libraries, `--pass-cache`, and the recorder still used with `qb64pe`; and
  `verification_text`, the family rule of the support files, `known.list`, `--check-list` and the inputs of a
  verification program; `press_any_key.py --no-keys` with a stand-in that waits for a key and times out.

**A change to one of these rules comes with a change to its test** (spec `testing/golden-corpus`). Not covered: the
locks and the private copies of `qb64pe` (they need the clone; the full runs cover them), and the functions of
the compile, qbasic and format suites that are not shared with the corpus suite.

### Verification programs (`--suite verification`)

With `qb64pe` it records the programs of `verification\` (`--verification-root` for another folder); with any
other compiler it compares that compiler against the recordings (below, change `verification-compare`).
`verification\run.sh [name ...]` starts the recording, so both spellings do the same.

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
  stores text with LF on every machine. A file git takes for binary (a NUL, a CR without LF; eight today) is
  stored and compared byte for byte.
- The program's log: the run's runtime log goes to a file (`QB64PE_LOG_HANDLERS=file`). When it holds entries
  of scope `QB64` (the program's own, `_LOGINFO` and the rest), they are written to `<name>.log.txt` with the
  stack frames that follow them, without each entry's timestamp and each frame's address, the folder masked,
  with LF line ends (`program_log`). A program that logged none gets no file, and a stale one is removed.
- Exit code: 0 when every program was recorded, whatever it did (a rejected or crashing program is a result);
  1 when a compile timed out and nothing was recorded for it. `run.sh` passes the code on and needs `python`
  on the path.
- The folder's full path, which the compiler prints when it names an included file, is written as
  `<verification>` in both files (with either slash and in any letter case), so no local path is recorded.
  Program names and `--glob` cannot be combined.
- `<name>.stdin` and `<name>.noprompt` work as in the corpus. Unlike the corpus there is no `press_any_key.py`:
  a program that reaches `END` on Windows waits for a key and is recorded as timed out, so end with `SYSTEM`.
- `<name>.root` lists what the program finds in the compiler's own folder, one path relative to `verification\`
  per line. A file (`$EXEICON`, `$EMBED`) is copied to the same path in the compiler's folder for the compile and
  removed after, with the folders it made; a file already there, or a listed file missing from `verification\`,
  fails the program with nothing recorded and nothing copied (a copy that fails part way removes what it made). A
  folder (`v58_root`, which holds the `libraries\` of `$USELIBRARY`; at most one) is the compiler's folder itself:
  the program is recorded with `qb64pe.exe` in a copy of the release under `target\qb64pe-root\<folder>\` (the
  clone's `qb64pe.exe`, `Makefile` and `settings\`, a directory junction `internal` to the clone's `internal\`,
  and the folder's contents), made again when the clone's `qb64pe.exe`, `Makefile`, settings or the folder change
  (`root-stamp.txt`); `qb64pe.exe` finds `internal\` and `libraries\` beside itself, so nothing is written into the
  clone but what every compile writes into `internal\temp`, under the clone's lock as before. A junction that cannot
  be made fails the program with nothing recorded (no fallback copies the toolchain). The compare copies files and
  folder alike under `root\` in the program's folder and gives the compiler `--include-root` there
  (`tests\verification\README.md`).
- At the timeout the program gets Ctrl+Break and 5 s before it is killed, so what it printed last (a prompt
  without a line end) is in the file, as it was under the shell's `timeout`.
- `exit=` holds the code as Git Bash reports it, since the files were first recorded by a shell script: the low
  byte of an ordinary code, 124 for a timeout, 139 for an access violation, 127 for most other crashes
  (`bash_exit_code` in the runner).
- A recording is one program at a time, in the reference clone: these files are recorded truth, like
  `--record`. `--suite all` does not include the suite. A run of named programs, with `--glob` or with `--list`
  writes `results-partial.json`; a full one `results-verification.json`.

**The compare** (any compiler other than `qb64pe`; `tests\verification\README.md` holds the lists it is judged by):

```
python tools\legacy_tests\run_legacy_tests.py --suite verification --qb64 target\release\qb64rust.exe ^
    --list tests\verification\pass.list --jobs 8 --build-cache target\build-cache --pass-cache target\pass-cache
```

- Each program is compiled with `-x -q -m` in a fresh folder `target\legacy-tests\verification\<name>\`, removed
  on a pass, which holds the program, its `.stdin` and `.noprompt`, and the support items of its family: the files
  and folders of `verification\` whose name begins with its family number (`v16_inc\` and `v16_y.bi` for
  `v16_m8_a`; `v37c.a` and `v37_lib\` for a `v37` program) and those with no family number
  (`cp437_all_bytes.bin`), leaving out programs, `.txt` files, executables and their build folders, scripts,
  documents, other programs' `.stdin` and `.noprompt`, and `v40_rc\` (`verification_support`). Nothing is
  written into `verification\`, and programs run side by side cannot see each other's files. A `v37` program
  whose libraries are missing fails with "run verification/v37_lib/build_libs.py first".
- The run is the recording's: stdin from the null device or the `.stdin`, `QB64PE_NOPROMPT` from `.noprompt` or
  `y`, a 60 s timeout with Ctrl+Break and 5 s before the kill, and no key presses, so a program waiting at `END`
  times out as it did. It starts through `press_any_key.py --no-keys --hidden-desktop`: the window goes on a
  desktop that is never shown (`QB64RUST_SHOW_WINDOWS=1` shows it), and the helper shares this runner's console,
  as the recorded program did, so the Ctrl+Break reaches the program.
- Judged by the recorded `<name>.out.txt`: `(not compiled)` passes when the compile fails, writes no executable
  and reports at least one error not marked "not supported yet" (the corpus rule; the text is not compared);
  any other recording passes when the build runs and `verification_text` of the run (the folder masked as
  `<verification>`, the `exit=` line as Git Bash reports it, the timeout lines) equals it, CRLF as LF and
  trailing newlines ignored, after the rules of `tests\verification\normalize\<name>.normalize` on both sides.
  A recorded timeout passes only on a timeout. No `<name>.out.txt` fails ("nothing recorded to compare").
  `<name>.compile.txt` is not read. A failure at the stage `result` names the first line that differs.
- The run's `program_log` must equal `<name>.log.txt`, or be empty when there is none; else the stage `log`,
  naming the first line that differs. Entries of the runtime's own scopes are never compared.
- The trace of divergences as in the corpus, with the sidecar in `tests\verification\divergences\`.
- `--list` (one name per line), `--glob`, program names, `--jobs`, `--build-cache`, `--pass-cache` (the key also
  holds the recording and its `.log.txt`, the sidecars, the `.stdin`, the `.noprompt` and every support item) and `--check-list`
  work as for the corpus. A program of `tests\verification\known.list` that fails is a known failure (`KFAIL`);
  one that passes is named at the end, to move to `pass.list`.

Results (executables, compiler output, run output, a JSON summary) go to `target\legacy-tests\` (ignored by
git). The summary is `results.json` for `--suite all`, `results-<suite>.json` for one whole suite, and
`results-partial.json` when `--category`, `--glob`, `--list` or program names narrow the run, so a partial run never
overwrites a full one. Since 2026-10-16 the summary holds a `run` record (format 1): the selection, the roots of the
corpus, compile and verification suites (relative to the repo), `QB64RUST_NO_FOLD`, `--cpp-opt`, `--record`, `--jobs`, whether a
build or pass cache was used, the SHA-256 of the compiler and of this runner, and when the run started and ended.

**A pass list judged from a full run** (`--check-list <list>`, corpus, compile and verification suites, `study\archive\31` item 3): runs
nothing, and judges every program of the list from the suite's last full summary, `results-<suite>.json`. Each
listed program must have `PASS` there, run (not kept by `--pass-cache`); a known failure fails it (a pass list needs
a pass), as do a missing result and any file of the program (the `.bas`, every `<name>.*` beside it, its
`<name>.files\`, a compile-suite sidecar, a verification program's two sidecars; not its `.exe`) changed after
the run started. And the run must be the same as a run of
the list would be: a full run, the same root, the same compiler build and runner (by hash), the same
`QB64RUST_NO_FOLD`, and no `--pass-cache`, `--record` or `--cpp-opt`; a summary written before the `run` record fails.
It prints the failures, the runner's summary line and `<list>: n of m passed in results-<suite>.json`; exit code 0
when all passed, 1 otherwise, 2 for a usage error. **Not seen:** an included file changed after the run, and
the reference clone (its libqb and toolchain, which a corpus program is linked with) changed after it. At a
change's close it stands for the slice list and the upstream pass list (`CLAUDE.md` working rule 9): the full suites
run those programs with the same options, so running the lists too ran 765 programs twice. Checked once on
2026-10-16 (the build of `m3-sound-handles`): `slice.list` 440 of 440 and `pass.list` 325 of 325, the same programs
as the list runs of that build passed; about a second each. The run without folding, the differential pass list
(another corpus root) and the call-site check are still run.

**The count blocks of the test READMEs** (`counts.py`, `study\archive\31` item 2): `python tools\legacy_tests\counts.py`
rewrites the text between `<!-- counts:NAME -->` and `<!-- /counts:NAME -->` in `tests\corpus\README.md`,
`tests\upstream\README.md`, `tests\differential\README.md` and `tests\verification\README.md`, and nothing outside
them; `--check` compares and
writes nothing (exit code 1 when a block differs). Membership blocks come from the lists and the folders (the
upstream "x of y": `pass.list` of the copy's programs less `deferred.list`). Result blocks come from the suite's
last full summary and are written only when it is of the release build at `target\release\qb64rust.exe` and of
this runner, a plain full run (no selection, folding on, no cache that skips a program); the pass list is judged
in it as `--check-list` does. Otherwise the block is left as it is, with its own run's date and build, and the
reason is printed. Run it at a change's close, after the full suites; tests: `test_counts.py`.

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
- <a id="locks"></a>**Locks: runs of `qb64rust` side by side and beside a run of `qb64pe`, which has the clone
  alone** (`clone_lock.py`, `test-parallel-runs`, design D5 of `libqb-copy`). This runner (so
  `verification\run.sh` and tier 2 too) and `tools\callsite\callsite.py` take a lock on
  `target\qb64pe-clone\in-use.lock` first: **alone** for a run that uses `qb64pe.exe` in the clone,
  **shared** for a run of `qb64rust` whose programs are in the clone's own test folders (a compile suite run
  from the clone's `tests\compile_tests`, the qbasic and format suites), **none** for any other run of
  `qb64rust` (the corpus suite, the compile suite of `tests\upstream`). A run that cannot have it stops at once
  with exit code 3, or with `--wait` prints that it is waiting and starts when it can. The system drops the
  lock when the run ends, however it ends.
  - With `qb64pe`: two runs would empty and read each other's `internal\temp`, and its `make` writes the
    libqb objects in `internal\c`. So the verification suite, a corpus run of the old compiler with one job, and
    `callsite.py` (its old side runs `qb64pe -z` in the clone) have it alone.
  - With `qb64rust` (any `--jobs`): its fragments, `qbx.cpp`, `qbx.o` and executable are in its own build
    folder (`PATH_INTERNAL_TEMP` points there), it builds against this repo's copy of libqb (`libqb\`, `make -C
    libqb`), and from the clone it reads only the toolchain and the files `DECLARE LIBRARY` names, which a
    compile with `qb64pe.exe` never writes; so it needs no lock of the clone, unless it runs programs in the
    clone's folders, where a run of `qb64pe` runs the same programs. The libqb objects its `make` builds in
    the copy when one it links is missing or older than its source have a lock of their own,
    `target\qb64pe-clone\objects.lock`, which the runner names in `QB64RUST_OBJECTS_LOCK` for every compile:
    the driver runs `make -q` on the objects of the build's flags under a shared lock and keeps it for the
    build when they are up to date; otherwise it builds alone (`crates\driver\src\build.rs`, `lock_objects`).
    So the full corpus, the full upstream suite and the run without folding can run side by side (each with
    its own results folder, which they have), a recording with `qb64pe.exe` can run beside them, and the
    builds within one `--jobs` run no longer race for a missing object either.
  - The one exception is `--jobs` with `qb64pe` (next point): it builds in its copies and can run beside
    another run that uses another results folder; it takes the clone's lock alone only to run a failed program
    again.
  - The CLI tests that build (`cargo test -p qb64rust-driver --test cli -- --ignored`) take no lock of the
    clone (they run no program there) and give every compiler they start `QB64RUST_OBJECTS_LOCK`
    (`crates\driver\tests\support`). Python's lock is `LockFileEx` over the whole file, the range Rust's
    `File::lock` takes, so the driver and these tools see each other's locks on the objects.
  - **A failure beside other builds runs again alone** (corpus and compile suites, `--jobs` above 1 with
    `qb64rust`): after the others, once, and both outcomes are printed (`[again]`). A failure that cannot
    pass alone is not run again (`SETTLED_STAGES`): wrong expected files, or a program expected to be
    rejected that was not rejected with a real error. The latter may be a failed C++ build after the
    compiler wrongly accepted the program, and its detail then names the build; alone it would fail with
    "compiled successfully, expected an error". A `compile` failure is run again, as its exit code is also
    the C++ build's. One that then passes is
    counted as passed with a `note:` at the end; it makes the exit code 1 unless it is **timing-sensitive**
    (`timing_sensitive`: a screen program, or one whose source names `SOUND`, `PLAY`, `BEEP`, `_SND…`,
    `TIMER`, `_DELAY`, `SLEEP`, `_LIMIT`, `_MOUSE…`, `INKEY$`, `_KEYHIT`, `_KEYDOWN`, `ON KEY` or `KEY(`),
    whose result may change with the load of the machine. The list is wide on purpose: a word in a comment
    counts, and so does `RANDOMIZE TIMER`, so 83 of the 471 corpus programs and 33 of the 404 upstream ones
    match (2026-10-17); a shared-build fault in one of them shows only as its `note:`, so read the notes.
    "Alone" is alone in this run: a run of `qb64rust` beside it may still be building.
  - Not covered: `qb64rust` started by hand without `QB64RUST_OBJECTS_LOCK` takes no lock.
  - A git worktree of this repo (the merge runs, below) takes the main checkout's lock files: `clone_lock.py`
    and the CLI tests find the main checkout from the worktree's `.git` file (`main_checkout`). Not covered:
    two separate clones of this repo that use one reference clone; the lock file is under each one's own
    `target\` (the reference clone is read-only and gets no lock file). Use a worktree, not a second clone.
- <a id="the-compiler-a-run-tests"></a>**The compiler a run tests** (`compiler_copy.py`, `study\archive\34` §3 item 2).
  A run of `qb64rust` (and `callsite.py`) copies the executable it is given, at its start, to
  `target\test-bin\<first 16 hex digits of its SHA-256>\qb64rust.exe` and compiles every program with that
  copy, so `cargo build --release` during the run replaces nothing the run reads: build, run tier 1 or a
  narrowed run of the new build while a long run goes on. The first line of the output names it:
  `[compiler] target/release/qb64rust.exe sha256 <hash> started <seconds since 1970>`; the results file still
  names the compiler as given, with the same `compiler_sha256`. `tools\close\close.py --check` judges a run by
  that hash against the current build and by its start against the test inputs changed since
  (`tools\close\README.md`). The copy behaves as the original: the driver finds the reference clone and the copy of libqb from
  the folder it was built in, a path fixed at build time. A copy not used for a day is removed at the next
  run's start (about 10 MB each).
- <a id="merge-runs-in-a-worktree"></a>**The runs at a merge into `main` in a worktree** (`study\archive\34` §3 item 3,
  optional). The full set at a merge (`CLAUDE.md` working rule 9, about an hour) may run in a second checkout
  of the merge commit, so that the next change goes on in this one:

  ```
  git worktree add ../QB64Rust-merge <merge commit>    # beside this repo, so ..\QB64pe is the clone for both
  cd ../QB64Rust-merge && cargo build --release        # its own target\, a full build the first time
  python tools/legacy_tests/run_legacy_tests.py --suite corpus --qb64 target/release/qb64rust.exe --jobs 8 ...
  git worktree remove ../QB64Rust-merge                # when the runs are read
  ```

  The worktree must sit beside this repo: the driver, the runner and `callsite.py` find the clone as
  `..\QB64pe` of the checkout they belong to. Its runs take this checkout's locks (above), so a run of
  `qb64pe` in either waits for the other's runs; `--build-cache` may name this checkout's
  `target\build-cache` to share its entries (a key holds every input, so a shared entry is never wrong). The
  worktree's `target\` is several GB; remove the worktree when its runs are read. Creating and removing one
  stages nothing (`CLAUDE.md` rule 1).
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
