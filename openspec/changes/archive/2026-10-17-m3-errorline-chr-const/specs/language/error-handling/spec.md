# Spec Delta

## ADDED Requirements

### Requirement: The line and file of an error
`_ERRORLINE` (LONG) SHALL give the main file's line where the last error was raised, 0 before any and kept after
`RESUME`; in an included file, the main file's line of the `$INCLUDE` of the outermost included file.
`_INCLERRORLINE` (LONG) SHALL give the line in the innermost included file, `_INCLERRORFILE$` its name without
its folder; 0 and empty in the main file (measured, `verification\v43_a_errorline`, `v43_b_include`).

#### Scenario: In the main file
- **WHEN** a handler that prints `_ERRORLINE; _INCLERRORLINE` and resumes next handles `ERROR 5` on line 9
- **THEN** it prints ` 9  0 `, and `_ERRORLINE` after the `RESUME NEXT` is still 9

#### Scenario: In an included file
- **WHEN** the file included on line 12 of the main file includes a file whose third line raises an error, and the
  handler prints `_ERRORLINE; _INCLERRORLINE; _INCLERRORFILE$`
- **THEN** it prints ` 12  3 ` and the inner file's name

#### Scenario: Not in a CONST
- **WHEN** `CONST c = _ERRORLINE` is compiled
- **THEN** it is a compile error

### Requirement: An error's message
`_ERRORMESSAGE$` SHALL give the message of the last error, and `_ERRORMESSAGE$(code)` the message of the code, a
LONG rounded half to even (measured, `verification\v43_a_errorline`).

#### Scenario: A message
- **WHEN** `PRINT _ERRORMESSAGE$(5); _ERRORMESSAGE$(9.6)` runs
- **THEN** it prints `Illegal function callDuplicate definition`

### Requirement: The line of a joined line
A line joined to the next by a line continuation SHALL be numbered as the last line of the join, for every
statement on it, in the functions above and in the message of an error nothing traps (measured,
`verification\v43_d_continued`, `v43_e_continued_untrapped`).

#### Scenario: A joined line
- **WHEN** `ERROR 14: x = 1 + _` stands on line 32 and `2` on line 33, under a handler that prints `_ERRORLINE`
- **THEN** it prints ` 33 `
