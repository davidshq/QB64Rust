# Spec Delta

## MODIFIED Requirements

### Requirement: Build options of a test run
`qb64rust test` SHALL take the options of a compile that choose roots and fixes (`--toolchain-root`,
`--qb64pe-root`, `--libqb-root`, `--include-root`, `--fixes`, `--fix`, `--no-settings`) with their meaning in a
compile, and SHALL reject the others (`-o`, `--dump`, `-y`) as a usage error with exit status 1.

#### Scenario: A rejected option
- **WHEN** `qb64rust test -o x.exe lib` runs
- **THEN** it is a usage error with exit status 1

#### Scenario: A fix on the command line
- **WHEN** `qb64rust test --fix whole-array-value=on lib` runs
- **THEN** each test program is built with that fix on

#### Scenario: A bad settings file beside a test file
- **WHEN** `qb64rust.toml` beside `resume_test.bas` holds `[fixes]` and `nonsense = true`, and `qb64rust test
  resume_test.bas --fixes off` runs
- **THEN** each of its tests is an error that names `qb64rust.toml` and its line 2, and the exit status is 1; with
  `--no-settings --fixes on` in place of `--fixes off`, the file is not read and the tests build and pass

`qb64rust test` SHALL take `--coverage`, which builds each test program as a compile's `--coverage` does. When
`QB64RUST_COVERAGE` names a file, each test SHALL write its record to a file of its own, and the runner SHALL append
that record to the named file after the test ends, one test at a time, so that the records of tests run at once
never mix. A record without its closing `end` line SHALL NOT be appended, and its test SHALL be an error. Without
`--coverage`, no test SHALL write a record, whatever `QB64RUST_COVERAGE` names.

#### Scenario: Counting the statements the tests run
- **WHEN** `qb64rust test --coverage lib/math_test.bas` runs with `QB64RUST_COVERAGE` naming a file
- **THEN** the file holds two whole records, one for each test, and each counts `AddUp&`'s statement in `math.bm`
  once; the same run without `--coverage` writes no file
