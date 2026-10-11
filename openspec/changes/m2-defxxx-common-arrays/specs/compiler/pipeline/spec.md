# Spec Delta

## ADDED Requirements

### Requirement: Array storage in the IR
An array of the IR SHALL be static (its bounds are constants, it exists for the whole program) or dynamic (it has
a number of dimensions and no bounds). The IR SHALL have operations that allocate a dynamic array with bounds
given as values (stating whether an allocated array is an error, is replaced, or is replaced keeping its elements
by flat position), that clear a static array, that release a dynamic one, and that erase an array parameter,
which is either kind when the program runs. The allocation SHALL be allowed to name a static array or a
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

#### Scenario: Static array
- **WHEN** `DIM s(5) AS LONG` is lowered in the main module without `$DYNAMIC`
- **THEN** the IR holds a static array with the bounds 0 and 5 and no operation at the `DIM` line

#### Scenario: Array argument
- **WHEN** `fill x()` is lowered for `SUB fill (a() AS LONG)`
- **THEN** the argument is a reference to the array `x`, and inside the SUB `a(1)` is an element of the parameter

### Requirement: Member arrays in the IR
An element place SHALL name its array as an array variable or as an array member of a place of a user type, so
that `x(0).n(1).q(2)` is one place built from elements and members. The bounds of a member array SHALL be part of
the user type in the program's list of types. A whole user-type store SHALL be one operation from a place to a
place of the same type. Byte offsets, the layout of a member array and how a `STRING` member is held SHALL NOT
appear in the IR.

#### Scenario: Chain of members and elements
- **WHEN** `x(0).n(1).q(2) = 77` is lowered
- **THEN** the IR shows one store whose place is element 2 of member `q` of element 1 of member `n` of element 0
  of `x`, each index converted to a 64-bit integer, and no offset arithmetic

#### Scenario: Whole value
- **WHEN** `q = p` is lowered for two variables of one user type
- **THEN** the IR holds one whole-value store from `p` to `q`

### Requirement: $STATIC and $DYNAMIC take effect
`$DYNAMIC` and `$STATIC` in a metacommand comment SHALL decide, for the declarations that follow by the old
compiler's rule for where the metacommand starts to apply, whether a `DIM` with constant bounds makes a static or
a dynamic array. They SHALL NOT be reported "not supported yet".

#### Scenario: Dynamic, then static again
- **WHEN** `'$DYNAMIC`, `DIM a(5)`, `'$STATIC`, `DIM b(5)` stand on four lines of the main module
- **THEN** `a` is a dynamic array and `b` a static one

#### Scenario: Not a metacommand
- **WHEN** `' note $DYNAMIC` stands above `DIM a(5)`
- **THEN** `a` is a static array
