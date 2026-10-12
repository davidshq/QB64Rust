# Spec Delta

## ADDED Requirements

### Requirement: Constants of the auto-included file
Every program SHALL know the constants QB64pe declares in its auto-included file `beforefirstline.bi` (the
Windows 64-bit branches), as if their `CONST` lines stood above the program's first line: visible in the main
module and in every procedure, each with the value and type the `CONST` evaluator gives its expression. A constant
whose expression the evaluator does not compute yet SHALL be reported "not supported yet" where it is used, and
SHALL NOT affect a program that does not use it.

#### Scenario: _TRUE and _FALSE
- **WHEN** `PRINT _TRUE; _FALSE` runs
- **THEN** it prints `-1  0 `, as with the old compiler

#### Scenario: In a procedure
- **WHEN** a SUB runs `IF _TRUE THEN PRINT _SECS_IN_MIN * _MINS_IN_HOUR`
- **THEN** it prints ` 3600 `

#### Scenario: A constant computed from other constants
- **WHEN** `PRINT _ONE_MB` runs
- **THEN** it prints what the old compiler's program prints for it

#### Scenario: A value not computed yet
- **WHEN** a program uses `_STR_CRLF`, whose `CONST` line needs `CHR$`
- **THEN** the only diagnostic is "not supported yet" at that use

#### Scenario: Not used
- **WHEN** a program names no constant of the auto-included file
- **THEN** it compiles as before, with no diagnostic about them
