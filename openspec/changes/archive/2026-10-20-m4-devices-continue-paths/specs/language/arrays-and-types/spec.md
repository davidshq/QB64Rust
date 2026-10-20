# Spec Delta

## MODIFIED Requirements

### Requirement: Member access and dotted names
`a.b` SHALL be a member access when a scalar variable `a` of a user type is visible at that point, and a plain
variable named `a.b` otherwise, also before a later `DIM a AS t` (measured, `verification/v16_m8_*`,
`v18_h_dotted_before_dim`); when an array `a` of a user type is visible and no such scalar, `a.b` SHALL be a compile
error. A member that its type does not have, a member written with another type's suffix, and a member of an
element of a numeric array SHALL be compile errors. `a(i).m` SHALL be the member `m` of element `a(i)`. A member SHALL
be readable, assignable, and passable by reference to a parameter of its exact type. Blanks around the dot after a
plain name SHALL be dropped: `a . b` SHALL be `a.b` in every one of these uses (`verification/v19_dot_blanks_*`).

#### Scenario: Dotted name without a TYPE
- **WHEN** `a.b = 3: a = 4: PRINT a.b; a` runs and no user-type variable `a` exists
- **THEN** it prints `3  4`

#### Scenario: Dotted name before the DIM
- **WHEN** `a.b = 3: PRINT a.b` runs, then `DIM a AS t` (a type with a LONG member `b`), then `PRINT a.b`
- **THEN** it prints `3`, then `0`

#### Scenario: Unknown member
- **WHEN** `a.c = 6` is compiled for a variable `a` of a type without a member `c`
- **THEN** the compiler reports an error

#### Scenario: Member of an element
- **WHEN** `a(1).b = 8` runs for `DIM a(2) AS t`
- **THEN** `PRINT a(1).b; a(2).b` prints `8  0`

#### Scenario: Member passed by reference
- **WHEN** `bump p.x` is called for a LONG member `x`
- **THEN** `p.x` is 1 higher afterwards

#### Scenario: Blanks around the dot of a member
- **WHEN** `a . s = 5: PRINT a . s: PRINT a.s` runs for `DIM a AS t` (a type with a LONG member `s`)
- **THEN** it prints ` 5 ` twice (`v19_dot_blanks_type`)

#### Scenario: Blanks around the dot of a plain dotted name
- **WHEN** `a.b = 3: PRINT a . b` runs and no user-type variable `a` exists
- **THEN** it prints ` 3 ` (`v19_dot_blanks_plain`)

#### Scenario: ERASE of a member array with blanks
- **WHEN** `ERASE a . S, b` runs for a member array `s(1 TO 3) AS LONG` of `a` and an array `b`
- **THEN** both are cleared, as with the old compiler (`v19_dot_blanks_erase`, `arrays/04_float_family`)

#### Scenario: An element of a member array with blanks
- **WHEN** `a . s(2) = 7: PRINT a.s(2)` runs for that member array
- **THEN** it prints ` 7 ` (`v47_c_dot_blanks`, which also has `LBOUND(a . s)`, `a . w . v`, `p . q . r` and a
  member passed by reference)

#### Scenario: A suffix before the dot
- **WHEN** a program has `s$ . x = "hi"`
- **THEN** the compiler reports an error that is not "not supported yet", as the old compiler does (`v47_x29`)
