# Spec Delta

## ADDED Requirements

### Requirement: Fixed-length strings to a char pointer
A `STRING * n` member of a value, or element of an array, passed to a library parameter `AS STRING` SHALL reach C
as a copy of its n bytes followed by a NUL, and the n bytes SHALL be copied back into the place after the call, its
indexes evaluated once, whatever the fixes (`study\39` row 10; a `QB64PE-DIVERGENCES.md` row). A `STRING * n`
variable of its own SHALL still pass its own bytes.

#### Scenario: A member read by strlen
- **WHEN** a TYPE holds `a AS STRING * 4` then `b AS STRING * 4`, `t.a = "abcd": t.b = "efgh"`, and a library
  function returning `strlen` of its `char*` is called with `t.a`
- **THEN** it returns 4, not 8 (`verification\v37_x66`, now a divergence)

#### Scenario: A write comes back
- **WHEN** a library SUB that upper-cases its `char*` in place is called with an element `e(i)` of a `STRING * 3`
  array, where `i` is read from a FUNCTION that counts its calls
- **THEN** `e(i)` is upper-cased after the call, and the FUNCTION was called once
