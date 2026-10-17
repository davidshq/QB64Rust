# testing/golden-corpus Specification

## Purpose
A fixed record of what the old compiler (QB64pe 4.7.0) does with a set of small headless programs, and a runner
that checks any compiler with qb64pe's command line against it. It is the first conformance target for the new
compiler.

## Requirements

### Requirement: Corpus layout
The corpus SHALL live in `tests/corpus/<group>/`. Each test SHALL be a `<name>.bas` with exactly one of
`<name>.output` (the program must compile, run, and print this), `<name>.err` (the program must fail to compile
with this compiler output) or `<name>.norun` (compile only; holds the reason it must not run), and MAY have a
`<name>.normalize` sidecar. A test with an `.output` MAY also have a `<name>.screen` (the state of the screen the
program must leave, spec `testing/screen-state`) and with it a `<name>.keys` (the keys it is given). Each group
SHALL have a `SOURCE.md` naming where its programs came from, the source commit and the licence.

#### Scenario: Program from QB64Fresh
- **WHEN** a reader opens `tests/corpus/runtime_comparison/SOURCE.md`
- **THEN** it names QB64Fresh's `tests/runtime_comparison/` at commit `85e6df8`, the MIT licence, `CLAUDE.md`
  rule 5, and the files that were not copied

#### Scenario: Programs are unchanged copies
- **WHEN** a corpus `.bas` from `runtime_comparison` is compared with the QB64Fresh file at `85e6df8`
- **THEN** the bytes are identical

#### Scenario: A screen program
- **WHEN** a reader opens `tests/corpus/screen/`
- **THEN** each program there has an `.output` and a `.screen`, and those that read the keyboard a `.keys`

### Requirement: Running a compiler against the corpus
`run_legacy_tests.py --suite corpus` SHALL, for each program, compile it with `-q -m -x` and no other build flags
in a fresh empty folder under the git-ignored results folder (never in the corpus folder), run the executable there with no arguments, `QB64PE_NOPROMPT=y` and a 60 s timeout, with stdin from the null
device, except on Windows, where the program SHALL run in a console of its own, with no window, whose input
receives key presses so that `END` ("Press any key to continue") returns, and compare the merged stdout and stderr with `<name>.output`
after applying `<name>.normalize`, treating CRLF as LF and ignoring trailing newlines. For a `.err` test it SHALL
require a failed compile, no executable, and compiler output equal to `<name>.err` ignoring CR characters.

#### Scenario: Matching output
- **WHEN** the old compiler is checked against a freshly recorded corpus
- **THEN** every program passes or is listed in `known_failures.txt` as `corpus:<group>/<name>` (reported KFAIL)

#### Scenario: Program writes files
- **WHEN** a program creates `rt_52_out.txt` in its working directory
- **THEN** the file is created in that program's own scratch folder, and the repo's working tree is unchanged
  after the run

#### Scenario: Program with an outside side effect
- **WHEN** `239_lprint` (which would send a page to the default printer) is checked or recorded
- **THEN** its `.norun` sidecar makes the runner compile it and require an executable, and the executable is never
  started

#### Scenario: Known failure that cannot be recorded
- **WHEN** a program is listed in `known_failures.txt` and has no `.output`, `.err` or `.norun` (it hangs, so
  nothing could be recorded)
- **THEN** the runner compiles it, does not run it, and reports it as KFAIL

#### Scenario: Hung program
- **WHEN** a program does not finish within 60 s
- **THEN** its process tree is killed and the test fails at stage `run` with detail `timeout`

### Requirement: Recording expected results
`run_legacy_tests.py --suite corpus --record` SHALL compile each selected program once with the given compiler,
run the executable twice, and write `<name>.output` or `<name>.err` only if both runs give the same normalised result. It SHALL
report programs whose two runs differ and SHALL NOT write files for them.

#### Scenario: Reproducible recording
- **WHEN** `--record` is run twice on the same compiler build
- **THEN** the second run changes no file (`git status` shows nothing new under `tests/corpus/`)

#### Scenario: Non-deterministic program
- **WHEN** a program prints `TIMER` and has no `.normalize` sidecar
- **THEN** recording reports it as non-deterministic and writes no expected file for it

### Requirement: Normalisation sidecar
A `<name>.normalize` file SHALL hold rules, one per line, as a regular expression, a tab, and a replacement;
lines starting with `#` are comments. The runner SHALL apply the rules in order to each output line before
comparing and before recording.

#### Scenario: Clock value
- **WHEN** `234_timer_value` prints `timer: 51234.56 ` (with the trailing space `PRINT` adds after a number) and
  its sidecar maps `^timer: *[0-9.]+ *$` to `timer: <TIMER>`
- **THEN** the recorded and compared line is `timer: <TIMER>`

### Requirement: Recorded files hold no local data
Recorded `.output` and `.err` files SHALL contain no absolute local paths, user names or host names
(`CLAUDE.md` rule 2), and SHALL be stored byte-exact by git (`-text` in `.gitattributes`).

