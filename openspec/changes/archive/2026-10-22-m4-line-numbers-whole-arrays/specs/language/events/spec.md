# Spec Delta

## MODIFIED Requirements

### Requirement: Events the old compiler does not have
`PEN`, `COM` and `UEVENT` used as events (`ON PEN GOSUB h`, `ON COM(1) GOSUB h`, `PEN ON`, `UEVENT ON`) SHALL be
compile errors that are not "not supported yet", as they are to the old compiler. A line number as a handler target
SHALL be taken as a label is (`language/control-flow`, "Line numbers as jump targets").

#### Scenario: PEN as an event
- **WHEN** a program has `ON PEN GOSUB h` or `PEN ON`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: STRIG left marked
- **WHEN** a program has `ON STRIG(0) GOSUB h` or `STRIG(0) ON`
- **THEN** it is no longer marked: it compiles as requirement "Joystick-button handlers" says, and no `STRIG` form
  is reported "not supported yet"

#### Scenario: A line number as the target
- **WHEN** a program has `ON TIMER(1) GOSUB 100` or `ON PLAY(5) GOSUB 10` and the line it names in the main module
- **THEN** it compiles, and the handler runs at that line as recorded (`verification\v42_x46`, `v29_x64`)
