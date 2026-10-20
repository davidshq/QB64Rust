# Spec Delta

## ADDED Requirements

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

## MODIFIED Requirements

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
