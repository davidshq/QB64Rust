# Spec Delta

## ADDED Requirements

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
