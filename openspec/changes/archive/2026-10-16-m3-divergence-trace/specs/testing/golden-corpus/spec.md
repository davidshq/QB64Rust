## ADDED Requirements

### Requirement: The run-time trace of divergences
A program built by the new compiler SHALL, when the environment variable `QB64RUST_DIVERGENCES` names a file,
append the line `D-0nn` to it the first time in a run that it takes the path of row D-nn of `DIVERGENCES.md`, for
every row with a run-time path, and SHALL write nothing without the variable. The runner SHALL set the variable for
every run of the corpus and compile suites, and SHALL fail a program whose trace names a row its
`<name>.divergences` file (one `D-0nn` per line, `#` comments) does not list.

#### Scenario: A row the program does not list
- **WHEN** a corpus program stores into `a(9).m` for `DIM a(3)` of a user type and has no `.divergences` file
- **THEN** its output may equal the recording, and the runner still fails it with the stage "divergences", naming
  `D-004`

#### Scenario: A listed row
- **WHEN** the same program has a `.divergences` file that lists `D-004`
- **THEN** it passes

#### Scenario: Once per row
- **WHEN** a program takes the path of D-006 in a loop of a thousand passes
- **THEN** the trace holds one line `D-006`

#### Scenario: Without the variable
- **WHEN** the program runs without `QB64RUST_DIVERGENCES`
- **THEN** it writes no trace file and its output is the same
