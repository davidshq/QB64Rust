# Spec Delta

## ADDED Requirements

### Requirement: Internal compiler error
When the compiler panics, it SHALL print `qb64rust: internal compiler error:` with the panic message, its source
location and the input file, and SHALL exit with code 3, which no other failure uses. It SHALL NOT leave an
executable behind.

#### Scenario: Forced panic
- **WHEN** `qb64rust` runs with `QB64RUST_TEST_PANIC=1`
- **THEN** it prints a line starting with `qb64rust: internal compiler error:` and exits with code 3

### Requirement: Compiler root for included files
The compiler root, against which an included file is looked up when it is not in the including file's folder,
SHALL be the folder holding the `qb64rust` executable, unless `--include-root <dir>` names another; a relative
`<dir>` is resolved once, against the working directory at start. Included files SHALL NOT be looked up relative
to the working directory itself (the old compiler changes to its own folder at start).

#### Scenario: Default root
- **WHEN** `qb64rust` runs without `--include-root`
- **THEN** included files not found next to the including file are looked up relative to the executable's folder
