# Spec Delta

## MODIFIED Requirements

### Requirement: Labels
A statement in the main module or in a procedure SHALL be able to carry a label, `name:` at the start of a line.
A label SHALL be the target of `GOTO`, `GOSUB`, `RETURN label`, `ON ERROR GOTO` and `RESUME` (spec
`language/control-flow`, "Labels per body"). Two labels with the same name in one body, and a reference to a label
that does not exist, SHALL be compile errors.

#### Scenario: Label on its own line and before a statement
- **WHEN** a program has `handler:` on a line of its own and `back: PRINT "at back"`
- **THEN** both are labels, and `PRINT "at back"` runs when control reaches `back`

### Requirement: Errors raised inside procedures
An error raised by a statement inside a procedure SHALL be handled by the main module's active handler, and
`RESUME NEXT` SHALL continue with the statement after it **inside the procedure**. `ON ERROR GOTO` inside a
procedure SHALL name a main-module label and set the same, program-wide handler; naming a label of the procedure
SHALL be a compile error, as with the old compiler, also when the main module has a label of that name. In the
main module, `ON ERROR GOTO` naming a label that stands only in a procedure SHALL be a compile error. `RESUME`
inside a procedure is not supported yet; `RESUME label` there naming a main-module label SHALL be a compile error,
as with the old compiler.

#### Scenario: Handler set inside a SUB
- **WHEN** a SUB executes `ON ERROR GOTO mainh` (a main-module label), then `ERROR 5`, then prints a line, and
  `mainh` executes `RESUME NEXT`
- **THEN** the handler runs, then the SUB prints its line

#### Scenario: ERROR inside a FUNCTION called from a PRINT
- **WHEN** `PRINT "e"; f&; "g"` calls `FUNCTION f&` that sets `f& = 1`, executes `ERROR 7`, then sets `f& = 2`,
  and the handler executes `RESUME NEXT`
- **THEN** the output is `e`, the handler's output, then ` 2 g`

#### Scenario: RETURN without GOSUB inside a SUB
- **WHEN** a SUB executes `RETURN` with no `GOSUB` pending, then prints `s2 after`, and the main handler prints
  `ERR` and executes `RESUME NEXT`
- **THEN** the handler prints ` 3 `, then the SUB prints `s2 after`

## ADDED Requirements

### Requirement: Errors in block headers
An error raised while a block header is evaluated SHALL be handled like an error in any statement, and the header
SHALL be the statement that `RESUME` re-runs. After `RESUME NEXT`, and after an untrapped error the runtime goes on
from, execution SHALL continue as in the old compiler: inside the `THEN` branch for an `IF` condition, inside the
body for a `WHILE` or `DO WHILE`/`DO UNTIL` condition, after the loop for a `LOOP WHILE`/`LOOP UNTIL` condition,
and at the start of the body for a `FOR` header, without assigning the loop variable. `IF c GOTO label` SHALL jump
when `c` raises. An `ELSEIF` condition that raises SHALL be tested with the placeholder value (spec
`compiler/pipeline`, "ELSEIF condition raises").

#### Scenario: IF condition raises
- **WHEN** `IF CHR$(k) = "a" THEN PRINT "in then" ELSE PRINT "in else"` runs with `k = -1` and the handler
  executes `RESUME NEXT`
- **THEN** the handler runs, then `in then` is printed

#### Scenario: LOOP UNTIL condition raises
- **WHEN** a `DO … LOOP UNTIL CHR$(k) = "a" OR n >= 3` loop whose body prints `n` runs with `k = -1`, and the
  handler executes `RESUME NEXT`
- **THEN** the body runs once, the handler runs, and execution continues after the loop

#### Scenario: FOR header raises
- **WHEN** `FOR i = 1 TO LEN(CHR$(k)) + 2: PRINT i;: NEXT: PRINT i` runs with `k = -1` and a handler that executes
  `RESUME NEXT`
- **THEN** the handler runs, then ` 0  1  2  3 ` is printed (the body starts with `i` unassigned, and the limit is
  the value computed with the failed call)

#### Scenario: WHILE condition raises
- **WHEN** `WHILE LEN(CHR$(k)) = 1 AND n < 2` raises with `k = -1`, and the handler sets `k = 65` and executes
  `RESUME NEXT`
- **THEN** the body runs, and the loop then continues by its condition
