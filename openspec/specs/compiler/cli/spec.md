# compiler/cli Specification

## Purpose
The `qb64rust` command line: compatible with the parts of qb64pe's command line that the corpus runner and the
M1 extension use, plus options for inspecting each compiler stage.

## Requirements

### Requirement: qb64pe-compatible compile
`qb64rust [-x] [-q] [-m] [-w] <file.bas> [-o <exe>]` SHALL compile the file to the executable (default: the
source name with `.exe`, next to the source). `-x`, `-q`, `-m` and `-w` SHALL be accepted; `-q` SHALL suppress
progress output. Exit status SHALL be 0 when the executable was built and 1 otherwise; on failure no executable
SHALL be left at the output path.

#### Scenario: Runner's compile line
- **WHEN** the corpus runner runs `qb64rust -q -m -x <prog>.bas -o <prog>.exe` on a slice program
- **THEN** `<prog>.exe` exists and the exit status is 0

#### Scenario: Program with an error
- **WHEN** the source has an error
- **THEN** the exit status is 1, the diagnostics are printed, and `<prog>.exe` does not exist

### Requirement: Diagnostics output
Each diagnostic SHALL be printed to stdout as `<file>:<line>:<column>: error: <message>`, with the file as given
on the command line and a 1-based byte column, followed by a summary line with the error count.

#### Scenario: Unknown statement
- **WHEN** line 3 of `p.bas` begins at column 1 with an unsupported keyword
- **THEN** the output contains a line starting `p.bas:3:1: error:`

### Requirement: Generate C++ only
`-z` SHALL stop after writing the C++ fragments to the build folder and print the folder's path.

#### Scenario: -z on a slice program
- **WHEN** `qb64rust -z p.bas` is run
- **THEN** the build folder holds `main0.txt` and the other fragments, and no executable is produced

### Requirement: Stage dumps
`--dump tokens|tree|typed|ir|cpp` SHALL print the named stage's output for the file to stdout and exit without
building, with status 0 if there were no errors and 1 otherwise.

#### Scenario: Typed dump
- **WHEN** `qb64rust --dump typed p.bas` is run on a valid program
- **THEN** each expression node is printed with its type, and the exit status is 0

### Requirement: Locating the reference runtime
The QB64pe reference clone SHALL be taken from `--qb64pe-root`, else from the environment variable
`QB64RUST_QB64PE_ROOT`, else from `..\QB64pe` next to the repository the binary was built from. If none exists,
the compiler SHALL fail with a message naming the option and the variable.

#### Scenario: No clone found
- **WHEN** no clone exists at any of the three places
- **THEN** compilation fails with exit status 1 and a message naming `--qb64pe-root` and `QB64RUST_QB64PE_ROOT`
