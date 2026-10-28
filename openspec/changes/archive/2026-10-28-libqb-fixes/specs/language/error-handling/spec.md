# Spec Delta

## MODIFIED Requirements

### Requirement: Error handler
`ON ERROR GOTO label` SHALL make the statements from `label` on the active error handler; `ON ERROR GOTO 0` SHALL
remove it. When a statement raises a trappable error and a handler is active and not already running, the rest of
the statement SHALL be skipped and the handler SHALL run, with `ERR` giving the error number. Without a handler,
the error SHALL be reported by the runtime as with the old compiler. Errors the runtime treats as fatal (for
example out of stack space, error 256) SHALL NOT be trappable. Error 11, division by zero, SHALL be trappable,
raised by `\`, `MOD` or `ERROR 11` (`study\39` row 12; a `QB64PE-DIVERGENCES.md` row).

#### Scenario: Handler runs
- **WHEN** `ON ERROR GOTO handler` precedes `ERROR 5`, and the handler prints `ERR`
- **THEN** it prints ` 5 `

#### Scenario: Fatal error is not trapped
- **WHEN** `ON ERROR GOTO handler` precedes `ERROR 256`
- **THEN** the handler does not run, and the program stops with "Out of stack space"

#### Scenario: ERROR 11 is trapped
- **WHEN** `ON ERROR GOTO handler` precedes `ERROR 11`, and the handler prints `ERR` and resumes next
- **THEN** it prints ` 11 `, and the program goes on

### Requirement: $ERRORLOCATION
`$ERRORLOCATION:ON` SHALL make the program record the line of each statement before it runs, as the old
compiler's program does, so that a critical error's message (out of stack space, the library errors) names that
line and its file: "Line: n (in main module)" or "Line: n (in file.bi)". `$ERRORLOCATION:OFF`, or neither, SHALL
keep today's message. The setting SHALL hold for the whole program, also from an included file alone (measured,
`verification\v58_e01`–`e26`). A division by zero is not a critical error (requirement "Error handler"): with no
handler it SHALL give the message of any other untrapped run-time error, with or without the setting.

#### Scenario: A critical error with the setting on
- **WHEN** a `$CONSOLE:ONLY` program with `$ERRORLOCATION:ON` recurses in a SUB until the stack runs out
- **THEN** its message names the line of the recursive call and "main module", as the old compiler's program's
  message does (`verification\v58`)

#### Scenario: In an included file
- **WHEN** the recursion is in a SUB of an included file
- **THEN** the message names the line in that file and the file, as the old compiler's program's message does

#### Scenario: The setting off
- **WHEN** the same program has `$ERRORLOCATION:OFF`, or no setting
- **THEN** the message ends with "Enable $ErrorLocation:ON for source location details.", as today

#### Scenario: Other errors are unchanged
- **WHEN** a program with `$ERRORLOCATION:ON` runs `ERROR 5` under a handler that prints `ERL; _ERRORLINE`, and
  another raises error 9 with no handler
- **THEN** each prints what the same program without the setting prints, as with the old compiler
  (`verification\v58_e05`, `e06`)

#### Scenario: A division by zero on a joined line
- **WHEN** a `$ERRORLOCATION:ON` program divides an integer by zero in a statement that starts on line 7 and
  continues on line 8 with `_`, with no handler and `QB64PE_NOPROMPT=y`
- **THEN** the message names line 8 and "Division by zero", as the old compiler's program's does
  (`verification\v58_e23`)

#### Scenario: A division by zero with the setting off
- **WHEN** a program without `$ERRORLOCATION` divides an integer by zero on line 6, with no handler and
  `QB64PE_NOPROMPT=y`
- **THEN** it prints `Runtime error: Line: 6 (in main module)` and `Division by zero`, as for any other untrapped
  error, where the old compiler's program prints its critical message with no line (`verification\v58_e21`)
