# Tasks

Standing rules: CLAUDE.md. What this change does differently: the family runs in Windows Sandbox, one sandbox at a
time, and puts a window on the screen while it runs (design D3, D7); its recordings come from QB64pe's builds run in
the sandbox (design D4).

## 1. What the sandbox gives (measured before the runner is written)

- [x] 1.1 `DECISIONS.md`: a row for testing the desktop built-ins in Windows Sandbox (design D1) and the reason
  `sandbox` of `tests\never_run.list` (design D6), naming `openspec\changes\desktop-sandbox\design.md`. Verify: the
  row is there.
- [x] 1.2 Measure, with scratch files under `target\sandbox-probe\`: the names of the sandbox's processes on the host
  while it runs and after `shutdown /s` (for the refusal and the end of a run, design D3 steps 1 and 5); whether a
  build of `qb64pe.exe` runs in the bare sandbox, and if not which DLLs it lacks (`objdump -p`, design D4); whether
  a window started from the logon script is in front, by a probe that calls `_SCREENPRINT "abc"` into itself and
  reads `INKEY$` (design D5); whether `_SCREENCLICK` on its own window is read by `_MOUSEINPUT`. Verify: each
  answer, with the probe's output, is written in `design.md` under "Context", and D3–D5 are corrected where a
  measurement differs from what they assume.

## 2. The runner

- [x] 2.1 `tools\sandbox\inside.ps1` (design D3 step 4): runs `C:\run\list.txt`'s executables in order, each in
  its own `results\<name>\` with `QB64RUST_COVERAGE`, a per-program limit and a kill on timeout, writes each `exit`,
  then `results\done`, then shuts the sandbox down. Verify: run once in the sandbox by hand with the 1.2 probe
  executable and a program that never ends; the second's `exit` reads `timeout` and `done` is written.
- [x] 2.2 `tools\sandbox\run.py` (design D3): the refusals (no Windows Sandbox, a sandbox running), the builds with
  `qb64rust --coverage` or `qb64pe.exe` (`--record`, through `tools\legacy_tests\compiler_copy.py`), the `.wsb`
  written under `target\sandbox\run\`, the start and the wait with the run's limit, the judge against
  `tests\sandbox\<name>.out.txt` (and `.exit.txt`), `--record`, `--glob`, `--timeout`. Verify: the unit tests of
  2.3 pass, and `git grep` finds no host path written by it under `tests\` or `tools\`.
- [x] 2.3 `tools\sandbox\test_run.py`: the `.wsb` (each sharing turned off, the one mapped folder, no host path other
  than the one passed in), the refusals (with the checks for the program and the processes faked), the judge (equal,
  a difference with its diff, a missing recording, a timeout, a missing result), the coverage check of design D6
  (a `sandbox` row run, one not run, a recording run that skips it). Verify: `python -m pytest
  tools\sandbox\test_run.py` passes.
- [x] 2.4 `tools\sandbox\README.md`: what the runner does, what the host needs (the Windows feature, turned on how),
  that it takes the screen for about a minute and runs alone, the commands to record and to run. Verify: the
  commands in it run as written.

## 3. The family and its recordings

- [x] 3.1 The seven programs of design D5 in `tests\sandbox\`, each writing `out.txt` in its working folder and
  printing nothing that follows the sandbox's window size (design D2), with the fallback of D5 for `_SCREENPRINT`
  and `_SCREENCLICK` if 1.2 found the foreground cannot be had. Verify: each compiles with `qb64rust` and with
  `qb64pe.exe`.
- [x] 3.2 Record: `python tools\sandbox\run.py --record`. Read every recording against what the program meant to
  show (a value read back, not a value of the machine). Verify: seven `.out.txt` files, each holding the values
  read back; `qb64pe.exe`'s builds exited 0, or an `.exit.txt` says why.
- [x] 3.3 Run: `python tools\sandbox\run.py`, logged to `target\sandbox.log`. A difference from a recording is a
  bug of the compiler or of libqb's copy, fixed in this change or written as a known bug in `STATUS.md` with the
  program kept failing. Verify: the run passes, or each failure has its `STATUS.md` line.
- [x] 3.4 `tests\sandbox\README.md`: what each program shows, one line each, and how its recording was made (QB64pe's
  build in the sandbox, the date, the clone's commit). Verify: one line per program, the commit named.

## 4. The coverage list and the cadence

- [x] 4.1 `tests\never_run.list`: the seven rows take the reason `sandbox` (the note keeps their design reference);
  the header names the reason. `REASONS` in `tools\coverage\builtin_runs.py` and `NEVER_RUN_REASONS` in
  `crates\driver\tests\inputs.rs` take `sandbox`; their tests have a `sandbox` row (`test_builtin_runs.py`, the
  `never_run` test of `inputs.rs`), and `builtin_runs.py`'s docstring and `tools\coverage\README.md` name it.
  Verify: `python -m pytest tools\coverage\test_builtin_runs.py` and `cargo test -p qb64rust-driver --test inputs`
  pass, and `run.py`'s coverage check passes on 3.3's run.
- [x] 4.2 `tools\close\close.py` (design D7): the step `sandbox`, alone and last, in the plan when `tests\sandbox\`,
  `tools\sandbox\` or `tests\never_run.list` changed or `--sandbox` is passed, with the line saying why it is left
  out otherwise; `--check` reads its log. Tests in `tools\close\test_close.py`. `tools\close\README.md`, "The test
  cadence": the closing set's line and the merge set's. Verify: `python -m pytest tools\close\test_close.py` passes
  and `close.py --plan` on this change lists the step.
- [x] 4.3 `docs\LAYOUT.md` rows for `tests\sandbox\` and `tools\sandbox\`; `SOMEDAY.md` rows for the `printer`,
  `keyboard` and `person` rows of `tests\never_run.list` in the sandbox, and for moving `verification\v49_d_clipboard`
  and `slice\s150_builtins_more` (the clipboard race of `STATUS.md`'s known bugs) into it. Verify: `python
  tools\repo_check\check_repo.py` passes.

## 5. Close

- [x] 5.1 `STATUS.md`'s "Next": a line for this change, built, waiting on the user's review. Verify: the line is
  there.
- [x] 5.2 The closing runs: `tools\close\close.py --check` passes.

## Workflow follow-up

- The user's review, then `/opsx:archive`.