#### Scenario: Machine-dependent value
- **WHEN** `212_environ_dollar` prints the length of `PATH`, which differs between machines
- **THEN** its sidecar replaces the number, and the recorded file is the same on any machine

#### Scenario: Local path in output
- **WHEN** a recorded output contains a drive-letter path or the user's name
- **THEN** it is masked by a `.normalize` rule before the file is staged

### Requirement: Optimiser-dependence report
`run_legacy_tests.py --suite corpus --cpp-opt` SHALL build with `-f:OptimizeCppProgram=true` and compare against
the same expected files. `tests/corpus/README.md` SHALL list the programs that fail in this mode, one line each with
the differing behaviour.

#### Scenario: LONG overflow
- **WHEN** the corpus is checked with `--cpp-opt` against the old compiler
- **THEN** `verification/v11_wrap_o2` fails (its LONG overflow result differs from the default build) and is
  listed in `tests/corpus/README.md`

### Requirement: Standard input sidecar
A corpus program MAY have `<name>.stdin`. `run_legacy_tests.py --suite corpus` SHALL then run the executable with
its standard input read from a copy of that file, byte for byte, when checking and when recording, for the old and
the new compiler alike; every other rule of the run (the scratch folder, the timeout, the comparison) SHALL stay
as it is. A program without the sidecar SHALL run exactly as before. A program with the sidecar SHALL end with
`SYSTEM`, and its `.stdin` SHALL hold an answer for every input statement it runs: the old compiler's program
waits for ever at `END`'s "Press any key to continue" and when its input has run out.

#### Scenario: Program that reads a line
- **WHEN** a program does `LINE INPUT s$: PRINT "["; s$; "]"` and its `.stdin` holds the line `abc`
- **THEN** the run prints `[abc]`, with either compiler

#### Scenario: Input runs out
- **WHEN** a program asks for more input than its `.stdin` holds
- **THEN** the run fails with a timeout, with either compiler (the program waits for more input), and nothing is
  recorded for it

#### Scenario: No sidecar
- **WHEN** the corpus is run after this change with no `.stdin` file present
- **THEN** every program's result is the same as before the change

### Requirement: The runner's rules are tested
The rules by which `run_legacy_tests.py` decides that a corpus program passes, and what `--record` writes, SHALL be
covered by tests of the runner itself (`tools\legacy_tests\test_run_legacy_tests.py`) that need no compiler, no
reference clone and no C++ build, and that run in CI with tier 1: the normalisation of program output and of
`.err` text, the rule for a rejection by a compiler other than `qb64pe`, `.normalize` files, the `--list` and
known-failures files, and for one corpus program every outcome (equal and different output, each kind of `.err`
result, `.norun`, `.stdin`, a known failure without an expected file, a failed or timed-out run, and `--record`
with equal runs, with differing runs and with a rejection). A change to one of these rules SHALL come with a
change to its test.

#### Scenario: A broken comparison is noticed
- **WHEN** the runner is changed so that a bare CR in a program's output no longer counts, or so that the two runs
  of `--record` need not be equal
- **THEN** `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` fails

#### Scenario: Runs without a compiler
- **WHEN** the tests run on a machine with Python and neither `qb64pe` nor `qb64rust`
- **THEN** they pass

### Requirement: Data files beside a program
A folder `<name>.files` beside a corpus program SHALL be copied, with everything in it, into the folder the
program is compiled and run in, before it is compiled, for a run and for a recording and with either compiler.
The files SHALL be part of what the pass cache compares: a program whose data file changed is run again. The
folder beside a program that does not run (one with an `.err` or `.norun` file) SHALL be an error in the test.
Git SHALL store the files byte for byte.

#### Scenario: A program that loads a file
- **WHEN** `g07_mountains.bas` runs `BLOAD "mtn.vga", VARPTR(mtn(1))` and `g07_mountains.files\MTN.VGA` exists
- **THEN** the program finds the file with either compiler and passes by the state of its screen

#### Scenario: A changed data file
- **WHEN** a program passed with `--pass-cache`, then a file of its `.files` folder changes
- **THEN** the next run with `--pass-cache` runs the program again

#### Scenario: Data files beside a program that does not run
- **WHEN** a program has an `.err` file and a `.files` folder
- **THEN** the runner reports a failure of the test that names the two

### Requirement: The run-time trace of divergences
A program built by the new compiler SHALL, when the environment variable `QB64RUST_DIVERGENCES` names a file,
append the line `D-0nn` to it the first time in a run that it takes the path of row D-nn of `DIVERGENCES.md`, for
every row with a run-time path, and SHALL write nothing without the variable. The runner SHALL set the variable for
every run of the corpus and compile suites, and SHALL fail a program whose trace names a row its
`<name>.divergences` file (one `D-0nn` per line, `#` comments) does not list.

