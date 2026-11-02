# Design

## Context

See `proposal.md`, "Why". What the code does now:

- **The runner applies the known lists during a run.** `run_legacy_tests.py`'s `main` builds `known` from
  `load_known_failures` (`tools\legacy_tests\known_failures.txt` and the suites' `known.list`) and, with
  `--platform linux`, `load_linux_list` of each suite's `known.linux.list`. As each result comes in, a `FAIL` whose
  key is in `known` becomes `KFAIL` with ` (known: <reason>)` added to its detail. A reason starting with
  `windows` skips the program (status `SKIP`, never run). The results file (`results-<suite>.json`) holds the run
  record (`run`: compiler and runner hashes, roots, options, `started`, `ended`), every result (`suite`, `test`,
  `kind`, `status`, `stage`, `detail`, `seconds`) and the summary.
- **`check` plans by fingerprint.** `tools\check\check.py`'s `make_plan` lets a step's record stand when it
  passed and its fingerprint (`inputs_of`: the command, every input file by hash, the externals, the needs'
  fingerprints) is unchanged; a failed record never stands; `--force`, and so `check merge`, runs every step. The
  record keeps each input file's hash (`files`).
- **The whole suites.** `steps.py`'s `whole` (Windows) and `linux_whole` (WSL, through `tools\linux\run.sh`) run a
  suite in full with no pass cache. `counts.py` and `--check-list` refuse a run with `--pass-cache`, so the
  Windows whole suites must stay plain runs. Nothing reads the Linux results but the step's verdict.
- **Measured on 2026-11-02** (worktree, `linux-verification` of `7334117`): 4,308 programs ran, their run times
  adding up to 5,796 s at `--jobs 8` (13 min wall). 228 programs take more than 5 s; the longest are
  `v59_a5_console_only` (113 s) and `v33_d03_debug_window` (104 s). `upstream-whole` took 273 s at the merge and
  46 s run again with a warm build cache.

## Goals / Non-Goals

**Goals:**

- A list-only fix to a whole suite costs seconds, in `check <step>`, `check full` and `check merge`.
- `check merge` run again after a fix does not redo the steps the fix cannot change.
- `counts.py` takes seconds.

**Non-Goals:**

- Re-judging after any change other than the known lists (a recording, a sidecar, the runner, the compiler): those
  need a run.
- The pass cache for the whole suites: on Windows `counts.py` and `--check-list` need plain runs; on Linux, D4.
- Narrowing the whole suites to the programs a change reaches (the quick layer's narrowing stays as it is).

## Decisions

### D1. `--rejudge`: the runner applies today's lists to stored results

`run_legacy_tests.py --rejudge --suite <s> [--platform linux]` (new function `rejudge` beside `check_list`) reads
`results-<s>.json`, builds `known` exactly as `main` does (one helper, `known_lists(args)`, used by both, so the two
cannot drift), and for each result: strips a ` (known: …)` suffix and takes a `KFAIL` back to `FAIL`, then applies
`known` as the run loop does. Every other status stays as it is. It writes back the results, the summary, and
`run["rejudged"]` (the time, and the hashes of the list files read), prints the summary line and exits as a run
would (0, or 1 with a failure).

It refuses (exit 2, with the reason) when: there is no run record of this runner version; the compiler's hash is
not the run's; a program's inputs changed after `run["started"]` (`program_inputs`, as `check_list` does); or a
program that the lists now skip, or now no longer skip, has no matching result (a program un-skipped was never
run). `--check-list` and `counts.py` take a re-judged file as the run it came from: the run record's other fields
are unchanged, so the checks they make still hold, and the `KFAIL` rule of a pass list applies to the new statuses.

*Instead of:* re-running only the programs whose list rows changed (`--list`): a partial run writes a partial
results file, which `counts.py` and `--check-list` refuse, so the whole suite would still have to run.

### D2. `check` plans a re-judge from a run fingerprint that leaves out the known lists

A new tuple in `steps.py`, `JUDGE_LISTS`: `tools/legacy_tests/known_failures.txt`, `tests/*/known.list`,
`tests/*/known.linux.list`. `inputs_of` computes, beside the fingerprint, a **run fingerprint**: the same hash with
the files matching `JUDGE_LISTS` left out and the needs' run fingerprints in place of their fingerprints. A step
whose needs differ only in the lists (`linux-sync`, whose inputs hold the lists) then has the same run fingerprint.
`run_step` writes it into the record (`run_fingerprint`), for a pass and for a failure.

A whole-suite step (`Step.rejudge`, the runner command of the re-judge; set on the three Windows and three Linux
whole steps) is planned as **"re-judge: only the known lists changed"** (`check.py`, `can_rejudge`) when its record,
passed or failed, has a `run_fingerprint` equal to now's and a different fingerprint, in any layer, by name and in
the merge; `--force` still runs every step. With nothing changed a failed step runs again, not re-judged: a failure
may have been the machine's, and a refused re-judge (results of another build) needs a run. The re-judge
runs as a step: its log, its record (with the new fingerprint, `"rejudged": true`), and the steps that need it
(`counts`, the list checks) see a new fingerprint and run. A failure never stands as it is (the spec's rule): it is
judged again under new lists, or run again.

*Instead of:* a separate command the user runs: the plan would still say "to run", and the user would have to
know the rule.

### D3. `check merge` lets the earlier merge's passes stand

`run_step` records `"merge": true` when the plan is the merge's. Under `check merge`, a step stands when its record
passed with `"merge": true` and the same fingerprint; the rest of `--force` stays (no pass cache; the exact Linux
sync, `linux-sync`'s forced form). A pass from `check full` or `check <step>` never stands for a merge (it may have
used the pass cache or narrowing). So a merge failed on a known list, fixed and committed, runs again as: the
writers (`update-lists`, `wording`, which find their output unchanged and stand), the sync, the re-judge of the
failed suites, and the steps that need them. `--force` alone keeps running everything.

### D4. Not built: the Linux pass cache

The proposal's first form gave the Linux whole suites the pass cache outside the merge. Dropped (the pragmatic
engineer, 2026-11-02): with D1 and D2 a list-only fix needs no run, the merge would not use the cache, and it would
be a second cache path on Linux that nothing exercises. A `SOMEDAY.md` row.

### D5. `counts.py`: list each folder once

Measured ("Measured"): `check_list` calls `program_inputs(suite, bas)` for each program of the three pass lists
(5,034 calls), and each call globs the program's folder (`verification\` holds about 15,000 files) for `<name>.*`,
so the folder is listed thousands of times. `program_inputs` already takes
`names` (the folder's sorted file names, listed once) for `--list-only`; `check_list` passes it too (a dict of
folder to sorted names, built on first use). The same for the corpus's and upstream's folders.

### D6. The merge's Linux sync follows git after an earlier merge's passed

Added after the build (the pragmatic engineer's review; user: "implement linux sync fix", 2026-11-02). Under the
merge every step ran its `forced` form, so `linux-sync` compared every file by content (285 s at the merge of
`linux-sync-per-tree`) even when a list fix changed two files. Since `linux-sync-per-tree` each copy follows one
tree, and the usual sync, from the files git reports changed since the last sync, is exact for it (about 2 s). So
in the merge a step with a `forced` form whose record is a passed merge's runs its usual form (`Planned.usual`, set
in `make_plan`'s merge branch; `command_of` takes it): the first merge in a tree, and `--force`, still check by
content. *Instead of:* the exact copy at every merge, which the copy per tree made unnecessary.

## Measured

Before (2026-11-02, worktree at `df67128`, `python -m cProfile counts.py --check`): 202.7 s in all, of which
`check_list` 201.2 s, `program_inputs` 199.6 s over 5,034 calls, `pathlib`'s glob 196.0 s (24,091 calls).
Everything else together: under 2 s. After task 1.1: `check_list` of the verification pass list 3.95 s (4,139 of
4,139, as before), the corpus's slice list 0.10 s (494 of 494), on the same results with the checkout's runner.

A re-judge (2026-11-02, checkout, after a whole run of the compile suite with this runner: 6 min 51 s, 401 pass, 3
known): with `noprompt/noprompt-continue-fatal`'s row taken out of `known_failures.txt`, `--rejudge` 0.60 s, that
program a failure, exit 1; with the row back, 0.59 s, 401 pass and 3 known, exit 0; `--check-list
tests/upstream/pass.list` on the re-judged file 0.65 s, 401 of 401. No program ran.

## Risks / Trade-offs

- **A list that is read but not in `JUDGE_LISTS`** would make a re-judge miss a change. The re-judge reads lists
  only through `known_lists`; a test asserts that each file it opens matches `JUDGE_LISTS`.
- **The runner's hash changes with this change**, so every stored result stops counting once: the first `just full`
  after it runs everything (the proposal's "Impact").
- **A re-judged failure stays a failed run underneath**: a program that failed and is now listed known passes the
  step, as a run would make it. That is what a run would give too; the known list is the decision, reviewed as
  code.
