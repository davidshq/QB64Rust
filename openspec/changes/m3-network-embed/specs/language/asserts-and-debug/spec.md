# Spec Delta

## MODIFIED Requirements

### Requirement: $DEBUG
`$DEBUG` SHALL be accepted anywhere the old compiler accepts it and set `_DEBUG_` and `_SOCKETS_` to 1 (the old
compiler's debugger code uses sockets), so that the program links libqb's sockets part as the old compiler's
does, and with `-w` the
compiler SHALL warn "$DEBUG features only work from the IDE" at it, as the old compiler does from the command
line. No code for the IDE's debugger SHALL be written: the program SHALL behave as the old compiler's program
does when it runs outside the IDE (`DIVERGENCES.md`).

#### Scenario: The flag
- **WHEN** a program has `$DEBUG` and `$IF _DEBUG_ THEN PRINT "on" $ELSE PRINT "off" $END IF`
- **THEN** it prints `on`

#### Scenario: The sockets flag
- **WHEN** a program has `$DEBUG` on its last line and `$IF _SOCKETS_ THEN PRINT "on" $END IF` above it
- **THEN** it prints `on`, as the old compiler's program does

#### Scenario: The sockets part
- **WHEN** a program with `$DEBUG` and no network function is built
- **THEN** it links libqb's sockets and HTTP part, as the old compiler's build does

#### Scenario: The warning
- **WHEN** a program with `$DEBUG` is compiled with `-w`
- **THEN** the warning is given at the metacommand's line and the program builds

#### Scenario: Same output as without it
- **WHEN** a program that prints and ends with `SYSTEM` is built once with `$DEBUG` and once without
- **THEN** both print what the old compiler's program prints
