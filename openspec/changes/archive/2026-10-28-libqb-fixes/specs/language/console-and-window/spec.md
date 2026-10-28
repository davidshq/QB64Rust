# Spec Delta

## MODIFIED Requirements

### Requirement: The end of a program and errors with a hidden window
A program with `$CONSOLE` SHALL end and report errors as the old compiler's program does: `SYSTEM` leaves at once;
`END`, and running off the last line, print the old compiler's "Press any key to continue" on the console and
wait for a key when the program's standard input is a console; when it is not (a file, a pipe, the null device)
they print the line and end at once (`study\39` row 2; a `QB64PE-DIVERGENCES.md` row); a trapped error is serviced
as under `$CONSOLE:ONLY`; an untrapped error prints the old compiler's message.

#### Scenario: A trapped error
- **WHEN** a program with `$CONSOLE`, `$SCREENHIDE` and a handler that prints `ERR` and does `RESUME NEXT` runs
  `_DEST _CONSOLE`, `PRINT "before"`, `ERROR 5`, `PRINT "after"`
- **THEN** it prints `before`, the handler's line with 5, and `after`

#### Scenario: An untrapped error
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` runs `_DEST _CONSOLE`, `PRINT "before"` and `ERROR 5` on
  line 7, with `QB64PE_NOPROMPT=y`
- **THEN** it prints `before`, an empty line, `Runtime error: Line: 7 (in main module)` and `Illegal function
  call`, and exits with code 0

#### Scenario: END with its input from the null device
- **WHEN** a `$CONSOLE:ONLY` program runs `PRINT "before END"` and `END`, with its standard input the null device
- **THEN** it prints `before END`, an empty line and `Press any key to continue`, and exits with code 0 without
  waiting

## ADDED Requirements

### Requirement: Console input at the end of the input
In a console program, `INPUT`, `LINE INPUT` and `INPUT$(n)` with no file number SHALL raise error 62 (Input past
end of file), trappable, when standard input has ended and nothing of the line was read, and SHALL never wait for
input that cannot come; a last line without a newline SHALL be read as a line (`study\39` row 2; a
`QB64PE-DIVERGENCES.md` row).

#### Scenario: INPUT after the last line
- **WHEN** a `$CONSOLE:ONLY` program with `ON ERROR GOTO h` (printing `ERR`, then `RESUME NEXT`) reads two
  `INPUT` lines from a standard input holding one line
- **THEN** the first `INPUT` reads the line, the second raises error 62, and the program goes on to its end

#### Scenario: LINE INPUT with no handler
- **WHEN** a `$CONSOLE:ONLY` program runs `LINE INPUT s$` with its standard input the null device and no handler,
  with `QB64PE_NOPROMPT=y`
- **THEN** it ends with libqb's message for error 62 and does not wait

#### Scenario: The variables after the error
- **WHEN** `a = 7: b$ = "old"`, then `INPUT a, b$` raises error 62 under a handler that resumes next, and the
  program prints them
- **THEN** it prints 0 and an empty string, the values `INPUT` gives them before it reads

#### Scenario: INPUT$ from a file
- **WHEN** a `$CONSOLE:ONLY` program whose standard input holds `abcdef` and a newline runs `s$ = INPUT$(3)` and
  then `t$ = INPUT$(10)` under a handler that resumes next
- **THEN** `s$` is `abc`, and the second raises error 62 after reading the rest of the input

#### Scenario: A last line without a newline
- **WHEN** a `$CONSOLE:ONLY` program runs `LINE INPUT s$: PRINT s$` with standard input `abc` and no newline
- **THEN** it prints `abc`
