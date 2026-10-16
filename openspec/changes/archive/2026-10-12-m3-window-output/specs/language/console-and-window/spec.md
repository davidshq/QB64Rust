# Spec Delta

## MODIFIED Requirements

### Requirement: A program with a console
A program SHALL have a console when `$CONSOLE` or `$CONSOLE:ONLY` stands anywhere in its compiled code: on any
line, after the last statement that runs, inside a procedure, in an included file, once or several times. A
metacommand in a `$IF` branch that is not taken SHALL NOT count. A program with neither SHALL be compiled and
built with a window and no console, as the old compiler builds it (`language/screen-output`).

#### Scenario: $CONSOLE as the last line
- **WHEN** a program is `_DEST _CONSOLE`, `PRINT "ok"`, `SYSTEM`, `$SCREENHIDE`, `$CONSOLE`
- **THEN** it compiles and prints `ok`

#### Scenario: $CONSOLE inside a SUB
- **WHEN** `$CONSOLE` and `$SCREENHIDE` stand inside a SUB and the main module does `_DEST _CONSOLE` and `PRINT
  "ok"` before it calls the SUB
- **THEN** `ok` is printed

#### Scenario: Twice
- **WHEN** a program has `$CONSOLE` twice and `$SCREENHIDE` twice
- **THEN** it compiles as with each once

#### Scenario: Only in a branch that is not taken
- **WHEN** a program's only `$CONSOLE` stands between `$IF 0 THEN` and `$END IF`
- **THEN** the program is compiled as one with a window and no console

#### Scenario: Neither metacommand
- **WHEN** a program is `PRINT 1`
- **THEN** it compiles with no diagnostic, and its settings say a window and no console

### Requirement: $SCREENHIDE and $SCREENSHOW
`$SCREENHIDE` SHALL start the program with its window hidden and `$SCREENSHOW` with it shown; with several of
them, the last one in the file SHALL count, wherever they stand. Neither gives the program a console.

#### Scenario: Hidden window, output on the console
- **WHEN** a program is `$CONSOLE`, `$SCREENHIDE`, `_DEST _CONSOLE`, `PRINT "ok"`, `SYSTEM`
- **THEN** it prints `ok`, exits with code 0 and shows no window

#### Scenario: $SCREENSHOW after $SCREENHIDE
- **WHEN** a program has `$SCREENHIDE` and below it `$SCREENSHOW`
- **THEN** it is built to start with the window shown

#### Scenario: $SCREENHIDE alone
- **WHEN** a program is `$SCREENHIDE`, `PRINT "x"`, `SYSTEM`
- **THEN** it compiles with no diagnostic, as a program with a hidden window and no console
