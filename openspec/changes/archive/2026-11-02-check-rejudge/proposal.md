# Proposal

## Why

At the merge of `linux-sync-per-tree` (2026-11-02), a fix that only changed which failures count as known (two rows
in `tools\legacy_tests\known_failures.txt`, two in `tests\verification\known.linux.list`) cost about 20 minutes of
whole suites: the runner applies the known lists only while it runs, so updating four verdicts meant running more
than 4,700 programs again. Comparing stored results with lists should take seconds. Two more costs showed up the
same day: `counts.py`, which only rewrites three README blocks from results files, takes 170 s, and `just merge`
run again after the fix would have forced all 37 steps (about 1 h 40 min).

## What Changes

- **Re-judging a run.** The runner gains `--rejudge`: it reads a suite's last results
  (`target\legacy-tests\results-<suite>.json`), applies the known lists as they are now, rewrites the statuses and
  the summary, and prints the verdict as a run does. No program is built or run. It refuses results of another
  compiler build or runner, and a run whose inputs other than the known lists have changed.
- **`check` re-judges when only the known lists changed.** A whole-suite step whose last run differs from now
  only in its known lists is planned as "re-judge", not "run". This also applies to a failed step: the run
  stays failed, but its results can be judged again. The same goes for `--force` and the merge, which otherwise
  run every step again.
- **A merge's passes stand for the next merge** when their fingerprints are unchanged, so a merge run again after
  a fix runs only what the fix reaches.
- **Not in this change:** the pass cache for the Linux whole suites (the pragmatic engineer, 2026-11-02: with the
  re-judge a list fix no longer needs it, the merge does not use it, and it would be a second cache path on Linux
  that nothing exercises; a `SOMEDAY.md` row).
- **`counts.py` in seconds.** It is measured first (`design.md`); the expected cause is `check_list` listing
  `verification\` (about 15,000 files) once per program.

## Capabilities

### Modified Capabilities

- `testing/check-workflow`: a result can be judged again without a run when only the known lists changed; a
  merge's passes stand for the next merge.
- `testing/verification-suite`: the known lists can be applied to stored results (`--rejudge`).

## Impact

- `tools\legacy_tests\run_legacy_tests.py` (`--rejudge`, `check_list`), `tools\legacy_tests\counts.py`,
  `tools\check\check.py` (planning a re-judge, the merge's standing passes), `tools\check\steps.py` (the judge
  inputs of the whole steps), and their tests (`test_run_legacy_tests.py`, `test_check.py`).
- `tools\check\README.md`: the paragraph added on 2026-11-02 ("After a fix to a failed `just merge`") becomes
  what the tool does by itself.
- Changing the runner invalidates every stored result once (each records the runner's hash), so the first
  `just full` after this change runs everything.
