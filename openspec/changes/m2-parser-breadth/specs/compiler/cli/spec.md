# Spec Delta

## ADDED Requirements

### Requirement: Internal compiler error
When the compiler panics, it SHALL print `qb64rust: internal compiler error:` with the panic message, its source
location and the input file, and SHALL exit with code 3, which no other failure uses. It SHALL NOT leave an
executable behind.

#### Scenario: Forced panic
- **WHEN** `qb64rust` runs with `QB64RUST_TEST_PANIC=1`
- **THEN** it prints a line starting with `qb64rust: internal compiler error:` and exits with code 3
