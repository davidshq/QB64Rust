# Tasks

Standing rules: `CLAUDE.md`. The scripts of group 5 are recorded with `qb64pe.exe` in batches, before the closing
runs (working rule 9).

## 1. Coverage build (design D1)

- [x] 1.1 `DECISIONS.md`: the row for this change (the user's word of 2026-10-19: the whole game, measured by line
  coverage; coverage from the new compiler, scripts with rules and digests, one copy with several scripts).
- [x] 1.2 `--coverage` in `crates\driver\src\main.rs` and the build options, part of the build cache's key; the
  counter, the table of entries (one per distinct file, line and column) and the `atexit` writer in
  `crates\codegen-cpp` (`procs.rs` `body`, `decl.rs` `global`). Check: the existing `cpp` snapshots are unchanged
  (no option, no change), and a new front-end or CLI test shows the counter's C++ for an `IF … THEN a: b ELSE c`
  and a `SELECT CASE`.
- [x] 1.3 Confirm on `--dump ir` and the table that the lowering's own `Jump`s share the position of their block's
  statement, so they add no entry. If they do not, give them no entry. Check: a CLI test (`--ignored`, it builds)
  runs the spec's three-statement program with `QB64RUST_COVERAGE` and finds counts 1, 0, 1; a second run appends
  a second record; a program with a `$INCLUDE` names the included file.
- [x] 1.4 `crates\README.md`: the option in the build and run instructions. Check: `cargo test` and clippy pass.

## 2. The copy's edits and the keys file (design D4, D5, D6)

- [x] 2.1 Measure `PLAY "V0"` with `qb64pe.exe`: a program in a new `verification\` family that plays the game's
  title tune and a `SOUND` after `PLAY "V0"`, and prints `PLAY(0)` and the time each takes. Does the volume hold
  across the game's later `PLAY` strings, are the waits for notes unchanged, and is nothing heard? Write the
  family's line in `verification\README.md`. If `V0` does not hold, the sound lines are listed as `edit` and the
  edit is left out.
- [x] 2.2 `cut_game.py`: the place and letter bytes in the 14 announcements, `PLAY "V0"` (per 2.1), and the fixed
  date. Write `g12_game.bas` again. Check: the two-turn script still passes with the new compiler, and recorded
  again with `qb64pe.exe` its states are unchanged (the announcements draw nothing).
- [x] 2.3 `screen_state.cpp`: the place bytes, `rule`, `play until`, `repeat <n>` … `end`, `digest`, and
  `QB64_SCREEN_DIGEST_AS_STATE`. Check: new screen programs in `tests\corpus\screen\` (`c13_rules`, `c14_digest`),
  written for both builds, recorded with `qb64pe.exe`, passing with the new compiler; one more shows that a wait
  with no rule ends with code 125 and the message (a runner test with a stand-in, as for `prompt`).
- [x] 2.4 `tools\screen\README.md`: the place bytes, the new steps, the digest and its variable. The spec's scenario
  for `c12_prompts` holds (its states are unchanged).

## 3. Scripts of a screen program, and the coverage run (design D2, D3)

- [x] 3.1 The runner: `<name>.<script>.keys`/`.screen`/`.output` as tests named `<name>.<script>`, one build for
  all; `--glob`, lists, `--record`, `--pass-cache` and `--check-list` take the test name; `--full-states` for one
  test; `--coverage <file>`. Runner tests for each, including the spec's "A script without its screen file". Check:
  the runner's tests pass, and `screen/g12_game` still runs under its old name.
- [x] 3.2 `tools\coverage\coverage.py`: read and merge records, refuse a stale hash, report by line, `--check`
  against `tests\programs\civil-war-strategy.unreached` (both directions, the fixed set of reasons), and the run of
  D2. Its own tests (`test_coverage.py`), one per scenario of the spec `testing/coverage`. A `README.md` in the
  folder.
- [x] 3.3 First measure: the coverage of the two-turn script alone, written into the change's design ("As built")
  as the starting point.

## 4. The first whole game (design D4, D5, D7)

- [x] 4.1 `g12_game.union_time`: Union, solo, Fast display, END TURN each month by rules to the time limit, the
  high-score name, the ending. Record it with `qb64pe.exe` and compare it. Write down how long a run takes, and
  split the script at a save if it is over 60 s (Risks). Check: it passes with the new compiler, is on
  `slice.list`, and its `.screen` stays within D5's share.
- [x] 4.2 Any difference found: fixed with a test that shows it alone (D8). Check: the test fails before the fix.

## 5. Scripts until the game is covered (design D7, D8)

- [x] 5.1 `rebel_capital`, `two_player`, `options`, `files`, `commands`, `endings`, in the order the coverage
  report asks for them. Each is recorded, compared, put on `slice.list`, and followed by a new coverage report.
  Differences are fixed as in 4.2.
- [x] 5.2 Further scripts for the lines still not run, until each is run or can be listed with a reason.
- [x] 5.3 `tests\programs\civil-war-strategy.unreached` complete, each line with its reason. `coverage.py --check`
  passes. The new `.screen` files together are under 15 MB.
- [x] 5.4 `tests\programs\README.md` and `tools\screen\README.md`: the scripts, the coverage check, and how to add
  a script. The spec's requirement on Civil War Strategy holds.

## 6. Cadence and close

- [x] 6.1 `close.py`: a `coverage` step (`coverage.py --check`), only when `g12_game`'s files, `cut_game.py`,
  `screen_state.cpp` or `codegen-cpp` changed; its test in `test_close.py`. `CLAUDE.md` working rule 9: the step in
  the closing set, and the coverage check in the merge set.
- [x] 6.2 `STATUS.md`'s "Next".
- [x] 6.3 The closing runs (`python tools\close\close.py --plan`, then `--check`).
- [x] 6.4 `python tools\testtime\test_time.py`.

## Workflow follow-up

- Archive the change after the user commits it.
