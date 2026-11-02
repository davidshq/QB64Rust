# Spec Delta

## ADDED Requirements

### Requirement: The exit code of a program ended by an error
A program that ends because of a runtime error SHALL exit with code 1: an error no handler takes, with
`QB64PE_NOPROMPT=y` or answered "no" at libqb's "Continue?", and a critical error (300 to 315, out of memory, the
other fatal ends of libqb). The old compiler's program exits 0 (`QB64PE-DIVERGENCES.md` D-012). A program that goes on
after the error SHALL keep the exit code its end gives.

#### Scenario: No handler
- **WHEN** a `$CONSOLE:ONLY` program runs `PRINT "before"` and `ERROR 5` on line 3, with `QB64PE_NOPROMPT=y`
- **THEN** it prints `before`, then `Runtime error: Line: 3 (in main module)` and `Illegal function call` on
  stderr, and exits with code 1

#### Scenario: A critical error
- **WHEN** a `$CONSOLE:ONLY` program with `$ASSERTS` runs `_ASSERT 0`
- **THEN** it ends with "_ASSERT failed" and exit code 1

#### Scenario: Going on after the error
- **WHEN** the same program as "No handler" runs with `QB64PE_NOPROMPT=continue` and then `SYSTEM`
- **THEN** it prints the message, goes on to `SYSTEM`, and exits with code 0

#### Scenario: A trapped error
- **WHEN** a program with a handler that does `RESUME NEXT` raises error 5 and then runs off its last line
- **THEN** it exits with code 0

#### Scenario: The trace
- **WHEN** the program of "No handler" runs with `QB64RUST_DIVERGENCES` naming a file
- **THEN** the file holds `D-012`; with `QB64PE_NOPROMPT=continue` it does not
