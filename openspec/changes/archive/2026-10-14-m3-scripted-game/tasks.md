# Tasks

Order: group 1 first (measurements and the user's answers), then 2 (the two steps), 3 (the copy), 4 (the script,
with the old compiler), 5 (the new compiler), 6 (close). Each of groups 2 to 5 ends with `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and `cargo test` (tier 1) green where it touched Rust, and
with its tests, lists and `README.md` lines in place, so the change can stop after any group.

Standing rules for every task: work inline, no subagents; nothing staged or committed by Claude: each group ends
by naming the files to stage and a commit message; text with backslashes goes through the Edit and Write tools;
`python tools\repo_check\check_repo.py --untracked` is clean before a group is reported done; the game in
`tests\programs` and the reference clone are not changed (`git status --porcelain` of the clone empty before and
after). **Test cadence** (`CLAUDE.md` working rule 9): while working, tier 1 and only the programs the work touches
(`--glob screen/g12*`, `--glob slice/s78*`, the 52 screen programs once after group 2 since every one of them uses
`screen_state.cpp`); tier 2, the run without folding and the full suites run once, in group 6. A new measured fact
goes into `study\00` §5 in the task that measures it.

## 1. Measurements and questions

- [x] 1.1 Measure, outside the repository, with a hand-made copy of the game with the edits of design D1 built by
  `qb64pe.exe` with `screen_state.cpp`: that `DEF SEG = 0: POKE 1264` reaches `cmem[0x4F0]` and nothing else
  writes it (a print of the byte from a scratch build of `screen_state.cpp`); the size of one `SCREEN 12` state
  of the title, the map, and a report; the time of a run through a short script (title, new game, one END TURN,
  quit). Verify: the facts are in `study\00` §5; D4's number of states and D5's run time are confirmed or
  corrected in `design.md`.
  - Done (2026-10-14; facts in `study\00` §5, "Civil War Strategy played from a keys file"). The steps of
    task 2.1 were written first in `screen_state.cpp` and `--state`/`--all` of task 2.4 in `state_to_png.py`, as
    the measurement needs both. A short game (new game, END TURN with a battle, Files, Quit) ends by itself in
    1.1–1.4 s, four runs byte for byte equal, eight states of 95–121 kB. **Changed in the design:** `prompt`
    counts announcements instead of waiting for a change (D2), since a menu announces again after every arrow.
- [x] 1.2 Put the three open questions of `design.md` to the user, with what 1.1 found; record the answers in
  `DECISIONS.md` and in `design.md`. Verify: a dated row in `DECISIONS.md`.
  - Done: all three answered as the default (sound off; 15–25 states; the edited copy run by a test).

## 2. `prompt` and `state` in the screen-state check

- [x] 2.1 `screen_state.cpp`: the steps `prompt` and `state` (design D2), the 30 s limit with exit code 124 and
  `QB64_SCREEN_PROMPT_MS`, states appended before the state at exit. Verify: a keys file without the new steps
  gives byte-for-byte the same state as before for the 52 screen programs (run once with the new compiler,
  `--category screen`); a line `prompt 5` is an error with code 125.
  - Done: 52 of 52 pass (`--category screen`, the build cache keyed on the file's contents); `prompt 5` gives
    "line 1: not a step" and code 125. `prompt` counts announcements (design D2, changed in task 1.1); a count
    past the expected one is code 125.
- [x] 2.2 `s78_prompts` in `tests\corpus\slice\` with its keys, recorded with `qb64pe.exe`, on `slice.list`
  (design D6: the scenarios "A key a pause would eat" and "A state while the program runs"). Verify: it passes
  with both compilers; with the `prompt` lines taken out of its keys file the recording differs (the pause eats a
  key), shown once and put back.
  - Done as **`tests\corpus\screen\c12_prompts`** (screen programs live in `screen\`, and the `c` series is the
    check's own): a box and a state at the first wait; a loop of 0.2 s that only looks for a key; a second read;
    a four-entry menu that announces after every key, with a state when entry 3 is chosen; three states in all
    (16 kB). Recorded with `qb64pe.exe`, passes with the new compiler. Built by `qb64pe.exe` outside the
    repository and run with the same keys without `prompt`: the loop eats `b` and every key after it, and the
    program never ends (stopped after 20 s). It goes on `slice.list` with `g12_game` in task 5.2.
- [x] 2.3 The runner's test for "A prompt that never comes" (skipped without the clone). Verify: it passes; it
  fails if the exit code is changed in `screen_state.cpp`.
  - Done as `screen_state_prompt_that_never_comes` in `crates\driver\tests\cli.rs` (`#[ignore]`d, beside
    `extra_linker_flags_link_the_screen_state_file`): the runner's own tests build nothing, the command-line
    tests that build do. It passes; with code 123 in `screen_state.cpp` it fails (`left: Some(123)`), put back.
    **Found:** libqb points stderr at the console the program was started from (`AttachConsole`, `freopen
    CONOUT$`, `libqb.cpp` 25816) unless it is an MSYS or Cygwin pipe, so the key thread's message never reaches
    a test's pipe. The message is therefore also the last line of the state file (`stopped <keys> line <n>:
    <why>`), which the runner keeps when a test fails; spec and design changed to say so.
- [x] 2.4 `state_to_png.py`: `--state <n>` picks one state of a file with several (default the last), and the
  file's states are counted on stderr. `tools\screen\README.md`: the two steps, the rule "a `prompt` before every
  key of a program with look-only reads", several states in one file. Verify: the PNG of each state of `s78_prompts`
  shows what the scenario says.
  - Done, with `--all` too (each state to `<out>-<n>.png`, used to look at the game's states). The PNGs of
    `c12_prompts`: the red box; then "first: x", "eaten: 0", "second: b" and entry 3 in yellow; files with one
    state (`g01_flags`, `c07_input_window`) show as before.

## 3. The copy of the game

- [x] 3.1 `cut_game.py`: the whole-game entry with its list of edits (design D1) and `g12_game.files\` with the
  data files and the written `CWS.CFG` (D3). Verify: `g12_game.bas` differs from `CWSTRAT.BAS` (line ends aside)
  only in the listed lines (a `diff` shown in the task's note); changing the start text of one edit makes the
  script stop and name it; the eleven `g01`–`g11` programs are written byte for byte as before.
  - Done: `COPIES` and `CONFIG_CHANGES` in `cut_game.py`. An edit names a text the line must hold **exactly
    once** (stronger than "starts with"; spec changed). `diff` of the game (CR removed) and the copy: lines 168,
    218, the 12 `DO WHILE INKEY$` lines, 3394, 4037, 4038, 4855, and seven comment lines after line 5075; line n
    of the copy is line n of the game. Edit (219, `RANDOMIZE TIMER`) stops with "CWSTRAT.BAS line 219 no
    longer holds 'RANDOMIZE TIMER' exactly once". `git status`: no change to `g01`–`g11`. `g12_game.files\`:
    the 14 files the game reads besides `CWS.CFG` (`CITIES.GRD`, `CWS.INI`, `CWSLEAD.DAT`, `HISCORE.CWS`, the eight `FACE`/`FORT`
    pictures, `MTN.VGA`, `CWSICON.VGA`) and `cws.ico` (`$EXEICON`), and `CWS.CFG` with `noise` 0.
- [x] 3.2 Check that every place the game waits for a key is announced: run the old compiler's build of the copy
  with a keys file of `prompt` and Esc/Enter only and see it reach the main menu; grep the copy for `INKEY$`,
  `INPUT` and `SLEEP` and list each read as announced or look-only in this task's note. Verify: the note lists the
  14 places and the look-only reads, each with its line.
  - Done. Grep of `g12_game.bas` for `INKEY$`, `INPUT$`, `INPUT` (not `#`), `LINE INPUT`, `SLEEP`, `_KEYHIT`,
    `_KEYDOWN`: **announced** (14): 662, 829, 1002, 1091, 2462, 2719, 3127, 3263, 3266, 3366, 3394 (`INPUT
    a$`), 3408, 3675, 4855 (the menu's `crsr`). **Look-only** (11), each reached only with `noise` 2, which the
    copy's `CWS.CFG` turns off: 186, 188, 190, 192, 194, 196 (the title music) and 3834–3842 (`SUB shen`).
    **Taken out**: 4038 (`TICK`). No `SLEEP`, `INPUT$`, `LINE INPUT`, `_KEYHIT` or `_KEYDOWN`; lines 808 and
    4044 are text in a `PRINT`. The run to the main menu and on through a turn: task 1.1 (the same edits).

## 4. The script, recorded with the old compiler

- [x] 4.1 Write `g12_game.keys` step by step, playing the old compiler's build (design D4), looking at every state
  with `state_to_png.py`. Verify: the run ends through the game's own Quit, by itself, within the time limit; the
  note lists the procedures of the game the script reaches and those it does not, and why.
  - Done: 223 lines, two turns as the Union (March to July 1861), 22 `state` steps. Ends through Files, Quit,
    Yes and END's prompt (`repeat space`), exit code 0, in 1.1–1.9 s; states equal byte for byte in every repeat
    (more than ten runs while writing it). Written by exploring: a scratch script ran the keys file with a
    trailing `prompt`, `state`, `prompt`, so that a run ends 3 s after its keys, and showed the last states.
    **Reached** (seen on the screens): the title and its menu; `SUB menu` with every kind of key it is given
    (Enter, Esc, arrows, wrapping round); `usa`, `topbar`, `flags`, `clrbot`, `clrrite`; `recruit`; `armies`
    and `movefrom` (two moves); `navy` (Build); the railroad move with `newcity` and `tinytrain`; `report` (Union
    Armies, Cities, Battles); `fortify`; the Utility menu; `filer` (save to `cws1.sav`); END TURN with
    the main module's `newmonth:`, `tupdate` (the month's events), the computer's moves, `battle` with `cannon` and its report
    (Jackson attacks Fremont at Harper's Ferry, twice), `victor`; `rusure` and `END`. **Not reached**, each
    needing a longer script or another start: a naval action (`navy` Sail and Raid, `barnacle`, `ironclad`,
    `schooner`), a retreat (`retreat`), `combine`, `relieve`, Capital, Supply, Detach (Confederacy only), the
    end of a game (`endit`, `capitol`, the high-score file), loading a game, the alternative map, two players,
    the history file, the title music (`shen`, `noise` 2). The game's own moves of McClellan and McDowell into
    Lexington and Manassas are made and drawn; no battle came of them in two turns.
  - **Found on the way, not a fault:** three runs seemed to disagree about the railroad picker; it was the
    numbering of states read off a contact sheet one step wrong. Every variant repeated byte for byte.
- [x] 4.2 `--record` `screen/g12_game` with `qb64pe.exe` (two equal runs). Verify: `g12_game.screen` holds the
  states of 4.1 and the exit state; its size is within the answer of 1.2; if a longer time limit is needed, the
  runner's `<name>.timeout` (design D5) with a runner test.
  - Done: recorded on the first try (12.3 s with the build), 23 states (22 and the one at exit), 2.5 MB;
    `g12_game.output` empty. No longer time limit needed, so the runner is not changed (D5). `.gitattributes`
    already keeps `*.screen`, `*.keys` and `*.files\**` byte for byte.

## 5. The new compiler

- [x] 5.1 Run `screen/g12_game` with the new compiler. Where a state differs: find the first state and the first
  row that differ (`state_to_png.py` of both), reduce it to a slice program recorded with `qb64pe.exe`, fix the
  compiler with that program on `slice.list`. A fault that needs more than a contained fix is put to the user.
  Verify: `g12_game` passes; each fix has its slice program; `tests\known_clean_not_passing.list` does not name
  `g12_game`.
  - Done with no fix: **`g12_game` passes with the new compiler on its first run**, all 23 states equal (5.9 s
    with the build). Shown to compare the states mid-game: with one pixel row of the 12th recorded state changed
    it fails at stage "screen" ("screen state differs from .screen"); the recording was put back and equals the
    scratch run's byte for byte. Not on `known_clean_not_passing.list`.
- [x] 5.2 `g12_game` and `s78_prompts` on `tests\corpus\slice.list`. Verify: `cargo test` passes (the clean
  programs check).
  - Done: `screen/c12_prompts` (task 2.2's name for `s78_prompts`) and `screen/g12_game`; `cargo test` passes.

## 6. Close

- [x] 6.1 Closing runs once (`CLAUDE.md` working rule 9): tier 1, the runner's tests, the call-site check if a
  compiler crate changed, the slice list with and without `QB64RUST_NO_FOLD=1`, the upstream and differential pass
  lists, the two full suites; then `python tools\testtime\test_time.py`. Verify: the numbers in `STATUS.md`'s
  table; the three files of `tools\testtime` named among those to stage.
  - Done (2026-10-14, release build, `--jobs 8 --build-cache`): tier 1 passes; the command-line tests that build
    17 of 17 (10 s; the new `screen_state_prompt_that_never_comes` among them); the runner's tests 68 of 68;
    **the slice list 386 of 386** (54 screen programs; 5 min 53 s, since every screen program is linked again
    with the changed `screen_state.cpp`). **Not run, and why that is safe:** no compiler crate changed (only
    `crates\driver\tests\cli.rs`), so the call-site check, the run without folding, the upstream and differential
    pass lists and the two full suites, which test the compiler's output, cannot change; the full corpus's
    screen programs are all on the slice list. `test_time.py` run (197 sessions, 3782 runs).
  - **Found at the close:** the repo check flagged seven control bytes in `g12_game.bas`; they are the game's
    own (CP437 symbols in its strings, the same in `CWSTRAT.BAS`, which `.gitattributes` marks `-text`). The copy
    is marked `-text` too; the check is clean.
- [x] 6.2 Documents: `tests\programs\README.md` (stage 3 done, what the script covers), `STATUS.md`, `ROADMAP.md`,
  `tests\corpus\README.md`, `tools\screen\README.md`, `study\00` §5, `DECISIONS.md` (the copy run by a test),
  `CLAUDE.md`'s layout table (`tools\screen\`, `tests\corpus\`), `SOMEDAY.md` (what the script does not reach, a
  second script, sound in a scripted game). Verify: `python tools\repo_check\check_repo.py --untracked` is clean;
  the files to stage and a commit message are given to the user.
