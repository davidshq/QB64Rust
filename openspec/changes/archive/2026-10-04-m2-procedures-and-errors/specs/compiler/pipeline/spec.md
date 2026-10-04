# Spec Delta

## ADDED Requirements

### Requirement: Symbol table
`sema` SHALL record every variable (including parameters and function results), procedure and label as a symbol
with its kind, its type where it has one, the span of its definition and the spans of all its references, and
SHALL answer which symbol, if any, a byte position in a file refers to. The definition of an implicit variable is
its first use. A statement that has an error MAY contribute no symbols.

#### Scenario: Definition and references of a variable
- **WHEN** a program has `DIM n AS LONG`, then `n = 1` and `PRINT n&`
- **THEN** one symbol `n` of type LONG has the name in the `DIM` as its definition and the two later names as
  references, and a position inside any of the three names gives that symbol

#### Scenario: Same name, different symbols
- **WHEN** the main module uses `x` and a SUB uses its own implicit `x`
- **THEN** the two are different symbols, and a position in the SUB's `x` gives the SUB's symbol

#### Scenario: Call before definition
- **WHEN** `PRINT twice&(n)` comes before `FUNCTION twice& (a AS LONG)`
- **THEN** the name in the call is a reference of the procedure symbol whose definition is the name in the header

## MODIFIED Requirements

### Requirement: ABI-neutral IR
The IR SHALL NOT refer to libqb or `qbx.cpp` symbols, C types, `passed` masks, by-value temporaries, storage
allocation or the event loop. Optional arguments SHALL be represented as present or absent; procedure arguments
as a reference to a variable or a by-value copy of a value; variables SHALL carry a storage class (main module,
procedure-static, per-call local, parameter, function result); operations that may raise a runtime error SHALL be
marked; labels SHALL be positions in a body, not jumps. The IR SHALL state that errors are handled per statement:
after a raising operation the rest of the statement is skipped, errors are serviced at the statement boundary, a
retry re-runs the statement that raised, and resuming next continues after it, in the procedure where it raised.

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
