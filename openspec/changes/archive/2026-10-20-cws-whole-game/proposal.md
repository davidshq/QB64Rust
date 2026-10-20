# Proposal: cws-whole-game

## Why

Civil War Strategy compiles and builds with the new compiler. `g12_game` plays two turns of it from a keys file,
and the screens match the old compiler's build. That is the opening of the game. Most of its 5,075 lines run
later: the battles the player fights, ships, the other side, two players, loading and saving, the options, and
the six ways a game ends. A difference in any of them would go unseen. The user asked for the whole game, not
just two turns, and chose line coverage as the measure (2026-10-19).

## What Changes

- **Coverage from the new compiler.** `qb64rust --coverage` builds a program that counts each statement it runs.
  At exit it writes the counts to the file `QB64RUST_COVERAGE` names. Without the option the C++ is unchanged.
- **A coverage report and check** (`tools\coverage\coverage.py`). It merges the runs of the game's scripts and
  sorts each line of `CWSTRAT.BAS` as run, partly run, not run, or holding no statement. The check fails when a
  line with a statement neither ran in full nor is listed in the game's list of unreached lines with a reason.
  Reasons include a line an edit of the copy removed, a file the game does not ship, and code no path reaches.
- **One copy of the game, several scripts.** The runner takes more than one keys file for a screen program
  (`<name>.<script>.keys`), each with its own recorded states. Each runs as its own test against the one
  `g12_game.bas`.
- **Keys files that can play a whole game.** `screen_state.cpp` learns three things:
  - each announcement of a wait carries the place the game waits at;
  - `rule` steps answer the waits at a given place with given keys, and `play until <place>` answers by the rules
    until the game waits there. A script can then end turn after turn without knowing how many "hit any key"
    screens each month brings.
  - a `digest` step writes a one-line hash of the screen instead of a full state, so a long game is compared at
    many points without megabytes of recordings.
- **More edits in the copy**, keeping line n of the copy as line n of the game:
  - a place number in each announcement;
  - `PLAY "V0"` at the start, so the sound lines can run without being heard;
  - a fixed date where the game writes `DATE$` into its history.
- **Scripts until the game is covered.** Full games to each ending, both sides, two players, every menu and
  option, files, ships, railroads, and commands. Each script is recorded with `qb64pe.exe` and passes with the new
  compiler, state for state and digest for digest. Every difference found is a bug of the new compiler and is
  fixed in this change.

## Capabilities

### New Capabilities

- `testing/coverage`: the coverage of a program's runs. What the counts file holds, how runs are merged and
  reported by line, and the check against a list of unreached lines with reasons.

### Modified Capabilities

- `compiler/cli`: the `--coverage` option and the `QB64RUST_COVERAGE` file.
- `testing/screen-state`: changes to two requirements, and one new one.
  - "Keys for a program with a window" gains places in announcements, `rule`, `play until` and `digest`.
  - "Civil War Strategy in a scripted game" grows from one two-turn script to the scripts that cover the game.
  - New: a screen program's several scripts.

## Impact

- `crates\driver` (the option, its build), `crates\codegen-cpp` (the counter at each statement, the table of
  statements, the writer at exit).
- `tools\screen\screen_state.cpp`, `tools\screen\cut_game.py`, `tools\screen\README.md`.
- `tools\legacy_tests\run_legacy_tests.py` and its tests (scripts of a program; a coverage run).
- New `tools\coverage\` with its tests.
- `tests\corpus\screen\g12_game.*` (new script, screen and output files), `tests\corpus\slice.list`, and the
  game's list of unreached lines.
- Disk: the recorded states of the scripts. Digests keep this to a few full states per script. The two-turn
  script's 2.5 MB stays.
- Machine time: each script is a full game run twice to record and once per compare. The scripts run locally,
  like every screen program; CI runs no screen program.
