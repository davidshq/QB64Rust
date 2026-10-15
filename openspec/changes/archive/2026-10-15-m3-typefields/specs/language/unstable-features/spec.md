# Spec Delta

## Purpose

The `$UNSTABLE` metacommand: the flags that switch on a feature the old compiler still marks as unstable, where a
flag takes effect, and the errors and warnings of its forms.

## ADDED Requirements

### Requirement: $UNSTABLE flags
`$UNSTABLE:TYPEFIELDS` SHALL allow the `_STATIC` and `_DYNAMIC` member markers (`language/arrays-and-types`) in
the `TYPE` blocks that stand below it in the source, also when it stands after the first statement; a `TYPE`
block above it SHALL be compiled as without it. A `$UNSTABLE:TYPEFIELDS` in a `$IF` branch that is not taken
SHALL have no effect. The flag SHALL be read in any letter case, with blanks after the colon, and SHALL be
allowed more than once. `$UNSTABLE:MIDI` and `$UNSTABLE:HTTP` SHALL be accepted and change nothing, with the
warning "No longer required, feature is stable now" at their line. Any other flag, an empty one and `$UNSTABLE`
without a colon SHALL be compile errors, as in the old compiler.

#### Scenario: The flag above the TYPE
- **WHEN** a program has `$UNSTABLE:TYPEFIELDS`, then a `TYPE` with `v(1) _DYNAMIC AS LONG`, `DIM x`, `REDIM
  x.v(2 TO 3) AS LONG: x.v(3) = 3: PRINT LBOUND(x.v); UBOUND(x.v); x.v(3)`
- **THEN** it compiles and prints ` 2  3  3 `

#### Scenario: The flag after the first statement
- **WHEN** `PRINT 0` stands above `$UNSTABLE:TYPEFIELDS`, and the `TYPE` below both
- **THEN** it compiles

#### Scenario: The flag below the TYPE
- **WHEN** `$UNSTABLE:TYPEFIELDS` stands below a `TYPE` with a marked member
- **THEN** the marker is a compile error that names the flag, as without it

#### Scenario: In a branch not taken
- **WHEN** `$UNSTABLE:TYPEFIELDS` stands in `$IF 0 THEN … $END IF` above a `TYPE` with a marked member
- **THEN** the marker is a compile error; with `$IF 1 THEN` it compiles

#### Scenario: Forms of the flag
- **WHEN** `$unstable:typefields`, `$UNSTABLE: TYPEFIELDS`, or `$UNSTABLE:TYPEFIELDS` twice stands above the `TYPE`
- **THEN** each compiles

#### Scenario: Stable flags
- **WHEN** `$UNSTABLE:MIDI` or `$UNSTABLE:HTTP` stands in a program
- **THEN** it compiles, with the warning at that line

#### Scenario: Other forms
- **WHEN** `$UNSTABLE:FOO`, `$UNSTABLE:` or `$UNSTABLE` is compiled
- **THEN** each is a compile error (the first two name the flag)
