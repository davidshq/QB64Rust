# Spec Delta

## ADDED Requirements

### Requirement: Line numbers in error handling
`ON ERROR GOTO n`, `ON ERROR GOTO _NEWHANDLER n` and `RESUME n` SHALL take a line number where they take a label,
with the same rules of body and the same errors (`language/control-flow`, "Line numbers as jump targets").
`ON ERROR GOTO 0` and `RESUME 0` SHALL keep their meaning; they name no line.

#### Scenario: A numbered handler
- **WHEN** `ON ERROR GOTO 900` precedes `ERROR 5`, and the line numbered 900 prints `ERR` and executes
  `RESUME NEXT`
- **THEN** it prints ` 5 ` and the program goes on after `ERROR 5`

#### Scenario: RESUME to a line number
- **WHEN** a handler executes `RESUME 100`
- **THEN** execution continues at the line numbered 100

#### Scenario: A numbered handler in the chain
- **WHEN** `ON ERROR GOTO _NEWHANDLER 10` is compiled with a line numbered 10 in the main module
- **THEN** it compiles and adds the handler to the chain as a label would (`verification\v50_x13`)

## MODIFIED Requirements

### Requirement: ERROR, ERR and ERL
`ERROR n` SHALL raise error `n`, rounded half to even when fractional (a value of 0 or less raises error 5;
errors the runtime treats as critical, such as 256, end the program even with a handler). `ERR` SHALL give the number of the error
being handled, 0 outside a handler and after `RESUME`. `ERL` SHALL give the last numbered line passed before the
error, in the order the program ran, in any body and any included file (a numbered line of a handler counts
too), and 0 when no numbered line was passed; it keeps that value after `RESUME`. Its value is the line's number
as the old compiler writes it into its C++ (`verification\v54`): `10.5` gives 10.5, `50.` 50, `1.10` 1.1, and a
suffixed number with a leading zero is read as C++ reads it (`010&` gives 8, as recorded in `v54_x25`). `ERR` SHALL
be typed LONG (the old compiler types it `_UNSIGNED LONG`; error numbers are below 2^31, so both print the same).

#### Scenario: ERROR with unusual values
- **WHEN** `ERROR 0`, `ERROR 2.5`, `ERROR 3.5` and `ERROR 70000` are each handled by a handler that prints `ERR`
- **THEN** it prints 5, 2, 4 and 70000

#### Scenario: ERR and ERL in a handler
- **WHEN** `ERROR 6` is handled by a handler that prints `ERR; ERL`, and no line of the program has a number
- **THEN** it prints ` 6  0 `

#### Scenario: ERL after a numbered line
- **WHEN** the lines `10 PRINT "a"` and `20 ERROR 6` run, and the handler prints `ERL`
- **THEN** it prints ` 20 `

#### Scenario: ERL after an unnumbered line
- **WHEN** the line numbered 30 runs, then an unnumbered line raises an error whose handler prints `ERL`
- **THEN** it prints ` 30 `, as the old compiler records (`verification\v54_a_line_numbers`)

#### Scenario: ERL after a numbered handler
- **WHEN** a handler whose first line is numbered 900 has handled an error with `RESUME NEXT`, and an unnumbered
  line then raises another
- **THEN** `ERL` is 900 (`verification\v54_a_line_numbers`)

#### Scenario: ERL in a procedure
- **WHEN** a FUNCTION's line `70 ERROR 5` raises an error handled in the main module
- **THEN** `ERL` is 70 in the handler and after the call (`verification\v54_x22`)
