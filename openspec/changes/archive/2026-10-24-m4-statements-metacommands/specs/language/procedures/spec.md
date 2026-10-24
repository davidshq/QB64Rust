# Spec Delta

## ADDED Requirements

### Requirement: A program FUNCTION as a statement
A program FUNCTION used as a statement SHALL be a compile error, whatever its name: `f 1` and `f(1)` with the old
compiler's "Syntax error" as the note, and `CALL f(1)` with its "Expected CALL sub-name [(...)]" (measured,
`verification\v21_x56`, `v57_f01`–`f11`).

#### Scenario: A FUNCTION with a fixed-length name
- **WHEN** a program defines `FUNCTION fs$5 (x AS STRING)` and has `CALL fs$5("a")`
- **THEN** the compile fails with an error at the `CALL`, not marked "not supported yet"
  (`verification\v21_x56`)

#### Scenario: Without CALL
- **WHEN** a program defines `FUNCTION g& (a)` and has `g& 1` or `g&(1)`
- **THEN** the compile fails with an error, with "Syntax error" as its note (`verification\v57_f04`, `f05`)

### Requirement: SHARED in the main module
A `SHARED` statement in the main module SHALL be a compile error, with the old compiler's "SHARED must be used
within a SUB/FUNCTION" as its note, whether the name is declared or not. `DIM SHARED` and `REDIM SHARED` SHALL stay
as they are.

#### Scenario: SHARED of a declared variable
- **WHEN** a program has `DIM x AS LONG` and then `SHARED x` in the main module
- **THEN** the compile fails with an error at `SHARED`, not marked "not supported yet"
  (`tests\snippets\qb64fresh\shared_variables__shared_outside_procedure_error`)

#### Scenario: SHARED of a name never declared
- **WHEN** the main module has `SHARED q`
- **THEN** the compile fails with the same error (`verification\v57_e13`)

### Requirement: ALIAS in a DECLARE line
`ALIAS name` in a `DECLARE SUB` or `DECLARE FUNCTION` line outside `DECLARE LIBRARY`, the name quoted or not and
also empty, SHALL be accepted and change nothing, as the old compiler ignores the line (requirement "DECLARE
lines"); the alias names no procedure (measured, `verification\v37_x08`, `v57_a01`–`a04`).

#### Scenario: ALIAS of a program SUB
- **WHEN** a program has `DECLARE SUB s ALIAS t ()` and defines `SUB s`
- **THEN** it compiles and runs as without the `DECLARE` line (`verification\v37_x08`)

#### Scenario: The alias as a statement
- **WHEN** that program also has the statement `t`
- **THEN** the compile fails with an error at `t`, as with no `DECLARE` line (`verification\v57_a04`)
