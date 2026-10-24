# Spec Delta

## ADDED Requirements

### Requirement: $ERRORLOCATION
`$ERRORLOCATION:ON` SHALL make the program record the line of each statement before it runs, as the old
compiler's program does, so that a critical error's message (out of stack space, division by zero, the library
errors) names that line and its file: "Line: n (in main module)" or "Line: n (in file.bi)". `$ERRORLOCATION:OFF`,
or neither, SHALL keep today's message. The setting SHALL hold for the whole program, also from an included file
alone (measured, `verification\v58_e01`–`e26`).

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
  continues on line 8 with `_`
- **THEN** the message names line 8, as the old compiler's program's does (`verification\v58_e23`)

### Requirement: $ERRORLOCATION set twice
A program SHALL be the old compiler's error when its `$ERRORLOCATION` lines do not all agree: one `ON` and one
`OFF` in any order, in the main file or an included one, and in a branch of `$IF` that is taken. The error SHALL
stand at the first `$ERRORLOCATION` line in file order whose setting differs from the last line's (measured,
`verification\v58_e08`–`e12`). Two lines with the same setting SHALL compile.

#### Scenario: On and then off
- **WHEN** a program has `$ERRORLOCATION:ON` on line 3 and `$ERRORLOCATION:OFF` on line 5, or the other way round
- **THEN** the compile fails with an error whose note is "$ErrorLocation is a whole-program option and cannot be
  changed within the program", at line 3 (`verification\v58_e08`, `e09`)

#### Scenario: Across an included file or a $IF
- **WHEN** the `OFF` stands in an included file, or in a `$IF WIN THEN` branch, after the main file's `ON`
- **THEN** the compile fails with that error at the main file's line (`verification\v58_e10`–`e12`); in a branch
  that is not taken it changes nothing (`e13`)

#### Scenario: The same setting twice
- **WHEN** a program has `$ERRORLOCATION:ON` twice
- **THEN** it compiles
