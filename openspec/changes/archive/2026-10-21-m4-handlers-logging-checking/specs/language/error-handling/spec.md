# Spec Delta

## ADDED Requirements

### Requirement: The handler chain
`ON ERROR GOTO _NEWHANDLER label` SHALL keep the active handler and make `label` the active one.
`ON ERROR GOTO _LASTHANDLER` SHALL make the handler kept last the active one again and forget it. With none kept,
no handler is active. `ON ERROR GOTO label` SHALL change the active handler and leave the kept ones alone.
`ON ERROR GOTO 0` SHALL forget them all. In a procedure the label SHALL be one of the main module, as for
`ON ERROR GOTO label`.

#### Scenario: Back to the earlier handler
- **WHEN** `ON ERROR GOTO a`, then `ON ERROR GOTO _NEWHANDLER b`, `ERROR 5`, `ON ERROR GOTO _LASTHANDLER` and
  `ERROR 6` run, each handler printing its name and `ERR` and resuming next
- **THEN** it prints `b 5` and then `a 6`

#### Scenario: Nothing kept
- **WHEN** `ON ERROR GOTO _LASTHANDLER` runs with nothing kept, followed by `ERROR 5`
- **THEN** no handler runs, and the runtime reports error 5 as without a handler

#### Scenario: The upstream program
- **WHEN** `on_error/errtest` is run
- **THEN** its output equals the recorded output

### Requirement: The handler chain's errors
These forms SHALL be compile errors, as in the old compiler: `ON ERROR GOTO _LASTHANDLER label` ("No label allowed
after _LASTHANDLER"), `ON ERROR GOTO _NEWHANDLER` with no label or with another word before the label
("Expected: ON ERROR GOTO [_NEWHANDLER] label"), and `ON ERROR GOTO _NEWHANDLER 0` ("Zero not allowed after
_NEWHANDLER").

#### Scenario: Zero after NEWHANDLER
- **WHEN** `ON ERROR GOTO _NEWHANDLER 0` is compiled
- **THEN** it is a compile error

#### Scenario: A label after LASTHANDLER
- **WHEN** `ON ERROR GOTO _LASTHANDLER h` is compiled, `h` being a label of the program
- **THEN** it is a compile error
