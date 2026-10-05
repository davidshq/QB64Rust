# Spec Delta

## MODIFIED Requirements

### Requirement: qb64pe-compatible compile
`qb64rust [-x] [-q] [-m] [-w] [-f:<setting>=<value>]... <file.bas> [-o <exe>]` SHALL compile the file to the
executable (default: the source name with `.exe`, next to the source). `-x`, `-q`, `-m` and `-w` SHALL be accepted;
`-q` SHALL suppress progress output. `-f:OptimizeCppProgram=true|false` SHALL build the C++ with or without
optimisation as `qb64pe` does; `-f:StripDebugSymbols=<value>` SHALL be accepted and ignored; any other setting
SHALL be an error "setting not supported yet". Exit status SHALL be 0 when the executable was built and 1
otherwise; on failure no executable SHALL be left at the output path.

#### Scenario: Runner's compile line
- **WHEN** the corpus runner runs `qb64rust -q -m -x <prog>.bas -o <prog>.exe` on a slice program
- **THEN** `<prog>.exe` exists and the exit status is 0

#### Scenario: Program with an error
- **WHEN** the source has an error
- **THEN** the exit status is 1, the diagnostics are printed, and `<prog>.exe` does not exist

#### Scenario: Compile suite's settings
- **WHEN** the upstream compile suite runs `qb64rust -f:OptimizeCppProgram=true -f:StripDebugSymbols=false -q -m -x
  <prog>.bas -o <exe>` on a program in `tests/upstream/pass.list`
- **THEN** the executable is built and the exit status is 0

#### Scenario: Unknown setting
- **WHEN** `-f:GenerateLicenseFile=true` is given
- **THEN** the exit status is 1 with the message that the setting is not supported yet, and no executable exists

### Requirement: Diagnostics output
Each diagnostic SHALL be printed to stdout as `<file>:<line>:<column>: error: <message>`, with the file as given
on the command line and a 1-based byte column; a diagnostic marked "not supported yet" SHALL be printed as
`<file>:<line>:<column>: error: not supported yet: <message>`. A summary line with the error count SHALL follow,
saying how many of the errors are marked.

#### Scenario: Unknown statement
- **WHEN** line 3 of `p.bas` begins at column 1 with an unsupported keyword
- **THEN** the output contains a line starting `p.bas:3:1: error: not supported yet:`

#### Scenario: Summary
- **WHEN** a program has three errors, two of them marked
- **THEN** the last line says `3 errors (2 not supported yet)`
