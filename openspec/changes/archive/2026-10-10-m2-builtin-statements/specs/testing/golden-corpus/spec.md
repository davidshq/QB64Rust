# Spec Delta

## ADDED Requirements

### Requirement: Standard input sidecar
A corpus program MAY have `<name>.stdin`. `run_legacy_tests.py --suite corpus` SHALL then run the executable with
its standard input read from a copy of that file, byte for byte, when checking and when recording, for the old and
the new compiler alike; every other rule of the run (the scratch folder, the timeout, the comparison) SHALL stay
as it is. A program without the sidecar SHALL run exactly as before. A program with the sidecar SHALL end with
`SYSTEM`, and its `.stdin` SHALL hold an answer for every input statement it runs: the old compiler's program
waits for ever at `END`'s "Press any key to continue" and when its input has run out.

#### Scenario: Program that reads a line
- **WHEN** a program does `LINE INPUT s$: PRINT "["; s$; "]"` and its `.stdin` holds the line `abc`
- **THEN** the run prints `[abc]`, with either compiler

#### Scenario: Input runs out
- **WHEN** a program asks for more input than its `.stdin` holds
- **THEN** the run fails with a timeout, with either compiler (the program waits for more input), and nothing is
  recorded for it

#### Scenario: No sidecar
- **WHEN** the corpus is run after this change with no `.stdin` file present
- **THEN** every program's result is the same as before the change
