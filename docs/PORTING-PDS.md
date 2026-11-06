# Porting a BASIC PDS 7.1 program to QB64Rust

This is a short guide for moving an old Microsoft BASIC PDS 7.1 program onto QB64Rust. It comes from the port of
W.R. Hutsell's *World War 2 in Western Europe* (`Hutsell-Games/world-war-2`, branch `qb64rust`, 2026-11-04). That
game is about 1,000 lines of main module plus three linked modules, a mouse library and DOS helper programs.

QB64Rust follows QB64pe, and QB64pe follows QuickBASIC 4.5. Most PDS-only features are missing from all three. So
the aim of a port is a program that builds with both QB64Rust and QB64pe, changed as little as possible.

## 0. Principles of a port

These are the rules the World War 2 port followed, most of them set by the user while it was done (2026-11-04 and
2026-11-05). The next two ports followed them: W.R. Hutsell's *Civil War Battleset* and *Civil War Strategy*.
Each game's `docs\port-study.md` has its plan.

**What a port is for.** A port has two goals: to try QB64Rust on a real program, and to bring the program to where
it can be refactored safely (user, 2026-11-04). It is not a refactoring itself.

**The original stays as it came.**
- Make a branch named `qb64rust` from `main`, which holds the original source. Do not start from an earlier
  branch that moved files or changed the code (user: "start with main, make a branch from that").
- Change as little as possible. Each change gets a numbered item in the port's `docs\devnotes.md`: what changed
  and why. No reformatting, no renaming of variables, no removal of a feature of the game.
- A change that is not needed to build, to run or to test is not made. The user asks "did you make any
  unnecessary edits?", so every edit must have its reason ready.
- A behavior of the game that may be a bug of the original stays as it is. It becomes a row of the port's
  `TODO.md` for the user to decide. Fixing it changes the recorded screens.

**Build with both compilers.** Build every program with QB64Rust and with QB64pe, and compare their screens. If
both show the same, a difference from what is expected is in the port, not in a compiler.

**Programs and files.**
- Keep the names by which the programs start each other. The QB64 builds take the names of the DOS programs.
  Remove the DOS programs of those names from the branch; they stay on `main`. Do not rename the builds to keep
  the DOS ones (user, 2026-11-05).
- Builds are not committed: `*.exe` goes in `.gitignore`, and so do the program's `stdout.txt` and `stderr.txt`.
- A GitHub Actions workflow builds the programs with QB64Rust on each merge into `main`, and when started by
  hand. Its artifact is a folder that can be unzipped and played (user, 2026-11-05).
- Remove what the port does not use: DOS sound drivers and players, DOS launchers (`.BAT`, `.PIF`), PDS build
  files (`.MAK`, `.LNK`, `.OBJ`, `.BTM`), demos, advertising and archives. They stay on `main`.
