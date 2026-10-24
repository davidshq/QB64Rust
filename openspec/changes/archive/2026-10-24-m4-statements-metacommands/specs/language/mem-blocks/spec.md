# Spec Delta

## ADDED Requirements

### Requirement: _MEMPUT and _MEMFILL after CALL
`CALL _MEMPUT(…)` and `CALL _MEMFILL(…)` SHALL be compile errors, with the old compiler's "Command not
implemented" as their note, for each form a recording shows the old compiler rejects, the `AS type` form among
them. `_MEMPUT` and `_MEMFILL` without `CALL` SHALL stay as they are.

#### Scenario: _MEMPUT with a type after CALL
- **WHEN** a program has `CALL _MEMPUT(m, m.OFFSET, 1 AS LONG)`
- **THEN** the compile fails with an error at the statement, not marked "not supported yet"
  (`verification\v32_x89`)

#### Scenario: _MEMFILL with a type after CALL
- **WHEN** a program has `CALL _MEMFILL(m, m.OFFSET, 8, 1 AS LONG)`
- **THEN** the compile fails with an error at the statement (`verification\v32_x91`)
