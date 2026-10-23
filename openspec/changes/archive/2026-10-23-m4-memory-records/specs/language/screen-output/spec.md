# Spec Delta

## MODIFIED Requirements

### Requirement: Forms of graphics GET and PUT
The array of a graphics `GET` or `PUT` SHALL be numeric: static or dynamic, of the main module or a procedure, or
an array parameter. A form the old compiler rejects (no array, one point for `GET`, two for `PUT`, an expression
in the array's place, an unknown action word, two action words, `_CLIP` after the action) SHALL be a compile
error. In the array's place the old compiler also takes a plain variable, a string, a `TYPE` variable or a member
of one, a name never declared, and an array of strings, of fixed-length strings or of a `TYPE`, whole or from an
element or a member of one: each SHALL pass the bytes the old compiler's program passes (a string its characters,
a member the bytes from it to the end of its variable or array, a name alone that is an array of strings or of
fixed-length strings the scalar of that name). An index that calls a FUNCTION there, and a `TYPE` that holds a
`STRING` (whose pointers the old compiler would overwrite), SHALL be reported "not supported yet".

#### Scenario: From an element, and an array parameter
- **WHEN** a program has `GET (1, 2)-(3, 4), a(5)` and, in a SUB with a parameter `p() AS INTEGER`, `PUT (1, 2), p,
  XOR`
- **THEN** each compiles to the runtime call the old compiler writes for it

#### Scenario: A rejected form
- **WHEN** a program has `PUT (1, 2), a, PSET XOR` or `GET (1, 2), a`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: An array of a user type
- **WHEN** a program has `GET (1, 2)-(3, 4), r` for an array `r` of a `TYPE`
- **THEN** it compiles with no diagnostic, to the runtime call the old compiler writes for it

#### Scenario: A plain variable too small
- **WHEN** `x = 1: PUT (0, 0), x` runs (`verification\v16_m5_put_graphics_no_array`)
- **THEN** it raises error 5, as with the old compiler

#### Scenario: Each new kind on the screen
- **WHEN** `verification\v56_d_get_put_kinds`, on a hidden screen (`$SCREENHIDE`), takes rectangles into a plain
  variable, a string, a fixed-length string, a `TYPE` variable and member, an undeclared name, and arrays of
  strings, fixed-length strings and a `TYPE`, prints the bytes taken, puts each back and prints the `POINT`s it
  drew, and takes rectangles into values too small
- **THEN** it prints what the old compiler's program printed

#### Scenario: An index that calls a FUNCTION
- **WHEN** a program has `GET (1, 2)-(3, 4), a(f(1))` for a FUNCTION `f`
- **THEN** the only diagnostic is "not supported yet"
