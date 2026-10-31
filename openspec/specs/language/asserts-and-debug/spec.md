# language/asserts-and-debug Specification

## Purpose
The metacommands `$ASSERTS`, `$ASSERTS:CONSOLE` and `$DEBUG`, the statement `_ASSERT` and its runtime errors: what
each does in a program built by this compiler, as the old compiler's program does outside its IDE.

## Requirements

### Requirement: $ASSERTS
`$ASSERTS` SHALL turn assertions on for the whole program, wherever it stands, and set `_ASSERTS_` to 1.
`$ASSERTS:CONSOLE` SHALL do the same and also act as `$CONSOLE` does. Neither SHALL be reported "not supported
yet". Their forms (case, blanks, what may follow them on the line) SHALL be those the old compiler accepts.

#### Scenario: Assertions on
- **WHEN** a program with `$ASSERTS` runs `_ASSERT 1 = 2`
- **THEN** it stops with runtime error 314, as the old compiler's program does

#### Scenario: Below the assertion
- **WHEN** `$ASSERTS` stands on the last line of a program whose first line is `_ASSERT 0`
- **THEN** the assertion fails, as with the old compiler

#### Scenario: With a console
- **WHEN** a program has `$ASSERTS:CONSOLE` and `_DEST _CONSOLE`, `PRINT "x"`
- **THEN** it has a console and a window, `_CONSOLE_` is 1 and `_ASSERTS_` is 1

### Requirement: _ASSERT
`_ASSERT condition[, message$]` SHALL do nothing in a program without `$ASSERTS` (the condition and the message
are still evaluated where the old compiler evaluates them). With `$ASSERTS`, a condition that is 0 SHALL raise
runtime error 314; in a program with a console and with a message, the message SHALL be written to the console and
error 315 raised instead, as libqb does. As with the old compiler, `ON ERROR` SHALL NOT trap these errors (libqb
treats 300 to 315 as errors no handler takes): the program stops with the runtime error, and exits with code 1
where the old compiler's program exits 0 (`QB64PE-DIVERGENCES.md` D-012).

#### Scenario: Without $ASSERTS
- **WHEN** a program without `$ASSERTS` runs `_ASSERT 0, "never"` and `PRINT "after"`
- **THEN** it prints `after`

#### Scenario: Not trapped
- **WHEN** a program with `$ASSERTS` and `ON ERROR GOTO h` runs `_ASSERT 0` and the handler prints `ERR`
- **THEN** the handler does not run: the program stops with "_ASSERT failed" at the `_ASSERT`'s line, as the old
  compiler's program does

#### Scenario: A message in a console program
- **WHEN** a `$CONSOLE:ONLY` program with `$ASSERTS` runs `_ASSERT 0, "bad"`
- **THEN** its output equals the old compiler's program's, and its exit code is 1

#### Scenario: Wrong arguments
- **WHEN** a program has `_ASSERT` with no argument, or with a string as the condition
- **THEN** it is a compile error, as with the old compiler

### Requirement: $DEBUG
`$DEBUG` SHALL be accepted anywhere the old compiler accepts it and set `_DEBUG_` and `_SOCKETS_` to 1 (the old
compiler's debugger code uses sockets), so that the program links libqb's sockets part as the old compiler's
does, and with `-w` the
compiler SHALL warn "$DEBUG features only work from the IDE" at it, as the old compiler does from the command
line. No code for the IDE's debugger SHALL be written: the program SHALL behave as the old compiler's program
does when it runs outside the IDE (`QB64PE-DIVERGENCES.md`).

#### Scenario: The flag
- **WHEN** a program has `$DEBUG` and `$IF _DEBUG_ THEN PRINT "on" $ELSE PRINT "off" $END IF`
- **THEN** it prints `on`

#### Scenario: The sockets flag
- **WHEN** a program has `$DEBUG` on its last line and `$IF _SOCKETS_ THEN PRINT "on" $END IF` above it
- **THEN** it prints `on`, as the old compiler's program does

#### Scenario: The sockets part
- **WHEN** a program with `$DEBUG` and no network function is built
- **THEN** it links libqb's sockets and HTTP part, as the old compiler's build does

#### Scenario: The warning
- **WHEN** a program with `$DEBUG` is compiled with `-w`
- **THEN** the warning is given at the metacommand's line and the program builds

#### Scenario: Same output as without it
- **WHEN** a program that prints and ends with `SYSTEM` is built once with `$DEBUG` and once without
- **THEN** both print what the old compiler's program prints
