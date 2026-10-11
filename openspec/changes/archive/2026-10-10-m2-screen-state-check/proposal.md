# Proposal

## Why

Every test so far compares what a program prints to the console. A program that draws has nothing to compare:
`SCREEN`, `LINE`, `COLOR`, `LOCATE`, `PAINT` and the rest change the window only. Those statements are most of
what Civil War Strategy needs (1,740 marked errors, 899 of them `LINE`) and all of M3's graphics work, so the
method to check them has to exist before the first of them is compiled. It is step 9a of the order of work
(`STATUS.md`, `DECISIONS.md` 2026-10-09 and 2026-10-10, `study\29` §2), brought forward because it is the
longest-lead unknown and needs only the old compiler to try.

## What Changes

- `tools\screen\screen_state.cpp`: one C++ file of this repo, given to the link of a test program as an extra
  source file. When the program runs with `QB64_SCREEN_STATE=<file>` it writes the state of the screen pages to
  that file at exit: mode, size, text cells with attributes or pixels, the palette, the cursor, and which page is
  displayed, written to and read from. With `QB64_SCREEN_KEYS=<file>` it gives the program the keys of that file.
  libqb and the reference clone are not changed.
- `qb64rust -f:ExtraLinkerFlags=<value>`: passed to the link as `qb64pe` passes it, so one flag serves both
  compilers.
- The corpus runner: a program with a `<name>.screen` file is built with the state file and run with the two
  variables; `--record` writes the state (two equal runs), a normal run compares it. `<name>.keys` holds the keys.
- First recordings in `tests\corpus\screen\`: text cells, every screen mode, pages, an image as destination, keys
  into `INKEY$` and `INPUT`, each kind of exit, and the first small drawing programs cut from Civil War Strategy.

## Not in this change

- A copy of libqb in this repo: not needed (`study\29` §2); it stays at M3.
- A session of the whole game with a fixed seed.
- The new compiler compiling a drawing statement or a windowed program: it still builds console programs only, so
  no `screen\` program is on `slice.list` yet. The comparison on its side is measured once by hand (design D7)
  and covered by the runner's tests.
- Upstream's programs without `$CONSOLE:ONLY`: measured, and they give nothing to record (design D8).
- QB64pe's `qbasic_testcases`: no licence stated, so not copied (design D8).
- Cut programs that load the game's pictures with `BLOAD`: the runner copies no data files yet.
- Sound, the window (title, size, full screen), hardware images and `_KEYDOWN`-style key state: not part of the
  state (design D3, D5).

## Impact

- New: `tools\screen\` (the file, a README, `state_to_png.py`, `cut_game.py`), `tests\corpus\screen\`,
  `verification\v23_post_keys.py`.
- `crates\driver` (`main.rs`, `build.rs`: the setting, the build key), its CLI tests.
- `tools\legacy_tests` (`run_legacy_tests.py`, its tests, the README), `.gitattributes`.
- Specs: new `testing/screen-state`; `compiler/cli` (the setting); `testing/golden-corpus` (the corpus layout).
- Documents: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md`, `CLAUDE.md` layout, `study\00` §5, `tests\corpus\README.md`,
  `tests\programs\README.md`, `crates\README.md`.
