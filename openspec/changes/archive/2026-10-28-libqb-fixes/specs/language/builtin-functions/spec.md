# Spec Delta

## MODIFIED Requirements

### Requirement: Bit functions
`_SETBIT(n, b)`, `_RESETBIT(n, b)` and `_TOGGLEBIT(n, b)` SHALL compile in any expression with the result type the
old compiler gives for `n`'s type, and SHALL give its values, also for a bit beyond `n`'s width and a negative bit
(measured, `verification\v58`), as `_READBIT` does. A bit number outside 0 to 63 SHALL be taken modulo 64 (its low
six bits), whether it is a constant or a variable, as the old compiler's program does for a variable
(`study\39` rows 17, 18; a `QB64PE-DIVERGENCES.md` row for constants).

#### Scenario: Values
- **WHEN** a program prints `_SETBIT(5, 1); _RESETBIT(5, 0); _TOGGLEBIT(5, 2)`
- **THEN** it prints ` 7  4  1 `, as with the old compiler

#### Scenario: Types and widths
- **WHEN** a program prints `_SETBIT(b%%, 7)`, `_SETBIT(u~%%, 7)`, `_TOGGLEBIT(x&&, 63)` and `_SETBIT(5, 40)`
- **THEN** it prints what the old compiler's program prints

#### Scenario: A constant bit number of 64 or more
- **WHEN** a program prints `_SETBIT(0, 64); _SETBIT(0, 65); _READBIT(2, 65); _SHL(1, 64)`, and the same with
  each bit number in a LONG variable
- **THEN** both lines print ` 1  2 -1  1 `

## ADDED Requirements

### Requirement: Fix bit-number-check
With the fix `bit-number-check` on, a bit number outside 0 to 63 in `_READBIT`, `_SETBIT`, `_RESETBIT` and
`_TOGGLEBIT` SHALL raise error 5, trappable; the function gives what it gives with the fix off. `_SHL`, `_SHR`,
`_ROL` and `_ROR` SHALL be unchanged by the fix.

#### Scenario: A bit of 64
- **WHEN** a program with a handler that prints `ERR` and resumes next runs `n& = 64: PRINT _SETBIT(5, n&)` with
  the fix on
- **THEN** the handler prints 5

#### Scenario: A negative bit
- **WHEN** `PRINT _READBIT(1, -1)` runs under the same handler with the fix on
- **THEN** the handler prints 5

#### Scenario: Off
- **WHEN** the same programs run with the fix off
- **THEN** no error is raised and they print the values of requirement "Bit functions"
