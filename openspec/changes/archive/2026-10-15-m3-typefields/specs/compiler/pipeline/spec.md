# Spec Delta

## MODIFIED Requirements

### Requirement: Member arrays in the IR
An element place SHALL name its array as an array variable or as an array member of a place of a user type, so
that `x(0).n(1).q(2)` is one place built from elements and members. The bounds of a member array SHALL be part of
the user type in the program's list of types, with whether it is a dynamic member array. A whole user-type store
SHALL be one operation from a place to a place of the same type. The operation that clears an array SHALL be
allowed to name a member array, and then finds the place it is part of once and is not guarded (an index outside
its bounds on the way raises error 9 and the member array of the element at position 0 is cleared, as in the old
compiler); no operation SHALL allocate or release a member array that is not dynamic. A dynamic member array
SHALL be allocated and released by the operations that allocate and release a dynamic array, naming it as a
member array, with the same modes and the same order of evaluation and errors. A bound of a member array and a
whole member array as an array argument SHALL name it the same way; the bound SHALL NOT evaluate the indexes on
the way to a member array that is not dynamic. Byte offsets, the layout of a member array, how a `STRING` member
or a dynamic member array is held, and the setting up, release and copy of the dynamic member arrays an owner
value holds SHALL NOT appear in the IR: they follow from the type.

#### Scenario: Chain of members and elements
- **WHEN** `x(0).n(1).q(2) = 77` is lowered
- **THEN** the IR shows one store whose place is element 2 of member `q` of element 1 of member `n` of element 0
  of `x`, each index converted to a 64-bit integer, and no offset arithmetic

#### Scenario: Whole value
- **WHEN** `q = p` is lowered for two variables of one user type
- **THEN** the IR holds one whole-value store from `p` to `q`

#### Scenario: REDIM and ERASE of a member array
- **WHEN** `REDIM a(i).v(0 TO 3)` and `ERASE t.v` are lowered for a member `v(0 TO 3) AS LONG`
- **THEN** the IR holds a clear of member `v` of element `i` of `a`, marked as raising, and a clear of member `v`
  of `t`, not marked; `REDIM _PRESERVE a(i).v(0 TO 3)` gives no operation

#### Scenario: REDIM and ERASE of a dynamic member array
- **WHEN** `REDIM _RETAIN a(i).v(2 TO 5) AS LONG` and `ERASE t.v` are lowered for a member `v(1) _DYNAMIC AS LONG`
- **THEN** the IR holds an allocation of member `v` of element `i` of `a` with bounds 2 and 5 and the mode
  `Retain`, and a release of member `v` of `t`

#### Scenario: Copy of an owner value
- **WHEN** `q = p` is lowered for two variables of a type with a dynamic member array
- **THEN** the IR holds one whole-value store from `p` to `q`, as for any user type
