# Tasks

Standing rules: CLAUDE.md.

## 1. The compare reads a diverged sidecar

- [x] 1.1 `run_legacy_tests.py`: `diverged_sidecar(name)` and its log beside `fixed_sidecar`. `verification_compare`
      judges by them in place of the recording and its log (design D1). `program_inputs` and the pass cache key
      take both files. A recording with `qb64pe` never reads them. Verified by new tests in
      `test_run_legacy_tests.py` with a stand-in compiler: a program that passes by its sidecar and fails by
      QB64pe's recording; a second line that differs from the sidecar fails at "result" and names the sidecar; a
      log sidecar; a changed sidecar misses the pass cache; `--list-only` names the sidecar.
- [x] 1.2 A diverged rejection (design D3): the line wanted is the program's `lines.list` row; with no row it fails
      at "error line"; such a row is never reported as not needed. Verified by runner tests: rejected on the
      pinned line, on another line, with no row.
- [x] 1.3 `tests\verification\README.md` gets a row for `diverged\` in its table of files, and "How the compare
      works" says which recording a program is judged by. `tools\legacy_tests\README.md` ("Verification
      programs") the same in one line. `docs\LAYOUT.md` gets a row for the folder. Verified by reading them
      against the spec delta.

## 2. The tier-1 check

- [x] 2.1 `crates\driver\tests\it\inputs.rs`, `verification_lists_are_exact`: the checks of the spec's "The diverged
      sidecars are checked" (design D2), the D rows read from `docs\DIVERGENCES-QB64PE.md`. Verified by
      `cargo test -p qb64rust-driver --test it inputs::` passing with no sidecar, then failing (and reverted) on a
      sidecar with no `.divergences`, a sidecar of a program still on `known.list`, and a stray file in the folder.
- [x] 2.2 `counts.py`: the membership block counts the programs of `pass.list` judged by a sidecar (design D6).
      Verified by `test_counts.py`. The block in `tests\verification\README.md` is not rewritten until the merge.

## 3. Writing and reviewing the sidecars

- [x] 3.1 Run the D programs of `known.list` twice with today's release build, by a script under `target\` (not
      committed) that builds each candidate with `verification_text` (design D5 steps 1 and 2). Leave out the 8
      programs of design D4 and any program whose two runs differ. Verified by the script's list of candidates
      and of programs left out, with the reason for each.
- [x] 3.2 Review each candidate against QB64pe's recording (design D5 step 3) and fill in the table "Review of the
      sidecars" in `design.md`. Write `diverged\<name>.out.txt` (and `.log.txt` where the log differs) for each
      program that passes the review, and its `divergences\<name>.divergences`. A program that does not pass the
      review gets the reason `bug: <what>` on `known.list` and a line in `STATUS.md`'s known bugs. Verified by
      the table having one row per candidate.
- [x] 3.3 For each diverged rejection, a `lines.list` row with the line of the error and the reason "diverged
      rejection: D-0nn". Verified by reading each program at that line.
- [x] 3.4 Move each reviewed program from `known.list` to `pass.list`; programs whose runs vary get `varies` after
      their D row (design D4). Verified by `cargo test -p qb64rust-driver --test it inputs::` and by
      `run_legacy_tests.py --suite verification --qb64 target\release\qb64rust.exe` on the moved programs alone,
      every one passing.

## 4. Closing

- [x] 4.1 `DECISIONS.md`: a row for the diverged sidecars (design D1 to D4) and for leaving out the shorter timeout
      (design D7). `STATUS.md`'s "Next" says the change is built. `TODO.md` row 1 stays until the archive, by
      `TODO.md`'s own rule. Verified by reading them.
- [x] 4.2 `just quick` in the background; say what ran and what stood. Verified by its log.

## Workflow follow-up

- Archive the change once the user has reviewed the sidecars; `TODO.md` row 1 goes then.
- At the merge into `main`, `counts.py` rewrites the count blocks, and `just merge` runs the whole suite.
