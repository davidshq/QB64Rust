# Spec Delta

## MODIFIED Requirements

### Requirement: Mode lines
Every `.bas` file under `tests/frontend/` SHALL start with a line `' TEST: <mode>`, where mode is one of
`parse-ok`, `check-ok`, `check-fail`, `typed`, `ir`, `cpp`, `coverage`, `debug-info`, `warnings`. The test harness
SHALL run each file according to its mode, and SHALL fail a file that has no valid mode line. `debug-info` SHALL
require no errors and snapshot the debug symbol file the program would get (spec `compiler/debug-info`), with the
files' paths relative to the test folder. `warnings` SHALL require no errors and snapshot the warnings as `-w`
prints them (spec `compiler/lints`).

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

#### Scenario: A warnings test
- **WHEN** a file with `' TEST: warnings` has a label nothing names
- **THEN** its snapshot holds the `unused-label` warning, as `-w` prints it

### Requirement: Real programs compile
`cargo test` SHALL require of every program under `tests/programs` that the front end reports no diagnostic
for it other than a lint warning (spec `compiler/lints`), and that it is lowered, validated and emitted as C++
fragments. A test that needs the reference clone SHALL build the executable of each such program and SHALL NOT run
it.

#### Scenario: The game compiles
- **WHEN** `cargo test` runs
- **THEN** Civil War Strategy has no diagnostic other than lint warnings, marked or not, and its C++ fragments are
  written

#### Scenario: A built-in is taken out
- **WHEN** a change makes the front end report "not supported yet" for a line of Civil War Strategy
- **THEN** `cargo test` fails and names the program and the line

#### Scenario: The game builds
- **WHEN** the test that needs the clone is run
- **THEN** the game's executable is built, with its icon, and the test does not start it
