# The test VM

The desktop family (`tests\sandbox\`) runs in a Hyper-V VM named `qb64rust-desktop`. Its runner is
`tools\sandbox\run.py` (`tools\sandbox\README.md`). The VM exists so that a run puts nothing on the host's
screen and never changes the host's display scale. Windows Sandbox did both. The VM is made once and kept. Every
run restores its checkpoint `ready`, so no run sees what an earlier one left, and turns the VM off at the end.
(OpenSpec change `windows-vm`; `DECISIONS.md` 2026-11-02.)

| File | What it is |
|---|---|
| `create.ps1` | Makes the VM: from a Windows 11 Enterprise evaluation ISO (`-Iso`, elevated), or again from the disk kept by that (`-FromBase`, not elevated). |
| `unattend.xml` | The answer file that sets Windows up without a screen. `create.ps1` fills in its placeholders. |
| `run.ps1` | Does one run of the family in the VM. `run.py` calls it. |

## What the VM is

- Generation 2, with Secure Boot and a virtual TPM, 4 processors, 4 GB of fixed memory and a 64 GB disk that
  grows as used. With 2 processors, `mousemove_readback` failed in about half the runs: Mesa draws on the CPU,
  and the window was not ready when the program moved the mouse. Video is 1920 × 1080, which Windows shows at 100%.
- No network adapter. It is online only once, while `create.ps1` makes it, to activate Windows.
- A local administrator `tester`, with a random password, logs on by itself. Sleep, the display timeout, the lock
  screen and the screen saver are off. Windows Update has no network to update from. Microsoft Defender skips
  `C:\run`, where a run's files go.
- `C:\mesa` holds Mesa's software OpenGL ("OpenGL" below). Without it no QB64 program can open its window there.
- The checkpoint `ready` holds the VM running, with `tester` logged on and idle. It is a standard checkpoint,
  with the memory, so a restore resumes at the desktop in about 6 seconds.
- `QB64RUST_VM` names another VM in place of `qb64rust-desktop`, for both scripts.

## What the host needs

- Windows 11 Pro, Enterprise or Education, with Hyper-V turned on. From an elevated PowerShell, then restart:
  ```
  Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All
  ```
- The user in the group "Hyper-V Administrators". From an elevated PowerShell:
  ```
  Add-LocalGroupMember -SID 'S-1-5-32-578' -Member "$env:USERDOMAIN\$env:USERNAME"
  ```
  Then sign out and back in. Windows gives a process its groups when the user signs in, so programs already open
  do not get the new group. With the group, runs need no elevation. Only `create.ps1 -Iso` does.
- Hyper-V's switch "Default Switch", which Windows makes with Hyper-V. The VM uses it for a minute while it is
  made.
- About 37 GB of disk on the drive of Hyper-V's disk folder (`(Get-VMHost).VirtualHardDiskPath`). The VM's disk
  takes 18 GB (measured 2026-11-03). The checkpoint's saved memory takes up to 4 GB more; its size could not be
  read without elevation. The base disk (below) takes about 12 GB more.
- The host online while the VM is made: `create.ps1 -Iso` downloads Mesa, about 70 MB, the first time.

## Making it

1. Download the ISO of Windows 11 Enterprise, 64-bit, from Microsoft's Evaluation Center
   (https://www.microsoft.com/en-us/evalcenter/download-windows-11-enterprise). Choose the plain Enterprise
   download, not LTSC.
2. From an elevated PowerShell, in the repo:
   ```
   .\tools\winvm\create.ps1 -Iso <path to the ISO>
   ```

It writes the image to the disk, which takes several minutes, and puts Mesa in its `C:\mesa`. Before the disk
ever boots, it keeps a copy of it, the base, in `qb64rust-desktop-base\` in Hyper-V's disk folder. It starts the
VM and waits for the first logon to finish, which took 4 minutes on 2026-11-03. Then it activates Windows, waits
a minute, takes the checkpoint and turns the VM off. It asks nothing. Its last line is `Made qb64rust-desktop
with its checkpoint 'ready' in N minutes.`

**Do not click in its console window while it runs.** A click starts a text selection, and the script stops at
its next line of output until a key is pressed. On 2026-11-03 this made the script report 620 minutes, though
the VM had finished its first logon 4 minutes after it was made.

To check the VM, from a normal PowerShell:

```
Get-VMSnapshot -VMName qb64rust-desktop
```

It lists `ready`.

`create.ps1` refuses (exit 2) when a VM of that name or its disk folder already exists, and prints the commands
that remove them. It never removes a VM itself. When it fails partway, it says what it left behind.

## The password

`create.ps1` makes a random password for `tester`. On the host it is kept only in
`%LOCALAPPDATA%\qb64rust\winvm\credential.xml`, a `PSCredential` written with `Export-Clixml`. Windows encrypts
the file for the user who wrote it (DPAPI), so only that user, on that machine, can read it. `run.ps1` reads it
to open a PowerShell Direct session in the VM. Inside the VM, the password is in plain text in the registry, for
auto-logon. That is acceptable for a throwaway account in a VM with no network. Neither the password nor the
file's path is ever in the repo. The repo's `unattend.xml` holds only the placeholder `@@PASSWORD@@`.

The base disk holds the same password. `create.ps1 -Iso` writes a new base and a new credential file together.
`-FromBase` reads the file, and refuses without it.

## Making it again after 90 days

The evaluation's license lasts 90 days from activation. When it has run out, `run.ps1` refuses to run (exit 2)
and says so. Remove the VM and make it again from the base, all from a normal PowerShell:

```
Remove-VM -Name qb64rust-desktop -Force
Remove-Item -Recurse -Force "$((Get-VMHost).VirtualHardDiskPath)\qb64rust-desktop"
.\tools\winvm\create.ps1 -FromBase
```

The base was never booted, so Windows starts its 90 days again when the new VM activates.

**`-FromBase` has never been run.** The VM made on 2026-11-03 came before the base, so it has none. At its first
expiry, about 2027-02-01, make it again with `-Iso` (elevated), which writes the base. Try `-FromBase` at the
expiry after that (`SOMEDAY.md`). Meanwhile
`python tools/sandbox/run.py --where sandbox` runs the family in Windows Sandbox. For a newer Windows build or
Mesa, make the VM with `-Iso` instead, which replaces the base.

To remove the VM for good, run the first two commands, then delete the base and the host's files:

```
Remove-Item -Recurse -Force "$((Get-VMHost).VirtualHardDiskPath)\qb64rust-desktop-base"
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\qb64rust\winvm"
```

## OpenGL

Every QB64 program that does not hide its screen opens an OpenGL window at startup. The VM's display is Hyper-V's
synthetic adapter, whose OpenGL 1.1 cannot make that window. A program then shows "Failed to initialize window"
and waits for OK. So the guest has Mesa's software OpenGL (llvmpipe) in `C:\mesa`:

| | |
|---|---|
| Files | `opengl32.dll`, `libgallium_wgl.dll`, from `x64\` of the archive |
| Source | https://github.com/pal1000/mesa-dist-win/releases, release 26.2.5, `mesa3d-26.2.5-release-msvc.7z` |
| SHA-256 of the archive | `997123ef4ce8896204ab912d5b54f19372c487ff9835bcefafa2fca23b8953c2` |
| License | MIT (Mesa) |

`create.ps1` downloads the archive once into `%LOCALAPPDATA%\qb64rust\winvm\mesa\`, checks the hash, and refuses
on a mismatch. Nothing of Mesa is in the repo. A run copies the two DLLs beside each program, sets
`GALLIUM_DRIVER=llvmpipe`, and removes them after the program ends (`tools\sandbox\inside.ps1 -OpenGL`).

## Its console

Do not open the VM's console in **enhanced session** mode. That mode shares the host's clipboard with the VM, and
the family's `_CLIPBOARDIMAGE` program must not reach the host's clipboard. `run.ps1` refuses to run while any
console window of the VM is open. To look inside the VM, open its console in Hyper-V Manager and choose the
basic session: in the View menu, turn off "Enhanced Session". Close it before a run.
