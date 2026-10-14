# Spec Delta

## Purpose

Fonts and text drawn in graphics: loading, choosing and freeing a font, the size of a font and of a text,
`_PRINTSTRING`, `_UPRINTSTRING` and the other Unicode font functions, `_PRINTMODE` and `_MAPUNICODE`, as the old
compiler's program behaves.

## ADDED Requirements

### Requirement: Forms of the font built-ins
`_LOADFONT`, `_FONT`, `_FREEFONT`, `_FONTWIDTH`, `_FONTHEIGHT`, `_PRINTSTRING`, `_PRINTWIDTH`, `_PRINTMODE`,
`_UFONTHEIGHT`, `_UPRINTWIDTH`, `_ULINESPACING`, `_UPRINTSTRING`, `_UCHARPOS` and `_MAPUNICODE` SHALL compile in
each form the old compiler accepts, with the call it writes; a form it rejects SHALL be a compile error that is not
"not supported yet".

#### Scenario: Statements with their words
- **WHEN** `_PRINTMODE _KEEPBACKGROUND`, `_PRINTMODE _FILLBACKGROUND, img`, `_PRINTSTRING (1, 2), "a"`,
  `_PRINTSTRING STEP(1, 2), "a", img` and `_FONT 16, img` are compiled
- **THEN** each compiles to the call the old compiler writes for it

#### Scenario: Result types
- **WHEN** `PRINT HEX$(_LOADFONT("cour.ttf", 16)); HEX$(_FONTWIDTH); HEX$(_PRINTWIDTH("abc"))` is compiled
- **THEN** each function has the result type the old compiler gives it

### Requirement: _UCHARPOS takes a LONG array
The second argument of `_UCHARPOS` SHALL be a whole array or an element of an array of LONG or `_UNSIGNED LONG`,
as in the old compiler (`verification\v31_x01_accepted`), whose elements, from that element or from the array's
first, receive the position of each character; any other argument there SHALL be the old compiler's compile
error, "Expected LONG array-name".

#### Scenario: Positions of the characters
- **WHEN** a program loads a font, `DIM p(10) AS LONG`, calls `n = _UCHARPOS("abc", p())` and prints `n` and the
  first four elements
- **THEN** it prints what the old compiler's program prints

#### Scenario: Not a LONG array
- **WHEN** `n = _UCHARPOS("abc", q%())` or `n = _UCHARPOS("abc", v&)` is compiled
- **THEN** it is the compile error "Expected LONG array-name"

### Requirement: Built-in fonts
`_FONT 8`, `_FONT 14` and `_FONT 16` SHALL choose libqb's built-in fonts, and `_FONTWIDTH`, `_FONTHEIGHT`, `_FONT`
as a function and `_PRINTWIDTH` SHALL give what the old compiler's program gives, without a font file.

#### Scenario: Text in each built-in font
- **WHEN** a screen program in `SCREEN 12` prints a line in each of the fonts 8, 14 and 16 with `_PRINTSTRING`
- **THEN** the state of its screen equals the old compiler's

### Requirement: Fonts loaded from Windows
`_LOADFONT` SHALL load a font file by a bare name from the font folders libqb searches (the user's and the
system's on Windows), with the options of its third argument and the index of its fourth, and `_FREEFONT` SHALL
free it; text drawn in a loaded font SHALL show what the old compiler's program shows.

#### Scenario: Courier New
- **WHEN** a screen program loads `cour.ttf` at size 16, chooses it with `_FONT` and prints text with `PRINT` and
  `_PRINTSTRING`
- **THEN** the state of its screen equals the old compiler's

#### Scenario: A font that is not there
- **WHEN** `PRINT _LOADFONT("no-such-font.ttf", 16)` runs
- **THEN** it prints what the old compiler's program prints

### Requirement: Unicode text and the print mode
`_UPRINTSTRING`, `_UPRINTWIDTH`, `_UFONTHEIGHT`, `_ULINESPACING`, `_PRINTMODE` and `_MAPUNICODE` SHALL draw and
measure text as libqb does, so that the screen and the values printed are the old compiler's program's.

#### Scenario: The background of text
- **WHEN** a screen program draws text over a drawing in each of `_FILLBACKGROUND`, `_KEEPBACKGROUND` and
  `_ONLYBACKGROUND`, and prints `_PRINTMODE`
- **THEN** the state of its screen equals the old compiler's

#### Scenario: A character mapped
- **WHEN** a program maps code 1 to another character with `_MAPUNICODE` and prints `_MAPUNICODE(1)`
- **THEN** it prints what the old compiler's program prints
