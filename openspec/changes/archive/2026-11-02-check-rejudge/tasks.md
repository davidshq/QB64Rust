# Tasks

Standing rules: CLAUDE.md.

## 1. `counts.py` in seconds

- [x] 1.1 `run_legacy_tests.py` `check_list`: list each program's folder once (a dict of folder to sorted file
      names) and pass `names` to `program_inputs` (design D5). Verified by `test_run_legacy_tests.py`: the result
      of `check_list` is unchanged for a program with sidecars, a `.files` folder and a changed input; and by
      `counts.py --check` in the worktree, timed (target: seconds), the time into design "Measured".

## 2. Re-judging stored results

- [x] 2.1 `known_lists(args)`: the `known` of `main` in one helper, used by `main` and by the re-judge (D1).
- [x] 2.2 `--rejudge --suite <s>`: reads `results-<s>.json`, takes `KFAIL` back to `FAIL` (the ` (known: …)` suffix
      stripped), applies today's lists, writes the results, the summary and `run["rejudged"]`, prints and exits as a
      run; refuses (exit 2) another runner version, another compiler build, a program changed after the run started,
      and a program whose skip changed. Tests: a failure made known, a known failure taken off the lists, a changed
      program, a skip changed, `--check-list` and `counts.py`'s `full_run` reading a re-judged file.
- [x] 2.3 A test that every file `known_lists` opens matches `steps.py`'s `JUDGE_LISTS` (design, "Risks").

## 3. `check` plans a re-judge

- [x] 3.1 `steps.py`: `JUDGE_LISTS`; `Step.rejudge`, the re-judge command, on the three Windows and three Linux whole
      steps (D2).
- [x] 3.2 `check.py`: `inputs_of` computes the run fingerprint (the lists left out, the needs' run fingerprints);
      `run_step` records it, and `"merge": true` under the merge (D3); `make_plan` plans "re-judge: only the known
      lists changed" (also in the merge; not under `--force`), and lets a merge's pass stand for a later merge.
- [x] 3.3 `test_check.py`: a known-list row added after a failed whole suite plans a re-judge and nothing else of
      that suite; a list row and a `crates/` file plans a run; a second `check merge` with only a list changed plans
      the re-judge, the sync and the steps that need them, and lets the rest stand; `--force` still runs everything.

- [x] 3.4 The merge's Linux sync in its usual form after an earlier merge's passed (design D6, added after the
      build): `Planned.usual`, `command_of`; `test_check.py`: content at the first merge and under `--force`, git's
      list after a merge's pass.

## 4. Close

- [x] 4.1 Measure in the worktree: `counts.py`; a re-judge of `upstream-whole` after a row added to
      `known_failures.txt` (time, and that nothing ran); into design "Measured".
- [x] 4.2 `tools\check\README.md`: the re-judge and the merge's standing passes; the
      paragraph "After a fix to a failed `just merge`" replaced by what the tool now does. `tools\legacy_tests\README.md`:
      `--rejudge`.
- [x] 4.3 `STATUS.md` "Next": this change built; `DECISIONS.md` row for D2 and D3 (a merge's passes stand for the
      next merge); `SOMEDAY.md` row for D4 (the Linux pass cache).

## Workflow follow-up

- `just quick` in the background before the files are handed over; `/opsx:archive` after the user's review.
