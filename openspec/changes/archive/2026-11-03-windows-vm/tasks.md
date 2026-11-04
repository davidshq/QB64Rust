# Tasks

Standing rules: CLAUDE.md. What this change does differently: some steps need the user, because they need
elevation, a restart or a download from Microsoft. Each says so and names the command for the user to run.

## 1. What the host allows (measured before anything is written)

- [x] 1.1 Measure, with a throwaway VM named `qb64rust-probe` (no OS, no disk) made and removed in the task: is
  the Hyper-V role and module on (`Get-WindowsOptionalFeature Microsoft-Hyper-V-All`, elevated)? Is the user in
  "Hyper-V Administrators"? Do `Get-VM`, `New-VM`, `Checkpoint-VM` and `Restore-VMSnapshot` work from a
  non-elevated shell? User steps if needed: turn on the feature and restart; add the user to the group and log
  off and on. Whether PowerShell Direct works without elevation is measured in 2.1, once a guest exists. Verify:
  the answers are written in `design.md` under "Context". If a cmdlet needs elevation, stop and ask the user
  (design D4, "Elevation").
- [x] 1.2 `DECISIONS.md`: a row for running the desktop family in a Hyper-V test VM (design D1, D3, D5). Its
  "instead of" names Windows Sandbox staying as `--where sandbox`. It notes that this reverses the "Hyper-V VM of
  our own" part of the 2026-10-30 row, and why (the user, 2026-11-02). Verify: the row is there and names
  `openspec\changes\windows-vm\design.md`.

## 2. Making the VM

- [x] 2.1 `tools\winvm\unattend.xml` and `tools\winvm\create.ps1` (design D2), with the refusal for an existing
  VM. `create.ps1` also sets the video resolution, the checkpoint type and the `ready` checkpoint (D3, D5). User
  step: download the Windows 11 Enterprise evaluation ISO from Microsoft's Evaluation Center, then run
  `create.ps1 -Iso <path>` from an elevated PowerShell. Then measure, non-elevated: does PowerShell Direct open a
  session with the saved credential? What resolution and display scale does the guest get? How long did it take
  to make, and how big are the VHDX and the checkpoint? Verify: `Get-VMSnapshot qb64rust-desktop` lists `ready`;
  a session reads `C:\ready`; the answers are written in `design.md` "Context"; `git grep` finds no host path or
  password under `tools\winvm\`. If PowerShell Direct needs elevation, stop and ask the user.
- [x] 2.2 `tools\winvm\README.md`: what the VM is for, what the host needs (the feature, the group, about 25 GB),
  how to make it and make it again after 90 days, how to remove it, where the credential is kept, and not to
  open its console in enhanced session mode. Verify: the commands in it run as written.

- [x] 2.3 Added 2026-11-03 (the user: option 1, and "keep a copy of never-booted disk"). `create.ps1` puts Mesa's
  software OpenGL in the guest's `C:\mesa` (pinned URL and SHA-256), keeps the never-booted disk as a base, and
  takes `-FromBase` (design D2, "OpenGL", "The base disk"). **Built; the measurement below is deferred** (the
  user, 2026-11-03: "defer 2.3, move into someday"). It is a `SOMEDAY.md` row, due when the VM's license runs
  out, about 2027-02-01. Mesa is in the current VM, put there by hand with the same files. Deferred: User step: remove the VM and run `create.ps1 -Iso`
  again from an elevated PowerShell, because the VM made in 2.1 has no base. Then measure: remove the VM, and
  `create.ps1 -FromBase` from a normal PowerShell. Verify: both make a VM whose `ready` holds `C:\mesa` and an
  activated license; the base's size is written in `design.md` "Context"; `tools\winvm\README.md` says both
  ways.

## 3. Running the family in the VM

- [x] 3.1 `tools\winvm\run.ps1` (design D4, D6): the refusals; restore, start, the heartbeat, the session, the
  clock check, the license check; the copy in; the interactive scheduled task; the wait; the copy out; the VM
  turned off in a `finally`; the exit codes 0, 1 and 2. `tools\sandbox\inside.ps1` takes `-Shutdown`, and
  `run.py`'s `.wsb` command passes it. Verify, by hand: one run with a probe program and one that never ends
  (as `desktop-sandbox` task 2.1 did). The second's `exit` reads `timeout`, `done` is written, and the VM is off
  afterwards. A second run does not find a file the first one left in `C:\`. Each refusal of D6 is seen once
  (the VM running; the VM renamed through `QB64RUST_VM`; a console window open).
- [x] 3.2 `tools\sandbox\run.py`: `--where vm|sandbox`, default `vm` (design D5). For `vm`: no display scale,
  `--scale` refused, `run.ps1` in place of the `.wsb` and `wait()`, its exit 2 passed on as a refusal. Then run
  `python tools/sandbox/run.py`, logged to `target\sandbox-vm.log`. A difference from a recording is a compiler
  or libqb bug, as in the sandbox. Or it comes from the machine (design, "Risks"); then the family is recorded
  again in the VM with `--record`, and each changed recording is read against what its program means to show.
  Verify: the run passes with nothing shown on the host's screen; `--where sandbox` still passes; the time from
  restore to turn-off is measured.
- [x] 3.3 `tools\sandbox\test_run.py`: `--where` chosen and its default; `--scale` refused for `vm`; the `.wsb`
  command passes `-Shutdown`; `run.ps1`'s exit codes 0, 1 and 2 mapped to a run, a timeout with the programs
  named, and a refusal. `run.ps1` is always faked; no test starts the VM or reads the display scale. Verify:
  `python -m pytest tools\sandbox\test_run.py` passes.
- [x] 3.4 `tools\sandbox\README.md` (the two places; the VM run takes no screen; how long a run takes) and
  `tests\sandbox\README.md` (where each recording was made, if 3.2 recorded any again). Verify: the commands in
  them run as written.

## 4. The cadence

- [x] 4.1 `tools\check\steps.py`: the `sandbox` step's inputs take `tools/winvm/**`. `test_check.py` holds the
  new glob to a file. Verify: `python -m unittest discover -s tools/check -p "test_*.py"` passes, and `just
  check sandbox` runs the family in the VM.
- [x] 4.2 Measure the VM run while another step loads the host (`just check verification`, started first). If it
  passes, drop `alone=True` from the `sandbox` step (design D5, "The `just full` step"). If not, keep it and
  write why in `tools\sandbox\README.md`. Verify: the result and the decision are in `design.md` "Context".
- [x] 4.3 `docs\LAYOUT.md`: a row for `tools\winvm\`, and the rows of `tools\sandbox\` and `tests\sandbox\` say
  where the family runs. `STATUS.md` "Next": this change's line. Verify: `python tools/repo_check/check_repo.py`
  passes.

## Workflow follow-up

- `just quick` in the background before the files are handed over to stage (`CLAUDE.md` rule 6).
- Archive the change once the user has reviewed it.
