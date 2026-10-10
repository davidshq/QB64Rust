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
`<name>.normalize` sidecar. Each group SHALL have a `SOURCE.md` naming
where its programs came from, the source commit and the licence.

#### Scenario: Program from QB64Fresh
- **WHEN** a reader opens `tests/corpus/runtime_comparison/SOURCE.md`
- **THEN** it names QB64Fresh's `tests/runtime_comparison/` at commit `85e6df8`, the MIT licence, `CLAUDE.md`
  rule 5, and the files that were not copied

#### Scenario: Programs are unchanged copies
- **WHEN** a corpus `.bas` from `runtime_comparison` is compared with the QB64Fresh file at `85e6df8`
- **THEN** the bytes are identical

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