- A DOS program with no source that shows a screen (a title screen, for example) cannot run on 64-bit Windows.
  Run it in DOSBox, keep a screenshot in `docs\`, and note what is lost (user, 2026-11-05).
- Before removing a data file, check whether the game needs it to exist. For example, a Files menu may offer
  Load only when a saved game exists.
- Saved games and high scores that came with the game go to `tests\data\`, where every test run starts from them.
  The copies that playing leaves beside the program are ignored by git (user, 2026-11-05).
- Data files go into a few folders by kind, such as `graphics\`, `sound\` and `scenarios\`. Each folder is a
  `CONST` in one include file (`FOLDERS.BI` in World War 2). There is no `src\` or `help\` folder. `docs\` holds
  the developer notes and the game's own documents (user, 2026-11-05).

**DOS facilities.** Replace what only DOS had with QB64's own statements: the mouse driver with `_MOUSE…`, the
sound players with `_SNDOPEN` and `_SNDPLAY`. Keep the game's own switches: if a command-line option turned sound
on in DOS, it still does.

**Tests before changes.** Unit tests cover the SUBs that draw nothing. Whole-game scripts play every scenario
from the title menu to the end and compare the screens (section 4). Coverage shows what neither reaches. The hooks
for the scripts do nothing unless an environment variable is set, so the game plays as before.

**What the port finds goes back to QB64Rust.** A message that misleads, a form that builds silently and does the
wrong thing, or a difference from the old compiler becomes a verification program, a `DECISIONS.md` or
`SOMEDAY.md` row, or a line in this guide. When the old compiler's behavior is in doubt, measure it with
`tools\qb45\qb45.py` (QB 4.5, or PDS 7.1 with `--compiler pds71`), not from memory.

**A clean build proves little.** Search the source for every PDS-only name (section 3). Look for every `GOSUB`
routine that can leave its SUB before its `RETURN` (section 3), by reading each routine: a scan of the first exit
misses some.

## 1. Make one program out of the modules

PDS links several `.BAS` modules into one executable. QB64 builds one source file.

- Rename each extra module to `.BM` with `git mv` (so its history follows) and `$INCLUDE` it at the end of the
  main module.
- Remove each module's header: its `DEFINT`, its `DECLARE` lines and its include of the shared `.BI`. The main
  module already has them.
- Move the main module's SUBs and FUNCTIONs to a `.BM` of their own, unchanged. Move its `DIM SHARED` lines into
  the shared `.BI`. A test file can then include the `.BI` and the `.BM` files and get every SUB without the
  main module's code. This is what makes unit tests possible (section 4).

## 2. Replace what PDS has and QB64 does not

What the World War 2 game needed, and the edit each took:

| PDS form | Edit | Count in the game |
|---|---|---|
| `DIR$(spec$)`, and `DIR$` with no argument for the next match | Write a `FUNCTION DIR$ (spec$)` with QB64's `_FILES$`. A bare `t$ = DIR$` becomes `t$ = DIR$("")`, because a QB64 FUNCTION cannot leave out its argument | 16 calls, 1 changed |
| `CURDIR$`, the current folder (PDS: `C:\GAMES`, with no separator at its end unless it is a root) | Write a `FUNCTION CURDIR$` that returns QB64's `_CWD$` without the separator `_CWD$` ends with. Both compilers build a bare `CURDIR$` with no message, as a string variable that is always `""` (section 3) | 2 calls, 0 changed; missed by the first port |
| `COMMON SHARED /name/ x` (a named `COMMON` block) | `DIM SHARED x`. Both compilers reject named blocks | 1 (`MOUSE.BI`) |
| `ON LOCAL ERROR GOTO label` in a SUB | Remove it, or write `ON ERROR GOTO` with the handler moved to the main module, and `ON ERROR GOTO 0` before each way out of the SUB. QB64 wants an error handler's label in the main module. Without the `ON ERROR GOTO 0` the handler stays active after the SUB returns, where `ON LOCAL ERROR` ends with its SUB (`verification\v17_d_on_error_sub_stays`). QB64pe says "Name already in use (ERROR)". QB64Rust's error says what to write instead | 1 |
| `CALL INTERRUPT` / `Interrupt 51` for the DOS mouse driver | Rewrite the mouse routine with `_MOUSEINPUT`, `_MOUSEX`, `_MOUSEY`, `_MOUSEBUTTON`, `_MOUSESHOW` and `_MOUSEHIDE`. The game used driver functions 0, 1, 2, 3, 7 and 8. A border set by functions 7 and 8 no longer holds the real cursor, so clamp the position the program reads instead | 1 SUB |
| `SHELL` of a DOS program with no source (title screen, sound players) | Comment it out and note what is lost. 64-bit Windows cannot run DOS executables | 4 programs |

Other PDS features that a port may meet, with the usual edit, are listed in `SOMEDAY.md` under "Popular BASIC
PDS 7.1 features": `REDIM PRESERVE` (becomes `REDIM _PRESERVE`), `CHDRIVE`, the `CURRENCY` type, arrays inside a
`TYPE`, `BYVAL` parameters and the add-on libraries such as `FORMAT$`.

## 3. Things that are easy to miss

- **A PDS function with no arguments can build without a message.** QB64 has no `CURDIR$`. QB64pe and
  QB64Rust both take a bare `CURDIR$` for a string variable, which is always `""` (measured 2026-11-05). The
  World War 2 port built and passed its scripts with it, and a review found it later: the PBM Drive menu
  showed a blank entry. A clean build does not show that every PDS name was replaced. Search the source for
  each PDS-only name in `SOMEDAY.md`'s list.
- **Data files are found beside the executable.** Build the program into the folder that holds its data
  files, or it starts and finds nothing. Do not overwrite the original DOS `.EXE` if the folder has one.
- **Include names and file names differ in case.** `'$INCLUDE: 'ww2.bi'` works on Windows with `WW2.BI` but
  fails on Linux. Make the names match before building on a case-sensitive file system.
- **Leave other main modules alone, or port them too.** The shareware main module (`WW2-S.BAS`) still expects
  the old modules and no longer builds.
- **A SUB that leaves its own `GOSUB` by `GOTO` breaks the next `RETURN` outside it.** QB64pe and QB64Rust
  keep one `GOSUB` stack for the whole program, so the SUB's entry stays on it after `END SUB`. The caller's
  next `RETURN` takes that entry and stops with "RETURN without GOSUB". QB 4.5 and PDS 7.1 drop the entry
  when the SUB ends, so the old program worked. The game's editor (`EDIT.BAS`) had this in its `menu` SUB: picking an item
  by its letter left `GOSUB selex` by `GOTO called`, and "Leave Edit" then stopped the program. The fix: let
  the routine set a flag and `RETURN`, and make the jump on the line after the `GOSUB`.
