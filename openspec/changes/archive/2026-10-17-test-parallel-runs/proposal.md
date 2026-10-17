# Proposal

## Why

At the close of `m3-events` the closing runs took about 35 minutes of machine time, one after another, and
waiting for test runs is 18% of all active time (`study\32` §1). The user answered `study\32` §5 (2026-10-17: "1.
Yes, adopt 2. Yes 3. Yes. Do it"; `DECISIONS.md`):

1. A lighter close, the full set at each merge into `main`: in `CLAUDE.md` working rule 9 (no code). The close runs
   the slice and upstream pass lists with `--pass-cache`, but the runner takes `--pass-cache` with the corpus
   suite only; the upstream pass list is the compile suite.
2. Runs of the new compiler side by side: today every run takes one lock, `target\qb64pe-clone\in-use.lock`,
   alone, though a build of the new compiler shares only libqb's objects with the clone (`study\32` §2).
3. A program that failed beside other builds runs again alone in the runner, instead of a person doing it by hand
   (`u02_mouse_text` at the close of `m3-events`).

## What changes

- **The clone's lock becomes a readers-writer lock** (`tools\legacy_tests\clone_lock.py`): a run that uses
  `qb64pe.exe` in the clone takes it alone, as now; a run that builds with the new compiler only (the runner with
  `qb64rust`, the CLI tests that build) takes it shared.
- **The driver locks the libqb objects around each `make`** when `QB64RUST_OBJECTS_LOCK` names a lock file:
  shared when `make -q` says the objects of the build's flags are up to date, alone otherwise. The runner and the
  CLI tests set it. This replaces `study\32` §2's warm-up (design D1).
- **The runner runs a failure again alone** with `qb64rust` and `--jobs` above 1, as it already does for `qb64pe`
  with its copies; a pass alone does not fail the run when the program is timing-sensitive (design D3).
- **`--pass-cache` works with the compile suite** (design D4).
- Documents: `tools\legacy_tests\README.md`, `crates\README.md`, `CLAUDE.md` working rule 9, `DECISIONS.md`,
  `study\32` §5, `STATUS.md`.

## What does not change

- The compiler's output: the C++, the executables and every recorded result are the same. Without
  `QB64RUST_OBJECTS_LOCK` the driver locks nothing, as before.
- The old compiler's runs: they still have the clone alone; `--jobs` with `qb64pe` still uses its copies.
- `callsite.py` keeps the lock alone: its old side runs `qb64pe -z` in the clone.

## Capabilities

- `testing/golden-corpus`: three added requirements (the clone's lock, the run again alone, the pass cache of the
  compile suite).
