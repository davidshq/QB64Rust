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

- recorded `(not compiled)`: passes when the compile fails, writes no executable, reports at least one error
  not marked "not supported yet", and the earliest such error of the program's own file stands on the line the
  recording names (requirement
  "The line of a rejection"). The message text is not compared.
- recorded output: passes when the compile succeeds and the run's output equals the recorded output, with the
  run folder masked as `<verification>`, the `exit=` line written as the recording writes it, CRLF taken as LF,
  and trailing newlines ignored. The compiler's output is not compared.
- recorded `(timed out after 60 s)`: passes when the run also times out and the output before the timeout is
  equal.
- no `<name>.out.txt` (an executable that did not start): fails with "nothing recorded to compare".

#### Scenario: Rejected by both
- **WHEN** the old compiler recorded `(not compiled)` with `LINE 4:`, and the new one reports `1 error` on line 4
  of the program
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
allowed), `--jobs`, `--build-cache` and `--pass-cache` as the corpus suite does. A program passed before SHALL
not be built or run again while the C++ the compiler writes for it, libqb's sources and edits, the C++
toolchain, the runner, its source, its recorded files and its sidecars are all unchanged; the executable's bytes
SHALL NOT be part of the key. A program on `known.list` as `varies` SHALL never be served from the cache.
`--check-list` SHALL judge a list from the last full compare (`results-verification.json`).

#### Scenario: Pass cache
- **WHEN** a program passed and is compared again with `--pass-cache` and nothing it reads has changed
- **THEN** its C++ is written and found unchanged, it is neither built nor run, and it is reported as passed before

#### Scenario: The compiler rebuilt, the C++ unchanged
- **WHEN** the compiler is built again from the same sources, or from a change that leaves the program's C++ as
  it was, and the list runs with `--pass-cache`
- **THEN** every program whose C++ is unchanged is reported as passed before

