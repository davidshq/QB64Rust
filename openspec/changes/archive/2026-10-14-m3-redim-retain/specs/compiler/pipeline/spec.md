## MODIFIED Requirements

### Requirement: Array storage in the IR
An array of the IR SHALL be static (its bounds are constants, it exists for the whole program) or dynamic (it has
a number of dimensions and no bounds). A dynamic array that lives for the program MAY carry constant bounds it is
allocated with when the program starts (`STATIC a(3)` in a procedure); no operation allocates it then, and the IR
SHALL NOT say where that allocation stands in the output. The IR SHALL have operations that allocate a dynamic
array with bounds given as values (stating whether an allocated array is an error, is replaced, is replaced
keeping its elements by flat position, or is replaced keeping its elements by their indexes), that clear a static
array, that release a dynamic one, and that erase an
array parameter, which is either kind when the program runs. The allocation SHALL be allowed to name a static array or a
parameter (it then raises an error when it runs). An array SHALL carry a storage class
as a scalar does, and an array parameter SHALL be a reference to the caller's array. The IR SHALL NOT state when a
per-call array is released, nor name a descriptor, an allocation function or a size in bytes.

#### Scenario: DIM with a variable bound
- **WHEN** `n = 4: DIM d(n) AS LONG` is lowered
- **THEN** the IR holds a dynamic array `d` of one dimension and an allocation with the bounds 0 and `n` converted
  to a 64-bit integer, marked as raising, and no descriptor slot or `calloc` appears before the C++ output

#### Scenario: REDIM _PRESERVE
- **WHEN** `REDIM _PRESERVE d(9) AS LONG` is lowered
- **THEN** the IR holds the same operation marked as keeping the elements

#### Scenario: REDIM _RETAIN
- **WHEN** `REDIM _RETAIN d(2 TO 9) AS LONG` is lowered
- **THEN** the IR holds the same operation marked as keeping the elements by their indexes, and no loop over the
  dimensions or old descriptor appears before the C++ output

#### Scenario: Static array
- **WHEN** `DIM s(5) AS LONG` is lowered in the main module without `$DYNAMIC`
- **THEN** the IR holds a static array with the bounds 0 and 5 and no operation at the `DIM` line

#### Scenario: STATIC array of a procedure
- **WHEN** a SUB with `STATIC c(3) AS LONG` is lowered
- **THEN** the IR holds a dynamic array `c` of one dimension with static storage, allocated at the start with the
  bounds 0 and 3, and no operation at the `STATIC` line

#### Scenario: Local array
- **WHEN** a SUB with `DIM t(5) AS LONG` is lowered
- **THEN** the IR holds a local dynamic array `t` and an allocation at the `DIM` line, and nothing that releases it

#### Scenario: Array argument
- **WHEN** `fill x()` is lowered for `SUB fill (a() AS LONG)`
- **THEN** the argument is a reference to the array `x`, and inside the SUB `a(1)` is an element of the parameter
