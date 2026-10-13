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
