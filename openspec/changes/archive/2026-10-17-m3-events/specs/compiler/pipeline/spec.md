# Spec Delta

## ADDED Requirements

### Requirement: Event handlers in the IR
The IR SHALL have one operation that sets up an event handler: the event (timer or key), its operands as values of
the types the old compiler converts them to, and the handler, either a label of the main module or a procedure with
its argument as a 64-bit integer value. The label MAY belong to the main module while the operation stands in a
procedure; no other jump SHALL leave its body.

#### Scenario: A timer handler set up in a procedure
- **WHEN** `ON TIMER(t, 1.5) GOSUB h` inside a SUB is lowered, `h` a label of the main module
- **THEN** the IR holds one handler setup for timer `t` (a 32-bit integer) every 1.5 seconds (a double) with the
  main module's label `h`, it passes the IR's validation, and the main module keeps `h` as a place that can be
  entered

#### Scenario: A key handler with a SUB
- **WHEN** `ON KEY(3) s x%` is lowered for `SUB s (n AS INTEGER)`
- **THEN** the IR holds one handler setup for key 3 with procedure `s` and the value of `x%` converted to a 64-bit
  integer
