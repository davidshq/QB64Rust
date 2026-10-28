# Spec Delta

## ADDED Requirements

### Requirement: Suites build with the fixes off
The test runner SHALL build every program of every suite with `--fixes off`, and SHALL say so in its first line, so
that a `qb64rust.toml` in the tree or above it changes no result. A run with `qb64pe.exe` SHALL pass no such
option.

#### Scenario: A settings file in the tree
- **WHEN** a `qb64rust.toml` turning every fix on stands in the repository's root and the verification suite runs
  with the new compiler
- **THEN** every program is compiled with `--fixes off`, and the results are those of a run without the file

### Requirement: A program recorded with the fixes on
A verification program with a sidecar `tests\verification\fixed\<name>.out.txt` SHALL be built and run a second
time with `--fixes on`, and that run SHALL pass when its output equals the sidecar by the compare's rules; it
SHALL be reported as `<name> [fixes on]`. Each switched fix SHALL have at least one such program.

#### Scenario: Both ways
- **WHEN** `v50_c_checking` has a sidecar `fixed\v50_c_checking.out.txt` and the suite runs
- **THEN** the program is compared once with its `qb64pe.exe` recording (fixes off) and once with the sidecar
  (fixes on), and each result is counted on its own
