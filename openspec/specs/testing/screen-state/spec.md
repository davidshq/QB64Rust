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
F1 to F12), `wait <milliseconds>`, `prompt`, `state`, and as the last line `repeat <name>`, which presses the key
until the program exits. No key SHALL be lost, whatever the number of keys and the speed of the program: a key is
given only when the program has read the ones before. A program announces that it waits for a key by adding 1
(modulo 256) to byte 0x4F0 of conventional memory (`DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255`); the
k-th `prompt` SHALL wait for the k-th announcement before the steps after it are taken. If it does not come
within 30 seconds (or the milliseconds of `QB64_SCREEN_PROMPT_MS`), the program SHALL be ended with exit code 124,
after the state as it is then, and a message that names the line of the `prompt` SHALL be the last line of the
state file and be written to stderr; if a `prompt` finds that the program has already announced more waits than
that, it SHALL end the program the same way with exit code 125 and a message that says so (a check that catches a
key given without a `prompt` when it is seen, not a guarantee). `state` SHALL write the state of the
screen at that moment to the file named by `QB64_SCREEN_STATE`, after the states written before it; the state at
exit SHALL follow the last of them. A keys file without `prompt` and `state` SHALL give its keys as before. A keys
file that cannot be read, or a line that is no step, SHALL end the program with exit code 125 and a message on
stderr.

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

#### Scenario: A key a pause would eat
- **WHEN** a program announces a wait, reads one key, then runs a loop that ends early when `INKEY$` returns a key,
  then announces a second wait, and the keys file is `prompt`, `text a`, `prompt`, `text b`
- **THEN** the loop runs to its end and the second wait reads `b`

#### Scenario: A state while the program runs
- **WHEN** a program draws a box, announces a wait, clears the screen after the key, and ends with `SYSTEM`, and
  the keys file is `prompt`, `state`, `text x`
- **THEN** the state file holds two states: the first with the box, the second without it

#### Scenario: A menu that announces after every key
- **WHEN** a menu announces a wait before each key it reads, and the keys file is `prompt`, `key down`,
  `prompt`, `key down`, `prompt`, `state`, `key enter`
- **THEN** the state shows the third entry chosen, whatever the speed of the program

#### Scenario: A prompt that never comes
- **WHEN** the keys file starts with `prompt` and the program never writes byte 0x4F0
- **THEN** the program is ended after 30 seconds with exit code 124, the state at that moment is written, and the
  last line of the state file names the line of the `prompt`

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

### Requirement: Civil War Strategy in a scripted game
`tools\screen\cut_game.py` SHALL write `tests\corpus\screen\g12_game.bas` from `tests\programs\civil-war-strategy\
CWSTRAT.BAS` with the game's lines byte for byte, line n of the copy being line n of the game, except for a list
of edits kept in the script. Each edit names a line of the game, a text the line must hold exactly once, and what
that text becomes. The edits are:
- a fixed seed in place of `RANDOMIZE TIMER`, and silent sound (`PLAY "V0"`) after it;
- no `_FULLSCREEN`, and a `TICK` that returns at once;
- a fixed date where the game writes `DATE$`;
- before each place the game waits for a key, an announcement that names the place, and for its menu the first
  letter of the menu's title.

