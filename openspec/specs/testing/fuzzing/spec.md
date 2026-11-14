# testing/fuzzing Specification

## Purpose

Coverage-guided fuzzing of the compiler's front end: which properties the fuzz targets check, how a run is
started and bounded, and how an input that fails becomes a permanent test.

## Requirements

### Requirement: Parse target
The fuzz target `parse` SHALL take any sequence of bytes as a program, lex and parse it, and fail when the
front end panics or when the tree does not print back exactly the bytes given.

#### Scenario: Parser panic
- **WHEN** an input makes the lexer or the parser panic
- **THEN** the run stops and writes the input to a file

#### Scenario: Round trip broken
- **WHEN** the tree of an input prints back bytes that differ from the input
- **THEN** the run stops and writes the input to a file

#### Scenario: Bytes that are not text
- **WHEN** the input holds NUL bytes, lone carriage returns or bytes above 127
- **THEN** it is parsed like any other input and the same two properties hold

### Requirement: Compile target
The fuzz target `compile` SHALL run an input through the lexer, the parser and the checker, and fail on a
panic. An input the checker accepts without errors SHALL also be lowered and emitted as C++ for a debug build
with coverage counters and value tracing, and a panic there SHALL fail too. `$INCLUDE` SHALL find no file.

#### Scenario: Checker panic
- **WHEN** an input makes the checker panic
- **THEN** the run stops and writes the input to a file

#### Scenario: Code generator panic
- **WHEN** an input that the checker accepts makes lowering or C++ generation panic
- **THEN** the run stops and writes the input to a file

#### Scenario: Include directive
- **WHEN** an input holds `'$INCLUDE: 'x.bi'`
- **THEN** no file is read from the disk and the include is reported as not found

### Requirement: Server target
The fuzz target `server` SHALL make the calls the language server makes on a document (its diagnostics, its
formatted text, its symbol index) on the input and fail when any of them panics.

#### Scenario: Server call panic
- **WHEN** an input makes the server's check, format or index call panic
- **THEN** the run stops and writes the input to a file

### Requirement: Time and memory limits
A fuzz run SHALL treat an input that takes more than 10 seconds, or more than 2 GB of memory, as a failure.
Inputs SHALL be at most 16 KB.

#### Scenario: Slow input
- **WHEN** an input of 16 KB or less takes more than 10 seconds in a target
- **THEN** the run stops and writes the input to a file

### Requirement: One set of properties
The fuzz targets, the seeded mutation test and the regression test SHALL check their inputs with the same
code, so that an input one of them fails is failed by the others.

#### Scenario: Finding replayed
- **WHEN** an input written by a fuzz run is put in the regressions folder
- **THEN** `cargo test` fails on it for the same reason, on Windows and on Linux

### Requirement: Starting a run
A fuzz run SHALL be started by hand, with a target and a number of minutes, and SHALL end at that time or at
the first failure. It SHALL NOT be a step of `quick`, `full` or `merge`. At its end it SHALL print the target,
the time run, the number of inputs tried and the paths of the inputs that failed.

#### Scenario: Clean run
- **WHEN** `just fuzz parse 10` runs for 10 minutes without a failure
- **THEN** it exits 0 and prints the number of inputs tried

#### Scenario: Run with a finding
- **WHEN** a run finds a failing input
- **THEN** it exits non-zero and prints the path of the input, made as small as the fuzzer can make it

#### Scenario: Quick and full
- **WHEN** `just quick` or `just full` runs
- **THEN** no fuzz run is started

### Requirement: Seeds
A fuzz run SHALL start from the programs of the corpus that fit the input limit and from the regression
inputs, and SHALL give the fuzzer the language's keywords as a dictionary. The corpus the fuzzer grows SHALL
be kept between runs on the same machine and SHALL NOT be committed.

#### Scenario: Second run
- **WHEN** a target is run a second time on the same machine
- **THEN** it starts from the inputs the first run kept

### Requirement: Regression inputs
Every input in `tests/fuzz/regressions/` SHALL be run by `cargo test` through the properties of all three
targets. The test SHALL fail and name the file when a property fails. A file SHALL hold the input's bytes
unchanged, and its name SHALL say the target that found it.

#### Scenario: Fixed finding stays fixed
- **WHEN** a change brings back a panic that a regression input once found
- **THEN** `cargo test` fails and names that file

#### Scenario: Nothing private
- **WHEN** a regression input is added
- **THEN** it holds no local path, host name or secret, as `check_repo.py` checks for every file
