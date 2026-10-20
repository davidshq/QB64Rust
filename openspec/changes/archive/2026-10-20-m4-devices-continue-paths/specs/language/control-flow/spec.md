# Spec Delta

## ADDED Requirements

### Requirement: _CONTINUE
`_CONTINUE`, alone on its statement, SHALL go on with the next pass of the innermost open `FOR`, `DO` or `WHILE`
loop of the current body, whatever other blocks lie between. In a `FOR` loop the step SHALL still be added and the
limit tested; in a `DO … LOOP WHILE|UNTIL c` the test SHALL still run; in a `DO WHILE`, `DO UNTIL` or `WHILE` loop
the test at the top SHALL run. Outside such a loop it SHALL be a compile error that is not "not supported yet".

#### Scenario: Skipping in a FOR loop
- **WHEN** `FOR i = 1 TO 4: IF i = 2 THEN _CONTINUE` … `PRINT i;: NEXT` runs
- **THEN** it prints ` 1  3  4 `

#### Scenario: From inside SELECT CASE
- **WHEN** a `FOR` loop's body is a `SELECT CASE` whose `CASE 3, 6: _CONTINUE` skips the rest of the body
- **THEN** the loop goes on with the next value, as with the old compiler (`colors/color_with_dest`)

#### Scenario: LOOP UNTIL still tests
- **WHEN** `n = 0: DO: n = n + 1: IF n < 3 THEN _CONTINUE` … `PRINT n;: LOOP UNTIL n >= 5` runs
- **THEN** it prints ` 3  4  5 ` and ends

#### Scenario: The innermost loop
- **WHEN** a `_CONTINUE` stands in a `WHILE` loop that stands in a `FOR` loop
- **THEN** it goes on with the `WHILE` loop

#### Scenario: Outside a loop
- **WHEN** a program has `_CONTINUE` outside any loop, or in a SUB whose only loop is around its call
- **THEN** the compiler reports an error that is not "not supported yet" (`verification\v47_x01`, `x02`)

#### Scenario: Not alone on its statement
- **WHEN** a program has `_CONTINUE 1` in a loop, or `_CONTINUE = 3`
- **THEN** the compiler reports an error that is not "not supported yet", as the old compiler does (`v47_x03`,
  `x26`)

#### Scenario: In a GOSUB subroutine and as a single-line IF's ELSE
- **WHEN** a loop after a label reached by `GOSUB` holds `IF i = 2 THEN _CONTINUE`, or a loop holds
  `IF i <> 3 THEN PRINT i; ELSE _CONTINUE`
- **THEN** each goes on with the next pass, as with the old compiler (`v47_x24`, `x25`)
