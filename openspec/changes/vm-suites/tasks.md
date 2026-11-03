# Tasks

Standing rules: CLAUDE.md. Different here: `create.ps1 -Iso` needs an elevated shell, so the user runs it
(task 2.4). Every other VM command runs without elevation.

## 1. Measure what the design assumes

- [x] 1.1 On the desktop VM's checkpoint (restore, then turn off, as a run does), check that `VCRUNTIME140.dll`
  is in the guest's `System32`, that `tar.exe` is there, and how long `Copy-Item -ToSession` takes for a 65 MB
  archive and for a tar of the clone without `.git`. Verify: the four facts are written under a new "Measured"
  heading at the end of `design.md`.
- [x] 1.2 On the same restored, running VM, hot-add a new empty VHDX to its SCSI controller. Check that the guest
  sees it online, format it, mount its volume at a folder such as the host tree's parent, and check that the next
  restore leaves the disk detached and the folder gone. Verify: the steps and timings are under "Measured". If
  hot-add fails, apply the fallback of the design's risks, rewrite D4, and tell the user before going on.

## 2. Make the suites VM

- [x] 2.1 `create.ps1`: add `-Name` and `-Shape Desktop|Suites` (12 processors, dynamic memory 2 / 4 / 24 GB for
  `Suites`). Name the base `qb64rust-base`, mark it read-only once written, and refuse `-Iso` while a VM's disk
  is its child, naming the VM and the commands that remove it (D1). Verify: `create.ps1 -WhatIf`-style dry runs
  of the refusal on a VM with a child disk, made with a scratch VHDX pair, print the refusal and exit 2.
- [x] 2.2 A credential file per VM, with the desktop VM's fallback to the old path, in `create.ps1` and
  `run.ps1` (D2). Verify: `python tools/sandbox/run.py` passes with the existing desktop VM and its old file.
