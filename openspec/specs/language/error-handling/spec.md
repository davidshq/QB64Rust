# language/error-handling Specification

## Purpose
How runtime errors are trapped and resumed: labels, `ON ERROR GOTO`, `RESUME`, `ERROR`, `ERR` and `ERL`. Errors
are handled per statement (spec `compiler/pipeline`, ABI-neutral IR). Every rule is measured with `qb64pe.exe`;
the scenarios are tests in `tests\corpus\slice\` or `tests\frontend\`.

## Requirements

### Requirement: Labels
A statement in the main module or in a procedure SHALL be able to carry a label, `name:` at the start of a line.
A label SHALL be the target of `GOTO`, `GOSUB`, `RETURN label`, `ON ERROR GOTO` and `RESUME` (spec
`language/control-flow`, "Labels per body"). Two labels with the same name in one body, and a reference to a label
that does not exist, SHALL be compile errors.

#### Scenario: Label on its own line and before a statement
- **WHEN** a program has `handler:` on a line of its own and `back: PRINT "at back"`
- **THEN** both are labels, and `PRINT "at back"` runs when control reaches `back`

### Requirement: Error handler
`ON ERROR GOTO label` SHALL make the statements from `label` on the active error handler; `ON ERROR GOTO 0` SHALL
remove it. When a statement raises a trappable error and a handler is active and not already running, the rest of
the statement SHALL be skipped and the handler SHALL run, with `ERR` giving the error number. Without a handler,
the error SHALL be reported by the runtime as with the old compiler. Errors the runtime treats as fatal (for
example division by zero, error 11) SHALL NOT be trappable.

#### Scenario: Handler runs
- **WHEN** `ON ERROR GOTO handler` precedes `ERROR 5`, and the handler prints `ERR`
- **THEN** it prints ` 5 `

#### Scenario: Fatal error is not trapped
- **WHEN** `ON ERROR GOTO handler` precedes `ERROR 11`
- **THEN** the handler does not run, and the program stops with "Division by zero"

### Requirement: Resume
`RESUME` and `RESUME 0` SHALL run the statement that raised the error again from its start; `RESUME NEXT` SHALL
continue with the statement after it; `RESUME label` SHALL continue at the label. Each SHALL end the handler and
reset `ERR` to 0. `RESUME` while no handler is running SHALL raise error 20 at run time.

#### Scenario: Retry re-runs the whole statement
- **WHEN** `PRINT "a"; CHR$(k); "b"` runs with `k = -1`, and the handler sets `k = 65` and executes `RESUME`
- **THEN** the output is `a`, then the handler's output, then `aAb`

#### Scenario: RESUME NEXT skips the rest of the statement
- **WHEN** `PRINT "c"; CHR$(300); "d"` raises error 5 and the handler executes `RESUME NEXT`
- **THEN** `c` is printed, the handler's output follows on the same line, and neither `d` nor the line end of
  that PRINT is printed

#### Scenario: Retry reaches a second handler
- **WHEN** a handler sets `ON ERROR GOTO h2` and executes `RESUME`, and the retried statement raises again
- **THEN** `h2` runs

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

### Requirement: ERROR, ERR and ERL
`ERROR n` SHALL raise error `n`, rounded half to even when fractional (a value of 0 or less raises error 5;
errors the runtime treats as critical, such as 256, end the program even with a handler). `ERR` SHALL give the number of the error
being handled, 0 outside a handler and after `RESUME`. `ERL` SHALL give the last numeric line label passed before
the error, 0 when there is none (numeric line labels are not supported yet, so `ERL` is 0). `ERR` SHALL be typed
LONG (the old compiler types it `_UNSIGNED LONG`; error numbers are below 2^31, so both print the same).

#### Scenario: ERROR with unusual values
- **WHEN** `ERROR 0`, `ERROR 2.5`, `ERROR 3.5` and `ERROR 70000` are each handled by a handler that prints `ERR`
- **THEN** it prints 5, 2, 4 and 70000

#### Scenario: ERR and ERL in a handler
- **WHEN** `ERROR 6` is handled by a handler that prints `ERR; ERL`
- **THEN** it prints ` 6  0 `

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
