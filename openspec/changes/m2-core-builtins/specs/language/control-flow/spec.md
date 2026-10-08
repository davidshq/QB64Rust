# Spec Delta

## ADDED Requirements

### Requirement: SELECT CASE
`SELECT CASE e` SHALL evaluate `e` once (a plain scalar variable is instead read at each test, as in the old
compiler) and run the body of the first `CASE` with an item that matches it, or else the `CASE ELSE` body; an item SHALL be a value (equal), `a TO b` (from `a` to `b` inclusive) or `IS op v` (one of
the six comparisons). Numeric and string selectors SHALL work; a numeric item for a string selector or the reverse
SHALL be a compile error.

#### Scenario: Items of each kind
- **WHEN** `SELECT CASE x` has `CASE 1, 3`, `CASE 4 TO 6`, `CASE IS > 10`, `CASE ELSE`, and runs for `x` = 3, 5, 11
  and 8
- **THEN** the first, second, third and `ELSE` bodies run, in that order

#### Scenario: String selector
- **WHEN** `SELECT CASE s$` has `CASE "a" TO "m"` and `CASE ELSE`, with `s$ = "dog"`
- **THEN** the first body runs

#### Scenario: Selector evaluated once
- **WHEN** the selector is a FUNCTION call that prints a line, and three `CASE` items are tested
- **THEN** the line is printed once

#### Scenario: Variable selector read at each test
- **WHEN** `SELECT CASE v` (a `SHARED` variable `v = 1`) has `CASE f`, where FUNCTION `f` sets `v = 2` and returns
  0, then `CASE 2` and `CASE ELSE`
- **THEN** the `CASE 2` body runs, as with the old compiler

#### Scenario: Recursion between the test and a later CASE
- **WHEN** a recursive FUNCTION computes `SELECT CASE n * 10` and, in a `CASE` item tested before the matching one,
  calls itself with another `n`
- **THEN** the outer `SELECT` tests its later items against the inner call's value, as with the old compiler (one
  static copy per `SELECT`; `DIVERGENCES-QB45.md` Q-002)

#### Scenario: Mismatched item
- **WHEN** `SELECT CASE x` has `CASE "a"` for a numeric `x`
- **THEN** it is a compile error

### Requirement: SELECT EVERYCASE
`SELECT EVERYCASE e` SHALL run the body of every `CASE` with a matching item, in order, and the `CASE ELSE` body
only when none matched.

#### Scenario: Two matching cases
- **WHEN** `SELECT EVERYCASE x` has `CASE IS > 1` and `CASE IS > 2` and `CASE ELSE`, with `x = 5`
- **THEN** the first two bodies run, and not the `ELSE` body

### Requirement: Errors in SELECT CASE
A runtime error raised in the selector or in a `CASE` item SHALL be handled by the IR's pending-error rule, so a
handler that resumes next sees the behaviour the old compiler shows, as measured (`verification\v20_*`).

#### Scenario: Selector raises
- **WHEN** `SELECT CASE ASC("")` with `CASE 0` and `CASE ELSE` runs under a handler that resumes next
- **THEN** the same body runs as with the old compiler

### Requirement: ON GOTO and ON GOSUB
`ON n GOTO l1, l2, …` SHALL jump to the n-th label, and `ON n GOSUB l1, l2, …` SHALL `GOSUB` to it; `n` SHALL be
converted to an integer as measured. When `n` is 0 or greater than the number of labels, execution SHALL continue
with the next statement; a negative `n` SHALL raise error 5. Labels SHALL be in the same body; line numbers as
targets SHALL be reported "not supported yet".

#### Scenario: Second label
- **WHEN** `ON 2 GOTO a, b, c` runs
- **THEN** execution continues at `b`

#### Scenario: Out of range
- **WHEN** `ON 4 GOTO a, b, c` runs, followed by `PRINT "next"`
- **THEN** it prints `next`

#### Scenario: Above 255
- **WHEN** `ON 258 GOTO a, b` runs, followed by `PRINT "next"`
- **THEN** it prints `next`, as with the old compiler (QuickBASIC 4.5 raises error 5: `DIVERGENCES-QB45.md` Q-001)

#### Scenario: Negative value
- **WHEN** `ON -1 GOTO a, b` runs under a handler that prints `ERR`
- **THEN** the handler prints 5

#### Scenario: ON GOSUB returns
- **WHEN** `ON 1 GOSUB g: PRINT "back"` runs and `g` prints `in g` and executes `RETURN`
- **THEN** the output is `in g`, then `back`