#### Scenario: A row the program does not list
- **WHEN** a corpus program stores into `a(9).m` for `DIM a(3)` of a user type and has no `.divergences` file
- **THEN** its output may equal the recording, and the runner still fails it with the stage "divergences", naming
  `D-004`

#### Scenario: A listed row
- **WHEN** the same program has a `.divergences` file that lists `D-004`
- **THEN** it passes

#### Scenario: Once per row
- **WHEN** a program takes the path of D-006 in a loop of a thousand passes
- **THEN** the trace holds one line `D-006`

#### Scenario: Without the variable
- **WHEN** the program runs without `QB64RUST_DIVERGENCES`
- **THEN** it writes no trace file and its output is the same

### Requirement: Runs of the new compiler share the reference clone
The runner, `callsite.py` and the CLI tests that build SHALL take the lock of the reference clone before they
use it: alone for a run that uses `qb64pe.exe` in the clone (`callsite.py` among them), shared for a run that
builds with the new compiler only. A run SHALL stop with exit code 3, or wait with `--wait`, while another holds the lock in a way that excludes
it. A run that takes it shared SHALL name the objects' lock in `QB64RUST_OBJECTS_LOCK` for every compile, and the
driver SHALL then take that lock around each `make`: shared when the clone's libqb objects for the build's flags
are up to date, alone otherwise. A run started in a git worktree of this repo SHALL take the lock files of the
main checkout.

#### Scenario: Two runs of the new compiler
- **WHEN** the CLI tests that build are running and a corpus run of `qb64rust` starts
- **THEN** the corpus run goes ahead without waiting

#### Scenario: A run of the old compiler
- **WHEN** the CLI tests that build are running and a corpus run of `qb64pe` starts without `--wait`
- **THEN** it stops with exit code 3

#### Scenario: A missing object
- **WHEN** a build's flags need a libqb object the clone does not have
- **THEN** the driver builds it holding the objects' lock alone, and a build that holds it shared waits until
  then

#### Scenario: A run in a worktree
- **WHEN** a corpus run of `qb64rust` is running in the main checkout and a run of `qb64pe` starts in a worktree
  without `--wait`
- **THEN** it stops with exit code 3

### Requirement: A failure beside other builds runs again alone
With the new compiler and `--jobs` above 1, the runner SHALL run each program that failed once more after the
others and report both outcomes. A program that passes then SHALL count as passed; the run SHALL fail unless the
program is timing-sensitive (a screen program, or one whose source names a statement that waits in real time,
reads the clock, plays sound, or reads the keyboard or the mouse while it runs).

#### Scenario: A timing-sensitive program
- **WHEN** a program that plays a sound fails with `--jobs 8` and passes alone
- **THEN** it counts as passed, a note names it, and the run does not fail on its account

#### Scenario: Another program
- **WHEN** a program that only prints fails with `--jobs 8` and passes alone
- **THEN** it counts as passed, a note names it, and the run fails

### Requirement: The pass cache in the compile suite
`--pass-cache` SHALL work with the compile suite as with the corpus suite: a program that passed with the same
executable, source, expected output, divergence sidecar and files of its folder SHALL be compiled and not run
again.

#### Scenario: Nothing changed
- **WHEN** an upstream program passed with `--pass-cache` and is run again with the same executable
- **THEN** it passes with "not run: passed before" and is not run

#### Scenario: A file of its folder changed
- **WHEN** a file is added to the program's category folder
- **THEN** the next run with `--pass-cache` runs it again

### Requirement: The build cache keys on what make reads
With `QB64RUST_BUILD_CACHE` set, the driver SHALL key a build on its fragments, the clone's `qbx.cpp` and
`Makefile`, its settings, the toolchain's executables, and the size and time of each file under the clone's
`internal\c` that no build writes. The files a build of either compiler writes there (libqb objects and
archives, `.tmp` files, `qbx<N>.cpp` and `qbx*.o` in `internal\c`) SHALL NOT be part of the key.

#### Scenario: A compile with the old compiler
- **WHEN** a program was built with the build cache, `qb64pe.exe` then compiled a program in the clone, and the
  program is built again
- **THEN** the build is answered from the cache

#### Scenario: A libqb source changed
- **WHEN** a source or header of libqb under `internal\c` changes
- **THEN** the key changes and the program is built with `make`

### Requirement: A run tests a copy of the compiler
A run of the runner with the new compiler, and `callsite.py`, SHALL compile with a copy of the executable made at
the run's start under `target\test-bin`, named by the executable's SHA-256, and SHALL print a line naming that
hash and the time the run started. `tools\close\close.py --check` SHALL judge a closing step that prints the line
by the hash against the current release build and by the start against the changed test inputs.

#### Scenario: A build during a run
- **WHEN** the release build is replaced while a run of the slice list goes on
- **THEN** the run's later programs are compiled by the same compiler as its first, and `close.py --check` names
  the step stale when the new build differs

#### Scenario: The same bytes built again
- **WHEN** the release build is built again with the same bytes after a closing run ended
- **THEN** `close.py --check` still counts the run
