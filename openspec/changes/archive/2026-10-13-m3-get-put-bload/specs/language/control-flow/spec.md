# Spec Delta

## ADDED Requirements

### Requirement: A SUB's name and a colon, with no jump to the name
A line that starts with the name of a SUB and a colon SHALL be a call of that SUB when the SUB can be called with
no argument and no jump of the program, in any body, names the word; a statement after the colon SHALL follow the
call. This is the old compiler's rule.

#### Scenario: A call
- **WHEN** a program has the line `topbar:` in its main module, a `SUB topbar` without parameters, and no jump to
  `topbar`
- **THEN** the line calls the SUB, as with the old compiler

#### Scenario: Twice in one body
- **WHEN** the main module has the line `one:` twice, the program has a `SUB one` without parameters and no jump
  to `one`
- **THEN** the SUB is called twice

#### Scenario: A statement after the colon
- **WHEN** the line is `one: PRINT "after"` and `SUB one` prints `sub one`
- **THEN** the output is `sub one`, then `after`

### Requirement: A SUB's name and a colon, with a jump to the name
When a jump anywhere in the program (`GOTO`, `GOSUB`, `RETURN`, `RESTORE`, `RESUME`, `ON ERROR GOTO`, `ON … GOTO`,
`ON … GOSUB`) names the word, every line that is the word and a colon SHALL be a label, with the rules of any
label, and `CALL name` SHALL still call the SUB. The name of a SUB that needs an argument, and the name of a
FUNCTION, SHALL always be a label there.

#### Scenario: A label beside a call
- **WHEN** a SUB has the line `ships:`, later `IF n < 2 GOTO ships` and `CALL ships`, and the program has a `SUB
  ships` without parameters
- **THEN** the `GOTO` continues at the line and the `CALL` calls the SUB

#### Scenario: A jump in another body
- **WHEN** the main module has the line `one:`, a SUB has its own line `one:` and `GOTO one`, and the program has a
  `SUB one` without parameters
- **THEN** the main module's line is a label and calls nothing

#### Scenario: A jump with no such line in its body
- **WHEN** a SUB executes `GOTO one`, `one:` stands only in the main module, and the program has a `SUB one`
- **THEN** it is a compile error (the label is not defined in the SUB)

#### Scenario: Twice in one body, with a jump
- **WHEN** the main module has the line `one:` twice and `GOTO one`, and the program has a `SUB one`
- **THEN** it is a compile error (a duplicate label)

#### Scenario: A SUB with a parameter, and a FUNCTION
- **WHEN** a program has the line `one:` and `SUB one (k AS INTEGER)`, or the line `fone:` and `FUNCTION fone`
- **THEN** the line is a label, with or without a jump to it
