# Tasks

Standing rules: `CLAUDE.md` (no subagents, nothing staged, backslashes through the tools, test cadence rule 9).

## 1. The cadence (no code)

- [x] 1.1 `CLAUDE.md` working rule 9: the close of `study\32` §4 item 1, the full set at each merge (item 2), the
  order of the close (item 5); `crates\README.md`'s "When to run what"; `DECISIONS.md`; `study\32` §5 answered.

## 2. The locks

- [x] 2.1 `clone_lock.py`: `lock_file` with a shared mode (`LockFileEx` over the whole file, `fcntl.flock`
  elsewhere; design D2); `lock_clone(shared=True)` names `objects.lock` in `QB64RUST_OBJECTS_LOCK`.
- [x] 2.2 The driver's `lock_objects` (`crates\driver\src\build.rs`, design D1): `make -q` on the objects of the
  build's flags under a shared lock, the build alone when they are not up to date.
- [x] 2.3 The runner takes the clone's lock shared for `qb64rust`; the CLI tests take it with `try_lock_shared` and
  give every compiler `QB64RUST_OBJECTS_LOCK`.
- [x] 2.4 Tests: `test_clone_lock.py` (two shared holders, shared against alone both ways, closing gives it back,
  the folder lock alone, the variable named only for a shared run). Checked by hand (design D1, D2): `make -q`
  0 for a built flag set, 1 for one never built and for a source pretended newer; a build waits for a holder alone
  and not for a shared one; a `qb64pe` run stops with exit code 3 beside the CLI tests, a `qb64rust` corpus run
  goes ahead beside them; 0.12 s per build that runs `make`.

## 3. The run again alone and the compile suite's pass cache

- [x] 3.1 `run_again_alone` and `timing_sensitive` (design D3), used for `qb64pe` with its copies as before and for
  `qb64rust` with `--jobs` above 1; tests (`RunAgainAlone`, 6).
- [x] 3.2 `--pass-cache` with the compile suite (design D4); tests (`CompilePassCache`, 3).
- [x] 3.3 After the review: a `qb64rust` failure at a stage where it cannot pass alone (`SETTLED_STAGES`, design
  D3) is not run again; tests (`RunAgainAlone`, now 8). The runner's tests: 112 of 112. Nothing else changes
  what a run of a list or the call-site check sees, so the closing runs of 4.1 stand.

## 4. Close

- [x] 4.1 The closing runs (working rule 9 as now written): tier 1 with fmt and clippy; the runner's tests, 110;
  the CLI tests that build, 37 of 37; the slice list 446 of 446 and the upstream pass list 336 of 336 with
  `--pass-cache` and `--build-cache`, started together, neither waiting for the other (the slice list in 1.8 min;
  nothing was skipped, as the cache held no pass from these executables yet); the call-site check 1864 of 1864
  equal, after them. No run without folding (`sema` untouched).
- [x] 4.2 Documents: `tools\legacy_tests\README.md` ("Locks", `--pass-cache`), the runner's docstring and help,
  `STATUS.md`.