The script SHALL stop with a message when a line no longer holds its text. The program's data files SHALL be
copies in `g12_game.files\`, its configuration one that turns sound off. Its scripts SHALL together play the game
through every menu and option and to each of its endings. Taken together they SHALL meet the game's coverage check.
Each script SHALL end the game through the game's own menu or ending, SHALL be recorded with the old compiler,
SHALL be on `tests\corpus\slice.list`, and SHALL pass with the new compiler by its output and every state and
digest of its state file.

#### Scenario: The game changes
- **WHEN** the line `RANDOMIZE TIMER` of `CWSTRAT.BAS` is moved by an edit of the game
- **THEN** `cut_game.py` stops and names the line, and writes no `g12_game.bas`

#### Scenario: The scripted game passes
- **WHEN** the tier 2 slice run reaches `screen/g12_game`
- **THEN** the program built by the new compiler plays the game to its end, and every state it writes equals the
  recorded one

#### Scenario: A whole game passes
- **WHEN** the tier 2 slice run reaches a script that plays a game to the end of its time limit
- **THEN** the new compiler's build reaches the same ending screen, with every digest on the way equal to the
  recorded one

#### Scenario: A drawing that differs mid-game
- **WHEN** the new compiler draws one pixel of the map wrong
- **THEN** `screen/g12_game` fails at stage "screen", and the states written are kept in the results folder

#### Scenario: Two runs of the old compiler's build
- **WHEN** `--record` runs `g12_game` twice
- **THEN** both runs write the same states, so the seed, the pauses and the keys leave nothing to timing

#### Scenario: The game is covered
- **WHEN** the coverage check runs every script of `g12_game`
- **THEN** every line of `CWSTRAT.BAS` with a statement ran in full or is in the game's list of unreached lines
  with its reason

### Requirement: Mouse steps in the keys file
A keys file SHALL also give the program mouse events, in order with its keys: `mouse <x> <y>` (a move to pixel x,
y of the window), `press <button>` and `release <button>` (1 left, 2 right, 3 middle, at the last position) and
`wheel <n>` (n steps, negative up). Each SHALL reach the queue that `_MOUSEINPUT` reads as the window's event
would.

#### Scenario: A click
- **WHEN** the keys file is `prompt`, `mouse 40 30`, `press 1`, `release 1`, and the program reads every event
  with `_MOUSEINPUT` and prints `_MOUSEX`, `_MOUSEY` and `_MOUSEBUTTON(1)` for each
- **THEN** it prints four events at 40, 30: the move and libqb's copy of it with the movement cleared, both with
  the button up, the press with it down, the release with it up

#### Scenario: A mistake in a mouse step
- **WHEN** a line reads `press 9` or `mouse 10`
- **THEN** the program exits with code 125 and stderr names the file and the line

### Requirement: A screen program for each image, font and mouse built-in
Every built-in of `language/images`, `language/fonts` and `language/mouse` that a test runs SHALL be used, in
code that runs, by at least one screen program of `tests/corpus/slice.list`, recorded with the old compiler.

#### Scenario: A built-in no screen program runs
- **WHEN** `_SETALPHA` is compiled by the new compiler
- **THEN** a screen program recorded with `qb64pe.exe` calls `_SETALPHA` in code that runs and is on `slice.list`

### Requirement: The place of a wait
A program MAY say where it waits by writing a place number (1 to 255) to byte 0x4F1 of conventional memory, and a
letter to byte 0x4F2, before it announces the wait. The check SHALL read both when it sees the announcement. A
program that writes neither SHALL be treated as before, at place 0.

#### Scenario: A program without places
- **WHEN** `c12_prompts`, which announces without places, runs with its keys file
- **THEN** its states are the ones recorded before this change

### Requirement: Rules in a keys file
`rule <place>[:<letter>] <key>...` SHALL set the keys that answer a wait at that place (and letter). It replaces
an earlier rule for the same place. `play until <place>[:<letter>]` SHALL answer each announced wait by its rule,
giving the rule's keys one by one at the waits that follow. It SHALL stop at the first wait at the named place,
which then counts as a `prompt`. A wait with no rule, or none at the place within 30 s, SHALL end the run (exit
code 124 or 125) with a message naming the place.

#### Scenario: Months of computer turns
- **WHEN** a keys file has `rule 7 key space` for the game's "hit any key" wait and `play until 14:M` for the main
  menu, and a month brings three battles
- **THEN** each of the three waits is answered with a space, and the steps after `play until` start at the main
  menu

#### Scenario: A wait no rule answers
- **WHEN** during `play until 14:M` the game waits at place 9 and there is no rule for 9
- **THEN** the run ends with exit code 125, and the state file's last line names place 9 and the keys file's line

### Requirement: Digests of the screen
`digest` SHALL write one line `digest <hex>` to the state file, a hash of the state the step `state` would write
at that moment. `state` and `digest` steps SHALL keep their order in the file.

#### Scenario: A pixel differs mid-game
- **WHEN** the new compiler's build draws one pixel differently before the 40th `digest` of a script
- **THEN** the screen stage fails, and the first line that differs is that digest

#### Scenario: The same screen
- **WHEN** two runs draw the same screen at a `digest`
- **THEN** both write the same line

### Requirement: Scripts of a screen program
A screen program MAY have scripts: `<name>.<script>.keys`, each with its `<name>.<script>.screen` and
`<name>.<script>.output`. The runner SHALL run each script as a test named `<name>.<script>`, built from the one
`<name>.bas`, by the rules for a screen program with `<name>.keys`. Lists, `--glob` and `--record` SHALL take the
test name.

#### Scenario: One build, two scripts
- **WHEN** `g12_game` has the scripts `union_time` and `options`, and the slice list names both
- **THEN** the program is built once, and two tests are reported, each judged by its own files

#### Scenario: A script without its screen file
- **WHEN** `g12_game.options.keys` exists without `g12_game.options.screen`
- **THEN** that test fails at stage "expected", and the other tests of the program run
