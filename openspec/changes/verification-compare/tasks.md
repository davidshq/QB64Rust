# Tasks

Standing rules: `CLAUDE.md`.

## 1. The compare in the runner (design D1–D4)

- [x] 1.1 `verification_text(out, rc, timed_out, folder)`: the text of `<name>.out.txt` from a run, used by
  `verification_test`. Check: a recording of three programs (one rejected, one ran, one `.stdin`) with `qb64pe`
  changes no file of `verification\`.
- [x] 1.2 `press_any_key.py --no-keys`: the hidden desktop without key presses. Runner test: a stand-in that
  waits at `END` times out under `--hidden-desktop --no-keys`.
- [x] 1.3 `verification_support(root, name)` and `verification_compare(bas, args, results, qb_root)`: the fresh
  folder, the compile, the run as recorded (D3), the judgement of D4, the divergence trace and the sidecars in
  `tests\verification\`. Remove the refusal in `main()` and dispatch by `is_old_compiler`. A missing `v37`
  library fails with "run v37_lib\build_libs.py first".
- [x] 1.4 `--list`, `--pass-cache`, `--check-list` for suite `verification`: `result_key`, `program_inputs` (no
  `.exe`, plus the two sidecars), `check_list`, `load_list` with plain names.
- [x] 1.5 Runner tests in `test_run_legacy_tests.py`, one per rule of the spec's "The compare's rules are
  tested". Check: `python -m unittest discover -s tools/legacy_tests -p "test_*.py"` passes.
- [x] 1.6 Checked for real on one program of each kind: `v20_x01` (rejected), `v17_a_pending` (ran), a
  `v22_e` program with `.stdin`, a recorded timeout, a `v16` program with an included file, a `v37` program
  with a library. Each gives the expected verdict, and `git status --porcelain verification` is unchanged.

## 2. The lists (design D5)

- [x] 2.1 `tests\verification\README.md` (what the folder holds, the reasons, how to run it, a counts block),
  empty `pass.list` and `known.list`, `divergences\`, `normalize\`.
- [x] 2.2 `verification_lists_are_exact` in `crates\driver\tests\inputs.rs`: every program named once across the
  two lists, every name a program, every `known.list` line with a reason from the vocabulary.
- [x] 2.3 `counts.py`: the membership and results blocks of `tests\verification\README.md`, with its test in
  `test_counts.py`.

## 3. The first full run and its sorting (design D6, D8)

- [x] 3.1 Build the `v37` libraries (`verification\v37_lib\build_libs.py`), then run the full compare in the
  background (`--suite verification --qb64 target\release\qb64rust.exe --jobs 8 --build-cache
  target\build-cache`, log in `target\`).
- [x] 3.2 Sort every failure by D6: local bugs fixed, each with a test made from its family's program; the rest
  listed with their reasons, sidecars written where a row or a rule applies. Any `DIVERGENCES.md` row or
  decision made on the way goes in `DECISIONS.md` in this task.
- [x] 3.3 Fill `pass.list` from the full run, and run the full compare again if 3.2 changed the compiler.
  Check: `--check-list tests/verification/pass.list` passes, and `verification_lists_are_exact` passes.
- [x] 3.4 The wider bugs as lines in `STATUS.md`'s known bugs, each naming its program in `known.list`.

## 4. Cadence and documents (design D7)

- [x] 4.1 `close.py`: the step `verification`, with its test in `test_close.py`. Check: `close.py --plan` lists it.
- [x] 4.2 `CLAUDE.md` working rule 9 (the closing set and the merge set name `tests\verification\pass.list`; the
  hand run of a family's accepted programs goes) and its layout table (`tests\verification\`).
- [x] 4.3 `tools\legacy_tests\README.md` (the compare, and the merge runs), the runner's docstring, and one line in
  `verification\README.md` naming the compare and `tests\verification\`.
- [x] 4.4 `SOMEDAY.md`: the row for the CI `tier2` job. `DECISIONS.md`: the row for this change (the compiler picks
  the mode, the lists in `tests\verification\`, the closing step).

## 5. Close

- [x] 5.1 `STATUS.md`'s "Next": item 1 done, M4's parity work first.
- [x] 5.2 The closing runs (`python tools\close\close.py --plan`, then `--check`).
- [x] 5.3 `python tools\testtime\test_time.py`.
