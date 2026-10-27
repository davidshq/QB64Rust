# Spec Delta

## ADDED Requirements

### Requirement: _CAST from a float out of range
`_CAST` of a float to an integer type SHALL give what the same conversion of a variable gives on x86-64, for a
constant as for a variable and with or without constant folding, including a value outside the type's range, NaN
and infinity (`verification\v49_a_cast`; `study\39` row 16; a `QB64PE-DIVERGENCES.md` row; the old compiler's C++
cast of a constant is undefined and differs from build to build).

#### Scenario: A constant out of range
- **WHEN** `PRINT _CAST(LONG, 3E9)` runs, and `x# = 3E9: PRINT _CAST(LONG, x#)` in the same program
- **THEN** both print `-2147483648` (`verification\v49_x18`, now a divergence)

#### Scenario: Without folding
- **WHEN** the same program is built with `QB64RUST_NO_FOLD=1`
- **THEN** its output is the same
