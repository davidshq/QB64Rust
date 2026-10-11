# Tasks

Step 9a of the order of work (`STATUS.md`; `DECISIONS.md` 2026-10-09 and 2026-10-10; `study\29` §2). Measured
first, then designed. Text with backslashes goes through the Edit and Write tools (`CLAUDE.md` rule 7); no
subagents (rule 1).

## 1. Measure

The programs of the measurements were built by hand with the state file on the link line. They are kept as the
programs of `tests\corpus\screen\` (task 5.1), named below, where they are recorded. Four small ones had no place
there and are described in the design only (D2, D3): a loop ended by closing the window, `SYSTEM 7`, a
`$CONSOLE:ONLY` program, and one taking its screen mode from `COMMAND$`. They were `verification\v23_*.bas` and
were removed (user, 2026-10-10): the verification suite cannot record a program that waits for keys.
`verification\v23_post_keys.py` is the helper of task 1.4.

- [x] 1.1 A prototype of the file, linked through `-f:ExtraLinkerFlags`, sees the pages and writes text cells,
  pixels and the palette (now `c01_text_cells`, `c02_screen12_shapes`). Found: it works as a source file on the
  link line; a build takes 2.8 s instead of 1.9 s.
- [x] 1.2 Every kind of exit (now `c08_end_prompt`, `c09_off_the_end`, `c10_runtime_error`, `c11_screenhide`; the
  window closed by the `close` step of `v23_post_keys.py`). Found: the state is written at each (design D2).
- [x] 1.3 What is written per screen mode, with several pages, and with an image as destination (now `m00`–`m256`,
  `c03_pages`, `c04_image_dest`). Found: design D3.
- [x] 1.4 Keys posted to the window (`v23_post_keys.py` on what are now `c05_inkey_keys` and `c07_input_window`).
  Found: they reach `INKEY$`, `INPUT` and `END`'s wait, with five limits, one of them only curable by timing
  (design D4).
- [x] 1.5 Keys given from inside the program through `keyboard_keydown`. Found: none lost, 600 in a row, no
  timing; taken as the method (design D4).
- [x] 1.6 The new compiler's build by hand. Found: the same state as the old compiler's, byte for byte (design
  D7).
- [x] 1.7 Upstream's 47 programs without `$CONSOLE:ONLY`. Found: 27 never run, 10 are console tests, 10 leave an
  empty text screen; the 20 that run give equal states in two runs; none is recorded (design D8).

## 2. The state file

- [x] 2.1 `tools\screen\screen_state.cpp`: the state (design D3) and the keys (design D4). Checked by hand on the
  measurement programs: text, pages, an image as destination, 32 bits, keys, a wrong keys file (exit code 125), `END`
  with `repeat`, a runtime error, and no variable set.
- [x] 2.2 `tools\screen\README.md`: what the file is, the two variables, the format, the keys file, the limits.
- [x] 2.3 `tools\screen\state_to_png.py` (design D9). Checked by eye on the game's cut programs: the end screen
  and the cannon are the game's pictures.

## 3. `qb64rust -f:ExtraLinkerFlags=`

- [x] 3.1 `main.rs`: the setting; `build.rs`: `CXXLIBS_EXTRA`, and the build key takes the value and the contents
  of each word that names a file (design D6; the key's version is 2). A unit test of the key; `cli.rs`
  (`extra_linker_flags_link_the_screen_state_file`, ignored like the others that need the clone): a program built
  with the state file writes its state.
- [x] 3.2 `crates\README.md`: the setting.

## 4. The runner

- [x] 4.1 `corpus_test`: `.screen` and `.keys` (design D5); the check that the file's path has no blank.
- [x] 4.2 `test_run_legacy_tests.py`: 14 tests of a screen program, the stand-in writing the state file (55 tests
  in all, 0.3 s). Checked that they bite: ten mistakes seeded one at a time into a copy of the runner by a scratch
  script (the state never compared, a missing state passes, differing states recorded, the keys not passed on, a
  stale state counts, the flag not given, a failed compile recorded as `.err`, the variable not set, `.screen`
  beside `.err` accepted, a blank in the path accepted). Found: all ten caught; the unchanged copy passes.
- [x] 4.3 `.gitattributes` (`.screen`, `.keys`); `tools\legacy_tests\README.md`.

## 5. First recordings

- [x] 5.1 `tests\corpus\screen\` with `SOURCE.md`: 23 programs of the method itself (`c01`–`c11`, one per screen
  mode `m00`–`m256`), from the measurement programs. Recorded with `--record` (two equal runs each); a normal run with
  the old compiler passes.
- [x] 5.2 Six drawing programs cut from Civil War Strategy, `g01`–`g06`, written by `tools\screen\cut_game.py`
  from the game's procedures and recorded. Not cut: what loads a `*.VGA` picture with `BLOAD` (the runner copies
  no data files; `SOMEDAY.md`).
- [x] 5.3 `qbasic_testcases`. Found: not taken, no licence stated (design D8); of the 9 that read no key, 6 end
  with equal states.
- [x] 5.4 The tier 1 lists: nothing to add. The new compiler rejects all 29 with only "not supported yet" errors
  (the first: programs without `$CONSOLE:ONLY`), so none is "clean and not passing".

## 6. Close

- [x] 6.1 `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`,
  the runner's tests, `python -m ruff format --check .`, `python -m ruff check .`,
  `tools\repo_check\check_repo.py --untracked`: clean. The full corpus with the old compiler: 336 programs, 331
  pass, the 5 known failures (`--jobs 8`; the 29 screen programs eight at a time). With the new compiler: 271
  pass, 56 rejected with "not supported yet" (27 as before, the 29 screen programs), 4 `.err` programs with only
  marked errors, the 5 known failures; `slice.list` 255 of 255. Every program was built again: the build key's
  version changed.
- [x] 6.2 Documents: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md` (the keys from inside the program, upstream's
  programs and `qbasic_testcases` left out), `CLAUDE.md` layout, `study\00` §5, `tests\corpus\README.md`,
  `tests\programs\README.md`, `SOMEDAY.md` (a desktop of their own for test windows, data files beside a corpus
  program, the testcases run from the clone).
- [x] 6.3 The fourteen `verification\v23_*.bas` removed (user, 2026-10-10: "if you don't think those v23 programs
  are worth saving, get rid of them"). Ten were twins of `tests\corpus\screen\` programs; the other four are
  described in the design. `v23_post_keys.py` stays.
