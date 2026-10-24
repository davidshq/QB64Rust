# language/screen-output Specification

## Purpose
What a program writes to its window and reads from it: text output without a console, the screen modes, the
text-screen and drawing statements, the functions that read the screen and the keyboard, and the pacing
statements. Every behaviour is the old compiler's, checked by the state of the screen a program leaves
(`testing/screen-state`).

## Requirements

### Requirement: Text output in a window
In a program whose destination is the window, `PRINT`, `WRITE`, `INPUT` and `LINE INPUT` SHALL write to and read
from the screen as the old compiler's program does: the same cells, attributes and cursor position at exit.

#### Scenario: PRINT with no metacommand
- **WHEN** a program is `PRINT "Hello";` and `SYSTEM`
- **THEN** it compiles, and the state of its screen at exit equals the one recorded from the old compiler's
  program: `SCREEN 0`, 80 by 25 cells, `Hello` in the first row with attribute 07

#### Scenario: INPUT in a window
- **WHEN** `tests\corpus\screen\c07_input_window` is built and run with its keys file
- **THEN** the state of its screen equals its recorded state

### Requirement: The end of a program with a window
A program with a window SHALL end as the old compiler's program does: `SYSTEM` leaves at once; `END`, and running
off the last line, print the old compiler's prompt into the bottom row and wait for a key; a runtime error with no
handler leaves the screen as the old compiler's program leaves it.

#### Scenario: END waits for a key
- **WHEN** `tests\corpus\screen\c08_end_prompt` is built and run with its keys file
- **THEN** the program ends after the key and the state of its screen equals its recorded state

#### Scenario: Off the last line
- **WHEN** `tests\corpus\screen\c09_off_the_end` is built and run with its keys file
- **THEN** the state of its screen equals its recorded state

#### Scenario: A runtime error
- **WHEN** `tests\corpus\screen\c10_runtime_error` is built and run
- **THEN** the state of its screen equals its recorded state

### Requirement: SCREEN
`SCREEN mode[, [colorswitch][, [activepage][, visualpage]]]` SHALL select the screen mode and pages as the old
compiler's program does, for every legacy mode (0, 1, 2, 7 to 13) and for an image handle; a mode the runtime
rejects SHALL raise the runtime error the old compiler's program raises. Each argument SHALL be a number of any
numeric type, converted as the old compiler converts it.

#### Scenario: Every legacy mode
- **WHEN** the `m…` programs of `tests\corpus\screen` that use only statements of this capability are built and run
- **THEN** each leaves the mode, size, palette and pixels of its recorded state

#### Scenario: Pages
- **WHEN** `tests\corpus\screen\c03_pages` is built and run
- **THEN** the pages displayed and written to, and the contents of each page, equal its recorded state

#### Scenario: A mode that does not exist
- **WHEN** `SCREEN 3` runs under an error handler
- **THEN** the error number is the one the old compiler's program reports

### Requirement: Text-screen statements
`CLS`, `COLOR`, `LOCATE`, `WIDTH` and `VIEW PRINT` SHALL be compiled in every form the old compiler accepts, with
arguments left out where it allows, and SHALL change the screen as the old compiler's program does. A form the old
compiler rejects SHALL be an error of the program, never "not supported yet".

#### Scenario: Colours, positions and the cursor
- **WHEN** `tests\corpus\screen\c01_text_cells` is built and run (`COLOR 14, 1`, `LOCATE 5, 10`, `LOCATE 3, 3, 1`)
- **THEN** the characters, attributes and cursor of its screen equal its recorded state

#### Scenario: Arguments left out
- **WHEN** a program runs `LOCATE , 5`, `COLOR , 2` and `CLS`
- **THEN** each changes only what the old compiler's program changes

#### Scenario: WIDTH
- **WHEN** `tests\corpus\screen\m00_width_80_50` is built and run
- **THEN** its screen has the cells of its recorded state

#### Scenario: A rejected form
- **WHEN** a program has `LOCATE 1, 2, 3, 4, 5, 6`
- **THEN** the compile fails with an error that is not "not supported yet"