- [x] 2.3 For `-Shape Suites`: Python 3.12 from python.org pinned by SHA-256 (the same minor version as the
  host's 3.12.6) and installed for all users; the guest's new-disk policy set to online; `cache.vhdx` made,
  formatted in the guest and detached before the checkpoint; Mesa also copied to the cache disk; the host's
  `qb64rust.exe` copied in and `qb64rust toolchain install` run as `tester` while online (D3, D4, D7, D11). If
  task 1.1 found no `VCRUNTIME140.dll`, install Microsoft's redistributable, pinned by SHA-256, and add a
  `SOMEDAY.md` row for a static C runtime in `qb64rust.exe`. Verify: `tools\winvm\README.md` has a section for the
  suites VM: its shape, what it holds, the pins, the disk it takes and how to make it again after 90 days.
- [x] 2.4 The user runs `create.ps1 -Iso <iso> -Name qb64rust-suites -Shape Suites` from an elevated shell.
  Verify: `Get-VMSnapshot -VMName qb64rust-suites` lists `ready`, `(Get-VHD <its disk>).ParentPath` is the base,
  `qb64rust toolchain path` in the guest says `installed:`, and the disk used by the base, the child, the checkpoint and `cache.vhdx` is written under "Measured".
- [x] 2.5 Add a `DECISIONS.md` row: a second VM for the program steps, on a differencing disk of a shared base,
  with a cache disk without a build cache (user, 2026-11-03: "A second, lean VM"), with its *Instead of*.
  Add `docs\LAYOUT.md` rows for the new files. Verify: `python tools/repo_check/check_repo.py` passes.

## 3. Start, fill and stop the suites VM

- [x] 3.1 `tools\winvm\suites.ps1 -Check` and `-Stop`, with the refusals of D6 and `QB64RUST_SUITES_VM` naming
  another VM. Verify: `-Check` exits 0 with the VM off, and 2 with a message when it is running or the name is
  unknown.
- [x] 3.2 `suites.ps1 -Start -Tree <path>`: restore, start, session, clock and license checks, then add and mount
  the cache disk at the tree's parent (D4, D5). Verify: after `-Start`, a guest `Test-Path` of
  `<parent>\QB64pe` and `<parent>\<tree>` is true; a file written to the guest's `C:\` is gone after `-Stop` and
  another `-Start` (spec: "A file left by a program").
- [x] 3.3 The tree's archive from `git ls-files -co --exclude-standard`, unpacked over the copy with the files
  dropped from the last list removed, and the clone refreshed by its stamp (D3). Verify: a file added, one edited
  and one deleted on the host are the same in the guest after the next `-Start`; `target\` in the copy is kept;
  the time of a `-Start` with nothing changed is under "Measured".

## 4. OpenGL for the programs in the VM

- [x] 4.1 List the tools that start built programs (the runner, `press_any_key.py`, `tools\coverage\coverage.py`,
  `tools\callsite\callsite.py` and any other found by searching for where an executable is started). Add the hard
  links of `QB64RUST_OPENGL_DIR` to each, through one helper in the runner (D7). Verify: a test in
  `tools\legacy_tests\test_run_legacy_tests.py` that the links are made with the variable set, and nothing is
  made without it; `python tools/check/tool_tests.py` passes.

## 5. A step in the VM, and `--where`

- [x] 5.1 `tools\winvm\step.ps1` (D6, D11): the toolchain guard before the command, the scheduled task in
  `tester`'s session, the log copied every 10 s, the outputs copied at the end and on a timeout, the exit code.
  Verify: with the guest's toolchain folder renamed for the test, the step fails at the guard and builds nothing;
  then, by hand, after `suites.ps1 -Start`, a `step.ps1` of `python tools/legacy_tests/run_legacy_tests.py
  --suite corpus --list` over three slice programs passes, and its log and results file are on the host.
- [x] 5.2 `steps.py`: `Step.vm`, `Step.outputs` and `Step.clears`; the whole suites' command as arguments, with
  `clears` in place of `rm -f`; `v37-libs` run in the guest before `verification` (D8, D9). Verify: a test in
  `test_check.py` that each `vm` step has `outputs` naming its results file, and that the whole steps' commands
  are lists.
- [x] 5.3 `check.py`: `--where`, `place_of`, the VM session around the plan's VM steps, the fallback line and the
  refusal, the `cli` host part (`cargo test --no-run --message-format=json`), the place in the fingerprint and
  the record, `status` showing it, and the report line for the clone's toolchain on the host (D6, D9, D10, D11).
  Verify: tests in `test_check.py`, with `suites.ps1`, `step.ps1` and `toolchain path` replaced by fakes, for each
  scenario of the `testing/check-workflow` delta: the VM restored once for three steps, off after a failing one,
  the fallback line once without Hyper-V, the refusal with `--where vm`, a host pass not standing for a VM run,
  `quick` planning the same steps as before, and the one report line when the toolchain is the clone's.
- [x] 5.4 `tools\check\README.md`: where a step runs, `--where`, the defaults, the fallback, and what a VM run
  copies out. Verify: the README's commands run as written.

## 6. Runs in the VM

- [x] 6.1 `just check slice --where vm`. Verify: it passes against the same recordings as on the host;
  `s141_tcp_loopback` passed in it; no window appeared on the host. Write each failure that is a host dependency
  into its known list with the reason, or fix it, and tell the user.
- [x] 6.2 `just check cli --where vm`. Verify: the CLI tests pass in the VM, finding their files by the paths
  built into the binary (spec: "The CLI tests in the VM").
- [ ] 6.3 The user starts `just full` in the worktree (with the VM by default), and `just full --where host`
  after it. Verify: both pass; each step's time in both places is under "Measured". If the VM's full run takes
  more than twice the host's, ask the user about a build cache on the cache disk before the default stays on the
  VM. Also write each VM step's time beside its limit (`vm_limit` in `check.py`). Look again at any step that took
  more than half its limit: a pass helped by the pass cache sets a short time, and the 30-minute minimum is all
  that covers a cold rebuild after it.
- [ ] 6.4 Closing: `STATUS.md`'s "Next"; the `QB64pe behaviors to review` rows of `SOMEDAY.md` for anything kept
  as found; `just quick` in the background. Verify: `just quick` passes, or what failed and why is written for
  the user.

## Workflow follow-up

- `just merge` runs in the worktree at the merge into `main`, with its whole suites in the VM. Its coverage
  records and results must be on the host for `builtins`, `counts` and the `-list` steps.
- Archive the change after the merge.
