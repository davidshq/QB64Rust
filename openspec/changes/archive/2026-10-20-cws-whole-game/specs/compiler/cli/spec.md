# Spec Delta

## ADDED Requirements

### Requirement: Coverage build
`qb64rust --coverage` with a compile line SHALL build a program that counts each statement it runs. When it exits
(`SYSTEM`, `END`, off the end, a runtime error, the window closed) with `QB64RUST_COVERAGE` naming a file, it SHALL
append one record to that file. The record names each source file with a hash of its text, then lists each statement
as file, line, column and count, unrun statements included. Without the option the C++ written SHALL be the same as
before.

#### Scenario: A run's counts
- **WHEN** a program of three statements, the second inside an `IF` that is false, is built with `--coverage` and
  run with `QB64RUST_COVERAGE=c.txt`
- **THEN** `c.txt` holds one record listing the three statements with counts 1, 0 and 1

#### Scenario: No variable, no file
- **WHEN** the same program runs without `QB64RUST_COVERAGE`
- **THEN** it writes no counts and behaves as the build without `--coverage`

#### Scenario: The build without the option
- **WHEN** a program is compiled without `--coverage`
- **THEN** its C++ is byte for byte the C++ written before this change

#### Scenario: Two runs into one file
- **WHEN** two runs of a coverage build name the same file
- **THEN** the file holds two records, the second after the first

#### Scenario: Statements in an included file
- **WHEN** a statement of a `$INCLUDE`d file runs
- **THEN** its record names the included file and the statement's line in that file
