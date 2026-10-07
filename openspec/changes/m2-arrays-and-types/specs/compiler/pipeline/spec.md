# Spec Delta

## MODIFIED Requirements

### Requirement: ABI-neutral IR
The IR SHALL NOT refer to libqb or `qbx.cpp` symbols, C types, `passed` masks, by-value temporaries, storage
allocation, array descriptors, byte offsets or the event loop. Optional arguments SHALL be represented as present
or absent; procedure arguments as a reference to a place or a by-value copy of a value; variables SHALL carry a
storage class (main module, procedure-static, per-call local, parameter, function result, hidden temporary of a
body); operations that may raise a runtime error SHALL be marked. A place SHALL be a variable, an element of an
array (one index per dimension, already converted to a 64-bit integer), or a member of a place of a user type;
values SHALL be read from places and stored into places. Types SHALL include user types, named by an id of the
program's list of user types. A body SHALL be a flat list of statements; labels SHALL be positions in a body, the
program's own and those the lowering adds, and control SHALL move between them only by jump, conditional jump,
`GOSUB` and `RETURN` operations of that body. The IR SHALL state that errors are pending and handled per
statement: a raising operation records a pending error and yields a placeholder value, the statement goes on (a
store made with that value still happens, by the rule of its place below), only the points the IR names check for
a pending error (each `PRINT` item: a raising item skips the rest of its statement; a procedure's entry: a
procedure entered while an error is pending returns at once; a jump or conditional jump is not taken while an
error is pending, unless the conditional jump is marked as using the placeholder value; a store into an array
element: its indexes are evaluated first, and while an error is pending after them the value is not evaluated
and the store is skipped), errors are serviced at the boundary of the statement where they were raised, or, when
a conditional jump using the placeholder value leaves that statement, at the boundary of the next statement that
runs; a retry re-runs the statement where the error is serviced, and resuming next continues after it, in the
procedure where it raised. The first error raised in a statement SHALL be the one serviced. A store into a variable
or into a member of a variable SHALL NOT check for a pending error. A store into a member of an array element SHALL
evaluate the value before the indexes, and SHALL be skipped when an index is out of range or when the indexes
raised an error while none was pending before them (`DIVERGENCES.md` D-004). An index out of range SHALL raise
error 9, and a read with it SHALL give the value at the array's first position.

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
- **THEN** `x` holds 0, the placeholder value, as with the old compiler (`verification/v17_a_pending`)

#### Scenario: A SUB called with a raising argument
- **WHEN** `show ASC("")` calls `SUB show (v AS SINGLE)` that prints a line, and the handler executes `RESUME NEXT`
- **THEN** the handler runs and the SUB prints nothing, as with the old compiler

#### Scenario: ELSEIF condition raises
- **WHEN** a block `IF` whose `IF` condition is false reaches `ELSEIF CHR$(k) = "a" THEN` with `k = -1`, its `ELSE`
  branch prints `else`, and the handler executes `RESUME NEXT`
- **THEN** the condition is tested with the placeholder value `""` (false), the error is serviced at the `ELSE`
  branch's `PRINT`, which prints nothing, and execution continues after it; with `RESUME` instead (and `k` set to
  65 by the handler), the `PRINT` is re-run and prints `else`

#### Scenario: FOR limits computed with a placeholder value
- **WHEN** `FOR i = 1 TO LEN(CHR$(-1)) + 2` raises an error in its limit and the program resumes next
- **THEN** all three hidden temporaries are stored (the limit from the placeholder value), the jump to the loop
  entry is not taken, and the body runs with `i` unassigned, as with the old compiler

#### Scenario: Places lowered
- **WHEN** `x(i) = 1: p.m = 2: a(i).m = 3: y = x(i) + p.m` is lowered for an array `x`, a user-type variable `p`
  and an array `a` of that type
- **THEN** the IR names an element place, a member place and a member of an element place, each index converted to
  a 64-bit integer, and no descriptor slot, `array_check` or byte offset appears before the C++ output

#### Scenario: Element store with a raising index
- **WHEN** `x(99) = 5` runs for `DIM x(10)` and the handler executes `RESUME NEXT`
- **THEN** error 9 is serviced and no element of `x` changes, as with the old compiler

#### Scenario: Element store with a raising value
- **WHEN** `x(9) = ASC("")` runs with `x(9)` holding 5 and the handler executes `RESUME NEXT`
- **THEN** `x(9)` holds 0, the placeholder value, as with the old compiler

#### Scenario: Member store with a raising value
- **WHEN** `p.m = ASC("")` runs with `p.m` holding 5 and the handler executes `RESUME NEXT`
- **THEN** `p.m` holds 0, the placeholder value, as with the old compiler

#### Scenario: Member of an element with a bad index
- **WHEN** `a(9).m = 5` runs for `DIM a(3) AS t` with `a(0).m` holding 70 and the handler executes `RESUME NEXT`
- **THEN** error 9 is serviced and `a(0).m` still holds 70 (the old compiler writes 5 there: `DIVERGENCES.md` D-004)

#### Scenario: Value before index in a member-of-element store
- **WHEN** `a(9).m = ASC("")` runs for the same array under a handler that prints `ERR`
- **THEN** the handler prints 5 once, the value's error, as with the old compiler, and no element changes
