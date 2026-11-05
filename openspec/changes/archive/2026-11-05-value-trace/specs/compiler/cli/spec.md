## ADDED Requirements

### Requirement: Value-trace build
`qb64rust --trace-values` with a compile line SHALL build a program that records the values stored into its
variables (spec `compiler/value-trace`). When it exits (`SYSTEM`, `END`, off the end, a runtime error, the window
closed) with `QB64RUST_TRACE` naming a file, it SHALL append one record to that file. Without the option the C++
written SHALL be the same as before this change. The option SHALL combine with `--coverage` and `--debug`.

#### Scenario: A run's record
- **WHEN** a program that sets `a = 1`, then `a = 2`, then `a = 2` is built with `--trace-values` and run with
  `QB64RUST_TRACE=t.txt`
- **THEN** `t.txt` holds one record in which `a` has 3 stores and the distinct values 1 (once) and 2 (twice)

#### Scenario: No variable, no file
- **WHEN** the same program runs without `QB64RUST_TRACE`
- **THEN** it writes no record and behaves as the build without `--trace-values`

#### Scenario: The build without the option
- **WHEN** a program is compiled without `--trace-values`
- **THEN** its C++ is byte for byte the C++ written before this change

#### Scenario: Two runs into one file
- **WHEN** two runs of a value-trace build name the same file
- **THEN** the file holds two records, the second after the first

#### Scenario: With coverage
- **WHEN** a program is built with `--trace-values --coverage` and run with both `QB64RUST_TRACE` and
  `QB64RUST_COVERAGE` set
- **THEN** each file holds its own record, and the counts record is the one the build with `--coverage` alone
  writes