- **Build with QB64pe as well.** If both compilers build it and show the same screens, a difference is a bug in
  one of them, not in the port. In the World War 2 port the two builds matched pixel for pixel.
- **A shape that `PAINT` fills can leak, with no message.** QB64 draws some shapes with other pixels than
  QB 4.5 and PDS 7.1 did: a `DRAW` string at scale 6, and `CIRCLE` with an aspect or with start
  and end angles (`DIVERGENCES-QB45.md` Q-019 and Q-020, both measured). An outline that was closed in DOS
  can be open by one pixel in QB64, and `PAINT` inside it then covers the screen. Civil War Strategy had three
  such leaks: a small train, a cannon's wheel and a piece of coast each painted the whole map. Both compilers
  build the program and show the same wrong screen, so comparing the two builds does not find it. Look at a
  screen of every kind the program draws. The fix is in the program: one length of the `DRAW` string or one
  radius changed, checked by looking at the screen again.
- **A script written for another version of the program can be out of step and still pass.** Civil War
  Strategy's port took ten scripts from QB64Rust's corpus, which plays a version of the game that never
  loads a saved game. One script relied on that. In the port it ran to its end with the same screens in both
  builds, while some keys went to other menus than its comments said. Read a reused script against the code,
  and look at its screens, before recording it.

## 4. Test the port before changing anything else

An old program has no tests, and a refactoring without them is a guess. Two kinds helped:

- **Unit tests** with `qb64rust test` (`MANUAL.md`, "Testing your code"). They call the SUBs that only compute
  (distances, movement rules, sorting) and check what they return. The game started with 20.
- **Whole-game scripts** with QB64Rust's screen-state check (`tools\screen\README.md`). A keys file plays the
  game from the title menu to the end, and the screen is compared with one recorded earlier. 37 scripts ran
  82% of the game's lines (`qb64rust --coverage`).

For the scripts to give the same game every run, the program needs a few small hooks, active only when an
environment variable is set:

- a fixed `RANDOMIZE` seed in place of `RANDOMIZE TIMER`;
- a call before each wait for a key that names the kind of wait, so the keys file sends each key when the game
  reads it;
- timed pauses that return at once, so a game takes seconds and does not depend on the machine's speed;
- a fixed date where the program saves `DATE$` into a file the screens show.

Two lessons from writing scripts: look at every screen before recording, because a script out of step can still
reach the end and exit with 0; and take no screen digest while an `INPUT` waits, because its cursor blinks by
the clock.

A game with many scenarios gets one generated script for each scenario and side. A game with one scenario
gets generated games that differ by seed, side and settings. Civil War Strategy has 14: the player only ends
each turn, the computer plays to its victory, and the script checks the history file the game wrote.

## 5. What the port found in QB64Rust

- `ON LOCAL ERROR GOTO` got "not supported yet", which suggested QB64pe has it. It does not. Since 2026-11-05
  it is a plain error that says to use `ON ERROR GOTO` with the handler in the main module, and
  `ON ERROR GOTO 0` before the SUB ends (`verification\v17_d_err_on_local_error_sub` and `_main`).
- `Interrupt 51, regs, regs` is rejected as a statement, where QB64pe accepts it.
- A named `COMMON` block is rejected by both compilers. QB64Rust's message is the clearer one.
- The single `GOSUB` stack (section 3) differs from QB 4.5 and PDS 7.1 (`DIVERGENCES-QB45.md` Q-018). Both were
  measured on 2026-11-05 with `verification\qb45\q45_gosub_left_in_sub`. QB 4.5 ran through `tools\qb45\qb45.py`,
  and PDS 7.1 through `qb45.py --compiler pds71`. PDS 7.1 was also run by hand in QBX and built without `/D`.
  All of these runs return to the main module. QB64Rust keeps the single stack as QB64pe has it (`study\00` §6,
  "Kept"). `verification\v17_d_gosub_left_in_sub` holds QB64Rust to QB64pe's error 3 there.

- `DRAW` at scale 6 and `CIRCLE` with an aspect and angles give other pixels than QB 4.5 and PDS 7.1
  (`DIVERGENCES-QB45.md` Q-019 and Q-020). The Civil War Strategy port found them on 2026-11-06 and measured
  them with `verification\qb45\q45_cws_paint_leaks`. QB64pe and QB64Rust give the same pixels as each other.
  QB64Rust keeps them, as it follows QB64pe.

A behavior of PDS itself can be measured the same way: write a `q45_` program and record it with
`qb45.py --compiler pds71` (`tools\qb45\README.md`, "PDS 7.1").

The full notes of each port are in the game's repo: `docs\devnotes.md` and `tests\README.md`.
