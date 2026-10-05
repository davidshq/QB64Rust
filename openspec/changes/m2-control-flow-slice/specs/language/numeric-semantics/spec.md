# Spec Delta

## ADDED Requirements

### Requirement: Comparisons
`=`, `<>`, `<`, `>`, `<=`, `>=` SHALL give -1 when true and 0 when false, typed LONG. Two strings SHALL compare
byte by byte; a string with a number SHALL be a compile error. When both operands are floating point, both SHALL
be compared at the narrower of their two float types (so a SINGLE variable equals the literal it was set from).

#### Scenario: Numbers and strings
- **WHEN** `PRINT 3 > 2; 2 > 3; "a" < "b"` runs
- **THEN** it prints `-1  0 -1 `

#### Scenario: SINGLE against a literal
- **WHEN** `a! = 2.1: PRINT a! = 2.1; 2.1 = a!` runs
- **THEN** it prints `-1 -1 `

### Requirement: Logical operators
`NOT`, `AND`, `OR`, `XOR`, `EQV` and `IMP` SHALL work bit by bit on integers; a floating-point operand SHALL first
be rounded half to even to a 64-bit integer. `_ANDALSO` and `_ORELSE` SHALL give -1 or 0 and SHALL NOT evaluate
their right operand when the left one decides the result. `_NEGATE` SHALL give -1 for 0 and 0 otherwise.

#### Scenario: Float operands are rounded
- **WHEN** `PRINT 1.5 AND 3; 2.5 OR 0; NOT 1.5; 5 XOR 3; 5 EQV 3; 5 IMP 3` runs
- **THEN** it prints ` 2  2 -3  6 -7 -5 `

#### Scenario: Short-circuit operators
- **WHEN** `PRINT 1 _ANDALSO 2; 0 _ORELSE 0; _NEGATE 0; _NEGATE 5` runs
- **THEN** it prints `-1  0 -1  0 `

### Requirement: Integer division and MOD
`\` and `MOD` SHALL round floating-point operands half to even to integers first, then divide truncating toward
zero; `MOD` SHALL give the remainder with the sign of the dividend. A divisor of 0 SHALL raise error 11, which is
fatal (not trappable).

#### Scenario: Signs and rounding
- **WHEN** `PRINT 7 \ 2; -7 \ 2; 7.5 \ 2; -7 MOD 3; 7.5 MOD 2` runs
- **THEN** it prints ` 3 -3  4 -1  0 `

### Requirement: Power
`^` SHALL be left-associative at run time and computed in extended precision; its result SHALL be typed as the old
compiler types it (an INTEGER operand counts as SINGLE, a LONG as DOUBLE, an `_INTEGER64` as `_FLOAT`; the widest
wins). A negative base with a non-integer exponent SHALL raise error 5.

#### Scenario: Associativity and typing
- **WHEN** `PRINT 2 ^ 3 ^ 2; 2 ^ 0.5` runs
- **THEN** it prints ` 64  1.414214 `
