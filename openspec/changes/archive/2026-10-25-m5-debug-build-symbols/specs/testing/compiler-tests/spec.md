# Spec Delta

## MODIFIED Requirements

### Requirement: Mode lines
Every `.bas` file under `tests/frontend/` SHALL start with a line `' TEST: <mode>`, where mode is one of
`parse-ok`, `check-ok`, `check-fail`, `typed`, `ir`, `cpp`, `coverage`, `debug-info`. The test harness SHALL run
each file according to its mode, and SHALL fail a file that has no valid mode line. `debug-info` SHALL require no
errors and snapshot the debug symbol file the program would get (spec `compiler/debug-info`), with the files'
paths relative to the test folder.

#### Scenario: check-fail test
- **WHEN** a file with `' TEST: check-fail` produces no diagnostics
- **THEN** the test fails

#### Scenario: Missing mode line
- **WHEN** a file under `tests/frontend/` does not start with a mode line
- **THEN** the harness reports that file as an error

#### Scenario: debug-info test
- **WHEN** a file with `' TEST: debug-info` declares a variable of each storage, a static and a dynamic array and a
  user type
- **THEN** the snapshot holds the JSON with each of them, and a change to a C name or a layout fails the test
