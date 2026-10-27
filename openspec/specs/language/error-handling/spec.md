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
being handled, 0 outside a handler and after `RESUME`. `ERL` SHALL give the last numbered line passed before the
error, in the order the program ran, in any body and any included file (a numbered line of a handler counts
too), and 0 when no numbered line was passed; it keeps that value after `RESUME`. Its value is the line's number
as the old compiler writes it into its C++ (`verification\v54`): `10.5` gives 10.5, `50.` 50, `1.10` 1.1, and a
suffixed number with a leading zero is read as C++ reads it (`010&` gives 8, as recorded in `v54_x25`). `ERR` SHALL
be typed LONG (the old compiler types it `_UNSIGNED LONG`; error numbers are below 2^31, so both print the same).

#### Scenario: ERROR with unusual values
- **WHEN** `ERROR 0`, `ERROR 2.5`, `ERROR 3.5` and `ERROR 70000` are each handled by a handler that prints `ERR`
- **THEN** it prints 5, 2, 4 and 70000

#### Scenario: ERR and ERL in a handler
- **WHEN** `ERROR 6` is handled by a handler that prints `ERR; ERL`, and no line of the program has a number
- **THEN** it prints ` 6  0 `

#### Scenario: ERL after a numbered line
- **WHEN** the lines `10 PRINT "a"` and `20 ERROR 6` run, and the handler prints `ERL`
- **THEN** it prints ` 20 `

#### Scenario: ERL after an unnumbered line
- **WHEN** the line numbered 30 runs, then an unnumbered line raises an error whose handler prints `ERL`
- **THEN** it prints ` 30 `, as the old compiler records (`verification\v54_a_line_numbers`)

#### Scenario: ERL after a numbered handler
- **WHEN** a handler whose first line is numbered 900 has handled an error with `RESUME NEXT`, and an unnumbered
  line then raises another
- **THEN** `ERL` is 900 (`verification\v54_a_line_numbers`)

#### Scenario: ERL in a procedure
- **WHEN** a FUNCTION's line `70 ERROR 5` raises an error handled in the main module
- **THEN** `ERL` is 70 in the handler and after the call (`verification\v54_x22`)

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

### Requirement: The line and file of an error
`_ERRORLINE` (LONG) SHALL give the main file's line where the last error was raised, 0 before any and kept after
`RESUME`; in an included file, the main file's line of the `$INCLUDE` of the outermost included file.
`_INCLERRORLINE` (LONG) SHALL give the line in the innermost included file, `_INCLERRORFILE$` its name without
its folder; 0 and empty in the main file (measured, `verification\v43_a_errorline`, `v43_b_include`).

#### Scenario: In the main file
- **WHEN** a handler that prints `_ERRORLINE; _INCLERRORLINE` and resumes next handles `ERROR 5` on line 9
- **THEN** it prints ` 9  0 `, and `_ERRORLINE` after the `RESUME NEXT` is still 9

#### Scenario: In an included file
- **WHEN** the file included on line 12 of the main file includes a file whose third line raises an error, and the
  handler prints `_ERRORLINE; _INCLERRORLINE; _INCLERRORFILE$`
- **THEN** it prints ` 12  3 ` and the inner file's name

#### Scenario: Not in a CONST
- **WHEN** `CONST c = _ERRORLINE` is compiled
- **THEN** it is a compile error

### Requirement: An error's message
`_ERRORMESSAGE$` SHALL give the message of the last error, and `_ERRORMESSAGE$(code)` the message of the code, a
LONG rounded half to even (measured, `verification\v43_a_errorline`).

#### Scenario: A message
- **WHEN** `PRINT _ERRORMESSAGE$(5); _ERRORMESSAGE$(9.6)` runs
- **THEN** it prints `Illegal function callDuplicate definition`

### Requirement: The line of a joined line
A line joined to the next by a line continuation SHALL be numbered as the last line of the join, for every
statement on it, in the functions above and in the message of an error nothing traps (measured,
`verification\v43_d_continued`, `v43_e_continued_untrapped`).

