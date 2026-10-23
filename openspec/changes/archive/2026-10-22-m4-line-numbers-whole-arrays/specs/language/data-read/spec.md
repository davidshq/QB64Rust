# Spec Delta

## MODIFIED Requirements

### Requirement: RESTORE
`RESTORE` SHALL make the next `READ` start at the program's first item, and `RESTORE label` at the first item of
the first `DATA` statement after that label in the file, as the old compiler resolves it: the label MAY stand in
the main module or in any procedure, whatever body the `RESTORE` is in, and when no `DATA` follows it the next
`READ` SHALL raise error 4. A label that does not exist SHALL be a compile error, as SHALL a label name that more
than one body has. A line number SHALL be a target as a label is, with the same rules.

#### Scenario: Back to the start
- **WHEN** `READ a: RESTORE: READ b: PRINT a; b` runs with `DATA 1,2`
- **THEN** it prints ` 1  1 `

#### Scenario: To a label
- **WHEN** `RESTORE two: READ s$: PRINT s$` runs with `DATA 1` and, after the label `two:`, `DATA x,9`
- **THEN** it prints `x`

#### Scenario: Unknown label
- **WHEN** `RESTORE nowhere` is compiled in a program with no label `nowhere`
- **THEN** it is a compile error

#### Scenario: A label of a procedure
- **WHEN** the main module runs `RESTORE inp: READ s$: PRINT s$` and SUB `p` holds the label `inp:` followed by
  `DATA q`
- **THEN** it prints `q`

#### Scenario: A label two bodies have
- **WHEN** `RESTORE lab` is compiled in a program whose main module and one SUB each have a label `lab:`
- **THEN** it is a compile error

#### Scenario: To a line number
- **WHEN** `RESTORE 100: READ s` runs and the line numbered 100 is `100 DATA d6`
- **THEN** `s` reads `d6`, as recorded (`verification\v22_c_restore`)

#### Scenario: A line number no line has
- **WHEN** `RESTORE 100` is compiled in a program with no line 100
- **THEN** it is a compile error, as recorded (`verification\v22_x86_restore_missing_line_number`)
