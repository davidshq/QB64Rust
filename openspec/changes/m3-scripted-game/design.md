# Design

## Context

See `proposal.md` for why and what was measured. What shapes the approach:

- The screen-state check (`tools\screen\screen_state.cpp`, `tools\screen\README.md`) is one source file linked
  into a test program of either compiler. It writes the screen's state at exit and gives keys from a file, one
  key whenever `INKEY$`'s buffer is empty. libqb and the reference clone are not changed for it, and that stays.
- The corpus runner (`tools\legacy_tests\run_legacy_tests.py`) runs a screen program with its `.keys`, copies
  `<name>.files\` into a fresh run folder for each run, compares the state file with `<name>.screen` as text, and
  `--record` writes `.screen` only when two runs agree.
- `tools\screen\cut_game.py` already writes programs cut from the game, copying its lines byte for byte and
  stopping when a line it names no longer starts as said.
- The game is kept byte for byte in `tests\programs` (`tests\programs\README.md`). What a test runs is a copy.

## Goals / Non-Goals

**Goals:**

- A game played from a keys file, its screens compared with the old compiler's at chosen points, with nothing
  left to timing: two runs of either build give the same bytes.
- As much of the game's own code run as a script of reasonable length reaches: title, menus, the map, recruiting,
  moves, ships, the computer's turn, battles, events, reports, saving, quitting.
- The new compiler passes it; a difference found is fixed here.

**Non-Goals:**

- Sound (the copy runs with `noise` 0; `s74`–`s76` cover sound), the window, full screen.
- A general recorder of games, a replay from a real player's keys, or keys with Shift/Ctrl/Alt.
- Changing `tests\programs\civil-war-strategy`.

## Decisions

### D1. A copy of the game with listed edits, not the game as it is

Scripting the game as it is would need the clock and the key timing controlled from outside: `RANDOMIZE TIMER`
can only be fixed by setting `rnd_seed` from another thread after the statement ran (a race), and `TICK` eats a
key that arrives while it waits. A copy with four kinds of edit removes both, and the edits are few and visible:

| Edit | Lines of `CWSTRAT.BAS` | Why |
|---|---|---|
| `RANDOMIZE TIMER` → `RANDOMIZE 1861` | 218 | A fixed seed; the `RND` before it (line 165) already has libqb's fixed start |
| `_FULLSCREEN _SQUAREPIXELS, _SMOOTH` taken out | 168 | A test must not take the whole screen of the machine it runs on; the state holds no window |
| The body of `SUB TICK` taken out | 4037–4038 | Pauses only wait; none draws. Without them no key is eaten and the run is short |
| An announcement before each wait | the 14 places of the proposal | So the key thread knows when the screen is complete and a key is wanted (D2) |

`cut_game.py` gets a second kind of entry: the whole game with a list of `(line, start, replacement lines)`
edits, each line checked to start as given, as its ranges are now. The announcement is the text
`DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG` (the game's `DEF SEG` with a picture loaded is
always followed by `DEF SEG` before any wait, so resetting it is safe). For `DO WHILE INKEY$ = "": LOOP` it is put
before that text on the same line, so a one-line `IF` keeps its shape; for the menu's `crsr:` line, which a
`GOTO crsr` repeats, the label line announces and the read gets a label of its own (`crsr0:`), so a key is
announced once per read and not once per empty read.

*Considered:* the game as it is, with `screen_state.cpp` patching `rnd_seed` and dropping keys during `TICK` (a
race in both, and `TICK` cannot be told apart from a menu); a `DECLARE LIBRARY` call into C for the announcement
(the new compiler does not compile `DECLARE LIBRARY`).

### D2. `prompt` and `state` in the keys file

`screen_state.cpp`'s key thread gets two steps:

- `prompt`: the byte **counts** announcements (the program adds 1, modulo 256), and the k-th `prompt` waits
  until it has counted k from its value at start (1 ms polls). The program writes it just before its read loop,
  so when the count is reached, everything the program draws before reading the key is drawn, and it draws
  nothing more until a key comes. After 30 s (`QB64_SCREEN_PROMPT_MS` to change it) the thread writes the state,
  then a line `stopped <file> line <n>: no prompt in time` at the end of the state file and the same on stderr
  (which libqb points at the parent's console, so a pipe does not see it; task 2.3), and ends the program with
  exit code 124. A count
  *past* k (the program announced a wait the keys file has no `prompt` for, because it was given a key without
  one) ends it with code 125 and "the program announced more waits than the keys file has prompts".
  *Changed while built (task 2.1):* "wait until the byte differs from the last value seen" would return too
  early: a menu announces again after every arrow key, so by the next `prompt` the byte has already moved.
- `state`: write the state as at exit, appended to `QB64_SCREEN_STATE`. The first write of a run opens the file
  for writing, every later one appends; the state at exit is the last block. The format of a block is unchanged
  (`screen-state 1` … `end`), so `state_to_png.py` and the runner need only know there may be several.

Reading the pages from the key thread while the main thread spins in `INKEY$` is safe: the main thread writes no
page then. libqb's display thread reads pages and never writes them.

Keys after a `prompt` are given as before (when the buffer is empty), so `prompt`, `text Grant`, `key enter` types
a name into the game's `INPUT`. A key without a `prompt` before it may be read by one of the game's look-only reads
(`IF INKEY$ <> ""`); the game's keys file therefore puts a `prompt` before every key it gives, and `README.md` says
so.

### D3. The game's configuration and files

`g12_game.files\` holds copies of every data file the game reads (`*.DAT`, `*.GRD`, `*.INI`, `*.VGA`,
`HISCORE.CWS`, `CWS.TXT`), and a `CWS.CFG` written by `cut_game.py`: the game's own values with `noise` 0 (no
sound, and no title music, which would wait for its notes) and `history` 0 (no `SHELL "copy …"`, no `DATE$` in a
file). The runner gives every run a fresh copy, so saves and the high-score file written by one run do not reach
the next. Choosing the side, difficulty and the rest happens through the game's own menus in the keys file, so
that their screens are compared too.

### D4. What the keys file plays, and where it takes states

Written by playing the old compiler's build by hand with the keys file growing step by step and each state looked
at with `state_to_png.py`. The script: the title and main menu; a new game as the Union; the map; one menu of each
kind (Troops: recruit and detach; Moves: one army into an enemy city so that a battle is fought; Ships; Railroad;
Inform: the reports; Utility; Files: save); END TURN, so that the computer moves and the month's events run;
enough turns that at least one battle and one naval action happen; quitting through the main menu (`CLS : END`,
then `repeat space`). A `state` at each new kind of screen, not at every key: about 15 to 25 states. Measured (task
1.1): one `SCREEN 12` state is 95 to 121 kB, so 25 states are about 3 MB. Which procedures of the game ran is checked by a list, not by tooling: `tasks.md` records the procedures
the script reaches and those it does not.

### D5. Run time

The runner ends a run after 60 s. With `TICK` empty and no sound the game's own work is drawing: a short game
(one END TURN with a battle) took 1.1 to 1.4 s (task 1.1), so a script of ten turns stays far below the limit. If
one run takes longer after all, the runner takes the time limit of a program from a file `<name>.timeout` beside
it, as it takes `.keys` and `.files\`, with a runner test; otherwise the runner is not changed.

### D6. Small programs for the two steps

A screen program in `tests\corpus\screen\`, `c12_prompts` (named `s78_prompts` in `slice\` at first; screen
programs live in `screen\`, and its `c` series is the check's own), recorded with `qb64pe.exe` and on
`slice.list`, holds
the scenarios "A key a pause would eat" and "A state while the program runs". "A prompt that never comes" would
take 30 s in the corpus, so it is a test in the runner's own suite instead: it builds a small program once (it needs the clone, so it is skipped without it) and runs it with the environment variable
`QB64_SCREEN_PROMPT_MS=500`, which shortens the limit and which nothing else sets.

## Risks / Trade-offs

- **An edit hides a fault of the new compiler in an edited line.** The edits touch `RANDOMIZE`, `_FULLSCREEN`,
  `TICK` and the waits; each is compiled and run in slice programs already (`TIMER`, `RANDOMIZE`, `INKEY$`,
  `_FULLSCREEN` as code not reached) and `CWSTRAT.BAS` itself stays under tier 1 and the build test.
- **A wait the list missed** makes the script stop there: the `prompt` times out with the line named, the state
  then is written, and the run fails rather than hanging.
- **The script follows the game's random numbers.** A change of the seed or of `RND`'s use reshuffles battles,
  and the keys file may then choose a menu entry that is not there. The seed is part of the copy, so this happens
  only when the game is edited; `cut_game.py` then stops first (D1).
- **Size of the recording.** One `SCREEN 12` state of a busy map may be hundreds of kB; D4 caps the number.
- **A difference only the game shows** may be a large fix. A fault that takes more than a contained fix is put to
  the user with what was found, as earlier changes did.

## Migration Plan

Nothing to migrate: the new steps are optional, and every existing keys file behaves as before (the 52 screen
programs are rerun at the close).

## Open Questions

**Answered 2026-10-14 (task 1.2, `DECISIONS.md`): all three as the default below.**

For the user, each with the default this design is written for:

1. **No sound in the scripted game** (`noise` 0). Sound at volume 0 would need an edit that writes `PLAY "MB V0"`
   and would add the notes' time to the run. Default: no sound.
2. **The size of the recording**: about 15 to 25 states, a few MB in the repo. Default: as D4.
3. **The copy runs the game's code with four kinds of edit** (D1), and `DECISIONS.md` gets a row that a test runs
   this copy, while the game itself is still never run by a test.