#### Scenario: A joined line
- **WHEN** `ERROR 14: x = 1 + _` stands on line 32 and `2` on line 33, under a handler that prints `_ERRORLINE`
- **THEN** it prints ` 33 `

### Requirement: The handler chain
`ON ERROR GOTO _NEWHANDLER label` SHALL keep the active handler and make `label` the active one.
`ON ERROR GOTO _LASTHANDLER` SHALL make the handler kept last the active one again and forget it. With none kept,
no handler is active. `ON ERROR GOTO label` SHALL change the active handler and leave the kept ones alone.
`ON ERROR GOTO 0` SHALL forget them all. In a procedure the label SHALL be one of the main module, as for
`ON ERROR GOTO label`.

#### Scenario: Back to the earlier handler
- **WHEN** `ON ERROR GOTO a`, then `ON ERROR GOTO _NEWHANDLER b`, `ERROR 5`, `ON ERROR GOTO _LASTHANDLER` and
  `ERROR 6` run, each handler printing its name and `ERR` and resuming next
- **THEN** it prints `b 5` and then `a 6`

#### Scenario: Nothing kept
- **WHEN** `ON ERROR GOTO _LASTHANDLER` runs with nothing kept, followed by `ERROR 5`
- **THEN** no handler runs, and the runtime reports error 5 as without a handler

#### Scenario: The upstream program
- **WHEN** `on_error/errtest` is run
- **THEN** its output equals the recorded output

