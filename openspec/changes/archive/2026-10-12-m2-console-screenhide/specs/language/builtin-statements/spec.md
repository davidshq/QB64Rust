# Spec Delta

## MODIFIED Requirements

### Requirement: Supported built-in statements
`sema` SHALL compile these built-in statements: `OPEN`, `CLOSE`, `SEEK`, `KILL`, `NAME`, `MKDIR`, `RMDIR`, `CHDIR`,
`PRINT #`, `WRITE`, `INPUT`, `LINE INPUT`, `READ`, `RESTORE`, `DATA`, `SWAP`, `MID$ … =`, `RANDOMIZE`, `SHELL`,
`ENVIRON`, `_DEST`, `_SOURCE`, `_ECHO`, and the window statements `_SCREENHIDE`, `_SCREENSHOW`, `_SCREENICON`,
`_TITLE`, `_ICON`, `_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR`, `_WINDOWSIZELIMIT`, `_FREEIMAGE`
(`language/console-and-window`). Any other built-in statement SHALL be reported "not supported yet" at its first word, and a supported
statement used in a form this change leaves out SHALL be reported "not supported yet" at that form.

#### Scenario: Supported statement compiles
- **WHEN** `KILL "gone.tmp"` is compiled in a program with no other statement
- **THEN** there is no diagnostic

#### Scenario: With CALL
- **WHEN** `CALL KILL("gone.tmp")` is compiled
- **THEN** it compiles as `KILL "gone.tmp"` does, as with the old compiler

#### Scenario: Unsupported statement
- **WHEN** a program contains `LOCATE 1, 1`
- **THEN** the only diagnostic is "not supported yet" at `LOCATE`

#### Scenario: Unsupported form of a supported statement
- **WHEN** a program contains `RESTORE 100`
- **THEN** the diagnostic is "not supported yet" and names line numbers

#### Scenario: A statement and a function of one name
- **WHEN** a program has `d& = _DEST` and `_DEST d&`
- **THEN** the first is the function and the second the statement, and both compile
