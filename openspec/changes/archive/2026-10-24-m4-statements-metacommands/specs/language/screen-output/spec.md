# Spec Delta

## ADDED Requirements

### Requirement: PALETTE USING
`PALETTE USING array(index)`, `PALETTE USING array` and `PALETTE USING array()` SHALL set the palette from the
elements of an `INTEGER` or `LONG` array, starting at that element or at the array's first, as the old compiler's
program does. They SHALL raise its run-time errors, and the screen SHALL equal the old compiler's. The forms the
old compiler rejects SHALL be its errors (measured, `verification\v27_x64`, `v57_u01`–`u06`).

#### Scenario: Compiled from an element and a whole array
- **WHEN** a program with `DIM a(15) AS INTEGER, b(15) AS LONG` has `PALETTE USING a(0)`, `PALETTE USING a` and
  `PALETTE USING b(2)` after its `SYSTEM`
- **THEN** it compiles and prints `ok` (`verification\v27_x64`)

#### Scenario: The palette it sets
- **WHEN** a screen program in `SCREEN 13` fills a `LONG` array and runs `PALETTE USING b(2)`, or one in `SCREEN 7`
  runs `PALETTE USING a` of an `INTEGER` array with an element of -1, and each draws with the colours it set
- **THEN** its palette and every pixel row equal its recorded state (`tests\corpus\screen\p01_palette_using`,
  `p02_palette_using_integer`; an `INTEGER` array in `SCREEN 13` is error 5 in libqb)

#### Scenario: Run-time errors
- **WHEN** a program sets the palette from arrays with a value of -1, a value of 64, and fewer elements left than
  the palette has, under a handler that prints `ERR`
- **THEN** it prints what the old compiler's program prints (`verification\v57_u04`)

#### Scenario: Nothing after USING, or a scalar
- **WHEN** a program has `PALETTE USING`, or `PALETTE USING v` with `v` an `INTEGER` variable
- **THEN** the compile fails with an error, with the old compiler's "Expected PALETTE USING array-name" as its
  note

#### Scenario: A SINGLE array
- **WHEN** a program has `DIM a(15) AS SINGLE` and `PALETTE USING a(0)`
- **THEN** the compile fails with an error, with "Array must be of type INTEGER or LONG" as its note
  (`verification\v57_u02`)
