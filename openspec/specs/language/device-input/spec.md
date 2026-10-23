# language/device-input Specification

## Purpose

Device input: a program reads the keyboard, the mouse and game controllers as numbered devices with buttons, axes
and wheels, and reads joysticks the QuickBASIC way with `STICK` and `STRIG`, as programs built with the old
compiler do.

## Requirements

### Requirement: The device functions
`_DEVICES`, `_DEVICE$[(n)]`, `_DEVICEINPUT[(n)]`, `_LASTBUTTON[(n)]`, `_LASTAXIS[(n)]`, `_LASTWHEEL[(n)]`,
`_BUTTON[(n)]`, `_BUTTONCHANGE[(n)]`, `_AXIS[(n)]` and `_WHEEL[(n)]` SHALL compile to the old compiler's calls, with
each argument converted to LONG and the result typed as the old compiler types it (`_DEVICE$` a string, `_AXIS` and
`_WHEEL` DOUBLE, the others LONG). The keyboard and the mouse SHALL always be devices, so `_DEVICES` is at least 2.

#### Scenario: Two devices at least
- **WHEN** a `$CONSOLE:ONLY` program runs `PRINT _DEVICES > 1`
- **THEN** it prints `-1` (`console_only/devices`, `deviceinput/test`)

#### Scenario: The keyboard and the mouse
- **WHEN** a program prints `_DEVICE$(1)`, `_DEVICE$(2)`, `_LASTBUTTON(1)` and `_LASTAXIS(2)` with no game controller
  attached
- **THEN** it prints what the old compiler's build prints on the same machine

#### Scenario: Each form compiles to the old compiler's call
- **WHEN** each function is compiled with and without its optional argument, and with an argument of each numeric
  type
- **THEN** the libqb call equals the old compiler's (the call-site check)

### Requirement: STICK and STRIG
`STICK(d[, a])` and `STRIG(b[, c])` SHALL compile to the old compiler's calls, telling libqb whether the second
argument was given. With no game controller attached they SHALL return what the old compiler's build returns. An
argument out of libqb's range SHALL raise error 5 at run time, as in the old compiler's build.

#### Scenario: No joystick
- **WHEN** a program prints `STICK(0)` and `STRIG(1)` with no game controller attached
- **THEN** it prints the values the old compiler's build prints

#### Scenario: Out of range
- **WHEN** a program with `ON ERROR` runs `x = STRIG(70000)` and `x = STICK(0, 0)`
- **THEN** each raises error 5, as in the old compiler's build

### Requirement: Errors of the device functions
A device function given more arguments than its form takes, or given a string argument, SHALL be a compile error that is not "not supported yet", as it is to the old compiler.

#### Scenario: Too many arguments
- **WHEN** a program has `x = _BUTTON(1, 2)` or `x = STICK(1, 2, 3)`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: A string argument
- **WHEN** a program has `x = _AXIS("1")`
- **THEN** the compiler reports an error that is not "not supported yet"

### Requirement: The device-input runtime part
A program that uses any device function, `STICK`, `STRIG`, the `STRIG` switch or `ON STRIG` SHALL be built with
libqb's device-input part, as the old compiler builds it. A program that uses none of them SHALL be built without
it. The part's licence SHALL be in the licence file when one is written.

#### Scenario: Linked when used
- **WHEN** a program that calls `_DEVICES` is built
- **THEN** it is linked with libqb's device-input part, and the build's make flags equal the old compiler's in
  order

#### Scenario: Not linked otherwise
- **WHEN** a program that uses no device input is built
- **THEN** its make flags have no device-input flag
