# Spec Delta

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
statements `SOUND`, `PLAY`, `BEEP` (`language/sound`), `PRINT USING` (`language/print-using`), the statements of
files read and written by position `GET`, `PUT`, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`
(`language/record-files`), `LPRINT` and `CHAIN`, and the image, font and mouse statements
(`language/images`, `language/fonts`, `language/mouse`): `_SAVEIMAGE`, `_PUTIMAGE`, `_MAPTRIANGLE`,
`_DEPTHBUFFER`, `_CLEARCOLOR`, `_SETALPHA`, `_BLEND`, `_DONTBLEND`, `_PALETTECOLOR`, `_COPYPALETTE`,
`_AUTODISPLAY`, `_CLIPBOARDIMAGE`, `_PRINTIMAGE`, `_FONT`, `_FREEFONT`, `_PRINTSTRING`, `_PRINTMODE`,
`_UPRINTSTRING`, `_MAPUNICODE`, `_MOUSEMOVE`, and the statements of memory blocks (`language/mem-blocks`):
`_MEMGET`, `_MEMPUT`, `_MEMFILL`, `_MEMCOPY`, `_MEMFREE`, and `_ASSERT` (`language/asserts-and-debug`). Any
other built-in
statement SHALL be reported "not supported yet" at its first word, and a supported
statement used in a form this change leaves out SHALL be reported "not supported yet" at that form.

#### Scenario: Supported statement compiles
- **WHEN** `KILL "gone.tmp"` is compiled in a program with no other statement
- **THEN** there is no diagnostic

#### Scenario: With CALL
- **WHEN** `CALL KILL("gone.tmp")` is compiled
- **THEN** it compiles as `KILL "gone.tmp"` does, as with the old compiler

#### Scenario: Unsupported statement
- **WHEN** a program contains `STOP`
- **THEN** the only diagnostic is "not supported yet" at `STOP`

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
- **THEN** each compiles to the runtime call the old compiler writes for it: the first to the graphics entry, the
  second to the file entry

#### Scenario: A statement of two words
- **WHEN** a program has `DEF SEG = 0` and `DEF SEG`
- **THEN** each compiles to the runtime call the old compiler writes for it, and what the compiler reports for a
  `DEF FN` line does not change

#### Scenario: An image statement
- **WHEN** `_PUTIMAGE (0, 0), img` is compiled in a program where `img` is a LONG
- **THEN** there is no diagnostic

#### Scenario: A statement of memory blocks
- **WHEN** `DIM m AS _MEM: m = _MEMNEW(8): _MEMPUT m, m.OFFSET, 1 AS LONG: _MEMFREE m` is compiled
- **THEN** there is no diagnostic

