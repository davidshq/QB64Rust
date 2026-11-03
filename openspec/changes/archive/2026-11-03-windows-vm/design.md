# Design

## Context

See `proposal.md`, "Why". How the family runs today (`tools\sandbox\README.md`, "What a run does"):

- `tools\sandbox\run.py` builds the seven programs on the host into `target\sandbox\run\bin\`, writes
  `list.txt`, `timeout.txt` and `inside.ps1` beside them, and opens a `.wsb` file that maps that folder to
  `C:\run`. It then waits for `results\done` and for the sandbox's processes to end.
- `tools\sandbox\inside.ps1` runs at logon in the sandbox's desktop session. It runs each program in its own
  `results\<name>\` with a time limit, writes `done`, and shuts the sandbox down.
- The executables need only system DLLs, so the guest needs no toolchain and no Python. Its PowerShell 5.1 is
  enough.
- The recordings hold values read back and true-or-false tests, not the desktop's size
  (`tests\sandbox\*.out.txt`). So the VM's desktop does not have to match the sandbox's.
- In `tools\check\steps.py` the `sandbox` step runs `python tools/sandbox/run.py` with `alone=True`, after
  every other step of `just full`.

What this machine has (2026-11-02): Windows 11 Pro, with Windows Sandbox and WSL 2 already on. Both run on the
Hyper-V platform. Whether the full Hyper-V role and its PowerShell module are turned on is not yet known: the
query needs elevation. Task 1.1 measures it.

Measured in task 1.1 (2026-11-02, non-elevated shell):

- The Hyper-V role is on. Its management service `vmms` is running and starts automatically, and the `Hyper-V`
  PowerShell module is installed and loads.
- The user is not in "Hyper-V Administrators" (`S-1-5-32-578`). The user's `BUILTIN\Administrators` membership
  is "deny only" in a non-elevated token, so it does not help.
- So `Get-VM` fails without elevation: "You do not have the required permission to complete this task."
- The user then added the account to the group from an elevated shell, and signed out and back in. A new group
  reaches only processes started after a new sign-in, because Windows builds the access token at sign-in.
- After that, everything worked from a non-elevated shell, on a throwaway VM `qb64rust-probe` (Generation 2, no
  disk) made and removed in the task: `Get-VM`, `New-VM`, `Set-VM -CheckpointType Standard`, `Checkpoint-VM`,
  `Get-VMSnapshot`, `Restore-VMSnapshot`, `Set-VMVideo` at 1920 × 1080, `Set-VMKeyProtector`, `Enable-VMTPM`,
  `Remove-VMSnapshot` and `Remove-VM`. Hyper-V's default disk folder is set. So the cmdlets the runner needs do
  not need elevation. Whether PowerShell Direct does is measured in task 2.1.

Measured in task 2.1 (2026-11-03), with the VM made from the ISO of build 26300.9457 (Windows 11 Enterprise
Evaluation, en-US):

- `create.ps1` worked first time, but needed administrator rights only for the disk: mounting the VHDX,
  partitioning it, `Expand-WindowsImage` and `bcdboot`. Windows setup ran without a screen. The first logon
  wrote `C:\ready` 4 minutes after the VM was made.
- The script reported 620 minutes. That came from the console, not the VM: a click in the console window starts
  QuickEdit selection, which stops the script at its next line of output until a key is pressed.
- PowerShell Direct opens a session from a non-elevated shell with the saved credential. From the restore to an
  open session takes about 12 seconds: 6 seconds to restore and start, and 6 more for the session.
- The guest's clock matched the host's within 3 seconds when the session opened.
- The guest's video is 1920 × 1080 (`Win32_VideoController`). The test account's `AppliedDPI` is 96, which is
  100%. Auto-logon is permanent (`AutoAdminLogon` 1, no `AutoLogonCount`). Defender excludes `C:\run`.
- The disk is 17.6 GB, with a 0.02 GB differencing disk for the checkpoint. The size of the checkpoint's saved
  memory could not be read: Hyper-V's configuration folder is closed to a non-elevated user.
- **The license is not activated.** `slmgr /dli` gives "Notification", reason `0xC004F009` (grace time expired),
  with `GracePeriodRemaining` 0, from the first boot. An evaluation is activated online, and the VM has no
  network. In 10 hours, the guest did not restart by itself (`System` log: two boots, both during setup). D6's
  license check, as written, would refuse every run.
- So Windows is activated while the VM is made (D2, "Activation"). On the VM already made, this was done by hand
  with the steps that `create.ps1` now takes. `slmgr /ato` succeeded on its first try, giving 129,600 minutes
  (90 days) of grace. `ready` was then taken again. Restored, it reads license status 1 (licensed), no network
  adapter in the host or the guest, and `tester` logged on. Windows had set Windows Update back from "Disabled"
  to "Manual" by itself. That does not matter with no network.

Measured in task 3.1 (2026-11-03), with throwaway programs built by `qb64rust -x`:

- `run.ps1` works end to end. Two runs in a row each took 55 to 58 s from the restore to the turn-off. Each
  exited 0, wrote `done`, and left the VM off. The program that never ends read `timeout`.
- **No QB64 program can start in the VM.** Even a four-line program that writes a file and ends shows a message
  box, "Failed to initialize window", and waits for OK, so it times out. The message comes from
  `libqb\internal\c\libqb\src\main-thread-gui.cpp`: every program that does not hide its screen creates an
  OpenGL window at startup, asking for 4× multisampling. The VM's only display is Hyper-V's synthetic video
  adapter, which offers only Windows' software OpenGL 1.1. Windows Sandbox works because it shares the host's
  GPU with its guest.
- The host's GPU (AMD Radeon) can be partitioned: `Get-VMHostPartitionableGpu` lists it, with 32 partitions.
- A console program in this guest opens in Windows Terminal, not the old console host.
- With Mesa's software OpenGL beside each program (D2, "OpenGL"), the programs run. Three runs of `probe` and
  `hang` took 29 to 33 s from the restore to the turn-off, about 7 s of it to a session. `probe` exited 0 and
  read the desktop as 1920 × 1080. It also writes a file to the guest's Public user folder, and the next run did not
  find it. The three refusals were each seen once, and each left the VM as it was: a VM name that does not exist
  (`QB64RUST_VM`), the VM running, and a console window open.
- A program ended at its limit holds its DLLs for a moment after `Stop-Process`. So `inside.ps1` waits for it to
  end before removing them; otherwise the 62 MB `libgallium_wgl.dll` was copied back to the host.

Measured in task 3.2 (2026-11-03), running the family with `python tools/sandbox/run.py`:

- The first run in the VM: 6 of the 7 pass, and the coverage check passes. 51 s from the restore to the
  turn-off, with nothing on the host's screen.
- `mousemove_readback` failed in about half of the full runs, though it passed alone. It read 0, 0 after
  `_MOUSEMOVE 50, 60` in the graphics screen, or 7, 4 after `_MOUSEMOVE 10, 5` in `SCREEN 0`. QB64pe's builds
  of the family failed in the VM the same way (1 run in 4 failed with 0, 0, and another with 7, 4), so it comes
  from the machine, not this compiler.
- What did not help, each over 6 to 8 runs: waits of 2 s in place of 1 s after `SCREEN` (1 failure in 6); the
  classic console host in place of Windows Terminal (2 in 4); `inside.ps1` started hidden (3 in 8); the guest's
  mouse device turned off (5 in 8).
- What did: 4 processors in place of 2. Mesa draws on the CPU. 14 full runs passed in a row with 4: 8 with
  scratch builds and 6 with `run.py`. The time from the start of the VM run to its end was 47 to 84 s, 50 s
  typically.
- QB64pe's builds of the family, with 4 processors: all seven matched the recordings in 4 runs of 4.
- The classic console host stays the default for consoles in the guest (`unattend.xml`), as in Windows
  Sandbox, which has no Windows Terminal.
- No recording changed, so none was made again in the VM.
- `--where sandbox` still passes: 7 of 7 and the coverage check, the sandbox ending after 55 s, at the host's
  display scale of 100%.

Measured in task 4.2 (2026-11-03): the family in the VM while `just check verification` ran on the host. Three
runs: at 88% host CPU (104 s from the start of the VM run to its end), at 100% (101 s), and at 20% as the step
ended (53 s). All passed, 7 of 7. So the `sandbox` step no longer runs alone: `alone=True` is dropped, and it
still starts last.

## Goals / Non-Goals

**Goals:**
- The seven programs run, are judged and are recorded in a VM, with nothing on the host's screen.
- Each run starts from the same state, without rebuilding the VM.
- Making the VM again (after 90 days, or after damage) is one command and no clicks.

**Non-Goals:**
- Toolchains, Python or the repo inside the VM. Programs are built on the host, as now.
- A network in the VM during a run, Windows Update, or keeping the VM's software current. The VM is frozen at its
  checkpoint. It is online only once, while it is made, to activate Windows (D2).
- Running other suites in the VM (`proposal.md`, "Out of this change").

## Decisions

### D1. Hyper-V, Generation 2, Windows 11 Enterprise evaluation

The VM is a Hyper-V Generation 2 VM with Secure Boot and a virtual TPM, as Windows 11 requires. It has 4
processors, 4 GB of fixed memory, a 64 GB dynamically expanding disk, and no network adapter. The design first
gave it 2 processors; with them, one program failed in about half the runs (Context, task 3.2). It is named
`qb64rust-desktop`; `QB64RUST_VM` overrides the name. The name is not private, so it may appear in the repo.

*Instead of:* VirtualBox or VMware. Each is another install, and either runs slowly beside the Hyper-V platform
that Windows Sandbox and WSL 2 already use. A Windows container has no interactive desktop (`DECISIONS.md`
2026-10-30).

**Disk use:** Windows takes about 20 GB of the disk once installed. The `ready` checkpoint saves the memory as
well (D3), which adds about 4 GB. The total is about 25 GB. Measured on 2026-11-03, the VM's disk takes 18 GB, and
the kept base disk ("The base disk") about 12 GB more. `tools\winvm\README.md` asks for about 37 GB. Each run's
writes go to the checkpoint's differencing disk, which a restore throws away, so the base disk does not grow with
use.

### D2. The image is written straight to the disk, with an answer file

`tools\winvm\create.ps1` (administrator) takes `-Iso <path>`. It mounts the ISO and makes the 64 GB VHDX in
Hyper-V's default disk folder (`(Get-VMHost).VirtualHardDiskPath`). It partitions the disk GPT (EFI, MSR,
Windows), applies `sources\install.wim` with `Expand-WindowsImage`, and makes it bootable with `bcdboot`. It
writes `tools\winvm\unattend.xml` into `Windows\Panther\` with the account's password filled in. Then it makes
the VM, attaches the disk, turns on the virtual TPM, and starts the VM.

The answer file's `specialize` and `oobeSystem` passes do the rest without a screen. They make the local
administrator `tester` and skip every out-of-box page (network, Microsoft account, privacy). They log `tester`
on automatically. The first logon's commands make that permanent (`AutoAdminLogon` in `Winlogon`; the answer
file's own `AutoLogon` stops after a count). They also:

- turn off sleep, the display timeout, the lock screen and hibernation;
- add a Microsoft Defender exclusion for `C:\run`, since Defender has no network for updates and must not hold
  or quarantine the test executables;
- write `C:\ready` last.

`create.ps1` waits for `C:\ready` through PowerShell Direct (D4). It then activates Windows (below), waits a
further minute for the guest to settle, and takes the checkpoint (D3).

**Activation.** An evaluation starts its 90 days only when it is activated online. Without that, it reports no
grace time left from its first boot (Context, task 2.1). So `create.ps1` puts the VM online once. It stops and
disables Windows Update in the guest, and connects a network adapter to Hyper-V's NAT switch, "Default Switch".
It runs `slmgr /ato` until the license reads activated, for at most 4 minutes. Then it removes the adapter, in a
`finally`, before the checkpoint. A run never has a network: the checkpoint has no adapter (the user,
2026-11-03: "activate it"). *Instead of:* leaving Windows unactivated and dropping the license check of D6. That
runs Windows in a mode with no known end, which an update might tighten.

*Instead of:* booting the ISO with an `autounattend.xml`. A Generation 2 VM has no floppy drive for the answer
file, so it needs a second ISO. Booting from DVD also waits for a "press any key" that nobody presses. And
`setup.exe` makes its own hardware checks. Writing the image straight to disk avoids all three. It is the same
method Microsoft's `Convert-WindowsImage` uses.

**OpenGL.** Every QB64 program that does not hide its screen opens an OpenGL window with 4× multisampling at
startup. The VM's synthetic display offers only Windows' OpenGL 1.1, so the window cannot be made, and the
program waits on a message box (Context, task 3.1). So `create.ps1` puts Mesa's software OpenGL (llvmpipe) in the
guest's `C:\mesa`: `opengl32.dll` and `libgallium_wgl.dll` from the Windows builds of `pal1000/mesa-dist-win`,
release 26.2.5, MIT license. The download's URL and SHA-256 are pinned in `create.ps1`, and the archive is kept
in `%LOCALAPPDATA%\qb64rust\winvm\mesa\`. `run.ps1` runs `inside.ps1 -OpenGL C:\mesa`. That copies the two DLLs
beside each program for its run, sets `GALLIUM_DRIVER=llvmpipe`, and removes the DLLs after it. Windows loads
`opengl32.dll` from the program's folder first, because it is not one of the "known DLLs" always taken from
System32. `run.ps1` refuses when the guest has no `C:\mesa`. (The user, 2026-11-03: option 1.) *Instead of:*
sharing the host's GPU with the VM (GPU partitioning), as Windows Sandbox does. That needs the host's driver
files copied into the guest after every driver update. Also, as far as is known (not measured), a VM with a GPU
partition cannot be saved with its memory, so each run would boot. *Instead of:* Windows Sandbox only, which takes the screen.

**The base disk.** `create.ps1 -Iso` keeps a copy of the new disk before it ever boots, in
`<Hyper-V disk folder>\qb64rust-desktop-base\`. `create.ps1 -FromBase` copies it in place of writing the image.
That needs no administrator rights, so making the VM again after 90 days needs no elevation. Every remake also
gets the same Windows build and the same Mesa. The base holds the password of the credential file, so
`-FromBase` refuses without that file. It costs about 12 GB of disk that stays. (The user, 2026-11-03: "keep a
copy of never-booted disk".) Windows' 90 days start at activation, which happens after the first boot. That a
copy of the base starts a fresh 90 days is believed, not yet measured: the first remake will show it.

**The password.** `create.ps1` generates a random password. It goes into the answer file on the VM's disk, and
into the guest's registry for auto-logon, where it is readable in plain text. That is acceptable for a
throwaway account in a VM with no network. On the host the password is kept as a `PSCredential` written with
`Export-Clixml` to `%LOCALAPPDATA%\qb64rust\winvm\credential.xml`. Windows encrypts that file for the user who
wrote it (DPAPI). Neither the password nor the file's path is ever in the repo (`CLAUDE.md` rule 2). The repo's
`unattend.xml` holds a placeholder.

**An existing VM.** `create.ps1` refuses if a VM of that name exists, and prints the two commands that remove it
(`Remove-VM`, then deleting its disk folder). It never deletes a VM itself.

### D3. A checkpoint of the running, logged-on VM, restored for every run

The `ready` checkpoint is a standard checkpoint (`Set-VM -CheckpointType Standard`), taken while `tester` is
logged on and idle. A standard checkpoint saves the memory with the disk. So restoring it and starting the VM
resumes at the logged-on desktop in seconds, with no boot and no logon.

Every run restores `ready` before it starts. That gives each run the sandbox's guarantee that nothing carries
over. One example: `_CONSOLEFONT` can change the console's settings under `HKCU\Console`, which a persistent VM
would keep. After a run the VM is turned off (`Stop-VM -TurnOff`). That frees its 4 GB of memory, and the next
restore would throw the state away anyway.

*Instead of:* a VM that keeps its state between runs, restored only when it has drifted. That would be faster by
a few seconds, but a run's result could depend on the runs before it. *Instead of:* a production checkpoint
(disk only), which boots and logs on at each run, about a minute more.

**The clock.** A restored VM resumes at the checkpoint's time. The Hyper-V time synchronization service corrects
it soon after. The runner waits until the guest's date matches the host's within a minute before it starts the
programs. This matters for the license check (D6) and for any program that reads the date.

### D4. A run is driven through PowerShell Direct, and the programs run as an interactive scheduled task

`tools\winvm\run.ps1 -Folder <run folder> -Limit <seconds>` does one run in the VM. `run.py` calls it in place
of opening the `.wsb` and waiting. It:

1. checks the refusals of D6 and exits 2 on one;
2. restores `ready`, starts the VM, and waits for its heartbeat and for a PowerShell Direct session
   (`New-PSSession -VMName ... -Credential ...`) to open;
3. copies the run folder to the guest's `C:\run` (`Copy-Item -ToSession`);
4. registers and starts a scheduled task that runs `inside.ps1` as `tester` with `LogonType Interactive`;
5. waits for `C:\run\results\done`, up to the limit;
6. copies `C:\run\results\` back into the run folder (`Copy-Item -FromSession`), including the partial results
   after a timeout;
7. turns the VM off, whatever happened, in a `finally`.

It exits 0 when `done` was written, 1 when the limit passed, and 2 when it refused. On 1 the results it could
copy are in the folder, so `run.py` names the programs with and without results, as it does for the sandbox.

**Why a scheduled task.** A PowerShell Direct session runs in session 0, which has no desktop. A window opened
there never appears, and `_SCREENMOVE` or `_SCREENPRINT` would act on nothing. A task with `LogonType
Interactive` runs in `tester`'s logged-on session, on its desktop, as Windows Sandbox's `LogonCommand` does. It
needs no password and is registered fresh for each run, on top of the restored checkpoint, so nothing has to
stay resident in the VM.

*Instead of:* an agent started at logon that watches `C:\run`. That is the "agent to keep" of the 2026-10-30
decision, and it would be frozen into the checkpoint. *Instead of:* `PsExec -i`, a third-party tool that would
have to be installed in the guest.

*Instead of:* a shared folder. That would need a network adapter and SMB inside the VM. The run's files are a
few megabytes, so copying them through the hypervisor costs nothing noticeable.

**Elevation.** Hyper-V's cmdlets and PowerShell Direct work without elevation for a member of the "Hyper-V
Administrators" group. That is believed, not measured: task 1.1 measures it. If either needs an elevated
process, the change stops there and the user is asked, because a UAC prompt for each run would not do.

### D5. One runner, with a choice of where

`run.py` gains `--where vm|sandbox`, default `vm`. What differs between the two:

| | `vm` | `sandbox` |
|---|---|---|
| start and wait | `tools\winvm\run.ps1` | the `.wsb`, then `wait()` on the processes |
| `inside.ps1` | run without `-Shutdown` | run with `-Shutdown` (from the `.wsb`'s command) |
| the host's display scale | never read or changed | as now: set for a recording, and with `--scale` |
| refusals | D6 | as now |

Building, `prepare()` (less the `.wsb` for `vm`), `judge()`, `record()` and the coverage check are shared.
`inside.ps1` takes a `-Shutdown` switch. Without it, the script writes `done` and stops, and the VM is turned off
from the host.

**The VM's display scale is 100%.** The guest's display is Hyper-V's synthetic video adapter with no console
window open. `create.ps1` sets its resolution to 1920 × 1080 (`Set-VMVideo`), at which Windows chooses 100%. A
recording made in the VM therefore needs no scale change. `run.py --where vm --scale N` is refused (exit 2):
there is no host scale to set for the VM.

**The `just full` step.** `steps.py`'s `sandbox` step keeps its name and command, which now runs in the VM. Its
inputs gain `tools/winvm/**`. It keeps `alone=True` until a run under load has been measured: the VM takes 4 GB
of memory and 4 processors while the other steps run. Task 4.2 decides whether to drop it. It did: the family
passed under load, so `alone=True` is dropped (Context, task 4.2).

### D6. Refusals in the VM

`run.ps1` refuses (exit 2), with a message naming what to do, when:

- the Hyper-V module cannot be loaded, or the user cannot use it. The message names the "Hyper-V
  Administrators" group and the Windows feature `Microsoft-Hyper-V-All`;
- the VM does not exist, or has no `ready` checkpoint. The message names `create.ps1`, and `--where sandbox` as
  the other way;
- the VM is running when the run starts. A run never takes over a VM someone is using, and the VM is left as it
  is;
- a console window of the VM is open (`vmconnect.exe` for that VM). In enhanced session mode it shares the
  host's clipboard, which the spec forbids;
- the evaluation license has expired: `GracePeriodRemaining` of the guest's Windows license is 0. This is read
  after the restore and the clock check (D3). The VM is turned off, and the message says to run `create.ps1`
  again.

## Risks / Trade-offs

- [PowerShell Direct or the Hyper-V cmdlets need elevation even for a group member] → Task 1.1 measures it
  first. If so, stop and ask the user before writing the runner.
- [A program's result in the VM differs from its sandbox recording, because the machines differ] → Task 3.2
  judges the VM against the existing recordings. A difference that comes from the machine, not the compiler, is
  recorded again in the VM with `qb64pe.exe`. `tests\sandbox\README.md` then says where each recording was made.
- [The guest's Windows from the evaluation ISO is a different build from the host's] → It is only a desktop to
  run seven programs on. A change in what a program reads back would show up as a difference in task 3.2.
- [The evaluation runs out every 90 days] → Making the VM again is one command, about 15 minutes of machine time
  and no clicks (D2). The runner says so when it happens (D6). Windows Sandbox stays available meanwhile.
- [`Set-VMVideo` may not set the resolution of a Generation 2 VM on this build] → The recordings do not depend
  on the desktop's size (Context). If it fails, the guest's default is used, as long as its scale is 100%. Task
  2.1 measures what the guest gets.
- [Resuming from a checkpoint with memory leaves the guest's clock behind] → The runner waits for the time
  synchronization (D3).
- [The `ready` checkpoint ties up about 4 GB of disk for the saved memory] → Accepted. It is what makes a run
  start in seconds.

## Migration Plan

Nothing to migrate. The VM is new, and Windows Sandbox stays as `--where sandbox`. To go back, make
`--where sandbox` the default again.

## Open Questions

- How long a run takes from restore to turn-off. It is measured in task 3.2 and written in
  `tools\sandbox\README.md`.
