# Design

## Context

See `proposal.md` for why. What the design rests on, as found on 2026-11-03:

- **`check` runs its steps one after another** (`tools\check\check.py`, `execute` and `run_step`). Each step is a
  command from `tools\check\steps.py`, as arguments or, for the WSL steps and the merge's whole suites, as a bash
  line. Its record goes to `target\check\<step>.json` with a fingerprint of its inputs.
- **The desktop VM's run** (`tools\winvm\run.ps1`): restore the checkpoint `ready`, start the VM, open a
  PowerShell Direct session as `tester`, copy files in with `Copy-Item -ToSession`, run a script in `tester`'s
  logged-on session through a scheduled task, poll for a `done` file, copy results out, turn the VM off. It takes
  about 6 s from the restore to the desktop.
- **`create.ps1`** writes the image to a new disk, keeps the never-booted disk as the base
  (`qb64rust-desktop-base\` in Hyper-V's disk folder), and keeps one credential file,
  `%LOCALAPPDATA%\qb64rust\winvm\credential.xml`. The desktop VM made on 2026-11-03 has no base: it came before
  the base was added. `-FromBase` has never run.
- **What the program steps read outside the tree.** The `..\QB64pe` clone (3.0 GB, 1.6 GB of it `.git`), for
  `qb64pe.exe`. Until 2026-11-03 it was also the toolchain `qb64rust` built with here, its fallback (`qb64rust
  toolchain path`: "clone: the toolchain of the QB64pe clone next to the repository"), because `qb64rust toolchain
  install` had never been run. It was run on 2026-11-03 with the user's agreement, and installed
  `llvm-mingw-20260922-ucrt-x86_64` under `%LOCALAPPDATA%\qb64rust\toolchain`. The two are byte for byte the same
  release (`DECISIONS.md` 2026-10-26), so no result changes. The build cache
  `target\build-cache` is 13 GB. The slice's pass cache `target\pass-cache-2` is 5.5 MB. The tracked tree is 65 MB.
- **The runner and the Python tools use the standard library only.** `run_legacy_tests.py` calls no `git`.
- **The CLI tests** (`crates\driver\tests\it`) find files with `env!("CARGO_MANIFEST_DIR")` (23 uses) and the
  compiler with `CARGO_BIN_EXE_qb64rust`, both absolute paths fixed when the test binary is built.
- **`qb64rust.exe` is built for `x86_64-pc-windows-msvc`** and loads `VCRUNTIME140.dll`.
- **The screen check reads the screen from libqb's memory** (`tools\screen\screen_state.cpp`), not from what
  OpenGL draws. Software OpenGL in the VM does not change a `.screen` file. Every program that opens a window still
  needs an OpenGL that can make one: Mesa's, as in the desktop VM.
- **The host:** 16 cores, 63 GB of memory, 176 GB free on `C:`, Hyper-V's disk folder on `C:`.

## Goals / Non-Goals

**Goals:**
- A VM step reads and judges exactly as on the host: the same runner, lists, recordings and `--jobs`.
- One restore per `check` run, however many VM steps it has.
- The host's files and caches are never written by a VM run, except the step's log, results and coverage
  records, which are copied out.

**Non-Goals:**
- Running `cargo` in the VM. Nothing is compiled from Rust there.
- Sharing the suites VM with the desktop family, or running the two at the same time.
- A build cache in the VM (see D4 and the risks).

## Decisions

### D1. A second VM on a differencing disk of the base

`qb64rust-suites` is a Generation 2 VM whose Windows disk is a differencing disk (`New-VHD -Differencing`) whose
parent is the base disk. Only what changes after the first boot is stored in it. Its shape is 12 processors and
dynamic memory: 4 GB at start, 2 GB at least, 24 GB at most. A standard checkpoint of an idle VM with dynamic
memory saves only the memory in use, not 24 GB.

*Instead of:* resizing the desktop VM (the user chose a second VM, 2026-11-03); a full copy of the base, 18 GB
like the desktop VM's disk; static memory, whose checkpoint saves all of it.

A differencing disk breaks if its parent changes. So `create.ps1` makes the base file read-only after writing it,
and `-Iso` refuses to write a new base while a VM has a disk whose parent it is. It names that VM and the commands
that remove it.

There is no base yet (Context). The base gets a name of its own, `qb64rust-base\qb64rust-base.vhdx` in Hyper-V's
disk folder, not one VM's. `create.ps1 -Iso <iso> -Name qb64rust-suites -Shape Suites` writes the base and then
makes the suites VM's disk as its child. At the desktop VM's remake (about 2027-02-01) it is made from the same
base with `-FromBase`. `create.ps1` reads the old `<name>-base` folder too, in case one exists.

*Code:* `tools\winvm\create.ps1`: new parameters `-Name` (default `qb64rust-desktop`, or `QB64RUST_VM` as now)
and `-Shape Desktop|Suites`. `-Shape Suites` always makes a differencing disk. Also the base's read-only mark
and the refusal.

### D2. A credential file per VM

The credential moves to `%LOCALAPPDATA%\qb64rust\winvm\<name>\credential.xml`. For `qb64rust-desktop`, the old
path is read when the new one is missing, so the existing desktop VM keeps working. The base disk holds a
password in its answer file, so every VM made from it has that password. `-Iso` keeps the base's credential as
`%LOCALAPPDATA%\qb64rust\winvm\qb64rust-base\credential.xml`, and each VM made from the base gets a copy of it
under its own name.

*Instead of:* one shared file, which a second `-Iso` would overwrite, locking the first VM out.

*Code:* `create.ps1` and `run.ps1` (`$credentialFile`), and the new `suites.ps1` (D5).

### D3. What goes into the VM, once and per run

| What | Where in the VM | When |
|---|---|---|
| Python 3.12, the python.org installer, pinned by SHA-256, installed for all users on `PATH` | the Windows disk | once, by `create.ps1 -Shape Suites`, while the VM is online |
| Mesa's two DLLs | `C:\mesa`, and on the cache disk for hard links (D7) | once, by `create.ps1`, as for the desktop VM |
| The C++ toolchain, by `qb64rust toolchain install` as `tester` (D11) | `tester`'s `%LOCALAPPDATA%\qb64rust\toolchain` | once, by `create.ps1 -Shape Suites`, while the VM is online |
| The QB64pe clone, without `.git` | the cache disk, `<parent>\QB64pe` | when the host clone's commit and dirty state differ from the copy's stamp |
| The tree | the cache disk, `<parent>\<tree folder>` | every run |
| `target\release\qb64rust.exe`, and for `cli` the test binary and `target\debug\qb64rust.exe` | the tree's copy | every run, after the host's build |

The installer, not the embeddable zip: the embeddable Python's `._pth` file keeps a script's own folder off
`sys.path`, and the tools import their neighbors.

Files go in as one `tar` archive each (Windows ships `tar.exe`), copied with `Copy-Item -ToSession` and unpacked
in the guest. Copying 24,000 small files one by one through PowerShell Direct would take minutes.

The tree's files are `git ls-files -co --exclude-standard`, as the Linux sync uses: tracked files and untracked
ones git does not ignore. The guest deletes the files of the last run's list that are not in this one, and nothing
else. So `target\` in the copy, with the pass caches, is kept. The archive is the whole tree every run (about
65 MB, seconds), not the changed files as the Linux sync does. That is simpler, and the measured cost decides
whether to change it (task 3.3).

*Code:* `tools\winvm\suites.ps1` (D5), and `tools\check\check.py` writes the archive.

### D4. The cache disk

A dynamic VHDX of 64 GB, `cache.vhdx` in the VM's disk folder, about 3 GB used: the clone (1.4 GB), one or two
tree copies (65 MB each, the checkout and the worktree), their `target\` folders without a build cache, and Mesa.
`create.ps1` makes it, attaches it once to format it in the guest (NTFS), and detaches it before the checkpoint
is taken. The guest's new-disk policy is set to online, so a disk added later is online at once.

Every run hot-adds it to the running VM's SCSI controller after the restore. The disk's contents are never part of
a checkpoint. A restore does not detach it, though (task 1.2): a drive added to a running VM stays in its settings
after the VM is turned off and after the next restore. So `suites.ps1 -Stop` removes the drive once the VM is off,
and `-Start` removes one left by a killed run before it restores.

Turning the VM off is a power cut for the guest. So `-Stop` first takes the cache disk offline in the guest, which
writes out what Windows still holds of it, and only then turns the VM off. When there is no session to do that
(a killed run), the disk is turned off as it is, and NTFS's journal keeps its folders sound. A file being written at
that moment may be lost or cut short. A pass-cache entry is only a marker whose presence means a pass, so a lost
one is a miss and a cut one still marks a real pass.

The runner's build cache (`--build-cache target\build-cache`) is not used in the VM: `check.py` sets
`QB64RUST_NO_BUILD_CACHE=1` in every VM step, and the runner then ignores `--build-cache`. A variable, not an edit
of the command, because `coverage.py` starts the runner with `--build-cache` itself. Its 13 GB on the host are
builds of every program variant. The pass cache serves what passed, so only changed or failing programs build,
and they build cold.

*Instead of:* a cache disk with a build cache (about 15 GB more); no cache disk (every run builds every program).

*Code:* `create.ps1` (makes and formats it), `suites.ps1 -Start` (adds and mounts it).

### D5. The tree at the host's path

The guest mounts the cache disk's volume at the host tree's parent folder. With the tree at
`<parent>\QB64Rust-full` on the host, the guest has `<parent>\QB64Rust-full` and `<parent>\QB64pe`, both on the
cache disk. The mount point folder is made on the Windows disk at each run, and the restore removes it again.

Paths built into host executables are then right in the guest: `CARGO_MANIFEST_DIR` and `CARGO_BIN_EXE_qb64rust`
in the CLI test binary, and the `..\QB64pe` the parity steps give `qb64pe.exe`. The parent path is given to the guest when the
run starts and is never written into the repo.

*Instead of:* a fixed guest path such as `C:\w`, with the CLI tests left on the host or rebuilt with remapped
paths (`cargo nextest archive --workspace-remap`), which needs nextest in the guest and covers only the test
binary's own paths.

*Code:* `suites.ps1 -Start -Tree <path>`.

### D6. One VM session per `check` run

`tools\winvm\suites.ps1` has three forms:
- `-Check`: the refusals that need no VM started. Hyper-V usable, the VM and its checkpoint there, the VM off,
  the credential there. Exit 0 or 2, as `run.ps1 -Check`.
- `-Start -Tree <path>`: restore, start, wait for PowerShell Direct, check the clock and the license (as
  `run.ps1`), add and mount the cache disk, bring the clone and the tree up to date.
- `-Stop`: turn the VM off.

`tools\winvm\step.ps1 -Tree <path> -Job <file>` runs one step. The job is a JSON file that `check.py` writes in
`target\check\vm\<step>.json`: the command as arguments, a command to run first (`vm_first`), the working folder
in the tree, the step's environment, the host files to copy in (`target\release\qb64rust.exe`, and for `cli` its
binary and the debug build), the outputs, the files to clear, and the limit. A file, not arguments: Windows
PowerShell's `-File` passes no list. `step.ps1` opens its own PowerShell Direct session (about a second), copies
the job's files in, and writes a batch file that runs the command with its output to a log in the guest. It
registers a scheduled task that runs that file in `tester`'s logged-on session, and polls for its exit code
file. Every 10 s it prints what the guest's log gained since, so `check`'s log of the step shows its progress. At
the end, or past the limit, it copies the step's outputs (D8) out, ends the task and any process it left, and
exits with the command's exit code, or 1 for a timeout.

`check.py`'s `execute` calls `suites.ps1 -Start` before the first VM step of the plan and `-Stop` in a `finally`
after the last. A VM step's command is then `step.ps1` wrapped around the step's own command. The step's limit is
three times the time of its last whole pass in the VM, at least an hour (30 minutes until task 6.3, which found a
cold `verification` at 26.5 minutes). A step that never passed in the VM
takes its last whole pass on the host instead. A pass on the host does not change the VM's time, since the host
keeps a build cache and is quicker. A failure or a narrowed run changes neither time, so a step that failed in a
second does not cut the next run's limit. `check.py` stops `step.ps1` itself 10 minutes
after the limit, in case Hyper-V's cmdlets hang around the step. Before a VM step, the host's copies of its cleared
files and results files are removed, so that a step that brings nothing back leaves nothing old for later steps.

A run killed before its `finally` leaves the VM running. The next run's `-Check` then refuses with the command
that turns it off. It never takes over a running VM.

*Instead of:* one long-lived PowerShell process holding a session, fed commands through a pipe: less waiting, but
a protocol of its own to test. A restore per step: about 30 s each, ten times in a full run.

*Code:* `tools\winvm\suites.ps1`, `tools\winvm\step.ps1`; `tools\check\check.py`, `execute` and `run_step`.

### D7. OpenGL for every program in the VM

The runner and the tools that start built programs link Mesa's two DLLs beside each executable before it runs
when the environment variable `QB64RUST_OPENGL_DIR` names a folder holding them. `step.ps1` sets it in the guest.
They are hard links from the cache disk's copy of Mesa: a link is instant and takes no space, where a copy of
`libgallium_wgl.dll` beside thousands of programs would not. Without the variable nothing changes, so the host and
Linux are untouched.

*Instead of:* replacing `System32\opengl32.dll` in the guest, which Windows protects; a copy per program, as the
desktop family's `inside.ps1` does for seven programs.

*Code:* `tools\legacy_tests\run_legacy_tests.py`, `link_opengl`, called just before each of the runner's five
starts of a built program: the compile suite's test, `addprefix`'s converter, the corpus's run (after the folder
is reset for each run), and the verification suite's run and its recording. Task 4.1 found no other tool that
starts a built program: `press_any_key.py` is started by the runner after the links are made, `coverage.py` runs
the runner, and `callsite.py` only has the C++ written (`-z`), never built.

The CLI tests start built programs themselves, mostly through `support::unattended` in `crates\driver\tests\it`,
which the runner's links do not reach. In task 6.2 four of them failed in the VM for want of an OpenGL that
could make a window. So `support::link_opengl`, called by `unattended`, does what the runner's `link_opengl`
does, under the same variable (user, 2026-11-03, over replacing the VM's `System32\opengl32.dll` or keeping `cli`
on the host). The test `a_build_changes_nothing_git_sees` compares `git status` of the tree and the clone, which
in the VM are copies without `.git`, and the VM has no git: it says so and returns when `QB64RUST_IN_SUITES_VM`
is set, which `step.ps1` sets (user, 2026-11-03, over installing git in the VM).

### D8. What a step copies out

A step gets two new fields in `steps.py`:
- `outputs`: globs of the files that host steps after it read, copied from the guest's tree to the host's at the
  step's end. They are the runner's `results-<suite>.json` under the step's `--results` folder, the coverage
  record under `target\coverage\merge\`, and the folders of the programs that failed, for their logs.
  Executables are never copied out.
- `clears`: files removed before the step runs, in its place. It replaces the `rm -f` of the merge's whole suites
  (`whole` in `steps.py`). Their command becomes a list of arguments, the same in both places, and no longer
  needs bash.

The rejudge of a whole suite (`--rejudge`) then reads the copied results file and runs on the host.

The folders of the failed programs are found from the results file itself: for each test whose status is not
`PASS`, the files and folders of its results folder whose names start with the test's key (`<group>-<name>`).
Copying the whole results folder instead would move about 26,000 files and 301 MB without the executables (the
host's `target\legacy-tests` on 2026-11-03) after every step.

*Code:* `tools\check\steps.py` (`Step.outputs`, `Step.clears`, `Step.vm`, `whole`), `check.py` (`run_step`).

### D9. Which steps may run in the VM, and the default

`Step.vm = True` marks the steps that may run in either place: `slice`, `upstream`, `verification`, `callsite`,
`cli`, `qbasic`, `addprefix`, `format`, `format-corpus`, `differential`, `nofold`, `coverage`, and the merge's
`corpus-whole`, `upstream-whole` and `verification-whole`. `v37-libs` runs in the guest as part of
`verification` and `verification-whole`, as `linux_whole` does it on Linux. Its libraries are git-ignored, so the
tree's archive does not carry them.

`check.py` gets `--where host|vm`. Without it:

| Run | VM steps go to |
|---|---|
| `check quick`, a step or program named alone | the host |
| `check full`, `check merge` | the VM, or the host with one line naming why when `suites.ps1 -Check` refuses |

With `--where vm`, a refusal fails the VM steps with its message. The host-only steps run either way.

The `cli` step, which needs `cargo test --no-run` to build its binary, gets a host part: `check.py` builds the
test binary with `--message-format=json` and reads its path from cargo's output. In the guest, the binary runs
with `--ignored` and the working directory cargo gives a test, `crates\driver`.

*Instead of:* the VM by default for `quick` as well, which slows the most frequent run by the restore and the
copy, about 30 s, before a measurement shows the VM is worth it there. The user can still give `--where vm`.

*Code:* `check.py`, `main` (the flag), a new `place_of`, and `execute`.

### D10. The place in the fingerprint and the record

The place is hashed into a step's fingerprint as one more input, like the `toolchain` and `clone` externals. A
record gets a `"where"` field, and `check status` and `check history` show it. A pass in one place never stands for
the other, which the spec asks for.

*Instead of:* a VM pass standing for the host, as the stronger run: then a host-only failure (one that depends
on the display, say) would go unseen.

*Code:* `check.py`, where a step's fingerprint is computed from its inputs and externals, and `print_plan`.

### D11. The installed toolchain, never the clone's

The VM gets the toolchain the way a user does. `create.ps1 -Shape Suites` copies the host's `qb64rust.exe` into
the VM and runs `qb64rust toolchain install` as `tester` while the VM is online. That exercises the download, the
SHA-256 check and the unpack on a clean Windows, which no test did before.

The clone sits at `..\QB64pe` in the VM for `qb64pe.exe`, so `qb64rust` could fall back to its toolchain there as
it did on the host. To stop that, `step.ps1` runs `qb64rust toolchain path` before the step's command. The step
fails, without running, unless the second line starts with `installed:`.

On the host, `check.py` runs `qb64rust toolchain path` once per run that has a step with the `toolchain`
external. When it reports the clone, the run's report gets one line naming the fallback and
`qb64rust toolchain install`. The run still goes on: a machine may use the clone on purpose, and the result is
the same.

The VM's toolchain changes only when the VM is made. When the pin in `qb64rust` moves to a new release, the VM's
check above fails with `qb64rust`'s own message, and the VM is made again from the base.

*Instead of:* letting the VM build with the clone's toolchain, which would repeat the host's hidden dependency
inside the VM; copying the host's installed folder in, which skips the install that a user runs.

*Code:* `create.ps1` (the install), `step.ps1` (the guard), `check.py`, `execute` (the report line).

## Risks / Trade-offs

- **[A full run in the VM is slower: 12 processors, cold builds, software OpenGL]** → Task 6 measures a full run
  in both places. If the VM's run takes more than twice the host's, the user is asked about a build cache on the
  cache disk (about 15 GB more) before `full` defaults to the VM.
- **[The clean VM shows a dependency on the host, such as `VCRUNTIME140.dll`]** → That is the point, but it must
  not block this change. Each is written down where it is found. A missing runtime is installed in the VM by
  `create.ps1`, pinned by SHA-256, with a `SOMEDAY.md` row for the product's fix (a static C runtime) and a line to
  the user. A test that depends on the host gets a row in its suite's known list with the reason.
- **[Hot-adding the disk, and its mount, fail on a restored VM]** → Task 1.2 tries it on the desktop VM's
  checkpoint before anything is built on it. The fallback is to restore, attach while the VM is saved if Hyper-V
  allows it, or else boot from a production checkpoint with the disk attached, about 1 to 2 minutes per run.
- **[The base disk changes and breaks the differencing disk]** → D1: read-only base, and `-Iso` refuses while it
  has a child.
- **[A step's log arrives up to 10 s late]** → Acceptable. A run's report is read from its log at the end.
- **[A killed run leaves the VM running]** → D6: the next run refuses and names `suites.ps1 -Stop`.
- **[Both VMs running at once use up to 28 GB of the host's 63 GB]** → They don't run together within a run:
  `order` puts `sandbox` last, after every VM step has finished and the suites VM is off. A separate `just check
  sandbox` during a full run is the user's choice.
- **[The license runs out every 90 days, for two VMs]** → Both are made from the same base, so one
  `create.ps1 -FromBase` per VM after expiry. `suites.ps1 -Check` refuses with the same message as `run.ps1`, and
  `full` then falls back to the host (D9).

## Migration Plan

1. The desktop VM keeps working through D2's fallback path. Nothing changes for it until its remake.
2. Until the suites VM is made, `full` falls back to the host and says so. Nothing breaks.
3. Rollback: `--where host`, or removing the suites VM, puts every step back on the host.

## Measured

On the desktop VM's checkpoint `ready` (Windows build 26300, 4 processors, 4 GB), restored as a run does, on
2026-11-03.

**Task 1.1.**
- `VCRUNTIME140.dll` is **not** in the guest's `System32`, nor are `VCRUNTIME140_1.dll` and `MSVCP140.dll`. So
  `qb64rust.exe` cannot start on a clean Windows as it is built now. The suites VM gets Microsoft's redistributable
  (task 2.3), and `SOMEDAY.md` gets a row for the product's fix.
- `tar.exe` is in `System32` (bsdtar, as on the host).
- `python.exe` on the guest's `PATH` is only Windows' Store stub, in `WindowsApps`. There is no Python.
- The tree's archive: 24,336 files, 86 MB (not the 65 MB of the design's context, which counted only tracked
  files). `git ls-files -co --exclude-standard -z` takes 0.3 s and `tar -c --null -T` 5.7 s on the host. Copying
  it in with `Copy-Item -ToSession` took 6.9 s, and unpacking it in the guest's `C:\run` 14.9 s.
- The clone without `.git`: 1,421 MB. Making the archive took 117 s, copying it in 99 s (about 14 MB/s), and
  unpacking it 16.7 s. This happens only when the clone's stamp changes.
- From the restore to an open PowerShell Direct session: 6 to 8 s.

**Task 1.2.**
- `New-VHD -Dynamic` of 64 GB: 1.1 s. `Add-VMHardDiskDrive -ControllerType SCSI` on the running VM: 0.4 s. The
  guest saw the disk 0.4 s later, online and writable (bus type SAS). Its new-disk policy is already
  `OnlineAll` on Windows 11 Enterprise, so `create.ps1` need not set it.
- GPT, one NTFS partition and its format: 3.3 s. `Add-PartitionAccessPath` at a new folder of the host tree's
  parent path: 0.1 s. A file written there landed on the new disk; the folder is a mount point.
- After `Stop-VM -TurnOff` the drive was still in the VM's settings, and still there after the next
  `Restore-VMSnapshot`. The guest's folder was gone, as the restore put back its `C:`. `Remove-VMHardDiskDrive`
  on the VM while off took 0.2 s. The restore after it gave a guest with one disk, as made. D4 is changed to
  match.

**Task 2.4** (the user ran `create.ps1 -Iso <iso> -Name qb64rust-suites -Shape Suites`, 2026-11-03). Its first
run stopped before writing anything: `New-VHD` takes only a `.vhd` or `.vhdx` name, and the new base was written
as `qb64rust-base.vhdx.new`. It is `qb64rust-base.new.vhdx` now, and the second run made the VM.
- `Get-VMSnapshot` lists `ready` (standard). The Windows disk's parent is `qb64rust-base\qb64rust-base.vhdx`. The
  cache disk is not in the VM's settings.
- 12 processors, dynamic memory 2 / 4 / 24 GB. Restored, the guest had 3.2 GB.
- In the guest: `qb64rust toolchain path` says `installed:`, of `llvm-mingw-20260922-ucrt-x86_64` under `tester`'s
  `%LOCALAPPDATA%`; `python` is Python 3.12.10 in `%ProgramFiles%\Python312`; `VCRUNTIME140.dll` is in
  `System32`; `C:\mesa` holds Mesa; no network adapter; the license active with 90 days.
- Disk on the host: the base 14.4 GB, the differencing disk 6.7 GB, the checkpoint's disk 0.14 GB, the cache
  disk 0.32 GB, so 7.2 GB beyond the base, less than the 10 to 17 GB estimated. The checkpoint's saved memory
  could not be read without elevation (`ProgramData\Microsoft\Windows\Hyper-V` is closed to the user).
- From the restore to a PowerShell Direct session: 19 s, where the desktop VM takes 6 to 8 s.

**Tasks 3.1 to 3.3** (`suites.ps1`, 2026-11-03).
- `-Check`: exit 0 with the VM off; exit 2 with the VM running ("a run never takes over a VM in use") and with
  `QB64RUST_SUITES_VM` naming no VM (the message names `create.ps1 -Shape Suites`).
- The first `-Start` took 444 s: the VM ready in 9 s, the clone copied in for 393 s, the tree (24,342 files) for
  40 s. In the guest, `<parent>\QB64pe` and the tree's copy were there at the host's paths.
- A file written to the guest's `C:\` was gone after `-Stop` and the next `-Start`. A marker in the copy's
  `target\` was kept. A file deleted, one edited and one added on the host were the same in the guest after it.
- A `-Start` with nothing changed: 64 s, of which 11 s to the ready VM and 52 s for the tree (40 to 72 s over
  three starts). A `-Stop`: 6 to 8 s. After each, the VM's settings held only its Windows disk.
- The guest's unpacking is about three times slower than in the desktop VM's `C:\run` (15 to 17 s for the tree),
  which Defender skips: in the suites VM Defender scans the copy (below). The tree stays copied whole each run, as
  D3 chose: it also puts back any tracked file a step changed in the copy, and the 52 s is small beside a `full`
  run. Copying only the changed files is the change to make if the time ever matters.

**Task 5.1** (`step.ps1`, 2026-11-03).
- With `tester`'s toolchain folder renamed, the guard refused ("clone: the toolchain of the QB64pe clone next to
  the repository") and nothing was built.
- Three slice programs (`slice/s01_literals`, `slice/s02_integer_wrap`, `screen/c01_text_cells`) passed in the
  VM in 74 s, cold. The log came to the host as the programs ran, and the results file after. Run again, the
  pass cache served two of the three, in 24 s.
- Found on the way: the runner names `target\qb64pe-clone\objects.lock` for its compiles but made that folder
  only when it took the clone's lock, so in a fresh tree every compile failed ("locking the libqb objects: The
  system cannot find the path specified"). The host had the folder from earlier runs. `clone_lock.py`'s
  `name_objects_lock` now makes it, as the CLI tests' `support::objects_lock` already did.
- Also found: a runner's per-program files are in `<results>\<suite>\`, beside the results file, and
  `outputs.py` now looks there.

**Task 6.1** (`just check slice --where vm`, 2026-11-03). 494 of 494 passed against the same recordings as on
the host, `slice/s141_tcp_loopback` among them (6.6 s), with the VM having no network adapter. 813 s for the
step: the VM ready in 14 s and the tree in 47 s, then every program built cold, as the VM's pass cache held only
the two programs of task 5.1. The host's last full slice run built from its build cache. The programs ran in the
guest's session, so none could open a window on the host's screen; that was not watched on the screen.

**Task 6.2** (`just check cli --where vm`, 2026-11-03). The first run: 115 of 120 passed, the binary finding its
files by the paths built into it. Four failed for want of OpenGL ("Failed to initialize window":
`screen_state_prompt_that_never_comes`, `screen_state_wait_no_rule_answers`, `screen_state_mistake_in_a_held_key`,
`stop_inside_sub_gl_ends_with_error_271`), and `a_build_changes_nothing_git_sees` found no git. With
`support::link_opengl` and the skip (D7): 120 of 120, 295 s for the step, 186 s of it the tests. The skip's line
is printed only under `--nocapture`, as `cargo test` keeps a passing test's output; the test's comment and this
paragraph say it. On the host the test runs whole and passes.

**Task 6.3** (2026-11-03 and 04). Run by Claude in the checkout, as the user asked, with `just full --uncommitted`
(the change's files uncommitted). The run on the host was left out (user: "why not just the vm?"): the host's times
are those of the worktree's last whole run, the `merge` of 2026-11-02 (72 min, no pass cache).
- In the VM, 15 of 17 steps passed at once. `verification` failed on two programs, both found by the VM:
  `v40_g7_utf8_in_name`, through `step.ps1`'s own `chcp 65001`, which gave a program's console UTF-8 where the
  host's has its usual code page, and "café" came out encoded twice (the line is gone, the batch file is ASCII);
  and `v59_d_conversions`, whose stencil mask read back as 2147483647 under Mesa against 255 from the host's GPU
  driver. The program now prints only the low 8 bits (user, 2026-11-04, per `CLAUDE.md` working rule 8), recorded
  again with `qb64pe.exe`: its recording is unchanged. `verification` then passed in the VM, 4,283 of 4,283.
  `sandbox` failed once (`notifypopup_err` at its time limit) and passed 7 of 7 alone right after.
- Times, VM against host, in minutes: `verification` 26.5 / 21.1, `callsite` 12.4 / 0.6 (the host's
  `target\callsite-cache` against none in the copy), `nofold` 7.9 / 1.5, `upstream` 5.4, `slice` 3.6, `cli` 2.8 / 1.6,
  `qbasic` 2.1 / 1.4, `differential` 1.5 / 0.2, `coverage` 0.9 / 0.6, `format-corpus` 0.6 / 0.1; the steps on the
  host alike in both runs. Where both have a time: 60 minutes in the VM against 36, 1.7 times, under the twice
  that would have asked for a build cache in the VM.
- Against the limits: every VM step took less than half of its limit but `verification`, 26.5 of 30 minutes, with
  its limit then the 30-minute floor, since its last pass was the 7-minute run the pass cache served. The floor is
  an hour now.
- The host slept 75 minutes (07:34 to 08:49, Windows' power events) during the first run of `verification` again,
  which `step.ps1` counted as wall time against the limit: the step was stopped. Nothing else came of it.

**Microsoft Defender on the host** (while writing `suites.ps1`). A first `suites.ps1` called `Add-MpPreference
-ExclusionPath` in the guest for the mount folder, so that Defender there would not scan the unpacked tree and
every program built in it. The host's Defender then refused to run the script at all ("This script contains
malicious content and has been blocked by your antivirus software"). The line was taken out, and the script ran.
So the guest's Defender scans the tree's copy and the programs built there. The desktop VM's exclusion of
`C:\run` stays: it is set once by the answer file, at the first logon. What the scans cost a run is measured with
the runs of task 6.
