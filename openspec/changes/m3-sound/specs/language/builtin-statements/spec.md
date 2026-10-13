# Spec Delta

## ADDED Requirements

### Requirement: A statement with a template after CALL
A supported built-in statement whose forms are read by a template SHALL compile after `CALL` as the old compiler
compiles it: the name alone is the statement with nothing after it, and the name with one pair of parentheses
around all that follows is the statement with what stands between them. Any other form SHALL be a compile error
that is not "not supported yet".

#### Scenario: Statements with a rule of their own
- **WHEN** a program has `CALL SWAP(a, b)`
- **THEN** it MAY be reported "not supported yet" (as `CLOSE` and the `MID$` statement after `CALL`)

#### Scenario: The template between the parentheses
- **WHEN** `CALL CLS`, `CALL LOCATE(, 2)`, `CALL LINE((1, 2)-(3, 4), 5, BF)`, `CALL SOUND(440, 1 WAIT)` and
  `CALL PLAY(a$, b$)` are compiled
- **THEN** each compiles to the runtime call of `CLS`, `LOCATE , 2`, `LINE (1, 2)-(3, 4), 5, BF`,
  `SOUND 440, 1 WAIT` and `PLAY a$, b$`

#### Scenario: Not one pair of parentheses around all that follows
- **WHEN** a program has `CALL CLS()`, `CALL LOCATE 1, 2`, `CALL SOUND(440, 1) WAIT` or `CALL VIEW PRINT`
- **THEN** the compiler reports an error that is not "not supported yet", as the old compiler rejects each

#### Scenario: No form matches
- **WHEN** a program has `CALL PSET(1, 2)`, `CALL SOUND(440)` or `CALL PLAY`
- **THEN** the compiler reports that the statement matches none of its forms

## MODIFIED Requirements

### Requirement: Supported built-in statements
`sema` SHALL compile these built-in statements: `OPEN`, `CLOSE`, `SEEK`, `KILL`, `NAME`, `MKDIR`, `RMDIR`, `CHDIR`,
`PRINT #`, `WRITE`, `INPUT`, `LINE INPUT`, `READ`, `RESTORE`, `DATA`, `SWAP`, `MID$ … =`, `RANDOMIZE`, `SHELL`,
`ENVIRON`, `_DEST`, `_SOURCE`, `_ECHO`, the window statements `_SCREENHIDE`, `_SCREENSHOW`, `_SCREENICON`,
`_TITLE`, `_ICON`, `_MOUSESHOW`, `_MOUSEHIDE`, `_MOUSECURSOR`, `_WINDOWSIZELIMIT`, `_FREEIMAGE`, `_FULLSCREEN`
(`language/console-and-window`), the screen statements `SCREEN`, `CLS`, `COLOR`, `LOCATE`, `WIDTH`, `VIEW
PRINT`, `PSET`, `PRESET`, `LINE`, `CIRCLE`, `PAINT`, `DRAW`, `VIEW`, `WINDOW`, `PALETTE`, `PCOPY`, `_LIMIT`,
`_DELAY`, `SLEEP`, `_DISPLAY`, graphics `GET` and graphics `PUT` (`language/screen-output`), and the statements of
conventional memory `DEF SEG`, `POKE`, `BLOAD`, `BSAVE` (`language/conventional-memory`), and the sound
statements `SOUND`, `PLAY`, `BEEP` (`language/sound`). Any other built-in
statement SHALL be reported "not supported yet" at its first word, and a supported
statement used in a form this change leaves out SHALL be reported "not supported yet" at that form.

#### Scenario: Supported statement compiles
- **WHEN** `KILL "gone.tmp"` is compiled in a program with no other statement
- **THEN** there is no diagnostic

#### Scenario: With CALL
- **WHEN** `CALL KILL("gone.tmp")` is compiled
- **THEN** it compiles as `KILL "gone.tmp"` does, as with the old compiler

#### Scenario: Unsupported statement
- **WHEN** a program contains `LPRINT "c"`
- **THEN** the only diagnostic is "not supported yet" at `LPRINT`

#### Scenario: Unsupported form of a supported statement
- **WHEN** a program contains `RESTORE 100`
- **THEN** the diagnostic is "not supported yet" and names line numbers

#### Scenario: A statement and a function of one name
- **WHEN** a program has `d& = _DEST` and `_DEST d&`
- **THEN** the first is the function and the second the statement, and both compile

#### Scenario: Two statements of one name
- **WHEN** a program has `VIEW PRINT 2 TO 10` and `VIEW (10, 10)-(50, 50)`
- **THEN** each compiles to the runtime call the old compiler writes for it

#### Scenario: The graphics form and the file form of one name
- **WHEN** a program has `PUT (10, 10), a, PSET` and `PUT #1, , x`
- **THEN** the first compiles to the runtime call the old compiler writes for it, and the second is reported "not
  supported yet"

#### Scenario: A statement of two words
- **WHEN** a program has `DEF SEG = 0` and `DEF SEG`
- **THEN** each compiles to the runtime call the old compiler writes for it, and what the compiler reports for a
  `DEF FN` line does not change
