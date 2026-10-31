# Proposal

## Why

Seven built-ins are compiled and never run, because they act on the desktop and a test may not move, click or type
on the user's desktop, or change their clipboard or console (`tests\never_run.list`, reason `desktop`):
`_SCREENMOVE`, `_MOUSEMOVE`, `_SCREENCLICK`, `_SCREENPRINT`, `_NOTIFYPOPUP`, `_CLIPBOARDIMAGE`, `_CONSOLEFONT`.
Only the call-site check holds them, and it compares the emitted call, not what the call does. Windows Sandbox
(Windows 11 Pro, turned on here 2026-10-30) gives a throwaway desktop of its own: a smoke test on 2026-10-30 ran an
executable built by `qb64rust` there with no toolchain, `_SCREENMOVE 100, 120` read back as `_SCREENX`/`_SCREENY`
100 and 120, `_MOUSEMOVE 50, 60` as `_MOUSEX`/`_MOUSEY` 50 and 60, and with clipboard redirection off the host's
clipboard was untouched (about 40 s from start to shutdown). A Windows container cannot do this: it has no
interactive desktop.

## What Changes

- **A test family run in Windows Sandbox**, `tests\sandbox\`: one program per desktop built-in that does the
  action and reads its effect back where it can (`_SCREENX`/`_SCREENY` after `_SCREENMOVE`, `_MOUSEX`/`_MOUSEY`
  after `_MOUSEMOVE`, the program's own keyboard input after `_SCREENPRINT`, its own mouse button after
  `_SCREENCLICK`, the image after a round trip through `_CLIPBOARDIMAGE`), and where nothing can be read back
  (`_NOTIFYPOPUP`, `_CONSOLEFONT`) that the call raises no error and the program goes on. Each program's output is
  recorded from QB64pe's build run in the sandbox, as the `verification` families are recorded with `qb64pe.exe`.
- **A runner**, `tools\sandbox\run.py`: builds the family on the host (with `qb64rust`, or with `qb64pe.exe` to
  record), writes a `.wsb` with the host's paths when it runs (no local path in the repo), starts the sandbox with
  networking, the clipboard, the printer, audio and video input turned off, runs each executable inside with a
  timeout, waits for the sandbox to shut down, and compares each output with its recording. It refuses to start
  when Windows Sandbox is not installed or a sandbox is already running, and says so; neither is a pass.
- **The coverage list learns where these run.** The seven `desktop` rows of `tests\never_run.list` take a new
  reason, `sandbox` (run only in the sandbox family). `builtin_runs.py --check` keeps holding the host suites to the
  list as today; the runner, built with `--coverage`, fails when a built-in listed as `sandbox` did not run in the
  family.
- **Its place in the cadence**: on request, and in the full set at a merge into `main`; a change's closing set takes
  it when the change touched the family, its runner or the list's `sandbox` rows, and `close.py --sandbox` asks for
  it otherwise (for a change to one of the seven built-ins' emission or libqb's code for them).
  It is never in CI or tier 1 and tier 2: it needs the Windows feature and puts the sandbox's window on the screen.

Out of it, as `SOMEDAY.md` rows: the `printer` rows (`LPRINT`, `_PRINTIMAGE`, with "Microsoft Print to PDF" as the
sandbox's printer), the `keyboard` rows, the `person` rows (dialogs driven by UI Automation), and moving the
clipboard programs of the known clipboard race (`verification\v49_d_clipboard`, `slice\s150_builtins_more`) into
the sandbox. A Hyper-V VM of our own (only if the sandbox falls short); Linux.

## Capabilities

### New Capabilities

- `testing/desktop-sandbox`: the sandbox family, its runner, its recordings, and what it requires of the host.

### Modified Capabilities

- `testing/coverage`: the reason set of `tests\never_run.list` gains `sandbox`, and a built-in so listed must run in
  the sandbox family.

## Impact

- New: `tests\sandbox\` (programs, recordings, README), `tools\sandbox\` (runner, its unit tests, README).
- Changed: `tests\never_run.list`, the reasons of its two readers (`tools\coverage\builtin_runs.py`, `REASONS`, and
  tier 1's `crates\driver\tests\inputs.rs`, `NEVER_RUN_REASONS`) and their tests, `tools\close\README.md` and
  `tools\close\close.py` (the merge set, the closing set, `--sandbox`), `docs\LAYOUT.md`, `SOMEDAY.md`,
  `DECISIONS.md`.
- No change to the compiler or libqb. The host needs Windows Sandbox turned on (`Containers-DisposableClientVM`); a machine without it
  cannot run the family, and the merge set says so instead of passing.
