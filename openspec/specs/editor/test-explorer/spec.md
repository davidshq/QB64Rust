# editor/test-explorer Specification

## Purpose
Shows the BASIC unit tests of the workspace in VS Code's Test Explorer and runs them through `qb64rust test`
(spec `compiler/unit-tests`), with each result at its SUB.

## Requirements

### Requirement: Tests in the Test Explorer
The extension SHALL list the tests of the workspace's `*_test.bas` files in the Test Explorer, one item per test
file and one per test SUB below it, taken from `qb64rust test --list --format json`. A test item SHALL open its SUB's
line. The list of a file SHALL be refreshed when the file is saved, created or deleted.

#### Scenario: Listed at open
- **WHEN** a workspace with `lib\math_test.bas` holding `SUB TestAdd` on line 5 is opened
- **THEN** the Test Explorer shows `math_test.bas` with `TestAdd` below it, and the item opens line 5

#### Scenario: A test added
- **WHEN** `SUB TestSub` is added to `math_test.bas` and the file is saved
- **THEN** `TestSub` appears below `math_test.bas`

### Requirement: Running tests
Running a test, a file or all tests SHALL run `qb64rust test --format json` with a filter for what was chosen, and
SHALL mark each test passed, failed (with the assertion's message at its line when one is known, else at the SUB),
errored or timed out. A build error SHALL be shown on each test of its file, with the compiler's diagnostic. The
test's output SHALL be shown in the test run's output. Cancelling the run SHALL end `qb64rust test`.

#### Scenario: A failure
- **WHEN** `TestAdd` fails with "sum: expected 4, got 5" and the user runs it
- **THEN** it is marked failed, with that message at the SUB

#### Scenario: A failed _ASSERT
- **WHEN** a test fails at an `_ASSERT` on line 9
- **THEN** the message is shown at line 9

#### Scenario: Cancelled
- **WHEN** the user cancels a run of all tests while a test is running
- **THEN** `qb64rust test` and the test's process end, and the tests not run stay unmarked

### Requirement: Needs qb64rust
Tests SHALL run only with the `qb64rust` binary and its toolchain (spec `editor/build-run`, "Compiler discovery"),
whatever `qb64rust.buildWith` says. When they are missing, the Test Explorer SHALL show no tests and a message
saying what is missing, with the action the language status offers.

#### Scenario: No qb64rust binary
- **WHEN** the `qb64rust` binary is not found and the workspace has a test file
- **THEN** no test is listed, and the message says `qb64rust` is needed to run tests
