# Spec Delta

## MODIFIED Requirements

### Requirement: Fixed-length string declarations
`DIM name AS STRING * n`, with `n` a number literal or the name of a numeric constant, SHALL declare a string of
exactly `n` bytes, in every storage class a scalar variable may have. As in the old compiler, `n` SHALL be read as a
32-bit integer: a result of 0 or below SHALL be a compile error (QB64pe: "Cannot create a fixed string of length
0", or a failed C++ build for a negative result), any other result is the length (4294967297 gives 1;
`DECISIONS.md`, 2026-10-08); a constant of a float SHALL be rounded half to even (2.5 gives 2, 3.5 gives 4,
`verification\v56_x14`, `x29`); an expression, a float or a negative literal after `*` SHALL be a compile error,
and so SHALL a name that is no numeric constant, such as a variable, a name never assigned or a string constant
("Number/Constant expected after *", as the old compiler says, `verification\v51_x75`, `v56_x15`–`x17`). The
suffix form `name$n` SHALL declare the same type, for `DIM` and for implicit variables, and SHALL be a variable other
than `name$`. `_UNSIGNED STRING * n` SHALL mean `STRING * n`. Every byte SHALL start as NUL (0), also for a local on
each call (`DIVERGENCES-QB45.md` Q-005).

#### Scenario: Initial bytes
- **WHEN** `DIM f AS STRING * 4: PRINT LEN(f); ASC(f, 1); ASC(f, 4)` runs
- **THEN** it prints ` 4  0  0 `

#### Scenario: Length of zero
- **WHEN** `DIM f AS STRING * 0` is compiled
- **THEN** it is a compile error

#### Scenario: A variable as the length
- **WHEN** `n = 2: DIM s AS STRING * n` is compiled
- **THEN** it is a compile error, with the old compiler's wording "Number/Constant expected after *" as a note

#### Scenario: A constant of a float as the length
- **WHEN** `CONST n = 2.7, m = 3.5, k = 4.5: DIM s AS STRING * n, t AS STRING * m, u AS STRING * k: PRINT LEN(s);
  LEN(t); LEN(u)` runs
- **THEN** it prints ` 3  4  4 `, as with the old compiler

#### Scenario: Length read as 32 bits
- **WHEN** `DIM f AS STRING * 4294967297: PRINT LEN(f)` runs
- **THEN** it prints ` 1 `, as with the old compiler

#### Scenario: Suffix form
- **WHEN** `DIM p$3: p$3 = "abcdef": p$ = "x": PRINT p$3; LEN(p$3); p$` runs
- **THEN** it prints `abc 3 x`