### Requirement: Drawing statements
`PSET`, `PRESET`, `LINE` (with `B` and `BF`, a style, and without its first point), `CIRCLE` (with start, end and
aspect), `PAINT`, `DRAW`, `VIEW`, `WINDOW`, `PALETTE` and `PCOPY` SHALL be compiled in every form the old compiler
accepts, `STEP` included, and SHALL draw the pixels the old compiler's program draws. A coordinate or colour
SHALL be a number of any numeric type, converted as the old compiler converts it.

#### Scenario: A box and a point
- **WHEN** `tests\corpus\screen\m01_screen_1` is built and run (`LINE (2, 20)-(30, 30), 1, BF`, `PSET (40, 25), 3`)
- **THEN** every pixel row of its screen equals its recorded state

#### Scenario: Shapes
- **WHEN** `tests\corpus\screen\c02_screen12_shapes` is built and run
- **THEN** every pixel row of its screen equals its recorded state

#### Scenario: A program cut from Civil War Strategy
- **WHEN** `tests\corpus\screen\g01_flags` is built and run
- **THEN** every pixel row of its screen equals its recorded state

#### Scenario: Drawing in a text mode
- **WHEN** `PSET (1, 1)` runs in `SCREEN 0` under an error handler
- **THEN** the error number is the one the old compiler's program reports

#### Scenario: A rejected form
- **WHEN** a program has `LINE (1, 2), 3`
- **THEN** the compile fails with an error that is not "not supported yet"

### Requirement: Functions that read the screen and the keyboard
`INKEY$`, `CSRLIN`, `POS(n)`, `POINT` (in both forms) and `SCREEN(row, column[, colorflag])` SHALL be compiled
with the result type the old compiler gives each, and SHALL return what the old compiler's program returns.
`INKEY$` and `CSRLIN` SHALL be callable only without parentheses, as in the old compiler.

#### Scenario: Keys
- **WHEN** `tests\corpus\screen\c05_inkey_keys` and `c06_inkey_many` are built and run with their keys files
- **THEN** each leaves the screen of its recorded state

#### Scenario: The cursor's place
- **WHEN** a program runs `LOCATE 4, 7: r = CSRLIN: c = POS(0): PRINT r; c`
- **THEN** it writes ` 4  7 ` at row 4, column 7

#### Scenario: A pixel read back
- **WHEN** a program in `SCREEN 12` runs `PSET (5, 5), 9` and prints `POINT(5, 5)`
- **THEN** it prints ` 9 `

### Requirement: Pacing statements
`_LIMIT fps`, `_DELAY seconds`, `SLEEP [seconds]` and `_DISPLAY` SHALL be compiled with the call the old compiler
writes for each, and a program that uses them SHALL leave the screen the old compiler's program leaves.

#### Scenario: A loop paced by _LIMIT
- **WHEN** a screen program draws in a loop of ten rounds with `_LIMIT 60`
- **THEN** it ends and the state of its screen equals its recorded state

#### Scenario: SLEEP ended by a key
- **WHEN** a screen program runs `SLEEP` and its keys file gives one key
- **THEN** it goes on after the key

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

### Requirement: _BLINK and the function _DISPLAY
`_BLINK ON` and `_BLINK OFF` SHALL set, and the function `_BLINK` SHALL give, whether colours 16 to 31 of the
text screen blink, as libqb does, and the function `_DISPLAY` SHALL give the handle of the image on display, as
the old compiler's program gives it. The forms the old compiler rejects SHALL be its errors (measured,
`verification\v58`).

#### Scenario: Blinking set and read
- **WHEN** a program prints `_BLINK`, runs `_BLINK OFF`, prints `_BLINK`, runs `_BLINK ON` and prints `_BLINK`
- **THEN** it prints what the old compiler's program prints

#### Scenario: The screen with blinking off
- **WHEN** a screen program runs `_BLINK OFF: COLOR 20, 1: PRINT "x"`
- **THEN** its screen state equals the old compiler's

#### Scenario: The image on display
- **WHEN** a program prints `_DISPLAY = 0` in screen 0 and `_DISPLAY = _DEST` after `SCREEN _NEWIMAGE(64, 48, 32)`
- **THEN** it prints what the old compiler's program prints
