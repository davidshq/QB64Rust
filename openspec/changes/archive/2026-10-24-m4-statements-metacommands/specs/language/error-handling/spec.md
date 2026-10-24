# Spec Delta

## ADDED Requirements

### Requirement: RESUME inside a procedure
`RESUME`, `RESUME 0`, `RESUME NEXT` and `RESUME label` inside a SUB SHALL compile, with a label of the SUB, and do
what the old compiler's program does: with no error being handled, error 20; otherwise `RESUME label` goes on at
the label and the other forms leave the SUB (measured, `verification\v57_x01`–`x08`). A label of the main module
SHALL be the old compiler's error, and `RESUME` in a FUNCTION a compile error (`v57_x05`, `x09`–`x12`).

#### Scenario: Never reached
- **WHEN** a SUB that raises error 5 under a main-module handler doing `RESUME NEXT` has `EXIT SUB`, then
  `RESUME back` with `back:` a label of the SUB
- **THEN** the program compiles, and prints the handler's line, the SUB's line after `back:` and the main
  module's line after the call (`verification\v17_d_resume_sub_label`)

#### Scenario: A label of the main module and of the SUB
- **WHEN** both the main module and the SUB have the label `back:` and the SUB's `RESUME back` is never reached
- **THEN** the program compiles and runs as the old compiler's (`verification\v17_d_resume_main_label_from_sub_both`)

#### Scenario: In a SUB the handler calls
- **WHEN** a main-module handler calls a SUB that runs `RESUME NEXT`, and the handler then prints a line and runs
  `RESUME NEXT` itself
- **THEN** the SUB is left at its `RESUME NEXT`, the handler's line is printed, and its own `RESUME NEXT` raises
  error 20 (`verification\v57_x03`)

#### Scenario: Reached inside a handler-less SUB
- **WHEN** a SUB runs `RESUME NEXT` with no error pending and no handler set
- **THEN** the program stops with the run-time error "RESUME without error" (`verification\v57_x06`)

#### Scenario: In a FUNCTION
- **WHEN** a FUNCTION holds `RESUME NEXT`
- **THEN** the compile fails with an error at it, not marked "not supported yet"
