## ADDED Requirements

### Requirement: Searching from the end with _INSTRREV
`sema` SHALL compile `_INSTRREV(s$, find$)` and `_INSTRREV(start, s$, find$)`. Each SHALL return what the old
compiler's program returns: the position of the last `find$` in `s$` that starts at or before `start` (the whole
string without `start`), and 0 when there is none. Edge cases (an empty `find$`, `start` of 0, below 0, or beyond
the length) SHALL give the old compiler's results as `verification\v52` records them. The result SHALL be a LONG.

#### Scenario: The last match
- **WHEN** `PRINT _INSTRREV("a.b.c", ".")` runs
- **THEN** it prints ` 4 `

#### Scenario: A start position
- **WHEN** `PRINT _INSTRREV(3, "a.b.c", ".")` runs
- **THEN** it prints ` 2 `

#### Scenario: No match
- **WHEN** `PRINT _INSTRREV("abc", "x")` runs
- **THEN** it prints ` 0 `

#### Scenario: Edge cases as recorded
- **WHEN** a program of `verification\v52` calls `_INSTRREV` with an empty string, an empty `find$`, or a `start` of
  0, below 0 or beyond the length
- **THEN** it prints what its recorded output holds

### Requirement: The reciprocal and hyperbolic functions at run time
`sema` SHALL compile `_ARCSEC`, `_ARCCSC`, `_ARCCOT`, `_SECH` and `_CSCH` of one numeric argument, outside a
`CONST` as well as inside it. As `_SEC` and `_COT` are, they SHALL be believed `_FLOAT` and computed by the old
compiler's runtime entries, so that a value prints as the old compiler's program prints it. An argument outside a
function's domain SHALL raise error 5 and return 0, as the old compiler's program does (`verification\v52_b_math`).

#### Scenario: Values
- **WHEN** `x# = 1.2: PRINT _ARCSEC(x#); _ARCCSC(x#); _ARCCOT(x#); _SECH(x#); _CSCH(x#)` runs
- **THEN** it prints what the old compiler's program prints

#### Scenario: Outside the domain
- **WHEN** `PRINT _ARCSEC(0.5)` runs (outside the domain of `_ARCSEC`)
- **THEN** it raises error 5, and after `RESUME NEXT` the value is 0

#### Scenario: The other domains
- **WHEN** a program calls `_ARCCSC(-0.5)`, `_SECH(88.1)`, `_CSCH(88.1)` or `_CSCH(0)`
- **THEN** each raises error 5 and returns 0