### Requirement: The handler chain's errors
These forms SHALL be compile errors, as in the old compiler: `ON ERROR GOTO _LASTHANDLER label` ("No label allowed
after _LASTHANDLER"), `ON ERROR GOTO _NEWHANDLER` with no label or with another word before the label
("Expected: ON ERROR GOTO [_NEWHANDLER] label"), and `ON ERROR GOTO _NEWHANDLER 0` ("Zero not allowed after
_NEWHANDLER").

#### Scenario: Zero after NEWHANDLER
- **WHEN** `ON ERROR GOTO _NEWHANDLER 0` is compiled
- **THEN** it is a compile error

#### Scenario: A label after LASTHANDLER
- **WHEN** `ON ERROR GOTO _LASTHANDLER h` is compiled, `h` being a label of the program
- **THEN** it is a compile error

### Requirement: Line numbers in error handling
`ON ERROR GOTO n`, `ON ERROR GOTO _NEWHANDLER n` and `RESUME n` SHALL take a line number where they take a label,
with the same rules of body and the same errors (`language/control-flow`, "Line numbers as jump targets").
`ON ERROR GOTO 0` and `RESUME 0` SHALL keep their meaning; they name no line.

#### Scenario: A numbered handler
- **WHEN** `ON ERROR GOTO 900` precedes `ERROR 5`, and the line numbered 900 prints `ERR` and executes
  `RESUME NEXT`
- **THEN** it prints ` 5 ` and the program goes on after `ERROR 5`

#### Scenario: RESUME to a line number
- **WHEN** a handler executes `RESUME 100`
- **THEN** execution continues at the line numbered 100

#### Scenario: A numbered handler in the chain
- **WHEN** `ON ERROR GOTO _NEWHANDLER 10` is compiled with a line numbered 10 in the main module
- **THEN** it compiles and adds the handler to the chain as a label would (`verification\v50_x13`)

### Requirement: RESUME inside a procedure
`RESUME`, `RESUME 0`, `RESUME NEXT` and `RESUME label` inside a SUB SHALL compile, with a label of the SUB, and do
what the old compiler's program does: with no error being handled, error 20; otherwise `RESUME label` goes on at
the label and the other forms leave the SUB (measured, `verification\v57_x01`–`x08`). A label of the main module
SHALL be the old compiler's error, and `RESUME` in a FUNCTION a compile error (`v57_x05`, `x09`–`x12`).

#### Scenario: Never reached
- **WHEN** a SUB that raises error 5 under a main-module handler doing `RESUME NEXT` has `EXIT SUB`, then
  `RESUME back` with `back:` a label of the SUB
- **THEN** the program compiles, and prints the handler's line, the SUB's line after `back:` and the main
  module's line after the call (`verification\v17_d_resume_sub_label`)

#### Scenario: A label of the main module and of the SUB
- **WHEN** both the main module and the SUB have the label `back:` and the SUB's `RESUME back` is never reached
- **THEN** the program compiles and runs as the old compiler's (`verification\v17_d_resume_main_label_from_sub_both`)

#### Scenario: In a SUB the handler calls
- **WHEN** a main-module handler calls a SUB that runs `RESUME NEXT`, and the handler then prints a line and runs
  `RESUME NEXT` itself
- **THEN** the SUB is left at its `RESUME NEXT`, the handler's line is printed, and its own `RESUME NEXT` raises
  error 20 (`verification\v57_x03`)

#### Scenario: Reached inside a handler-less SUB
- **WHEN** a SUB runs `RESUME NEXT` with no error pending and no handler set
- **THEN** the program stops with the run-time error "RESUME without error" (`verification\v57_x06`)

#### Scenario: In a FUNCTION
- **WHEN** a FUNCTION holds `RESUME NEXT`
- **THEN** the compile fails with an error at it, not marked "not supported yet"

### Requirement: $ERRORLOCATION
`$ERRORLOCATION:ON` SHALL make the program record the line of each statement before it runs, as the old
compiler's program does, so that a critical error's message (out of stack space, division by zero, the library
errors) names that line and its file: "Line: n (in main module)" or "Line: n (in file.bi)". `$ERRORLOCATION:OFF`,
or neither, SHALL keep today's message. The setting SHALL hold for the whole program, also from an included file
alone (measured, `verification\v58_e01`–`e26`).

#### Scenario: A critical error with the setting on
- **WHEN** a `$CONSOLE:ONLY` program with `$ERRORLOCATION:ON` recurses in a SUB until the stack runs out
- **THEN** its message names the line of the recursive call and "main module", as the old compiler's program's
  message does (`verification\v58`)

#### Scenario: In an included file
- **WHEN** the recursion is in a SUB of an included file
- **THEN** the message names the line in that file and the file, as the old compiler's program's message does

#### Scenario: The setting off
- **WHEN** the same program has `$ERRORLOCATION:OFF`, or no setting
- **THEN** the message ends with "Enable $ErrorLocation:ON for source location details.", as today

#### Scenario: Other errors are unchanged
- **WHEN** a program with `$ERRORLOCATION:ON` runs `ERROR 5` under a handler that prints `ERL; _ERRORLINE`, and
  another raises error 9 with no handler
- **THEN** each prints what the same program without the setting prints, as with the old compiler
  (`verification\v58_e05`, `e06`)

#### Scenario: A division by zero on a joined line
- **WHEN** a `$ERRORLOCATION:ON` program divides an integer by zero in a statement that starts on line 7 and
  continues on line 8 with `_`
- **THEN** the message names line 8, as the old compiler's program's does (`verification\v58_e23`)

### Requirement: $ERRORLOCATION set twice
A program SHALL be the old compiler's error when its `$ERRORLOCATION` lines do not all agree: one `ON` and one
`OFF` in any order, in the main file or an included one, and in a branch of `$IF` that is taken. The error SHALL
stand at the first `$ERRORLOCATION` line in file order whose setting differs from the last line's (measured,
`verification\v58_e08`–`e12`). Two lines with the same setting SHALL compile.

#### Scenario: On and then off
- **WHEN** a program has `$ERRORLOCATION:ON` on line 3 and `$ERRORLOCATION:OFF` on line 5, or the other way round
- **THEN** the compile fails with an error whose note is "$ErrorLocation is a whole-program option and cannot be
  changed within the program", at line 3 (`verification\v58_e08`, `e09`)

#### Scenario: Across an included file or a $IF
- **WHEN** the `OFF` stands in an included file, or in a `$IF WIN THEN` branch, after the main file's `ON`
- **THEN** the compile fails with that error at the main file's line (`verification\v58_e10`–`e12`); in a branch
  that is not taken it changes nothing (`e13`)

#### Scenario: The same setting twice
- **WHEN** a program has `$ERRORLOCATION:ON` twice
- **THEN** it compiles

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