#### Scenario: libqb edited
- **WHEN** a file of libqb's copy or its edits changes
- **THEN** the next run with `--pass-cache` builds and runs every program again

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
every one. A program of `pass.list` with a `diverged\` sidecar passes by that sidecar. `tests\verification\known.list`
SHALL name every other program of `verification\`, each with a reason:
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

#### Scenario: A diverged program on the pass list
- **WHEN** a program of `pass.list` has a `diverged\` sidecar and the pass list is run
- **THEN** the program is judged by the sidecar, not by QB64pe's recording

### Requirement: The compare's rules are tested
`test_run_legacy_tests.py` SHALL test each compare rule with a stand-in compiler and program, without the clone:
a rejection that passes and one with only marked errors, equal and different output, a different exit code, a
timeout on both sides, a missing `.out.txt`, the folder mask, the support files copied, the verification folder
left unwritten, a normalisation sidecar, a divergence that is not listed, a rejection on the wrong line, a
rejection whose line is not compared, and an exception of `lines.list`, kept and unneeded.

#### Scenario: Runner tests
- **WHEN** `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` runs
- **THEN** the tests of every rule above pass

### Requirement: The line of a rejection
For a program recorded `(not compiled)`, the compare SHALL take the first `LINE n:` of `<name>.compile.txt`, and
the lowest line of the program's own file holding an error of the new compiler not marked "not supported yet",
whatever order the errors are printed in. It SHALL fail the program at the stage "error line", naming both, unless
that line is n; with no such error in that file it SHALL fail too.

#### Scenario: Rejected on another line
- **WHEN** the recording is `(not compiled)` with `LINE 6:`, and the new compiler's only error is on line 7
- **THEN** the program fails at the stage "error line", naming lines 6 and 7

#### Scenario: An earlier error printed later
- **WHEN** the recording names `LINE 7:`, and the new compiler prints an error on line 7 and then one on line 3
- **THEN** the program fails at the stage "error line", naming lines 7 and 3

#### Scenario: Rejected only for an error in another file
- **WHEN** the recording names `LINE 3:` of a program and gives no included file, and the new compiler's only
  unmarked errors are in a file the program includes
- **THEN** the program fails at the stage "error line"

#### Scenario: A marked error first
- **WHEN** the recording names `LINE 5:`, and the new compiler reports a marked error on line 2 and an unmarked
  one on line 5
- **THEN** the program passes

### Requirement: Rejections whose line is not compared
The compare SHALL compare no line for a program recorded `(not compiled)` when `<name>.compile.txt` has no
`LINE n:` or has `LINE 0:` (QB64pe's "[INFORMATION UNAVAILABLE]"), or when its message places the error in an
included file ("in line … of … included"); such a program passes on any rejection, as before.

#### Scenario: No line recorded
- **WHEN** the recording is `(not compiled)` after a failed C++ build, with no `LINE n:`
- **THEN** the program passes on any rejection, as before

#### Scenario: Line 0 recorded
- **WHEN** the recording is "$IF without $END IF", "Caused by (or after):[INFORMATION UNAVAILABLE]", `LINE 0:`
- **THEN** no line is compared

#### Scenario: QB64pe's error in an included file
- **WHEN** the recording's message is "File v16_only_main.bi not found in line 2 of <verification>\v16_inc/x2.bi
  included" with `LINE 5:`
- **THEN** no line is compared

### Requirement: Exceptions to the line of a rejection
`tests\verification\lines.list` SHALL hold one row per program whose rejection the new compiler places on another
line than QB64pe: the program, the line the new compiler's rejection is pinned to instead, and a reason. That line
SHALL stand for the recorded one, so the program still fails on any third line. A row SHALL name a program recorded
`(not compiled)` whose recording names a line that is compared (requirement "Rejections whose line is not
compared"). A row that a full compare finds unneeded (the program rejected on the recorded line, or no line
compared) SHALL be reported so that it can be removed.

#### Scenario: A block error
- **WHEN** `lines.list` has the row `v16_m4_sub_missing_end 3 the line the block opens` and the recording names
  `LINE 9:`, and the new compiler's earliest unmarked error is on line 3
- **THEN** the program passes

#### Scenario: The exception's line moves
- **WHEN** the same row stands and the new compiler's earliest unmarked error is on line 5
- **THEN** the program fails at the stage "error line"

#### Scenario: An exception no longer needed
- **WHEN** a full compare finds a program of `lines.list` whose earliest unmarked error is on the recorded line
- **THEN** the program passes and the run reports its row as unneeded

#### Scenario: The list checked
- **WHEN** the tier-1 check of the lists runs and a row of `lines.list` names a program not recorded
  `(not compiled)` or whose recording names no line that is compared, or the row gives no line or no reason
- **THEN** the check fails, naming the row

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

### Requirement: Stored results judged again
`run_legacy_tests.py --rejudge --suite <suite>` SHALL read the suite's last results, apply the known lists as they
are now (with `--platform linux`, the Linux lists too), rewrite each program's status (a failure on a list is a
known failure, a known failure off every list is a failure), the summary and the results file, and print the
summary and exit code a run would. It SHALL build and run no program. It SHALL refuse, with a message and exit
code 2, results with no run record of this runner, of another compiler build, or of a program whose files changed
after the run started. A program the lists now skip SHALL be reported as skipped. The results file SHALL record
that it was judged again and when, and `--check-list` and the count blocks SHALL read it as the run it came from.

#### Scenario: A failure made known
- **WHEN** a full run of the verification suite on Linux failed only on `v24_x93_large_bytes`, which is then added
  to `known.linux.list`, and `--rejudge --suite verification --platform linux` runs
- **THEN** that program is a known failure, the summary has no failure, the exit code is 0, and no program ran

#### Scenario: A known failure taken off the lists
- **WHEN** a program's row is removed from `known.list` and its stored result is a known failure
- **THEN** the re-judge reports it as a failure and exits 1

#### Scenario: A changed program
- **WHEN** a program's `.out.txt` changed after the run started
- **THEN** the re-judge refuses and names the program

### Requirement: A program judged by its diverged output
When `tests\verification\diverged\<name>.out.txt` exists, the compare SHALL judge the program by that file in
place of QB64pe's recording, on every platform, by the same rules. When `diverged\<name>.log.txt` exists, it
SHALL stand in for the recorded log. A recording with `qb64pe` SHALL neither read nor write these files.

#### Scenario: Our own output passes
- **WHEN** `v17_g_precedence` has a sidecar `diverged\v17_g_precedence.out.txt` that gives `5 IMP 3 IMP 0: 4`
  where QB64pe's recording gives 7, and the build prints what the sidecar holds
- **THEN** the program passes

#### Scenario: A regression beside the divergence
- **WHEN** the build prints the divergent line as the sidecar holds it, but another line differs from the sidecar
- **THEN** the program fails at the stage "result", naming the sidecar and the first line that differs

#### Scenario: The log
- **WHEN** a program has `diverged\<name>.log.txt` and its run logs what that file holds, not what QB64pe logged
- **THEN** the program passes

### Requirement: A diverged rejection stands on a pinned line
A `diverged\<name>.out.txt` that holds `(not compiled)` SHALL pass only when the compile fails as the rule for
`(not compiled)` asks and the earliest unmarked error of the program's own file is on the line its row of
`lines.list` gives. Without such a row the program SHALL fail at the stage "error line".

#### Scenario: Rejected on the pinned line
- **WHEN** `v17_e_const_fdiv0` has a sidecar `(not compiled)` and a row of `lines.list` giving line 3, and the
  compile fails with its earliest unmarked error on line 3
- **THEN** the program passes

#### Scenario: No row
- **WHEN** a program's sidecar holds `(not compiled)` and `lines.list` has no row for it
- **THEN** the program fails at the stage "error line", saying that a diverged rejection needs a row

### Requirement: The diverged sidecars are checked
Tier 1 SHALL fail when a file of `tests\verification\diverged\` is not `<name>.out.txt` or `<name>.log.txt` of a
program of the suite, when that program is not on `pass.list`, when a `.log.txt` has no `.out.txt` beside it,
or when the program has no `divergences\<name>.divergences` naming at least one row of `QB64PE-DIVERGENCES.md`.

#### Scenario: No divergences file
- **WHEN** `diverged\v04_accidental.out.txt` exists and `divergences\v04_accidental.divergences` does not
- **THEN** the tier-1 check of the lists fails and names the program

#### Scenario: Still on known.list
- **WHEN** a program has a `diverged\` sidecar and its row is on `known.list`
- **THEN** the tier-1 check of the lists fails and names the program
