## MODIFIED Requirements

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

### Requirement: The compare's rules are tested
`test_run_legacy_tests.py` SHALL test each compare rule with a stand-in compiler and program, without the clone:
a rejection that passes and one with only marked errors, equal and different output, a different exit code, a
timeout on both sides, a missing `.out.txt`, the folder mask, the support files copied, the verification folder
left unwritten, a normalisation sidecar, a divergence that is not listed, a rejection on the wrong line, a
rejection whose line is not compared, and an exception of `lines.list`, kept and unneeded.

#### Scenario: Runner tests
- **WHEN** `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` runs
- **THEN** the tests of every rule above pass

## ADDED Requirements

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
