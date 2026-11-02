# Spec Delta

## ADDED Requirements

### Requirement: The exit code of D-012
In every suite that compares exit codes, the runner SHALL expect exit code 1 where the recording, the `.exitcode`
sidecar or the default says 0, for a run of the new compiler's program whose trace names `D-012`. Such a run needs no
`.divergences` sidecar for `D-012`. A run whose trace does not name `D-012` SHALL be held to the recorded code as
before, and a run with `qb64pe.exe` SHALL be recorded as it exits.

#### Scenario: A recording of an untrapped error
- **WHEN** a verification program's recording ends in `Runtime error` and `exit=0`, and the new compiler's program
  prints the same, traces `D-012` and exits with 1
- **THEN** it passes, with no `.divergences` sidecar

#### Scenario: Exit code 1 without the trace
- **WHEN** a corpus program with no `.exitcode` exits with 1 and its trace does not name `D-012`
- **THEN** it fails at the exit code, as before

#### Scenario: The trace with exit code 0
- **WHEN** a program's trace names `D-012` but it exits with 0
- **THEN** it fails at the exit code

#### Scenario: Recording with the old compiler
- **WHEN** the verification suite records with `qb64pe.exe` a program that ends on an untrapped error
- **THEN** the recording holds `exit=0`, as before
