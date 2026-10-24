# Spec Delta

## MODIFIED Requirements

### Requirement: READ
`READ target, …` SHALL store the next item into each target in turn (a variable, an array element or a member), a
numeric target taking the item's value converted to the target's type and a string target its text, as the old
compiler's runtime does. Reading when no item is left SHALL raise error 4. An unquoted item that is no number, and
every quoted item, read into a numeric target SHALL raise error 2, and a number outside the target's range error
6; the target SHALL then be 0, every later target of that `READ` SHALL be 0 or, a string, unchanged, and the item
SHALL stay unread, so that the next `READ` meets it again. A target that is not a place SHALL be a compile error,
as SHALL the name of the FUNCTION the statement stands in. A target in parentheses SHALL be the place inside
them, as in the old compiler; a whole array SHALL be its first element in memory (`language/arrays-and-types`,
requirement "A whole array as a value").

#### Scenario: Numbers and strings
- **WHEN** `READ a&, s$, d#: PRINT a&; s$; d#` runs with `DATA 1,"two",3.5`
- **THEN** it prints ` 1 two 3.5 `

#### Scenario: Out of data
- **WHEN** a second `READ a` runs with `DATA 1` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 4

#### Scenario: Into an element
- **WHEN** `DIM x(2) AS INTEGER: FOR i = 0 TO 2: READ x(i): NEXT: PRINT x(0); x(1); x(2)` runs with `DATA 4,5,6`
- **THEN** it prints ` 4  5  6 `

#### Scenario: Target is not a place
- **WHEN** `READ 5` is compiled
- **THEN** it is a compile error

#### Scenario: An item that is no number stays
- **WHEN** `READ a&, b&, c&` runs with `DATA 11, zz, 33` under a handler that prints `ERR` and resumes next, and
  then `READ s$`
- **THEN** the handler prints 2 once, `a&` is 11, `b&` and `c&` are 0, and `s$` is `zz`

#### Scenario: A quoted number
- **WHEN** `READ a&` runs with `DATA "12"` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 2 and `a&` is 0, as with the old compiler

#### Scenario: Too large for the target
- **WHEN** `READ b%%` runs with `DATA 300` under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 6 and `b%%` is 0

#### Scenario: The FUNCTION's own name
- **WHEN** `READ f&` is compiled inside `FUNCTION f&`
- **THEN** it is a compile error, as in the old compiler

#### Scenario: A target in parentheses
- **WHEN** `READ (a): PRINT a` runs with `DATA 2`
- **THEN** it prints ` 2 `, as the old compiler's program (`verification\v22_x84`)

#### Scenario: An element, two pairs, two targets
- **WHEN** `READ (x(1))`, `READ ((a))` and `READ (a), (b)` run
- **THEN** each reads into the places inside the parentheses (`verification\v57_p04`, `p05`, `p13`)
