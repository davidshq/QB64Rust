# Spec Delta

## ADDED Requirements

### Requirement: A block of no bytes
`_MEMNEW(0)` SHALL give a valid block of size 0 that owns no memory: `_MEMFREE` of it SHALL free nothing, whatever
blocks were taken and freed before it, and a second `_MEMFREE` of it SHALL raise error 307 as for any freed
block (`study\39` row 9; a `QB64PE-DIVERGENCES.md` row).

#### Scenario: Freed after another block was freed
- **WHEN** `m = _MEMNEW(16): _MEMFREE m: n = _MEMNEW(0): _MEMFREE n: PRINT "after"` runs
- **THEN** it prints `after` and ends normally

#### Scenario: Other orders
- **WHEN** a block of `_MEMNEW(0)` is freed first, two are freed in a row, or one is freed between two blocks of
  16 bytes
- **THEN** each program ends normally with `_MEMEXISTS` 0 for every freed block

#### Scenario: Freed twice
- **WHEN** a program with a handler frees the same `_MEMNEW(0)` block twice
- **THEN** the second `_MEMFREE` raises error 307
