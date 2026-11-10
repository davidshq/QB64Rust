# The test VMs

There are two Hyper-V test VMs, made from the same base disk:

- **`qb64rust-desktop`**: the desktop family (`tests\sandbox\`) runs in it. Its runner is `tools\sandbox\run.py`
  (`tools\sandbox\README.md`). (OpenSpec change `windows-vm`; `DECISIONS.md` 2026-11-02.)
- **`qb64rust-suites`**: `check`'s steps that build and run programs run in it (`tools\check\README.md`). See
  "The suites VM" below. (OpenSpec change `vm-suites`.)

They exist so that a run puts nothing on the host's screen, never changes the host's display scale, and passes on
a clean Windows rather than because of what the host has. Each VM is made once and kept. Every run restores its
checkpoint `ready`, so no run sees what an earlier one left, and turns the VM off at the end. Most of this page
is about the desktop VM; the suites VM differs where its section says.

| File | What it is |
|---|---|
| `create.ps1` | Makes a VM: from a Windows 11 Enterprise evaluation ISO (`-Iso`, elevated), which writes the base disk too, or from that base (`-FromBase`, not elevated). `-Name` and `-Shape Desktop` or `-Shape Suites` choose which. |
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
  read without elevation. The base disk (below) takes about 12 GB more. The suites VM, about 10 to 17 GB more
  ("The suites VM").
- The host online while the VM is made: `create.ps1 -Iso` downloads Mesa, about 70 MB, the first time.

## Making it

1. Download the ISO of Windows 11 Enterprise, 64-bit, from Microsoft's Evaluation Center
   (https://www.microsoft.com/en-us/evalcenter/download-windows-11-enterprise). Choose the plain Enterprise
   download, not LTSC.
2. From an elevated PowerShell, in the repo:
   ```
   .\tools\winvm\create.ps1 -Iso <path to the ISO>
   ```

It writes the image to a new disk, which takes several minutes, and puts Mesa in its `C:\mesa`. That disk, never
booted, is the base: `qb64rust-base\qb64rust-base.vhdx` in Hyper-V's disk folder, marked read-only. The VM's own
disk is a copy of it. (The desktop VM made on 2026-11-03 came before the base, so there is none yet.) It starts the
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

`-Iso` also refuses while any VM has a disk that is a child of the base (the suites VM's is), since a new base
would break that disk. It names the VM and the commands that remove it. That refusal comes before the check for
an elevated shell.

## The password

`create.ps1 -Iso` makes a random password for `tester`. On the host it is kept only in files under
`%LOCALAPPDATA%\qb64rust\winvm\`: the base's in `qb64rust-base\credential.xml`, and a copy for each VM made from
it in `<VM name>\credential.xml`. Each is a `PSCredential` written with `Export-Clixml`. Windows encrypts the file
for the user who wrote it (DPAPI), so only that user, on that machine, can read it. `run.ps1` and `suites.ps1`
read their VM's file to open a PowerShell Direct session in it. Inside the VM, the password is in plain text in
the registry, for auto-logon. That is acceptable for a throwaway account in a VM with no network. Neither the
password nor the file's path is ever in the repo. The repo's `unattend.xml` holds only the placeholder
`@@PASSWORD@@`.

The base disk holds the same password. `create.ps1 -Iso` writes a new base and its credential file together.
`-FromBase` reads the base's file, and refuses without it. Making one VM never replaces another VM's file.

The desktop VM made on 2026-11-03 came before files per VM. Its only file is
`%LOCALAPPDATA%\qb64rust\winvm\credential.xml`, and `run.ps1` reads that one when `qb64rust-desktop\credential.xml`
is missing.

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

To remove both VMs for good, remove each with the first two commands, then delete the base and the host's files:

```
Set-ItemProperty "$((Get-VMHost).VirtualHardDiskPath)\qb64rust-base\qb64rust-base.vhdx" IsReadOnly $false
Remove-Item -Recurse -Force "$((Get-VMHost).VirtualHardDiskPath)\qb64rust-base"
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

## The suites VM

`qb64rust-suites` is where `check` runs the steps that build and run programs: the runner's lists and whole
suites, the call-site check, the coverage check and the CLI tests (`tools\check\README.md`). It is kept apart from
the desktop VM, whose four processors the desktop family was measured with. (OpenSpec change `vm-suites`, design
D1 to D5 and D11.)

**Its shape.** Generation 2, with Secure Boot and a virtual TPM, like the desktop VM. 8 processors. Dynamic
memory: 4 GB at start, at least 2 GB, at most 24 GB, so that its checkpoint saves only the memory in use. Its
Windows disk is a differencing disk whose parent is the base: it stores only what changed after its first boot.
No network adapter, and its console is opened only in the basic session, as for the desktop VM.

**What it holds**, put in by `create.ps1 -Shape Suites` before the checkpoint:

| What | Where in the VM | Pin |
|---|---|---|
| Microsoft's Visual C++ runtime 14.44.35211, which `qb64rust.exe` and the CLI test binary need (`VCRUNTIME140.dll`; a clean Windows 11 does not have it) | `System32` | `VC_redist.x64.exe`, SHA-256 `cc0ff0eb1dc3f5188ae6300faef32bf5beeba4bdd6e8e445a9184072096b713b`, from Microsoft's versioned link in `create.ps1`; signed by Microsoft Corporation |
| Python 3.12.10, the last 3.12 release with a Windows installer, installed for all users and on `PATH` | `%ProgramFiles%\Python312` | `python-3.12.10-amd64.exe` from python.org, SHA-256 `67b5635e80ea51072b87941312d00ec8927c4db9ba18938f7ad2d27b328b95fb`; signed by the Python Software Foundation |
| The C++ toolchain, installed by `qb64rust toolchain install --from-zip` run as `tester`, from the zip the host downloaded | `tester`'s `%LOCALAPPDATA%\qb64rust\toolchain` | the pin inside `qb64rust`, which `create.ps1` reads from `crates\driver\src\toolchain.rs` |
| Mesa's software OpenGL, as in the desktop VM | `C:\mesa`, and on the cache disk | as in "OpenGL" below |

`create.ps1` downloads both installers and the toolchain's zip once into `%LOCALAPPDATA%\qb64rust\winvm\installers\`,
checks their hashes, and refuses on a mismatch. Nothing of them is in the repo. It also needs
`target\release\qb64rust.exe` built on the host, which it copies into the VM to install the toolchain and removes
again.

The guest does not download the toolchain itself. On 2026-11-05 every TLS download in the VM through Hyper-V's
Default Switch broke partway through with `SEC_E_DECRYPT_FAILURE`, with curl and with `Invoke-WebRequest`. The
host's own download of the same file worked. The Default Switch's settings cannot be changed. The VM still goes
online, for a minute or two, to activate Windows.

The VM has no Rust and no cargo. The host builds `qb64rust.exe` and the CLI test binary, and a run copies them in.

**The cache disk.** `cache.vhdx` in the VM's disk folder: 64 GB at most, growing as used, NTFS. It is not part of
the checkpoint. `suites.ps1` attaches it after each restore and mounts it at the parent folder of the host's tree,
so that the tree's copy is at the same path in the VM as on the host. It keeps, between runs, the copy of each tree
run there with its `target\` (the runner's pass caches, no build cache), the QB64pe clone without `.git`, the
community clones without `.git` (`<parent>\community`, copied only by a run with `-Community`, the step `community`),
and `qb64rust-vm\` with Mesa and the stamps of what was copied. Nothing on it goes back to the host's caches.

**The disk it takes** (measured when it was made, 2026-11-03). The base 14.4 GB. The differencing disk 6.7 GB,
the checkpoint's disk 0.14 GB and the cache disk 0.32 GB, which grows with the copies and pass caches (about 3 GB
expected). The checkpoint's saved memory is not readable without elevation.

**Making it.** It needs the base. There is none until a `create.ps1 -Iso` writes it, so the first time, from an
elevated PowerShell:

```
.\tools\winvm\create.ps1 -Iso <path to the ISO> -Name qb64rust-suites -Shape Suites
```

Once the base exists, from a normal PowerShell:

```
.\tools\winvm\create.ps1 -FromBase -Name qb64rust-suites -Shape Suites
```

Either goes online for a minute or two, to activate Windows and to download the toolchain. To check it:

```
Get-VMSnapshot -VMName qb64rust-suites
(Get-VHD "$((Get-VMHost).VirtualHardDiskPath)\qb64rust-suites\qb64rust-suites.vhdx").ParentPath
```

The first lists `ready`; the second names the base.

**Making it again after 90 days**, or when `qb64rust`'s pinned toolchain moves to a new release (a step in the VM
then fails, saying the toolchain is not the installed one). From a normal PowerShell:

```
Remove-VM -Name qb64rust-suites -Force
Remove-Item -Recurse -Force "$((Get-VMHost).VirtualHardDiskPath)\qb64rust-suites"
.\tools\winvm\create.ps1 -FromBase -Name qb64rust-suites -Shape Suites
```

This removes the cache disk too, so the first run after it copies the clone in again and has no pass cache.

`QB64RUST_SUITES_VM` names another VM in place of `qb64rust-suites`, for `create.ps1 -Shape Suites` and
`suites.ps1`.

## Its console

Do not open the VM's console in **enhanced session** mode. That mode shares the host's clipboard with the VM, and
the family's `_CLIPBOARDIMAGE` program must not reach the host's clipboard. `run.ps1` refuses to run while any
console window of the VM is open. To look inside the VM, open its console in Hyper-V Manager and choose the
basic session: in the View menu, turn off "Enhanced Session". Close it before a run.
