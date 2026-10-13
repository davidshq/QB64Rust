# testing/screen-state Specification

## Purpose
Checks a program that has a window by the state of the screen it leaves at exit: text cells or pixels, palette,
cursor and pages, written by one source file linked into the test program of either compiler, which also gives
the program its keys. The corpus runner records that state with the old compiler and compares it.

## Requirements

### Requirement: The state of the screen at exit
`tools\screen\screen_state.cpp`, linked into a program of either compiler as an extra source file, SHALL write the
state of the program's screen to the file named by the environment variable `QB64_SCREEN_STATE` when the program
exits, however it exits (`SYSTEM`, `END`, running off the end, the window closed, a runtime error). The file SHALL
hold the number of screen pages and which page is displayed, written to and read from; for each valid page its
mode, kind (text or graphics), size, bits per pixel, font, text cursor, colour and background, its palette unless
it has 32 bits per pixel, and every text cell with its attribute or every pixel; and the same for a destination
or source image that is no screen page. Two runs of a program that draws the same SHALL give the same bytes.
Without the variable the file SHALL change nothing about the program. libqb and the reference clone SHALL NOT be
changed for it.

#### Scenario: A text screen
- **WHEN** a program prints `Hello` in yellow on blue at the top left, ends with `SYSTEM`, and runs with
  `QB64_SCREEN_STATE` set
- **THEN** the file's first page is `mode 0 text 80x25`, its first row holds `Hello` and its attributes are
  `1E*5 07*75`

#### Scenario: A drawing
- **WHEN** a `SCREEN 12` program draws a filled box in colour 4 from (10, 10) to (100, 50) and ends
- **THEN** rows 10 to 50 of page 0 read `00*10 04*91 00*539`, written once and followed by `= 40`

#### Scenario: Pages
- **WHEN** a program ends after `SCREEN 7, , 0, 1` with three pages in use
- **THEN** the second line reads `pages 3 display 1 write 0 read 0` and three page records follow

#### Scenario: A runtime error
- **WHEN** a program stops with an unhandled error and `QB64PE_NOPROMPT=y`
- **THEN** the state as it was at the error is written

#### Scenario: Not asked for
- **WHEN** a program built with the file runs without `QB64_SCREEN_STATE` and `QB64_SCREEN_KEYS`
- **THEN** it runs as a program built without the file, and no file is written

### Requirement: Keys for a program with a window
With `QB64_SCREEN_KEYS=<file>` the same file SHALL give the program the keys the file lists, in order, as if
typed: `text <printable ASCII>`, `key <name>` (Enter, Esc, Tab, Backspace, Space, the arrow and editing keys,
F1 to F12), `wait <milliseconds>`, and as the last line `repeat <name>`, which presses the key until the program
exits. No key SHALL be lost, whatever the number of keys and the speed of the program: a key is given only when
the program has read the ones before. A keys file that cannot be read, or a line that is no step, SHALL end the
program with exit code 125 and a message on stderr.

#### Scenario: More keys than the key buffer holds
- **WHEN** the keys file holds 600 characters and then Enter, and the program reads `INKEY$` until Enter
- **THEN** the program receives all 601 keys in order

#### Scenario: Special keys between characters
- **WHEN** the keys are `a`, Up, `Z`, F1, Esc, `q`, Enter
- **THEN** `INKEY$` returns `a`, `CHR$(0) + "H"`, `Z`, `CHR$(0) + ";"`, `CHR$(27)`, `q`, `CHR$(13)`

#### Scenario: END's prompt
- **WHEN** a program ends with `END` and the keys file is `repeat space`
- **THEN** the program exits

#### Scenario: A mistake in the keys file
- **WHEN** a line reads `key bogus`
- **THEN** the program exits with code 125 and stderr names the file and the line

### Requirement: Screen programs in the corpus
A corpus program with a `<name>.screen` file SHALL be built with the state file and run with `QB64_SCREEN_STATE`
set, and with `QB64_SCREEN_KEYS` naming a copy of `<name>.keys` when that file exists. Its test SHALL pass only if
the state written equals `<name>.screen` (CRLF as LF), in addition to every other rule for a corpus program. A run
that writes no state SHALL fail. `--record` SHALL write `<name>.screen` only if both runs wrote a state and the two
are equal. A `<name>.screen` beside an `.err` or `.norun` file, and a `<name>.keys` without a `<name>.screen`,
SHALL be reported as an error in the test.

#### Scenario: Equal state
- **WHEN** a screen program's state equals its `.screen` and its output equals its `.output`
- **THEN** the test passes

#### Scenario: A different pixel
- **WHEN** the state differs from `.screen` in one row
- **THEN** the test fails at stage "screen", and the state written is kept in the results folder

#### Scenario: No state
- **WHEN** the program runs and exits with code 0 but writes no state file
- **THEN** the test fails at stage "screen"

#### Scenario: Keys without a screen file
- **WHEN** a program has a `<name>.keys` and no `<name>.screen`
- **THEN** the test fails at stage "expected" and nothing is compiled

#### Scenario: Recording
- **WHEN** `--record` runs a program whose `.screen` is empty, and both runs write the same state
- **THEN** `.screen` holds that state

#### Scenario: Runs differ
- **WHEN** `--record` runs a program whose two states differ
- **THEN** nothing is written and the program is reported as non-deterministic

### Requirement: Screen programs of the new compiler
`tests/corpus/slice.list` SHALL name every screen program the new compiler passes, and the tier 2 slice run SHALL
check each by its screen state as well as by its output. A screen program that the new compiler compiles without
an error and that is not on the list SHALL be named in `tests/known_clean_not_passing.list` with the reason, as
any other clean program (`testing/compiler-tests`, "Clean programs are listed").

#### Scenario: A listed screen program
- **WHEN** the tier 2 slice run reaches `screen/m01_screen_1`
- **THEN** it builds the program with the state file, runs it, and passes only if the state written equals
  `m01_screen_1.screen`

#### Scenario: A wrong pixel from the new compiler
- **WHEN** the new compiler's program draws one pixel the old compiler's program does not draw
- **THEN** the tier 2 slice run fails for that program at stage "screen"

#### Scenario: A clean screen program that is not listed
- **WHEN** a program of `tests/corpus/screen` compiles without an error and is on neither list
- **THEN** `cargo test` fails and names it

### Requirement: The state does not depend on the clock
The state file SHALL hold nothing the runtime derives from the time of day or from how long the program ran. In
`SCREEN 10` the runtime sets the first four palette entries of the displayed page at every refresh from the
mode's own palette (entries 4 to 7) and the clock; the state file SHALL write those four entries as 0.

#### Scenario: SCREEN 10 under load
- **WHEN** `tests\corpus\screen\m10_screen_10` is run several times at once with other screen programs
- **THEN** every run writes the same state, whether or not its screen was refreshed before it exited

### Requirement: A screen program for each screen built-in
Every statement and function of `language/screen-output` SHALL be used, in code that runs, by at least one screen
program of `tests/corpus/slice.list`, recorded with the old compiler.

#### Scenario: A statement the first 29 programs do not use
- **WHEN** `WINDOW` is compiled by the new compiler
- **THEN** a screen program recorded with `qb64pe.exe` draws through a `WINDOW` and is on `slice.list`
