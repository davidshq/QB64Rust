# Spec Delta

## MODIFIED Requirements

### Requirement: The end of a program and errors with a hidden window
A program with `$CONSOLE` SHALL end and report errors as the old compiler's program does: `SYSTEM` leaves at once;
`END`, and running off the last line, print the old compiler's "Press any key to continue" on the console and
wait for a key when the program's standard input is a console; when it is not (a file, a pipe, the null device)
they print the line and end at once (`study\39` row 2; a `QB64PE-DIVERGENCES.md` row); a trapped error is serviced
as under `$CONSOLE:ONLY`; an untrapped error prints the old compiler's message, and the program exits with code 1
where the old compiler's exits 0 (`QB64PE-DIVERGENCES.md` D-012).

#### Scenario: A trapped error
- **WHEN** a program with `$CONSOLE`, `$SCREENHIDE` and a handler that prints `ERR` and does `RESUME NEXT` runs
  `_DEST _CONSOLE`, `PRINT "before"`, `ERROR 5`, `PRINT "after"`
- **THEN** it prints `before`, the handler's line with 5, and `after`

#### Scenario: An untrapped error
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` runs `_DEST _CONSOLE`, `PRINT "before"` and `ERROR 5` on
  line 7, with `QB64PE_NOPROMPT=y`
- **THEN** it prints `before`, an empty line, `Runtime error: Line: 7 (in main module)` and `Illegal function
  call`, and exits with code 1

#### Scenario: END with its input from the null device
- **WHEN** a `$CONSOLE:ONLY` program runs `PRINT "before END"` and `END`, with its standard input the null device
- **THEN** it prints `before END`, an empty line and `Press any key to continue`, and exits with code 0 without
  waiting
