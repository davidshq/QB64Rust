# testing/verification-suite Specification

## Purpose
The verification programs measure the old compiler, and their recordings hold the new compiler to it. One suite
of the runner records them with `qb64pe.exe` and compares the new compiler against them.

## Requirements

### Requirement: Recording with the old compiler
`run_legacy_tests.py --suite verification` run with `qb64pe` SHALL record each program of `verification\` as it
does before this change. It writes `<name>.compile.txt` and `<name>.out.txt` beside the program, masks the folder
as `<verification>`, writes exit codes as Git Bash reports them, and leaves a file alone when it differs only by
CRLF against LF.

#### Scenario: Recording unchanged
- **WHEN** a program whose recorded files are current is recorded again with `qb64pe`
- **THEN** no file of `verification\` changes

### Requirement: Comparing the new compiler
`--suite verification` run with any compiler other than `qb64pe` SHALL compare and SHALL NOT write into the
verification folder. Each program SHALL be compiled with `-x -q -m` and run in a fresh folder of its own, which
holds the program, its `.stdin` and `.noprompt` sidecars, and the support files of its family: the files and
subfolders of the verification folder whose name begins with the program's family number (`v16` for
`v16_m8_a`, so `v16_inc\` and `v16_y.bi`; `v37` takes `v37c.a`), excluding programs, recorded files,
executables, scripts and documents, and those that begin with no family number (`cp437_all_bytes.bin`). The run SHALL use stdin from the null
device (or the `.stdin` copy), `QB64PE_NOPROMPT` as recorded, a 60 s timeout and no key presses. The outcome
SHALL be judged by what was recorded:

- recorded `(not compiled)`: passes when the compile fails, writes no executable and reports at least one error
  not marked "not supported yet". The message text is not compared.
- recorded output: passes when the compile succeeds and the run's output equals the recorded output, with the
  run folder masked as `<verification>`, the `exit=` line written as the recording writes it, CRLF taken as LF,
  and trailing newlines ignored. The compiler's output is not compared.
- recorded `(timed out after 60 s)`: passes when the run also times out and the output before the timeout is
  equal.
- no `<name>.out.txt` (an executable that did not start): fails with "nothing recorded to compare".

#### Scenario: Rejected by both
- **WHEN** the old compiler recorded `(not compiled)` and the new one reports `1 error`
- **THEN** the program passes

#### Scenario: Rejected only with marked errors
- **WHEN** the old compiler recorded `(not compiled)` and the new one reports `2 errors (2 not supported yet)`
- **THEN** the program fails with "only errors marked not supported yet"

#### Scenario: Output differs
- **WHEN** the recorded output ends with `exit=0` and the new build's run prints a different line or ends with
  another code
- **THEN** the program fails, naming the stage "result"

#### Scenario: The verification folder is left alone
- **WHEN** the whole suite is compared with `qb64rust`
- **THEN** `git status --porcelain verification` prints the same as before the run

### Requirement: Selecting and caching the compare
The compare SHALL take program names, `--glob`, `--list <file>` (one program name per line, `#` comments
allowed), `--jobs`, `--build-cache` and `--pass-cache` as the corpus suite does. A program passed before is not
run again only when its executable, source, recorded files and sidecars are all unchanged. `--check-list` SHALL
judge a list from the last full compare (`results-verification.json`).

#### Scenario: Pass cache
- **WHEN** a program passed and is compared again with `--pass-cache` and nothing it reads has changed
- **THEN** it is compiled, not run, and reported as passed before

### Requirement: Divergences and normalisation of a compared program
A run of the compare SHALL name a fresh trace file in `QB64RUST_DIVERGENCES`. A row of `QB64PE-DIVERGENCES.md` that
`tests\verification\divergences\<name>.divergences` does not list SHALL fail the program. When
`tests\verification\normalize\<name>.normalize` exists, its rules SHALL be applied to both the recorded and the
actual output before they are compared.

#### Scenario: A divergence not listed
- **WHEN** a compared program takes the path of D-026 and has no sidecar listing it
- **THEN** it fails, naming the stage "divergences"

