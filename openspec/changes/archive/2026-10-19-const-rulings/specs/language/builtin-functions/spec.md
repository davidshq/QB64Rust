## ADDED Requirements

### Requirement: _COTH is the hyperbolic cotangent
`_COTH(x)` SHALL be supported at run time as the old compiler calls it (libqb's `func_coth`, believed `_FLOAT`,
held `double`). Its value SHALL be the hyperbolic cotangent of `x`, with no upper limit and error 5 at 0, through
this repo's copy of libqb (`DIVERGENCES.md` D-031: QB64pe's `func_coth` is `2 * EXP(x) - 1`, with error 5 above
44.014845).

#### Scenario: Values
- **WHEN** `x# = .2: y# = -.5: z# = 45: PRINT CLNG(_COTH(x#) * 1000); CLNG(_COTH(y#) * 1000); _COTH(z#)` runs
- **THEN** it prints ` 5066 -2164  1 `

#### Scenario: At 0
- **WHEN** `PRINT _COTH(0)` runs under `ON ERROR` with a handler that prints `ERR`
- **THEN** the handler prints 5

#### Scenario: The same call as the old compiler
- **WHEN** the call-site check compares `d = _COTH(e)`, `_COTH(.5)`, `_COTH(n)` and `_COTH(sh)`
- **THEN** each libqb call equals the old compiler's
