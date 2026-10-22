## ADDED Requirements

### Requirement: ASC statement
`ASC(s$) = n` and `ASC(s$, i) = n` SHALL set the byte at position 1, or at `i`, of the string place `s$` to the
low byte of `n`, as the old compiler's inline code does: the place, `i` and `n` evaluated in that order, each
number converted to LONG, error 5 for an empty string or a position out of range. A place that is not a string,
and an expression that is not a place, SHALL be compile errors, as `verification\v52` records them.

#### Scenario: One byte set
- **WHEN** `a$ = "a/b": ASC(a$, 2) = 92: PRINT a$` runs
- **THEN** it prints `a\b`

#### Scenario: The first byte
- **WHEN** `a$ = "xyz": ASC(a$) = 65: PRINT a$` runs
- **THEN** it prints `Ayz`

#### Scenario: Out of range
- **WHEN** `a$ = "ab": ASC(a$, 3) = 65` runs under a handler that prints `ERR`, and so does `b$ = "": ASC(b$) = 65`
- **THEN** each handler prints 5, and the strings are unchanged

#### Scenario: A value beyond a byte
- **WHEN** `a$ = "a": ASC(a$) = 321: PRINT ASC(a$)` runs
- **THEN** it prints 65, the low byte (`verification\v52_d_asc`)

#### Scenario: Not a string place
- **WHEN** a program has `ASC(n%) = 65` or `ASC("ab") = 65`
- **THEN** it is a compile error
