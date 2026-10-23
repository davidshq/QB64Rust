# Spec Delta

## ADDED Requirements

### Requirement: Joystick-button handlers
`ON STRIG(b) GOSUB label`, `ON STRIG(b, c) GOSUB label` and the same with a SUB's name and an optional argument
SHALL set the handler of button `b` (of controller `c` when given) to a label of the main module or a SUB, as
`ON KEY` does, with `b` and `c` converted to LONG. `STRIG[(b[, c])] ON|OFF|STOP` SHALL switch that event as the
other switches do. The program SHALL be built with libqb's device-input part.

#### Scenario: Accepted forms
- **WHEN** a program has `ON STRIG(0, 1) GOSUB h`, `STRIG(0, 1) ON` and `STRIG ON` and prints `ok`
- **THEN** it compiles with no diagnostic and prints `ok`, as with the old compiler (`verification\v42_x11`)

#### Scenario: A handler SUB
- **WHEN** `ON STRIG(0) s 3` is compiled for `SUB s (n AS INTEGER)`
- **THEN** it compiles to the old compiler's setup call and dispatch

#### Scenario: Each form compiles to the old compiler's call
- **WHEN** each `ON STRIG` and `STRIG` switch form is compiled
- **THEN** the libqb calls equal the old compiler's (the call-site check)

#### Scenario: Wrong operands
- **WHEN** a program has `ON STRIG GOSUB h`, `ON STRIG(1, 2, 3) GOSUB h`, `ON STRIG() GOSUB h`,
  `ON STRIG(, 1) GOSUB h`, `ON STRIG(1 GOSUB h`, `ON STRIG(1)` or `ON STRIG("a") GOSUB h`
- **THEN** each is a compile error that is not "not supported yet" (`verification\v47_x04`–`x09`, `x21`)

#### Scenario: Wrong handler SUBs
- **WHEN** `ON STRIG(0) s` names a SUB with one parameter, `ON STRIG(0) s 3` a SUB with none, or a SUB whose
  parameter is SINGLE
- **THEN** each is a compile error that is not "not supported yet" (`v47_x10`–`x12`)

#### Scenario: An odd button number
- **WHEN** a program runs `ON STRIG(1) GOSUB h` or `STRIG(1) STOP` (an odd number is a button's "currently down"
  state)
- **THEN** it raises error 5 at run time, as the old compiler's build does (`v47_x13`, `x27`; libqb's
  `onstrig_setup` and `sub_strig`)

## MODIFIED Requirements

### Requirement: Events the old compiler does not have
`PEN`, `COM` and `UEVENT` used as events (`ON PEN GOSUB h`, `ON COM(1) GOSUB h`, `PEN ON`, `UEVENT ON`) SHALL be
compile errors that are not "not supported yet", as they are to the old compiler. A line number as a handler target
SHALL stay marked "not supported yet".

#### Scenario: PEN as an event
- **WHEN** a program has `ON PEN GOSUB h` or `PEN ON`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: STRIG left marked
- **WHEN** a program has `ON STRIG(0) GOSUB h` or `STRIG(0) ON`
- **THEN** it is no longer marked: it compiles as requirement "Joystick-button handlers" says, and no `STRIG` form
  is reported "not supported yet"

#### Scenario: A line number as the target
- **WHEN** a program has `ON TIMER(1) GOSUB 100`
- **THEN** the compiler reports "not supported yet", naming line numbers
