# Spec Delta

## ADDED Requirements

### Requirement: A constant as the length of a member
A `TYPE` member `m AS STRING * n`, with `n` the name of a numeric `CONST` without a suffix defined above the
`TYPE` block, SHALL be a fixed-length string member of the constant's value as the old compiler rounds it for a
`DIM` (requirement "Fixed-length string declarations"). Any other name SHALL be the old compiler's error "Invalid
number/constant after STRING * type" (measured, `verification\v56_x30`, `v57_t01`–`t03`).

#### Scenario: A CONST above the block
- **WHEN** a program has `CONST n = 2`, a `TYPE r` with the member `f AS STRING * n`, and `DIM x AS r`
- **THEN** `PRINT LEN(x)` prints ` 2 ` (`verification\v56_x30`)

#### Scenario: A CONST of an expression
- **WHEN** the constant is `CONST n = 2 + 1`
- **THEN** `PRINT LEN(x)` prints ` 3 ` (`verification\v57_t01`)

#### Scenario: A CONST below the block, or with a suffix
- **WHEN** `CONST n = 2` stands below the `TYPE` block, or the member is `f AS STRING * n&` with `CONST n& = 4` above
- **THEN** the compile fails with an error at the length in the member (`verification\v57_t02`, `t03`)

#### Scenario: A variable as the length
- **WHEN** a `TYPE` member is `f AS STRING * v` with `v` a variable
- **THEN** the compile fails with an error at `v`, as for a name that is no `CONST` above the block
  (`verification\v57_t02`; `v51_x75` records the same for a `DIM`)
