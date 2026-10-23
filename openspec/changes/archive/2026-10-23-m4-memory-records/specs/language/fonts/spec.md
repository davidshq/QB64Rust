# Spec Delta

## MODIFIED Requirements

### Requirement: _UCHARPOS takes a LONG array
The second argument of `_UCHARPOS` SHALL be a whole array or an element of an array of LONG or `_UNSIGNED LONG`,
as in the old compiler (`verification\v31_x01_accepted`), whose elements, from that element or from the array's
first, receive the position of each character; any other argument there SHALL be the old compiler's compile
error, "Expected LONG array-name". A whole array of a LONG name that is no array (`nd&()`) SHALL be an array used
without `DIM`, as in the old compiler.

#### Scenario: Positions of the characters
- **WHEN** a program loads a font, `DIM p(10) AS LONG`, calls `n = _UCHARPOS("abc", p())` and prints `n` and the
  first four elements
- **THEN** it prints what the old compiler's program prints

#### Scenario: Not a LONG array
- **WHEN** `n = _UCHARPOS("abc", q%())` or `n = _UCHARPOS("abc", v&)` is compiled
- **THEN** it is the compile error "Expected LONG array-name"

#### Scenario: An array that is not declared
- **WHEN** a program has `n = _UCHARPOS("abc", nd&())` and no array `nd&` (`verification\v31_x01_accepted`,
  `v56_e_undeclared_arrays`)
- **THEN** it compiles with no diagnostic, and the program prints what the old compiler's program prints
