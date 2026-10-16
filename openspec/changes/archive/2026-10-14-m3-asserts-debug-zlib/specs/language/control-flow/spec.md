# Spec Delta

## ADDED Requirements

### Requirement: END and SYSTEM with an exit code
`SYSTEM n` and `END n` SHALL end the program as `SYSTEM` and `END` do, with `n`, converted as the old compiler
converts it (to an `_INTEGER64`, then to the process's exit code as libqb does), as the process's exit code. A
pending error SHALL be handled before `n` is stored, as the old compiler's program does. `n` SHALL be any numeric
expression the old compiler accepts there, and a string a compile error, as with it.

#### Scenario: An exit code
- **WHEN** a console program runs `PRINT "x": SYSTEM 3`
- **THEN** it prints `x` and its exit code is 3

#### Scenario: END with a code
- **WHEN** a console program runs `END 7`
- **THEN** its exit code is what the old compiler's program gives (7, as measured)

#### Scenario: A value out of range
- **WHEN** a program runs `SYSTEM -1` or `SYSTEM 2.7`
- **THEN** its exit code is the old compiler's program's

#### Scenario: A string
- **WHEN** a program has `SYSTEM "a"`
- **THEN** it is a compile error, as with the old compiler