### Requirement: Pass list and known differences
`tests\verification\pass.list` SHALL name programs that the new compiler passes, and comparing it SHALL pass
every one. `tests\verification\known.list` SHALL name every other program of `verification\`, each with a reason:
a row of `QB64PE-DIVERGENCES.md`, "not supported yet", "varies", "not recorded" (no `<name>.out.txt`), "recorded
elsewhere" (recorded with another `--verification-root`), or "bug: <what>". No program SHALL be in both lists or
in neither. A program of `known.list` that passes in a full compare SHALL be reported so that it can move to
`pass.list`, unless its reason is "varies".

#### Scenario: Every program is accounted for
- **WHEN** the tier-1 check of the lists runs
- **THEN** each `.bas` of `verification\` is named exactly once across `pass.list` and `known.list`, and each
  name in either list is a program of `verification\`

#### Scenario: Pass list run
- **WHEN** `run_legacy_tests.py --suite verification --qb64 target/release/qb64rust.exe --list
  tests/verification/pass.list` is run
- **THEN** every listed program passes

### Requirement: The compare's rules are tested
`test_run_legacy_tests.py` SHALL test each compare rule with a stand-in compiler and program, without the clone:
a rejection that passes and one with only marked errors, equal and different output, a different exit code, a
timeout on both sides, a missing `.out.txt`, the folder mask, the support files copied, the verification folder
left unwritten, a normalisation sidecar, and a divergence that is not listed.

#### Scenario: Runner tests
- **WHEN** `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` runs
- **THEN** the tests of every rule above pass

### Requirement: The program's log
Recording with `qb64pe` SHALL write `<name>.log.txt` beside a program whose first run logged an entry of the
program's own scope (`QB64`), and SHALL remove a stale one otherwise. Comparing SHALL hold the new build's log to
`<name>.log.txt`, or to no entry when none is recorded. A difference SHALL fail the program, naming the stage
"log".

#### Scenario: A line number differs
- **WHEN** a program's new build names line 6 where the recording names line 5
- **THEN** the program fails, naming the stage "log"

#### Scenario: Nothing recorded, nothing logged
- **WHEN** a program has no `<name>.log.txt` and its new build logs only entries of the runtime's own scopes
- **THEN** the log does not fail it

### Requirement: What of the log is kept
Both the recording and the compare SHALL keep only the entries of scope `QB64` and their stack frames. They SHALL
remove each entry's timestamp and each frame's address, and SHALL mask the verification or run folder as
`<verification>`, so that no local path is recorded.

#### Scenario: Equal log
- **WHEN** a program's new build writes the same entries as recorded, at other times and addresses and from
  another folder
- **THEN** the program passes

#### Scenario: Runner tests
- **WHEN** `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` runs
- **THEN** the tests of the log's recording, cut, mask and compare pass with a stand-in compiler

### Requirement: A program recorded with a compiler root of its own
A verification program with a sidecar `<name>.root`, which names a folder of `verification\` that holds a
`libraries\` folder, SHALL be recorded with `qb64pe.exe` run from a copy of the release under `target\` whose
`libraries\` is that folder, and SHALL be compiled by any other compiler with that folder as its compiler root
(`--include-root`). The clone SHALL NOT be written to, and the copy SHALL be made again when the clone's
`qb64pe.exe` changes.

#### Scenario: A library found only through the sidecar
- **WHEN** `v58_u01` has `v58_u01.root` naming `v58_root`, and `v58_root\libraries\descriptors\test\a.ini` exists
- **THEN** its recording is the old compiler's run with that library, and the new compiler's compile finds the
  same library

#### Scenario: The clone untouched
- **WHEN** the programs with a `.root` sidecar are recorded
- **THEN** `git status` of the clone shows no change, and no `libraries\` folder exists in it

#### Scenario: Without the sidecar
- **WHEN** a program with `$USELIBRARY` has no `.root` sidecar
- **THEN** it is recorded with the clone, whose release has no `libraries\` folder, and its recording is the old
  compiler's "Library descriptor … not found"

### Requirement: Suites build with the fixes off
The test runner SHALL build every program of every suite with `--fixes off`, and SHALL say so in its first line, so
that a `qb64rust.toml` in the tree or above it changes no result. A run with `qb64pe.exe` SHALL pass no such
option.

#### Scenario: A settings file in the tree
- **WHEN** a `qb64rust.toml` turning every fix on stands in the repository's root and the verification suite runs
  with the new compiler
- **THEN** every program is compiled with `--fixes off`, and the results are those of a run without the file

### Requirement: A program recorded with the fixes on
A verification program with a sidecar `tests\verification\fixed\<name>.out.txt` SHALL be built and run a second
time with `--fixes on`, and that run SHALL pass when its output equals the sidecar by the compare's rules; it
SHALL be reported as `<name> [fixes on]`. Each switched fix SHALL have at least one such program.

#### Scenario: Both ways
- **WHEN** `v50_c_checking` has a sidecar `fixed\v50_c_checking.out.txt` and the suite runs
- **THEN** the program is compared once with its `qb64pe.exe` recording (fixes off) and once with the sidecar
  (fixes on), and each result is counted on its own
