# Spec Delta

## MODIFIED Requirements

### Requirement: Built-in coverage
Every built-in function and every built-in statement that `sema` compiles SHALL be used in at least one program of
`tests/corpus/slice.list` (so its output is compared with the old compiler's in tier 2) and in at least one
front-end test that pins it in tier 1: a `typed` test for a function (its result type and argument conversions),
an `ir` test for a statement (its slots, places and items). `cargo test` SHALL fail and name the built-in when
either is missing.

#### Scenario: Built-in added without a slice program
- **WHEN** `sema` starts compiling a built-in that no slice program calls
- **THEN** `cargo test` fails and names that built-in

#### Scenario: Built-in added without a typed test
- **WHEN** `sema` starts compiling a built-in that no `typed` front-end test calls
- **THEN** `cargo test` fails and names that built-in

#### Scenario: Statement added without an ir test
- **WHEN** `sema` starts compiling a built-in statement that no `ir` front-end test uses
- **THEN** `cargo test` fails and names that statement
