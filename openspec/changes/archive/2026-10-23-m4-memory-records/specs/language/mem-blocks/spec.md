# Spec Delta

## MODIFIED Requirements

### Requirement: A block of a reference
`_MEM(reference)` SHALL give the block of a variable, a fixed-length string, an element (from it to the array's
end), a whole array `a()`, a member or a member array, and `_MEMELEMENT(reference)` the block of the one element
or value, with the old compiler's `SIZE`, `TYPE` and `ELEMENTSIZE`; for a value of a `TYPE` with a `STRING`
member, a `STRING` member counts 8 bytes, as in the old compiler. A reference the old compiler rejects (a
variable-length string) SHALL be a compile error with its message.

#### Scenario: Whole array, element and element alone
- **WHEN** `DIM b(10) AS INTEGER` and the program prints `SIZE`, `TYPE` and `ELEMENTSIZE` of `_MEM(b())`, then
  `SIZE` of `_MEM(b(4))`, then `SIZE` of `_MEMELEMENT(b(4))`
- **THEN** it prints ` 22  65666  2 `, ` 14 ` and ` 2 `, as with the old compiler

#### Scenario: Fixed-length string
- **WHEN** `DIM s AS STRING * 6, m AS _MEM: m = _MEM(s): PRINT m.SIZE; m.TYPE` runs
- **THEN** it prints ` 6  512 `

#### Scenario: Variable-length string
- **WHEN** `DIM s AS STRING, m AS _MEM: m = _MEM(s)` is compiled
- **THEN** it is a compile error ("_MEM cannot reference variable-length strings")

#### Scenario: A TYPE with a STRING member
- **WHEN** a `TYPE ts` has `n AS STRING` and `v AS LONG`, and `DIM a(3) AS ts, m AS _MEM: m = _MEM(a()): PRINT
  m.SIZE` runs (`verification\v48_x11`), and the forms of `verification\v56_g2_mem_string_member_types`
- **THEN** it prints ` 48 `, and `v56_g2` prints what the old compiler's program printed
