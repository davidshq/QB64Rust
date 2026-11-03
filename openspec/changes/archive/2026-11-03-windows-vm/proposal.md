# Proposal

## Why

The sandbox family (`tests\sandbox\`, seven programs) runs in Windows Sandbox. The sandbox's window takes the
screen for about a minute, so the run has to happen alone, with the mouse and keyboard left untouched. A recording
run also changes the host's display scale to 100% and puts it back afterwards. The user asked for a Windows VM that
is made once and kept, not rebuilt for each run, sized for the desktop tests only (about 22 GB). This reverses
part of the `DECISIONS.md` row of 2026-10-30, which turned down "a Hyper-V VM of our own" because of the image,
the license, auto-logon and an agent to keep running. The design answers each of those.

## What Changes

- **A test VM, made once by a script.** `tools\winvm\create.ps1`, run once as administrator, builds a Hyper-V
  Generation 2 VM from a Windows 11 Enterprise evaluation ISO that the user downloads. Windows setup is never
  shown. The image is written straight to a new virtual disk with an answer file. The VM has a local test
  account that logs on by itself, and a fixed display. It is online once, while it is made, to activate Windows,
  and has no network adapter after that. When it is ready, the script saves a
  checkpoint named `ready`, taken while the account is logged on and idle.
- **Each run starts from `ready`.** The runner restores the checkpoint, which takes seconds and resumes at the
  logged-on desktop. It copies the executables in, runs them in the logged-on session and copies the results
  out. Then it turns the VM off. The VM is never rebuilt for a run, and no run sees what an earlier run left.
- **`tools\sandbox\run.py` gains a choice of where the family runs**: `--where vm` (the new default) or
  `--where sandbox` (Windows Sandbox, as now). Building, judging, recording and the coverage check stay the same
  for both. Only starting, copying and waiting differ. In the VM nothing appears on the host's screen, and the
  host's display scale is never changed.
- **The VM's lifetime.** The evaluation license lasts 90 days. When it runs out, `create.ps1` is run again. The
  runner refuses with a message that says so, and the user can still pick `--where sandbox`.

Out of this change: moving other programs into the VM (the clipboard race of `verification\v49_d_clipboard`,
the screen programs); building `qb64rust` or running whole suites inside the VM; the check on a machine without
QB64pe (`SOMEDAY.md`).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `testing/desktop-sandbox`: the family may run in the test VM as well as in Windows Sandbox. The requirements
  that the run is closed to the host, and on when a run cannot start, change to cover the VM.

## Impact

- New: `tools\winvm\` (`create.ps1`, the answer-file template, the script that drives a run in the VM, a
  README).
- Changed: `tools\sandbox\run.py` (`--where`), `tools\sandbox\inside.ps1` (shuts down only in the sandbox),
  `tools\sandbox\test_run.py`, `tools\sandbox\README.md`, `tests\sandbox\README.md` (where the recordings were
  made), `docs\LAYOUT.md`, `DECISIONS.md`.
- No change to the compiler or libqb.
- The host needs Hyper-V turned on, the user in the "Hyper-V Administrators" group, and about 37 GB of disk
  (measured 2026-11-03, with the kept base disk; `tools\winvm\README.md`).
  Nothing private goes into the repo: the VM's disk is in Hyper-V's default folder, and the test account's
  password is kept under the user's profile, encrypted for that user.
