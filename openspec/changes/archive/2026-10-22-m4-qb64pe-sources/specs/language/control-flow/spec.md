## ADDED Requirements

### Requirement: STOP
`STOP` SHALL end the program as `SYSTEM` does without a code: no "Press any key" prompt, the files closed, and the
exit code the one set so far. `STOP` followed by an expression SHALL check the expression as a number, as the old
compiler does (a string is a compile error), and SHALL NOT evaluate it at run time. `STOP` SHALL be allowed in the
main module and in a SUB or FUNCTION, and after `THEN` or `ELSE` on one line.

#### Scenario: STOP ends the program
- **WHEN** a console program runs `PRINT "a": STOP: PRINT "b"`
- **THEN** it prints `a` and ends with exit code 0

#### Scenario: STOP in a SUB after THEN
- **WHEN** a SUB runs `IF x = 1 THEN PRINT "e": STOP` with `x = 1`
- **THEN** the program prints `e` and ends

#### Scenario: An expression is not evaluated
- **WHEN** a program runs `STOP f(1)`, where the FUNCTION `f` prints a line
- **THEN** nothing of `f` is printed (`verification\v52_x12`)

#### Scenario: A window program
- **WHEN** a program with a window runs `STOP`
- **THEN** it ends without waiting for a key, as the old compiler's program does (`verification\v52_x14`)
