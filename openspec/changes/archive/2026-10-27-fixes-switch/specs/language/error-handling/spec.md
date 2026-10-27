# Spec Delta

## ADDED Requirements

### Requirement: RESUME leaves a SUB through its exit
A `RESUME`, `RESUME 0` or `RESUME NEXT` that leaves a SUB (requirement "RESUME inside a procedure") SHALL leave it
as `EXIT SUB` does, whatever the fixes: the SUB's strings SHALL be released and the `_MEM` blocks of its locals
SHALL no longer be valid (`study\39` row 13; a `QB64PE-DIVERGENCES.md` row).

#### Scenario: A block of a local after RESUME
- **WHEN** a SUB takes `m = _MEM(x)` of a local `x`, raises error 5, and a main-module handler's call of the SUB
  ends at the SUB's `RESUME NEXT`, after which the main module tests `_MEMEXISTS(m)` of a copy of `m` kept in a
  shared variable
- **THEN** `_MEMEXISTS` gives 0

### Requirement: Fix resume-after-block
With the fix `resume-after-block` on, `RESUME NEXT` after an error raised in the condition of a `WHILE`, `DO
WHILE`, `DO UNTIL`, `LOOP WHILE`, `LOOP UNTIL`, `IF` or `ELSEIF` SHALL go on after the whole block (after its
`WEND`, `LOOP` or `END IF`; after the line of a one-line `IF`). `RESUME` SHALL still evaluate the condition again.
With the fix off, requirement "Errors in block headers" holds.

#### Scenario: WHILE condition raises
- **WHEN** `WHILE LEN(CHR$(k)) = 1: PRINT "body": WEND: PRINT "after"` runs with `k = -1`, the fix on, and a
  handler that executes `RESUME NEXT`
- **THEN** the handler runs, then `after` is printed and `body` is not

#### Scenario: IF condition raises
- **WHEN** `IF CHR$(k) = "a" THEN PRINT "in then" ELSE PRINT "in else"` then `PRINT "next"` runs with `k = -1`,
  the fix on, and a handler executing `RESUME NEXT`
- **THEN** the handler runs, then only `next` is printed

#### Scenario: Fix off
- **WHEN** the same `WHILE` runs with the fixes off
- **THEN** `body` is printed as the old compiler prints it (requirement "Errors in block headers")

### Requirement: Fix resume-in-procedure
With the fix `resume-in-procedure` on, every form of `RESUME` written inside a SUB or FUNCTION SHALL be a compile
error at it, saying that `RESUME` belongs in a handler of the main module, as in QuickBASIC 4.5
(`verification\qb45\q45_resume_in_sub`).

#### Scenario: RESUME NEXT in a SUB
- **WHEN** a SUB holds `RESUME NEXT` and the fix is on
- **THEN** the compile fails with an error at `RESUME NEXT`, not marked "not supported yet"

#### Scenario: Fix off
- **WHEN** the same program is built with the fixes off
- **THEN** it compiles (requirement "RESUME inside a procedure")
