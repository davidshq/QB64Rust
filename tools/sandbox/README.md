# The sandbox family's runner

`run.py` runs the programs of `tests\sandbox\` in Windows Sandbox: the built-ins that act on the desktop, the
clipboard or the console (`_SCREENMOVE`, `_MOUSEMOVE`, `_SCREENCLICK`, `_SCREENPRINT`, `_NOTIFYPOPUP`,
`_CLIPBOARDIMAGE`, `_CONSOLEFONT`), which no test may run on the user's desktop (spec `testing/desktop-sandbox`;
OpenSpec change `desktop-sandbox`).

```
python tools/sandbox/run.py > target/sandbox.log 2>&1      # build with qb64rust --coverage, run, judge, coverage check
python tools/sandbox/run.py --record                       # build with qb64pe.exe, run, write the recordings
python tools/sandbox/run.py --glob "screen*"               # a part of the family (no coverage check)
python tools/sandbox/run.py --timeout 30                   # the limit of one program, in seconds (default 20)
python -m unittest tools/sandbox/test_run.py               # its own tests, no sandbox needed
```

**It takes the screen for about a minute.** The sandbox's window opens on the host's desktop and the programs move
its mouse and type into it; nothing reaches the host's own desktop, clipboard or console, but run it alone, after
any other test run, and leave the mouse and keyboard alone while it runs. In `tools\close\close.py`'s plan it is the
last step.

## What the host needs

Windows Sandbox, a feature of Windows 10 and 11 Pro, Enterprise and Education (not Home), with virtualization
turned on in the firmware. Turn it on in "Turn Windows features on or off" (Windows Sandbox), or from an
administrator's PowerShell:

```
Enable-WindowsOptionalFeature -Online -FeatureName Containers-DisposableClientVM
```

then restart. Without it the runner refuses to start (exit 2), naming the feature; with a sandbox already open
(one runs at a time) it refuses too and leaves that sandbox open. A recording run (`--record`) needs the primary
monitor's display scale at 100% and sets it itself for the run ("The display scale" below), refusing only when
that fails: the sandbox's desktop takes the host's, and under another scale QB64pe's `_SCREENCLICK` lands at the
scale times its place on the desktop (at 125%, a click at 100, 80 in the window read back as 166, 151); this
project's libqb fixes that (`QB64PE-DIVERGENCES.md` D-056), so a judging run goes ahead at any scale and prints
the one it ran at. The family passes at 100% and 125% (2026-10-30; at 125% `_SCREENCLICK` lands 2 pixels off
across, which its program allows); other scales are not measured, so a failure there may be the scale's. A refusal
is not a pass. No toolchain and no Python
are needed inside: the executables need only system DLLs, and the script inside is PowerShell.

## What a run does

1. Builds each program into `target\sandbox\run\bin\<name>.exe`: with a copy of `target\release\qb64rust.exe`
   (`tools\legacy_tests\compiler_copy.py`; the log's first line names its hash, which `close.py --check` reads) and
   `--coverage --fixes off`, or under `--record` with the clone's `qb64pe.exe`, holding the clone's lock
   (`tools\legacy_tests\clone_lock.py`). A program that does not build fails.
2. Writes `target\sandbox\run\sandbox.wsb`: that folder mapped read-write to `C:\run`, and networking, the
   clipboard, the printer, audio and video input not shared. The host's path is written there at run time only;
   nothing under `tests\` or `tools\` holds one.
3. Starts the sandbox. At logon, `inside.ps1` (copied to `C:\run`) runs the programs in order, each copied to its
   own `results\<name>\` and run there (a QB64 program makes its executable's folder the current one), with
   `QB64RUST_COVERAGE` naming `results\<name>\counts`. It writes `results\<name>\exit` (the exit code, or `timeout`
   when the program was ended at the limit), then `results\done`, then shuts the sandbox down.
4. Waits for `done` and for the virtual machine's processes (`WindowsSandboxServer.exe`, `vmmemWindowsSandbox`) to
   end, then closes the sandbox's window (`WindowsSandboxRemoteSession.exe`), which stays open after the shutdown.
   The run's limit is 120 s plus the program limit times the number of programs; past it the runner ends the
   sandbox and fails, naming the programs with results and those without. A run of the seven takes about a minute.
5. Judges each program: its `out.txt` against `tests\sandbox\<name>.out.txt` (line ends aside: a program writes
   CRLF, git keeps LF), with a unified diff on a difference, and its exit code against `<name>.exit.txt` (0 when
   there is none). A missing recording, a missing result, a timeout or a missing `out.txt` fails. `--record`
   writes the recordings instead (and `<name>.exit.txt` for an exit code other than 0).
6. The coverage check (design D6): each built-in that `tests\never_run.list` gives the reason `sandbox` must have run
   in a program of the family, read from the counts with `tools\coverage\builtin_runs.py`'s merge. Skipped, and
   said so, for a recording (`qb64pe.exe`'s builds count nothing) and for `--glob`.

The last lines of its output are `coverage: PASS|FAIL (...)` and `sandbox: N pass, M fail`; exit 0 only when every
program passed and the coverage check did.

## The display scale

The runner reads and sets the host's display scale itself (user, 2026-10-30: "you set settings and document for
yourself how to do it in future"), so nobody has to open Settings:

- **Reading it:** `display_scale()` asks the primary monitor's effective DPI (`GetDpiForMonitor`, `MDT_EFFECTIVE_DPI`)
  on a thread made per-monitor DPI aware for the call (`SetThreadDpiAwarenessContext(-4)`), and divides by 96:
  120 is 125%. Not the registry's `AppliedDPI`, which keeps the value of the logon (it said 96 while the screen was
  at 125%), and not `GetDpiForSystem`, which a process that is not DPI aware reads as 96.
- **Setting it:** `set_display_scale(percent)` calls `SystemParametersInfoW(SPI_SETLOGICALDPIOVERRIDE, steps, NULL,
  SPIF_UPDATEINIFILE)`, `SPI_SETLOGICALDPIOVERRIDE` being `0x009F`, undocumented and what the Settings page itself
  calls: `steps` is the scale as a number of steps along Windows' list (100, 125, 150, 175, 200, 225, 250, 300, …)
  from the one it recommends for the monitor, negative below it. It takes effect at once, no logoff, and is read
  back within a few seconds. The step in effect is read from nowhere: the registry's `HKCU\Control
  Panel\Desktop\PerMonitorSettings\<monitor>\DpiValue` said 0 while -1 was in effect (2026-10-30; trusting it set
  the display to 150%). So the runner first sets step 0, which shows the recommended scale, then the target's step
  from it (`SCALES.index(target) - SCALES.index(recommended)`): two calls at most, each read back.
  Measured 2026-10-30 on this machine: 125% recommended, `-1` gave 100%, `0` gave 125% back.
- **The tests never call it:** every test of `test_run.py` that reaches `main` or `set_display_scale` stands the API
  and the reads in. On 2026-10-30 one test ran `main(["--record"])` with the read patched and the call not, and set
  the machine's display to 100%.
- **When:** a recording run sets 100% (`qb64pe.exe`'s libqb has no D-056) and `--scale N` sets N for a judging run;
  either puts the scale found back when the run ends, whatever its result, and says so (`display scale: 125% set to
  100% for the run`, `put back to 125%`). A run without `--scale` leaves the scale alone.
- **The sandbox follows the host:** its desktop takes the host's scale when it starts, so the switch must come
  before the sandbox is started, as the runner does; a sandbox already open keeps the old one.

## When it runs

On request; at a change's close when `close.py` finds the change touched `tests\sandbox\`, `tools\sandbox\` or
`tests\never_run.list`, or with `close.py --sandbox` (a change to one of the seven built-ins' emission or to libqb's
code for them); always at a merge into `main` (`tools\close\README.md`, "The test cadence"). Never in CI: the hosted
runners have no Windows Sandbox and no desktop.
