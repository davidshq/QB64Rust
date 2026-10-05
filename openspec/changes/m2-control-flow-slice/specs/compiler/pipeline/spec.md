# Spec Delta

## MODIFIED Requirements

### Requirement: ABI-neutral IR
The IR SHALL NOT refer to libqb or `qbx.cpp` symbols, C types, `passed` masks, by-value temporaries, storage
allocation or the event loop. Optional arguments SHALL be represented as present or absent; procedure arguments
as a reference to a variable or a by-value copy of a value; variables SHALL carry a storage class (main module,
procedure-static, per-call local, parameter, function result, hidden temporary of a body); operations that may
raise a runtime error SHALL be marked. A body SHALL be a flat list of statements; labels SHALL be positions in a
body, the program's own and those the lowering adds, and control SHALL move between them only by jump, conditional
jump, `GOSUB` and `RETURN` operations of that body. The IR SHALL state that errors are pending and handled per
statement: a raising operation records a pending error and yields a placeholder value, the statement goes on (a
store or a call made with that value still happens), only the points the IR names check for a pending error (each
`PRINT` item: a raising item skips the rest of its statement; a jump or conditional jump is not taken while an
error is pending, unless the conditional jump is marked as using the placeholder value), errors are serviced at
the statement boundary, a retry re-runs the statement that raised, and resuming next continues after it, in the
procedure where it raised.

#### Scenario: Optional argument absent
- **WHEN** `INSTR("hello", "ll")` is lowered
- **THEN** the IR call has three argument slots, the first absent, and the C++ passes a placeholder with a
  `passed` mask of 0

#### Scenario: By-reference and by-value arguments
- **WHEN** `CALL bump(n)` and `bump (n)` are lowered for `SUB bump (x AS LONG)` and a LONG `n`
- **THEN** the first IR call passes a reference to `n` and the second a copy of its value, and only the C++
  output names the temporary that holds the copy

#### Scenario: Error inside a PRINT
- **WHEN** an item of a `PRINT` statement raises a runtime error that is not trapped (end to end: `s12_error_in_print`
  in `tests/corpus/slice`, run with `QB64PE_NOPROMPT=continue` from its `.noprompt` file, so the program goes on)
- **THEN** the remaining items of that statement, and its line end, are not printed, and the next statement's
  output follows on the same line, as with the old compiler

#### Scenario: FOR loop lowered
- **WHEN** `FOR i% = 1 TO n: PRINT i%: NEXT` is lowered
- **THEN** the IR holds the limit and the step in hidden temporaries wider than `i%`, and the loop is statements
  with labels and jumps; no C type, `fornext_` name or `goto` text appears before the C++ output

#### Scenario: Header error follows from the pending-error rule
- **WHEN** the condition of a `WHILE` raises an error and the program resumes next
- **THEN** the conditional jump that leaves the loop is not taken, because an error is pending, so the body runs,
  as with the old compiler

#### Scenario: A store made with a placeholder value
- **WHEN** `x = 5: x = ASC("")` runs and the handler executes `RESUME NEXT`
- **THEN** `x` holds what the old compiler leaves in it (measured in `verification/v17_*`, task 1.1), and the IR
  rule and the C++ output agree with that

#### Scenario: FOR limits computed with a placeholder value
- **WHEN** `FOR i = 1 TO LEN(CHR$(-1)) + 2` raises an error in its limit and the program resumes next
- **THEN** all three hidden temporaries are stored (the limit from the placeholder value), the jump to the loop
  entry is not taken, and the body runs with `i` unassigned, as with the old compiler
