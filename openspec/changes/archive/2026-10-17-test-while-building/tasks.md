# Tasks

Standing rules: `CLAUDE.md` (no subagents, nothing staged, backslashes through the tools, test cadence rule 9).

## 1. The build key (design D1)

- [x] 1.1 `build_key` leaves out what builds write in the clone (`written_by_builds` in `crates\driver\src\build.rs`,
  the runner's rule); unit tests `build_key_leaves_out_what_builds_write` and `written_by_builds_is_the_runners`.
- [x] 1.2 Checked for real: five slice programs built, one compiled with `qb64pe.exe` in the clone (its `qbx.o`
  rewritten), the five again from the cache (0.4–0.6 s against 3.0 s).

## 2. The copy a run tests (design D2)

- [x] 2.1 `tools\legacy_tests\compiler_copy.py` (`run_copy`, `prune`, `header`, `read_header`) and its tests
  (`test_compiler_copy.py`, 5).
- [x] 2.2 The runner and `callsite.py` compile with the copy and print the `[compiler]` line; the results file
  names the compiler as given. Seen in two narrowed runs of the slice list.
- [x] 2.3 `close.py --check` judges the steps with a copy by its hash and start (`Step.copy`, `BUILT_FROM`); tests
  in `test_close.py` (5 new).

## 3. Locks from a worktree (design D3)

- [x] 3.1 `clone_lock.main_checkout` and the CLI tests' `main_checkout`; tests (`MainCheckout`, 3, on the layout
  git writes).
- [x] 3.2 The merge runs in a worktree: `tools\legacy_tests\README.md`, `CLAUDE.md` working rule 9.

## 4. Close

- [x] 4.1 The closing runs (`close.py --check` passes): tier 1 348 passed; fmt and clippy clean; the wording check;
  the slice list 453 of 453 and the upstream pass list 347 of 347, side by side, each with the `[compiler]` line,
  while tier 1, fmt, clippy and the wording check ran beside them (the pass cache skipped none: the regenerated
  wording table gave a new build; the build cache missed once for every program, its keys being new); the
  call-site check 1884 of 1884 equal, queued behind them with `--wait`; the CLI tests that build, 39 of 39; the
  demand reports and counts. Also the runner's tests 120, the close tool's 29, the call-site self-test.
- [x] 4.2 Documents: `tools\legacy_tests\README.md` (the build cache, "Locks", "The compiler a run tests", "The
  runs at a merge into `main` in a worktree"), the runner's docstring, `tools\close\README.md`, `crates\README.md`,
  `CLAUDE.md` working rule 9, `DECISIONS.md`, `study\34`, `STATUS.md`.
