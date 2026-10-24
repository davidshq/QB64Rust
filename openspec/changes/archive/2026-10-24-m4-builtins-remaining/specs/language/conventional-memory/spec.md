# Spec Delta

## ADDED Requirements

### Requirement: INP, OUT and WAIT
`INP(port)`, `OUT port, value` and `WAIT port, and[, xor]` SHALL compile with the old compiler's libqb calls and
SHALL do what libqb's emulated ports do: the palette ports `&H3C0` (`OUT`), `&H3C7`, `&H3C8` and `&H3C9`, the
keyboard port `&H60`, and the display status port `&H3DA`; any other port SHALL do what the old compiler's
program does, and a port out of range SHALL raise error 6 (measured, `verification\v58`).

#### Scenario: The palette through its ports
- **WHEN** a program in screen 13 runs `OUT &H3C8, 1: OUT &H3C9, 63: OUT &H3C9, 0: OUT &H3C9, 0`, then
  `OUT &H3C7, 1` and prints `INP(&H3C9)` three times
- **THEN** it prints what the old compiler's program prints

#### Scenario: Waiting for the display
- **WHEN** a program runs `WAIT &H3DA, 8` and prints `"ok"`
- **THEN** it ends and prints `ok`, as the old compiler's program does

#### Scenario: A port libqb does not emulate
- **WHEN** a program prints `INP(&H379)` and runs `OUT &H378, 1`
- **THEN** it prints and ends as the old compiler's program does, error or value

#### Scenario: A port out of range
- **WHEN** a program evaluates `INP(70000)` under a handler that prints `ERR`
- **THEN** it prints ` 6 `
