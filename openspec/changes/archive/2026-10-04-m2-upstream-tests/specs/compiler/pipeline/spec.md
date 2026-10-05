# Spec Delta

## ADDED Requirements

### Requirement: Unsupported constructs are marked
Every diagnostic SHALL say whether it reports an error in the program or a construct the compiler does not handle
yet. A diagnostic about a construct not handled yet SHALL be marked "not supported yet"; it SHALL NOT be used for
an error the old compiler also reports.

#### Scenario: Statement not handled yet
- **WHEN** a program uses `FOR`
- **THEN** the diagnostic at `FOR` is marked "not supported yet"

#### Scenario: Real error
- **WHEN** a program calls a SUB with the wrong number of arguments
- **THEN** the diagnostic is not marked

#### Scenario: Syntax the parser does not know
- **WHEN** the parser cannot read a statement because it uses syntax not handled yet (an operator such as `MOD`
  inside an otherwise supported statement)
- **THEN** the diagnostic is marked "not supported yet", not reported as a syntax error
