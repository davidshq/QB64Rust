# Proposal

## Why

Civil War Strategy is M3's real-program graphics target (`DECISIONS.md` 2026-10-09). Since `m3-sound` it compiles
with no diagnostic and builds, and its C++ has been compared with the old compiler's call by call, but **no test
compares what it shows when it is played**: the eleven programs cut from it (`tests\corpus\screen\g01`–`g11`)
each run one procedure over written values, and the game itself was started once by hand. Stage 3 of
`tests\programs\README.md` is the step that closes this gap: a scripted game with a fixed seed whose screens are
compared with the old compiler's. The user chose it as the next step of M3 (2026-10-14).

Measured before this proposal (reading `CWSTRAT.BAS` and libqb at the reference clone's commit):

- **The game cannot be scripted as it stands.** It seeds with `RANDOMIZE TIMER` (line 218); it switches to full
  screen (line 168); and its pause `SUB TICK` (lines 4036–4039, 68 calls) waits up to a time *or until a key*,
  which it eats. The screen-state check gives the next key as soon as the key buffer is empty, so whether a key
  reaches a menu or is eaten by a `TICK` depends on timing.
- **The screen-state check writes one state, at exit.** The game's only way out through its menus is `CLS : END`,
  so the state at exit is an empty screen. Comparing a game needs states taken while it is played.
- **It waits for a key at 14 places**: the menu's `crsr` loop (every menu), twelve `DO WHILE INKEY$ = "": LOOP`
  lines, and one `INPUT` (the high-score name). The other `INKEY$` reads (the title music, one animation) only look
  for a key and go on.
- **A program can tell the test harness that it waits without a call into C.** `DEF SEG = 0: POKE 1264, n` writes
  `cmem[0x4F0]`, which libqb never reads or writes (`DEF SEG` alone points at 0x500). `screen_state.cpp` can watch
  that byte from its key thread.
- **The runner compares the whole state file as text**, so states written one after the other into one file are
  compared by the runner with no change. Its run timeout is 60 s (`CORPUS_RUN_TIMEOUT`).
- **Sound is configured by the game's data file**: `CWS.CFG` sets `noise` (0 none, 1 effects, 2 music too) and
  `turbo!` (the pause). Tests silence sound with `PLAY "MB V0"`.

## What Changes

- **A copy of the game for scripted play**, `tests\corpus\screen\g12_game.bas`, written by
  `tools\screen\cut_game.py` from `CWSTRAT.BAS` with a short list of line edits, each checked against the game's
  text: a fixed seed in place of `RANDOMIZE TIMER`; no `_FULLSCREEN`; a `TICK` that returns at once; and, before
  each of the 14 waits, a line that announces it (`DEF SEG = 0: POKE 1264, …: DEF SEG`). Every other line is the
  game's, byte for byte. Its data files are copied into `g12_game.files\`, with a `CWS.CFG` of its own (no sound).
  The game in `tests\programs` stays untouched.
- **`screen_state.cpp` learns two steps of the keys file**: `prompt` waits until the program announces its next
  wait (or fails after a time limit), and `state` writes the screen's state at that moment, appended to the state
  file before the state at exit. A keys file without them behaves as before.
- **A keys file that plays the game**: a new game, its menus, recruiting, moving, ships, ending turns so that the
  computer moves and battles are fought, the reports, saving, and quitting; with a `state` at the screens worth
  comparing. Recorded with `qb64pe.exe` (`--record` runs it twice and refuses states that differ).
- **The new compiler passes it**, and `g12_game` joins `tests\corpus\slice.list`. A difference it finds in the
  compiler is fixed in this change.
- **`state_to_png.py`** shows any one of several states in a file.
- **Small screen programs** show the two new steps in isolation (a slice program with two prompts and a state
  between them; a prompt that never comes).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `testing/screen-state`: the keys file gains `prompt` and `state` (states taken while a program runs, appended
  before the state at exit); a new requirement for the scripted game (a copy of a real program with listed edits,
  its states compared).

## Impact

- Changed: `tools\screen\screen_state.cpp`, `tools\screen\cut_game.py`, `tools\screen\state_to_png.py`,
  `tools\screen\README.md`; possibly `tools\legacy_tests\run_legacy_tests.py` (a longer run timeout for one
  program, only if the game's run needs it, with a runner test); `tests\corpus\slice.list`; new
  `tests\corpus\screen\g12_game.*` (with `.files\`), new slice programs for the two steps; compiler crates only
  where the game shows a difference.
- No change to libqb or the reference clone. The game in `tests\programs` is not edited.
- Not in this change: what is heard (the copy runs without sound; sound is covered by `s74`–`s76`); the window
  (title, size, full screen), which the state does not hold; a second scripted game (the alternative map, a
  game as the Confederacy) unless the first leaves a part of the game's code unreached that one more script
  reaches cheaply; the mouse, fonts and image built-ins (the game uses none).
- `STATUS.md`, `ROADMAP.md`, `tests\programs\README.md` (stage 3 done), `tests\corpus\README.md`, `study\00` §5
  (the measured facts), `DECISIONS.md` (this change replaces "no test runs the game" with "a test runs a copy with
  listed edits"), and `CLAUDE.md`'s layout table updated when done.
