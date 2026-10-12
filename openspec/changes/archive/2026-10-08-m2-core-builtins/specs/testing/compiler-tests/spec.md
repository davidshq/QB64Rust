# Spec Delta

## ADDED Requirements

### Requirement: Built-in coverage
Every built-in function that `sema` compiles SHALL be called in at least one program of `tests/corpus/slice.list`
(so its output is compared with the old compiler's in tier 2) and in at least one `typed` front-end test (so its
result type and argument conversions are pinned in tier 1). `cargo test` SHALL fail and name the built-in when
either is missing.

#### Scenario: Built-in added without a slice program
- **WHEN** `sema` starts compiling a built-in that no slice program calls
- **THEN** `cargo test` fails and names that built-in

#### Scenario: Built-in added without a typed test
- **WHEN** `sema` starts compiling a built-in that no `typed` front-end test calls
- **THEN** `cargo test` fails and names that built-in
