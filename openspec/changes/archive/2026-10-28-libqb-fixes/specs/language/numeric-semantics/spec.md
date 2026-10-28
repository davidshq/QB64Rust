# Spec Delta

## MODIFIED Requirements

### Requirement: Integer division and MOD
`\` and `MOD` SHALL round floating-point operands half to even to integers first, then divide truncating toward
zero; `MOD` SHALL give the remainder with the sign of the dividend. A divisor of 0 SHALL raise error 11, which is
trappable as other run-time errors are, the result being 0 (`study\39` row 12; a `QB64PE-DIVERGENCES.md` row; as
QuickBASIC 4.5, `DIVERGENCES-QB45.md` Q-003). The smallest value of a signed type divided by -1 SHALL raise
error 6 (Overflow, trappable) with `\` and SHALL give 0 with `MOD` (the old compiler's program crashes,
`QB64PE-DIVERGENCES.md` D-006).

#### Scenario: Signs and rounding
- **WHEN** `PRINT 7 \ 2; -7 \ 2; 7.5 \ 2; -7 MOD 3; 7.5 MOD 2` runs
- **THEN** it prints ` 3 -3  4 -1  0 `

#### Scenario: Smallest LONG divided by -1
- **WHEN** `x& = -2147483648: m& = -1` and, under an `ON ERROR` handler that prints `ERR` and resumes next,
  `PRINT x& \ m&` and then `PRINT x& MOD m&` run
- **THEN** the handler prints 6 for the first, and the second prints ` 0 `

#### Scenario: Division by zero trapped
- **WHEN** `z% = 0` and, under an `ON ERROR` handler that prints `ERR` and resumes next, `x% = 7 \ z%: PRINT x%`
  and `y% = 7 MOD z%` run
- **THEN** the handler prints 11 for each, and the program goes on to its end

#### Scenario: Division by zero with no handler
- **WHEN** `z% = 0: x% = 7 \ z%` runs with no handler and `QB64PE_NOPROMPT=y`
- **THEN** the program ends with libqb's message for error 11, "Division by zero"
