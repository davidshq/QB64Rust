# Spec Delta

## MODIFIED Requirements

### Requirement: A block that is no longer valid
A block SHALL stop being valid when it is freed, when the SUB or FUNCTION whose variable it describes returns,
and when its array is redimensioned or erased. Reading, writing or freeing through it, or outside its region,
SHALL be the old compiler's runtime error (300 to 313), which ends the program with libqb's message and is not
trapped by `ON ERROR`, as in the old compiler. `ERASE` of a dynamic array SHALL make its blocks invalid as `REDIM`
does, whatever the fixes (`study\39` row 8; a `QB64PE-DIVERGENCES.md` row); `ERASE` of a static array only clears
it, and its blocks stay valid.

#### Scenario: Write after free
- **WHEN** a program with `ON ERROR GOTO h` frees a block from `_MEMNEW` and then `_MEMPUT`s into it
- **THEN** it stops with "Memory has been freed" and the handler does not run, as with the old compiler

#### Scenario: Write outside the region
- **WHEN** `DIM a AS LONG, m AS _MEM: m = _MEM(a): _MEMPUT m, m.OFFSET + 2, 1 AS LONG` runs
- **THEN** it stops with "Memory region out of range"

#### Scenario: Block of an erased array
- **WHEN** `REDIM d(5) AS LONG`, `m = _MEM(d())`, `ERASE d`, then `PRINT _MEMEXISTS(m)` runs
- **THEN** it prints `0`, and a `_MEMPUT` through `m` then stops with the error a block of an array gives after
  `REDIM` (`verification\v32_d29_erase_dynamic`, now a divergence)

#### Scenario: Block of an erased static array
- **WHEN** `DIM s(5) AS LONG`, `m = _MEM(s())`, `ERASE s`, then `PRINT _MEMEXISTS(m)` runs
- **THEN** it prints `-1`, and a `_MEMPUT` through `m` writes into `s`
