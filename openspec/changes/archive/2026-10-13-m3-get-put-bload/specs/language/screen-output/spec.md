# Spec Delta

## ADDED Requirements

### Requirement: Graphics GET and PUT
`GET [STEP](x1, y1)-[STEP](x2, y2), array` SHALL copy the rectangle of the screen into the array, and `PUT
[STEP](x, y), array[, [_CLIP] [PSET|PRESET|AND|OR|XOR][, colour]]` SHALL draw it back, each in every form of its
template as the runtime call the old compiler writes: the array given whole (`a`, `a()`) or from an element
(`a(i)`), as the bytes from that element to the array's end.

#### Scenario: Taken and put back
- **WHEN** a screen program draws in `SCREEN 12`, takes a rectangle with `GET (10, 10)-(40, 30), a`, and draws it
  at another place with `PUT (100, 50), a, PSET`
- **THEN** the state of its screen is the one recorded with the old compiler

#### Scenario: Each action
- **WHEN** a screen program puts one picture over a drawn background with no action word and with `PSET`,
  `PRESET`, `AND`, `OR` and `XOR`
- **THEN** the state of its screen is the one recorded with the old compiler

#### Scenario: The message box of Civil War Strategy
- **WHEN** `g06_message_box`, cut from the game, takes the screen under a box, draws the box and puts the screen
  back
- **THEN** the state of its screen is the one recorded with the old compiler

### Requirement: Forms of graphics GET and PUT
The array of a graphics `GET` or `PUT` SHALL be numeric: static or dynamic, of the main module or a procedure, or
an array parameter. A form the old compiler rejects (no array, one point for `GET`, two for `PUT`, an expression
in the array's place, an unknown action word, two action words, `_CLIP` after the action) SHALL be a compile
error. An array of a user type or of strings and a plain variable in the array's place, which the old compiler
accepts, SHALL be reported "not supported yet".

#### Scenario: From an element, and an array parameter
- **WHEN** a program has `GET (1, 2)-(3, 4), a(5)` and, in a SUB with a parameter `p() AS INTEGER`, `PUT (1, 2), p,
  XOR`
- **THEN** each compiles to the runtime call the old compiler writes for it

#### Scenario: A rejected form
- **WHEN** a program has `PUT (1, 2), a, PSET XOR` or `GET (1, 2), a`
- **THEN** the compiler reports an error that is not "not supported yet"

#### Scenario: An array of a user type
- **WHEN** a program has `GET (1, 2)-(3, 4), r` for an array `r` of a `TYPE`
- **THEN** the only diagnostic is "not supported yet"

### Requirement: TAB and SPC in text output
`TAB(n)` and `SPC(n)` SHALL be functions that give a string, as in the old compiler: as an item of `PRINT` they
SHALL move the cursor to column `n`, on the next line when the cursor is past it, or print `n` blanks, in the
window, on the console and in a file as the old compiler's program does. A string item MAY follow them without a
separator; a number or another `TAB` without one SHALL be a compile error.

#### Scenario: Columns in a window
- **WHEN** a screen program runs `PRINT "a"; TAB(5); "b"`, `PRINT "abcdefgh"; TAB(5); "below"` and `PRINT "a";
  SPC(3); "b"`
- **THEN** the state of its screen is the one recorded with the old compiler

#### Scenario: A report of Civil War Strategy
- **WHEN** a program cut from the game prints a report laid out with `TAB`
- **THEN** the state of its screen is the one recorded with the old compiler
