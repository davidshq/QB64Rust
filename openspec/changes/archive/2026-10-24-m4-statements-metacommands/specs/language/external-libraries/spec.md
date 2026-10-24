# Spec Delta

## ADDED Requirements

### Requirement: A library procedure and a program procedure of one name
A library FUNCTION and a program SUB of the same name without its suffix, or a library SUB and a program
FUNCTION, SHALL compile as in the old compiler: the name as a statement SHALL call the SUB, and the name as a
value, with its suffix or without, SHALL call the FUNCTION, whichever of the two is the library's (measured,
`verification\v57_l01`–`l04`).

#### Scenario: A library FUNCTION and a program SUB
- **WHEN** a `DECLARE LIBRARY` block declares `FUNCTION h_l& (BYVAL v AS LONG)` and the program defines `SUB h_l`
- **THEN** it compiles (`verification\v37_x22`)

#### Scenario: Both called
- **WHEN** that program calls `h_l` as a statement and prints `h_l&(2)`, and another prints `h_l(2)`
- **THEN** the statement runs the program's SUB, and each value is the library FUNCTION's, 3
  (`verification\v57_l01`, `l02`)

#### Scenario: A library SUB and a program FUNCTION
- **WHEN** a `DECLARE LIBRARY` block declares `SUB h_v (BYVAL v AS LONG)`, the program defines `FUNCTION h_v&`, and
  the program runs `h_v 3` and prints `h_v&(2)`
- **THEN** the statement calls the library SUB and the value is the program FUNCTION's (`verification\v57_l03`)
