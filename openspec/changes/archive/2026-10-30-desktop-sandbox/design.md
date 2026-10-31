# Design

## Context

See `proposal.md`, "Why". What the smoke test of 2026-10-30 showed (scratch files under `target\`, not kept):

- Windows Sandbox starts from a `.wsb` file (`Start-Process <file>.wsb`; this build has no `wsb` command line). Its
  `LogonCommand` runs as `WDAGUtilityAccount` in the sandbox's logged-on session, with a desktop. A mapped folder
  can be written; a `shutdown /s` inside closes the sandbox. First start about 75 s, later ones about 40 s.
- With `<ClipboardRedirection>Disable</ClipboardRedirection>` the sandbox's `Set-Clipboard` did not reach the host.
- An executable built by `qb64rust` imports only system DLLs (the Universal C runtime, `user32`, `gdi32`, `GLU32`,
  `comdlg32`, `ole32`, `winmm`, `shell32`) and ran in the bare sandbox: a window, `_SCREENMOVE` and `_MOUSEMOVE`
  read back exactly, exit 0.
- The sandbox's desktop was 1432 × 736: its size follows the sandbox's window on the host, so it is not a constant.
- The sandbox has PowerShell 5.1 and no Python.

Measured in task 1.2 (2026-10-30, probes under `target\sandbox-probe\`, not kept; four runs through `inside.ps1`):

- **The host's processes.** At the start `WindowsSandbox.exe` (the launcher, gone within a second), then
  `WindowsSandboxServer.exe`, `vmmemWindowsSandbox` and `vmwp.exe`, and the client's window
  `WindowsSandboxRemoteSession.exe` (title "Windows Sandbox"). After `shutdown /s` inside, `vmmemWindowsSandbox` and
  `vmwp.exe` end within about 5 s and `WindowsSandboxServer.exe` within about 15 s; **the client's window stays
  open** until it is closed (one from an earlier session was still open hours later). `ManagedWindowsVM.exe` starts
  with the first sandbox and stays after it. `vmwp.exe` is also WSL's and Docker's, so it tells nothing. No
  `WindowsSandboxClient.exe` on this build. A run: `done` written 37 to 50 s after the start, the virtual machine
  gone 55 to 70 s after it.
- **QB64pe's builds** import only system DLLs, the same set as `qb64rust`'s (`objdump -p`: the Universal C runtime,
  `KERNEL32`, `USER32`, `GDI32`, `GLU32`, `SHELL32`, `WINMM`, `comdlg32`, `ole32`), and ran in the bare sandbox,
  exit 0.
- **A QB64 program makes its executable's folder the current one** at its start (both compilers' builds): three
  probes started from `bin\` with their own working folders all wrote `bin\out.txt`.
- **The foreground.** Every program's window reports `_WINDOWHASFOCUS` -1, but only the session's first window gets
  the keys of `_SCREENPRINT "abc" + CHR$(13)`: `INKEY$` read `97 98 99 13` in the first program, nothing in the
  second and third, whichever compiler built them (the order swapped between two runs). After a `_SCREENCLICK` on
  its own window, every program read the four keys, first or not, both compilers' builds.
- **`_SCREENCLICK`** at the window's place plus 100, 100 is seen through `_MOUSEINPUT` with `_MOUSEBUTTON(1)` down
  at `_MOUSEX`, `_MOUSEY` 99, 99, in both builds: libqb scales the place by `65535 / width`, not `/ (width - 1)`.
- **The window's first place cascades** with each program of a session (112, 135; then 138, 161; then 164, 187).
- The desktop was 1432 × 736 in all four runs (the sandbox's window opened at the same size).

## Goals / Non-Goals

**Goals:** the seven built-ins run and are judged by QB64pe's result; the run is one command; nothing it does can
reach the user's desktop, clipboard or console.

**Non-Goals:** a general "run any suite in the sandbox" mode; running in parallel (one sandbox at a time is the
platform's limit); the other reasons of `tests\never_run.list` (`proposal.md`, out of it).

## Decisions

### D1. Windows Sandbox, not a container or a VM of our own

A Windows container has no interactive desktop: a window there is not shown, so `_SCREENMOVE` and the rest have
nothing to act on. A Hyper-V VM would work but needs an image, a license or an evaluation copy, auto-logon and a
checkpoint to keep, and PowerShell Direct runs in session 0, so tests would need an agent in the user's session.
The sandbox is a clean machine at every start, with nothing to maintain. A VM stays the fallback if the sandbox
falls short (a `SOMEDAY.md` row only if that happens).

### D2. A family of its own, `tests\sandbox\`, not a `verification` family

The `verification` folder's programs are run on the host by `run_legacy_tests.py`; a desktop program there would
move the user's window. `tests\sandbox\` holds `<name>.bas` and its recording `<name>.out.txt`, and a `README.md`
saying what each program shows (the role a family's line in `verification\README.md` plays). Names are
`<builtin>_<what>` (`screenmove_readback.bas`), as the call-site programs are named.

A program writes its result to `out.txt` in its working folder, not to standard output: `_CONSOLEFONT` needs the
console to be a real one, not a redirected handle, and `_SCREENPRINT` needs the program's window in front, which a
console window started for the output would take. Values that follow the sandbox's window (the desktop's size, the
window's position before a move) are not printed; a program prints what the call set and what it reads back.

### D3. The runner: Python on the host, PowerShell inside

`tools\sandbox\run.py` (host, Python, unit tests `tools\sandbox\test_run.py`):

1. Refuse to start (exit 2, a message) when `%SystemRoot%\System32\WindowsSandbox.exe` is missing (naming
   `Containers-DisposableClientVM`) or a `WindowsSandboxServer.exe`, `vmmemWindowsSandbox` or
   `WindowsSandboxRemoteSession.exe` process runs (measured in task 1.2, "Context").
2. Build each program of the family (or `--glob`) into `target\sandbox\run\bin\<name>.exe`: with `qb64rust` and
   `--coverage`, through the copy of the release build a test run uses (`tools\legacy_tests\compiler_copy.py`; the
   log's first line names its hash, as `close.py --check` reads it), or with `qb64pe.exe` under `--record`, in the
   clone under the clone's lock as the `verification` families are recorded (`clone_lock.py`). A build failure
   fails that program.
3. Write `target\sandbox\run\sandbox.wsb` with the host path of `target\sandbox\run` mapped to `C:\run`
   (read-write: results come back through it), networking, clipboard, printer, audio and video input disabled, and
   `LogonCommand` `powershell -NoProfile -ExecutionPolicy Bypass -File C:\run\inside.ps1`. The `.wsb` lives only under
   `target\`.
4. Copy `tools\sandbox\inside.ps1` there. Inside, for each `bin\<name>.exe` in `list.txt`'s order: a folder
   `results\<name>\` with a copy of the executable (a QB64 program makes its executable's folder the current one,
   "Context"), `QB64RUST_COVERAGE` set to `results\<name>\counts`, `Start-Process -PassThru` of the copy, wait up
   to the per-program limit (`--timeout`, default 20 s), kill on timeout, write `results\<name>\exit` (the code or
   `timeout`). Then `results\done`, then `shutdown /s /t 0`.
5. Start the sandbox, wait for `results\done` and for the virtual machine's processes (`WindowsSandboxServer.exe`,
   `vmmemWindowsSandbox`) to end, up to the run's limit (default: 120 s plus the per-program limit times the number
   of programs); then close the client's window (`WindowsSandboxRemoteSession.exe`), which stays open after the
   shutdown. Past the limit, end the sandbox's processes and fail, naming the programs with and without results.
   A run that refused to start left any sandbox alone, so the processes it ends are its own.
6. Judge: `--record` writes each `out.txt` to `tests\sandbox\<name>.out.txt` (and an exit code other than 0 as
   `<name>.exit.txt`); otherwise each `out.txt` and exit is compared with the recording, with a unified diff on a
   difference. Then the coverage check (D6). Exit 0 only if every program passed and the check passed.

Why Python on the host: the rest of the test tools are Python, and the unit tests can hold the `.wsb` writer, the
judge and the coverage check without a sandbox. Why PowerShell inside: the sandbox has no Python, and installing one
at every start costs time and needs the network, which is off.

### D4. Recordings from QB64pe's build in the sandbox

As the `verification` families are QB64pe's output, so are these: `run.py --record` builds with `qb64pe.exe` and
runs the builds in the sandbox. A QB64pe build runs there as it is (task 1.2: system DLLs only), so nothing is
copied beside it. Where QB64pe's result depends on the machine (CLAUDE.md working
rule 8), the program prints only what does not.

### D5. What each program reads back

| Built-in | The program | Read back |
|---|---|---|
| `_SCREENMOVE` | moves its window to 100, 120, and to `_MIDDLE` | `_SCREENX`, `_SCREENY` after the first; after `_MIDDLE`, whether the window's center is within 1 pixel of the desktop's |
| `_MOUSEMOVE` | moves the cursor to 50, 60 in its window | `_MOUSEX`, `_MOUSEY` after draining `_MOUSEINPUT` |
| `_SCREENPRINT` | types `abc` and Enter into itself | `INKEY$` read until Enter |
| `_SCREENCLICK` | clicks at a point of its own window | `_MOUSEBUTTON(1)` seen down through `_MOUSEINPUT`, and `_MOUSEX`, `_MOUSEY` there |
| `_CLIPBOARDIMAGE` | writes a 4 × 4 image with known pixels, reads it back | width, height and the pixels of the image read |
| `_NOTIFYPOPUP` | shows a notification | `ERR` after it, and a line after |
| `_CONSOLEFONT` | sets the console's font (`$CONSOLE`) | `ERR` after it, and a line after |

`_SCREENPRINT` needs the program's window in front, and only the session's first window gets the keys (task 1.2):
the program clicks its own window (`_SCREENCLICK`) before typing, which gives every program the keys whatever its
place in the session; no fallback to the `ERR` form is needed. `_SCREENCLICK`'s place is read back as within 1
pixel (libqb's scaling, "Context"), and the `_MIDDLE` of `_SCREENMOVE` as centered across within 1 pixel and down
within 40 pixels above (the work area leaves out the taskbar), so neither depends on the sandbox's window size.

### D6. `sandbox` as a reason of `tests\never_run.list`, held by the runner

The seven `desktop` rows become `sandbox` rows; the reason `desktop` stays in the set, now with no rows, for a
desktop built-in the sandbox cannot run. Both readers of the reasons take it: `REASONS` in
`tools\coverage\builtin_runs.py` and `NEVER_RUN_REASONS` in `crates\driver\tests\inputs.rs` (tier 1). For the host
suites a `sandbox` row is like any other: `builtin_runs.py --check` fails if one runs there. The runner merges its
programs' counts with `builtin_runs.Builtins` (imported, not copied) and fails on a `sandbox` row that ran in none.
A recording run is built by `qb64pe.exe` and has no counts; it skips this check and says so.

Rejected: feeding the sandbox's counts to `builtin_runs.py --check` at a merge. It would tie the merge set's
coverage check to a Windows feature, and a missing counts file would read as "not run" rather than "not measured".

### D7. Its place in the cadence

`tools\close\close.py`: a step `sandbox` (`python tools\sandbox\run.py`, run alone, after the other runs, since it
puts a window on the screen) in the closing set when the change touched `tests\sandbox\`, `tools\sandbox\` or
`tests\never_run.list`, or `--sandbox` is passed, and always in the merge set (`tools\close\README.md`, "The test
cadence"). `--check` reads the run's log as it does the others'. Not in CI: the hosted runners have no Windows
Sandbox, and the run needs a desktop.

## Risks / Trade-offs

- [The sandbox's window takes the user's screen and focus for about a minute] → run alone and last in a set; the
  README says so. A test cannot reach the host's desktop, but the user sees it.
- [The foreground may not be had inside, so `_SCREENPRINT`/`_SCREENCLICK` cannot be read back] → measured in task
  1.2: only the first window gets keys; the program clicks itself first (D5).
- [A Windows update changes the sandbox (process names, `.wsb` settings)] → the refusal and timeout messages name
  what was looked for; the unit tests hold the runner's logic without a sandbox.
- [The first start after a reboot or update takes longer] → the run's limit allows 120 s before the programs.
- [One machine only: the sandbox's results are not seen by CI] → accepted; the family is a local check like the
  Linux family before it ran in CI.
