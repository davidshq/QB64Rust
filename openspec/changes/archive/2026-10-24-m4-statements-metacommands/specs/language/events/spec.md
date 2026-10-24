# Spec Delta

## ADDED Requirements

### Requirement: An event handler without its closing parenthesis
`ON STRIG(b GOSUB label`, `ON KEY(n GOSUB label` and `ON TIMER(t GOSUB label`, with the `)` missing before
`GOSUB`, SHALL be compile errors with the old compiler's "Expected )" as their note (measured, `verification\v47_x08`,
`v57_e01`, `e02`).

#### Scenario: ON STRIG
- **WHEN** a program has `ON STRIG(1 GOSUB h`
- **THEN** the compile fails with an error at `GOSUB`, not marked "not supported yet" (`verification\v47_x08`)

#### Scenario: ON KEY and ON TIMER
- **WHEN** a program has `ON KEY(1 GOSUB h` or `ON TIMER(1 GOSUB h`
- **THEN** the compile fails with an error at `GOSUB`, with "Expected )" as its note (`verification\v57_e01`, `e02`)
