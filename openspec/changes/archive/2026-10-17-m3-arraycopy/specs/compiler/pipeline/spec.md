## ADDED Requirements

### Requirement: Partial array copies in the IR
The IR SHALL have one operation for `_ARRAYCOPY`: a source and a destination, each an array variable or a member
array, with the source's ranges (none for the whole array) and the destination's starts (none for the whole
array) as 64-bit integer values, evaluated in the statement's order. Bounds checks, the copy by kind and the
choice of runtime entry point SHALL follow from the operands and SHALL NOT appear as other operations.

#### Scenario: A range to a start
- **WHEN** `_ARRAYCOPY s(2 TO 4) TO d(f(1))` is lowered
- **THEN** the IR holds one copy from `s` with the range 2 to 4 to `d` at the value of `f(1)`, each converted to a
  64-bit integer, marked as raising, and no loop, descriptor or bounds check appears before the C++ output

#### Scenario: Member arrays
- **WHEN** `_ARRAYCOPY t(1).v() TO a(1, 1)` is lowered for a member array `v` of two dimensions
- **THEN** the IR holds one copy from member `v` of `t(1)` as a whole to `a` at the two starts
