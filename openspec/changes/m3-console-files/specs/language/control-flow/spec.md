# Spec Delta

## ADDED Requirements

### Requirement: EXIT SELECT and EXIT CASE
`EXIT SELECT` SHALL continue after the `END SELECT` of the innermost `SELECT` that has a `CASE` or `CASE ELSE`
open around it, also from inside an `IF` or a loop within that `CASE`, and also in `SELECT EVERYCASE`. `EXIT
CASE` SHALL do the same in a plain `SELECT`, and in `SELECT EVERYCASE` SHALL go on with the next `CASE`. With
no `CASE` open around it either SHALL be a compile error.

#### Scenario: Out of a CASE from inside an IF
- **WHEN** `SELECT CASE 1: CASE 1: PRINT "a": IF 1 THEN EXIT SELECT` … `PRINT "b": CASE ELSE: PRINT "c": END
  SELECT: PRINT "d"` runs
- **THEN** it prints `a` and `d`

#### Scenario: A loop around the SELECT goes on
- **WHEN** a `FOR i = 1 TO 3` loop holds a `SELECT CASE i` whose `CASE 2` runs `EXIT SELECT` before a `PRINT`,
  and the loop prints `i` after `END SELECT`
- **THEN** it prints 1, 2 and 3, and the `PRINT` in `CASE 2` does not run

#### Scenario: From a loop inside the CASE
- **WHEN** a `CASE` body holds a `DO` loop that runs `EXIT SELECT`
- **THEN** execution continues after `END SELECT`

#### Scenario: EVERYCASE
- **WHEN** `SELECT EVERYCASE 5` has `CASE IS > 1` with `EXIT CASE` before a `PRINT`, then `CASE IS > 2` with
  `EXIT SELECT` before a `PRINT`, then `CASE IS > 3` with a `PRINT`
- **THEN** nothing is printed: the first leaves its `CASE`, the second the whole block

#### Scenario: The nearest SELECT
- **WHEN** a `SELECT` inside a `CASE` of another runs `EXIT SELECT`
- **THEN** execution continues after the inner `END SELECT`, inside the outer `CASE`

#### Scenario: No CASE open
- **WHEN** a program has `EXIT SELECT` outside any `SELECT`, in a SUB called from a `CASE`, or between `SELECT
  CASE x` and its first `CASE`
- **THEN** each is a compile error
