# Proposal

## Why

Only the desktop family (`tests\sandbox\`, seven programs) runs in the Hyper-V test VM. Every other step that runs
programs runs on the host. The user wants a choice of where these steps run, with the VM as the usual place
(user, 2026-11-03: "We should have options for what runs on the VM, and everything is probably ideal").

The host keeps two problems the VM solves:

- **Windows on the host's screen.** The CLI tests (`cli`, the one step marked to run alone) open program windows on
  the host's desktop. The screen programs of the corpus run on a hidden desktop, but still on the host.
- **Hidden dependencies on the host.** A test can pass because of something the host happens to have. Until
  2026-11-03 `qb64rust` built every test here with the toolchain of the `..\QB64pe` clone, its fallback, because
  `qb64rust toolchain install` had never been run. It has been run now (user, 2026-11-03). Nothing noticed the
  fallback. And `qb64rust.exe` loads `VCRUNTIME140.dll`, which a clean Windows may not have. A clean VM that is
  restored before every run would show such dependencies.

## What Changes

- **A second test VM, `qb64rust-suites`, for the steps that run programs.** It is kept apart from the desktop
  VM, whose four processors the desktop family was measured with (user, 2026-11-03, over resizing the desktop VM).
  It is built lean, to take about 10 to 17 GB beyond the base disk:
  - its Windows disk is a differencing disk on the kept base disk of `create.ps1`;
  - it has dynamic memory, so that its checkpoint saves little memory;
  - a small cache disk is attached after each restore. It keeps the tree's copy, the QB64pe clone (without
    `.git`) and the pass caches between runs. It keeps no build cache.
  - It has Python, from a pinned release checked by SHA-256 as Mesa is. It has no Rust and no cargo.
  - It gets the C++ toolchain as a user does, with `qb64rust toolchain install`, while it is online to be made.
    The clone in the VM serves only `qb64pe.exe`'s parity runs. A step in the VM fails if `qb64rust` would build
    with the clone's toolchain.
- **`check` says which toolchain the program steps used.** On the host, a build with the clone's toolchain
  prints one line in the run's report, so the fallback cannot go unnoticed again.
- **`check` gets a place for each step: `--where host` or `--where vm`.** The steps that can run in the VM are
  those that run programs: `slice`, `upstream`, `verification`, `differential`, `nofold`, `qbasic`, `addprefix`,
  `format`, `format-corpus`, `callsite`, `coverage` and `cli`, and the merge's whole suites. `fmt`, `build`,
  `clippy`, `nextest`, `lists`, `doc`, `tools` and the Linux steps stay on the host.
- **Defaults.** `check quick` runs on the host. `check full` and `check merge` run their VM steps in the VM. When
  the VM cannot be used (no Hyper-V, no VM, CI, Linux), `full` and `merge` run those steps on the host and print
  one line saying so. With `--where vm` given, they refuse instead.
- **The place is part of a result.** A pass on the host does not stand for a run in the VM, and the other way
  round.
- **How a step runs in the VM.** The host builds `qb64rust.exe` and the CLI test binary. The tree, those files and
  the step's command go into the VM. The step runs in the test account's logged-on session. Its log is copied out
  while it runs, and its results are copied out when it ends. The VM is restored once per `check` run and turned
  off at its end.
- **`create.ps1` makes either VM.** It gets the VM's name and shape as parameters. Each VM gets its own credential
  file, where today there is one for all.

Out of this change:
- running `fmt`, `clippy`, the build or the Rust unit tests in the VM;
- moving the desktop family to the new VM;
- a fix for `VCRUNTIME140.dll` if the VM shows it is missing. That fix is a decision for the product, not for
  testing. The VM then gets the runtime installed, and a `SOMEDAY.md` row records it.

## Capabilities

### New Capabilities

- `testing/suites-vm`: the second test VM: how it is made, what it holds, what persists between runs, and how a
  step runs in it and returns its results.

### Modified Capabilities

- `testing/check-workflow`: a step has a place, host or VM. `full` and `merge` default to the VM for the steps
  that run programs. The place is part of the fingerprint. The VM's absence makes the run fall back to the host,
  or refuse when the VM was asked for.

## Impact

- New: `tools\winvm\step.ps1` (runs one step in the VM), a cache-disk setup in `tools\winvm\`, Python's pin in
  `tools\winvm\README.md`.
- Changed: `tools\winvm\create.ps1` (name, shape, differencing disk, credential per VM), `tools\check\check.py`
  (`--where`, the VM session of a run), `tools\check\steps.py` (which steps may run in the VM, and what each copies
  out), `tools\check\test_check.py`, `tools\check\README.md`, `tools\winvm\README.md`, `docs\LAYOUT.md`,
  `DECISIONS.md`.
- The runner and the tools that start programs get one addition: when a variable names Mesa's folder, they link
  its OpenGL DLLs beside each program, which a VM without a GPU needs. Nothing else changes in the compiler,
  libqb, the runner's judging or any recording.
- The host needs about 10 to 17 GB more disk, plus about 12 GB for the base disk. The desktop VM needs that base
  anyway at its next remake, about 2027-02-01. 176 GB were free on 2026-11-03.
- Full runs in the VM have 12 processors, not the host's 16, and no build cache. They may take longer. This is
  measured in the tasks.
