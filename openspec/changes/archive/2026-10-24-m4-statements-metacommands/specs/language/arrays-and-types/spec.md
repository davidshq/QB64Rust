# Spec Delta

## ADDED Requirements

### Requirement: LBOUND and UBOUND of a parenthesised array name
`LBOUND((a))` and `UBOUND((a))`, with or without a dimension and with any number of parentheses, SHALL give the
bound of the array `a` as without the inner parentheses, for a static or dynamic array and an array parameter, as
in the old compiler (measured, `verification\v51_x34`, `v57_p08`–`p10`, `p14`, `p15`). Other expressions in
parentheses SHALL keep the errors and marks they have.

#### Scenario: A static array
- **WHEN** a program has `DIM a(10) AS LONG` and prints `LBOUND((a))`
- **THEN** it prints ` 0 ` (`verification\v51_x34`)

#### Scenario: With a dimension
- **WHEN** a program has `DIM b(2 TO 5, 3) AS INTEGER` and prints `UBOUND((b))`, `UBOUND((b), 1)` and
  `UBOUND((b), 2)`
- **THEN** it prints ` 5 `, ` 5 ` and ` 3 ` (`verification\v57_p09`)
