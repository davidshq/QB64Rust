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
statements `SOUND`, `PLAY`, `BEEP` and the sound handle statements `_SNDCLOSE`, `_SNDPLAY`, `_SNDPLAYCOPY`, `_SNDPLAYFILE`, `_SNDPAUSE`, `_SNDSTOP`, `_SNDLOOP`, `_SNDVOL`, `_SNDBAL`, `_SNDSETPOS`, `_SNDLIMIT`, `_SNDRAW`, `_SNDRAWBATCH`, `_SNDRAWDONE`, `_MIDISOUNDBANK`, `_WAVE` (`language/sound`), `PRINT USING` (`language/print-using`), the statements of
files read and written by position `GET`, `PUT`, `FIELD`, `LSET`, `RSET`, `LOCK`, `UNLOCK`
(`language/record-files`), `LPRINT` and `CHAIN`, and the image, font and mouse statements
(`language/images`, `language/fonts`, `language/mouse`): `_SAVEIMAGE`, `_PUTIMAGE`, `_MAPTRIANGLE`,
`_DEPTHBUFFER`, `_CLEARCOLOR`, `_SETALPHA`, `_BLEND`, `_DONTBLEND`, `_PALETTECOLOR`, `_COPYPALETTE`,
`_AUTODISPLAY`, `_CLIPBOARDIMAGE`, `_PRINTIMAGE`, `_FONT`, `_FREEFONT`, `_PRINTSTRING`, `_PRINTMODE`,
`_UPRINTSTRING`, `_MAPUNICODE`, `_MOUSEMOVE`, and the statements of memory blocks (`language/mem-blocks`):
`_MEMGET`, `_MEMPUT`, `_MEMFILL`, `_MEMCOPY`, `_MEMFREE`, and `_ASSERT` (`language/asserts-and-debug`), and `CLEAR` and `_CLIPBOARD$ = s$`. Any
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

## ADDED Requirements

### Requirement: CLEAR
`CLEAR [a][, [b][, c]]` SHALL reset what the old compiler's runtime resets: every variable of the main module and
every `STATIC` and `SHARED` variable to zero or empty, each static array's elements, each dynamic array released,
the `DATA` position, the open files closed, the error handler and the `RETURN` points.

#### Scenario: Variables reset
- **WHEN** `a = 5: s$ = "x": DIM b(3): b(1) = 2: CLEAR: PRINT a; LEN(s$); b(1)` runs
- **THEN** it prints ` 0  0  0 `, as with the old compiler

#### Scenario: A dynamic array released
- **WHEN** `REDIM d(5): CLEAR` is followed by `PRINT d(1)` under a handler that prints `ERR`
- **THEN** the handler prints 9, as with the old compiler

#### Scenario: A STATIC variable and the DATA position
- **WHEN** a SUB counts calls in a `STATIC` variable and a `READ` follows `CLEAR`
- **THEN** the count starts again from 1 and the `READ` gives the first `DATA` item, as with the old compiler

#### Scenario: RETURN after CLEAR
- **WHEN** a `GOSUB` runs `CLEAR`, sets a handler and runs `RETURN`
- **THEN** the handler gets error 3, as with the old compiler

#### Scenario: Strings in a user type's member array
- **WHEN** the upstream program `arrays/95_type_codegen_clear_inline_owner` runs
- **THEN** it prints `PASS 95_type_codegen_clear_inline_owner.bas`

### Requirement: The forms of CLEAR
`CLEAR`'s arguments SHALL be numbers, evaluated and ignored, and `CALL CLEAR` SHALL be `CLEAR`. `CLEAR` in a SUB
or FUNCTION, with a parenthesis anywhere among its arguments, with a fourth argument or with a string SHALL be the
old compiler's errors.

#### Scenario: Arguments ignored
- **WHEN** `x = 5: CLEAR x + 1, , 2 * 3: PRINT x` runs
- **THEN** it prints ` 0 `, as with the old compiler

#### Scenario: In a SUB
- **WHEN** a SUB holds `CLEAR`
- **THEN** it is the compile error "CLEAR cannot be used inside a SUB/FUNCTION", as in the old compiler

#### Scenario: A parenthesis
- **WHEN** a program has `CLEAR F(1)`
- **THEN** it is the compile error "Incorrect number of arguments", as in the old compiler

#### Scenario: Wrong forms
- **WHEN** a program has `CLEAR , , LEN("a")`, `CLEAR 1, 2, 3, 4` or `CLEAR "a"`
- **THEN** each is a compile error, as in the old compiler
