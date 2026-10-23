# Spec Delta

## ADDED Requirements

### Requirement: A jump to a built-in's name
A line that is a built-in statement's name and a colon (`BEEP:`) SHALL be that statement, not a label. A line
that is a built-in function's name and a colon, and a jump to a built-in's name whether a line of that name
stands in the file or not, SHALL be compile errors that are not "not supported yet", as the old compiler's are.

#### Scenario: GOTO a built-in statement's name
- **WHEN** a program has `BEEP: l = 1` and then `GOTO BEEP`, or the same with `CLS` or `SOUND`
- **THEN** each is a compile error that is not "not supported yet" (`verification\v29_x78_beep_goto`,
  `v29_x79_sound_goto`, `v27_x63_goto_statement_name`)

#### Scenario: The other jumps
- **WHEN** a program has `GOTO abs` or `GOTO CLS` with no such line, `ON 1 GOTO BEEP`, `GOSUB BEEP`, `RESTORE
  BEEP` or `ON ERROR GOTO BEEP` beside a line `BEEP: l = 1`
- **THEN** each is a compile error that is not "not supported yet" (the old compiler: "Invalid label!" for `GOTO`
  and `ON … GOTO`, "Invalid label" for the others; `verification\v55_x36`–`x38`, `x42`, `x43`, `x45`)

#### Scenario: A line named like a built-in function
- **WHEN** a program has the line `abs: l = 1` or `LEN: l = 1`
- **THEN** it is a compile error on that line that is not "not supported yet" (the old compiler: "Syntax error";
  `verification\v55_x35`, `x44`)
