# Spec Delta

## ADDED Requirements

### Requirement: Control characters
`_CONTROLCHR OFF` SHALL make text output show the characters 0 to 31 as glyphs instead of acting on them, and
`_CONTROLCHR ON` SHALL make them act again, as in the old compiler's program. The function `_CONTROLCHR` SHALL
give -1 while they are shown as glyphs and 0 otherwise.

#### Scenario: A bell and a line feed shown
- **WHEN** a screen program runs `_CONTROLCHR OFF: PRINT "a" + CHR$(7) + CHR$(10) + "b"; _CONTROLCHR`
- **THEN** its screen holds what the old compiler's program's screen holds: the two characters as glyphs on one
  row, and `-1`

#### Scenario: Back on
- **WHEN** the program then runs `_CONTROLCHR ON: PRINT _CONTROLCHR`
- **THEN** it prints ` 0 `

### Requirement: The default colours
`_DEFAULTCOLOR` and `_BACKGROUNDCOLOR` SHALL give, as an `_UNSIGNED LONG`, the foreground and background colour
of the destination, or with an argument of the image or page it names, as the old compiler's program gives them.
A handle of no image SHALL raise the old compiler's error.

#### Scenario: After COLOR
- **WHEN** a program in `SCREEN 12` runs `COLOR 14, 1: PRINT _DEFAULTCOLOR; _BACKGROUNDCOLOR`
- **THEN** it prints ` 14  1 `, as the old compiler's program does

#### Scenario: Of an image
- **WHEN** `h& = _NEWIMAGE(8, 8, 32): PRINT _DEFAULTCOLOR(h&) = _RGB32(255, 255, 255)` runs
- **THEN** it prints what the old compiler's program prints

#### Scenario: A handle of no image
- **WHEN** `c~& = _BACKGROUNDCOLOR(-12345)` runs under a handler
- **THEN** error 258 is raised, as in the old compiler
