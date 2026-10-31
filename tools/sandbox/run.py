#!/usr/bin/env python3
"""The sandbox family, run in Windows Sandbox (spec testing/desktop-sandbox; OpenSpec change desktop-sandbox, design D3).

    python tools/sandbox/run.py              # build with qb64rust --coverage, run, judge, check coverage
    python tools/sandbox/run.py --record     # build with qb64pe.exe, run, write the recordings
    python tools/sandbox/run.py --glob 'screenmove_*'

The programs of tests/sandbox/ act on the desktop (_SCREENMOVE, _MOUSEMOVE, _SCREENCLICK, ...), so they run in a
throwaway sandbox, never on the host's desktop. The runner builds each into target/sandbox/run/bin/, writes
target/sandbox/run/sandbox.wsb (that folder mapped to C:\\run; the clipboard, networking, the printer, audio and
video input not shared) and starts it; inside.ps1 runs the executables one at a time, each in its own
results/<name>/ with a time limit, and shuts the sandbox down. The runner waits for that, up to the run's limit,
then compares each program's out.txt and exit code with tests/sandbox/<name>.out.txt and <name>.exit.txt (0 when
there is none), and holds the coverage counts to the `sandbox` rows of tests/never_run.list: each must have run in
a program of the family.

It refuses to start (exit 2) when Windows Sandbox is not installed or a sandbox is already running, and a recording
run when the display scale cannot be set to 100% (the sandbox takes the host's, and qb64pe.exe's _SCREENCLICK misses
under another, D-056: a recording run, or --scale N, sets the host's scale for the run and puts it back after); a
refusal is no pass. Exit 1 when a program fails or the coverage check does; 0 when all pass. The sandbox's window takes the
screen for about a minute: run it alone, after any other test run.
"""

from __future__ import annotations

import argparse
import csv
import difflib
import fnmatch
import io
import os
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path
from xml.sax.saxutils import escape

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO / "tools" / "legacy_tests"))
sys.path.insert(0, str(REPO / "tools" / "coverage"))
# pylint: disable=wrong-import-position,import-error
from builtin_runs import LIST, Builtins, read_list
from clone_lock import clear_temp, lock_clone, name_objects_lock
from compiler_copy import header, run_copy
from coverage import CoverageError

FAMILY = REPO / "tests" / "sandbox"
RUN = REPO / "target" / "sandbox" / "run"
INSIDE = Path(__file__).resolve().with_name("inside.ps1")
EXE = REPO / "target" / "release" / "qb64rust.exe"
QB_ROOT = REPO.parent / "QB64pe"
INCLUDE_ROOT = REPO / "tests" / "upstream" / "root"
# Windows Sandbox's program, and the Windows feature that installs it.
SANDBOX = Path(os.environ.get("SystemRoot", r"C:\Windows")) / "System32" / "WindowsSandbox.exe"
FEATURE = "Containers-DisposableClientVM"
# The host's processes of a sandbox (design.md, "Context", measured in task 1.2): the virtual machine's, which end
# when it shuts down, and the client's window, which stays open after that until it is closed. Any of them: a sandbox
# is open. (WindowsSandbox.exe, the launcher, ends within a second; ManagedWindowsVM.exe stays after any sandbox.)
VM_PROCESSES = ("WindowsSandboxServer.exe", "vmmemWindowsSandbox")
CLIENT = "WindowsSandboxRemoteSession.exe"
PROCESSES = VM_PROCESSES + (CLIENT,)
# The run's limit: the sandbox's start and shutdown, plus the programs' limits.
START_LIMIT = 120
COMPILE_TIMEOUT = 600
# Where the sandbox sees the run's folder (inside.ps1).
INSIDE_FOLDER = r"C:\run"


def wsb(host_folder: Path) -> str:
    """The sandbox's configuration: host_folder mapped read-write to C:\\run, nothing else shared, inside.ps1 run at
    logon. The one host path in it is host_folder's."""
    command = rf"powershell -NoProfile -ExecutionPolicy Bypass -File {INSIDE_FOLDER}\inside.ps1"
    return f"""<Configuration>
  <Networking>Disable</Networking>
  <ClipboardRedirection>Disable</ClipboardRedirection>
  <PrinterRedirection>Disable</PrinterRedirection>
  <AudioInput>Disable</AudioInput>
  <VideoInput>Disable</VideoInput>
  <MappedFolders>
    <MappedFolder>
      <HostFolder>{escape(str(host_folder))}</HostFolder>
      <SandboxFolder>{INSIDE_FOLDER}</SandboxFolder>
      <ReadOnly>false</ReadOnly>
    </MappedFolder>
  </MappedFolders>
  <LogonCommand>
    <Command>{escape(command)}</Command>
  </LogonCommand>
</Configuration>
"""


def running_processes(which: tuple[str, ...] = PROCESSES) -> list[str]:
    """The sandbox's processes of `which` running on the host."""
    out = subprocess.run(
        ["tasklist", "/fo", "csv", "/nh"],
        capture_output=True,
        text=True,
        errors="replace",
        check=False,
    ).stdout
    names = {row[0].lower() for row in csv.reader(io.StringIO(out)) if row}
    return [p for p in which if p.lower() in names]


def display_scale() -> int | None:
    """The primary monitor's display scale in percent, as it is now (the registry's AppliedDPI keeps the value of
    the logon), or None when it cannot be read. The sandbox's desktop takes the host's."""
    try:
        import ctypes  # pylint: disable=import-outside-toplevel
        from ctypes import wintypes  # pylint: disable=import-outside-toplevel

        user32, shcore = ctypes.windll.user32, ctypes.windll.shcore  # type: ignore[attr-defined]
        # Per-monitor aware for this thread, so the DPI read is the monitor's effective one, not 96.
        before = user32.SetThreadDpiAwarenessContext(ctypes.c_void_p(-4))
        try:
            monitor = user32.MonitorFromPoint(wintypes.POINT(0, 0), 1)  # MONITOR_DEFAULTTOPRIMARY
            x, y = ctypes.c_uint(), ctypes.c_uint()
            if shcore.GetDpiForMonitor(ctypes.c_void_p(monitor), 0, ctypes.byref(x), ctypes.byref(y)) != 0:
                return None
        finally:
            user32.SetThreadDpiAwarenessContext(ctypes.c_void_p(before))
        return round(x.value * 100 / 96)
    except (AttributeError, OSError):
        return None


# Windows' display scales, in the order of its Settings page: the override below counts steps along it.
SCALES = (100, 125, 150, 175, 200, 225, 250, 300, 350, 400, 450, 500)
# SystemParametersInfo's SPI_SETLOGICALDPIOVERRIDE (not documented; what Settings > Display > Scale calls): the
# primary monitor's scale as a number of steps along SCALES from the one Windows recommends for it, in effect at
# once, no logoff. The step in effect is read from nowhere (the registry's PerMonitorSettings said 0 while -1 was
# in effect, 2026-10-30): one call with step 0 shows the recommended scale, and the step for any other follows.
# README, "The display scale".
SPI_SETLOGICALDPIOVERRIDE = 0x009F


def scale_offset(recommended: int, target: int) -> int | None:
    """The override that gives `target` when `recommended` is the monitor's recommended scale; None off SCALES."""
    if recommended not in SCALES or target not in SCALES:
        return None
    return SCALES.index(target) - SCALES.index(recommended)


def set_override(step: int, read, sleep) -> int | None:
    """Sets the override to `step` and reads the scale back once it has changed (up to five seconds)."""
    import ctypes  # pylint: disable=import-outside-toplevel

    before = read()
    ctypes.windll.user32.SystemParametersInfoW(SPI_SETLOGICALDPIOVERRIDE, step, None, 1)  # type: ignore[attr-defined]
    for _ in range(10):
        sleep(0.5)
        now = read()
        if now != before:
            return now
    return read()


def set_display_scale(target: int, read=None, sleep=time.sleep) -> bool:
    """Sets the primary monitor's scale to `target` percent and reads it back; False when it cannot be set. Step 0
    first, which shows the recommended scale (and is the target's own step when that is it), then the target's."""
    read = read or display_scale
    if read() == target:
        return True
    recommended = set_override(0, read, sleep)
    if recommended == target:
        return True
    step = scale_offset(recommended, target) if recommended is not None else None
    if step is None:
        return False
    return set_override(step, read, sleep) == target


def refusal(
    installed: bool, running: list[str], scale: int | None = 100, recording: bool = False
) -> str | None:
    """Why the run cannot start, or None. The display scale is held to 100% for a recording only: qb64pe.exe's libqb
    has no D-056, so its _SCREENCLICK misses under another scale (main sets it first, so a refusal means it could not
    be set)."""
    if not installed:
        return (
            f"Windows Sandbox is not installed ({SANDBOX} not found): turn on the Windows feature {FEATURE} "
            "(Windows 10/11 Pro or Enterprise; tools/sandbox/README.md), then restart"
        )
    if running:
        return (
            f"a sandbox is running ({', '.join(running)}): one runs at a time; close it, or wait for its run "
            "to end. It is left open"
        )
    if recording and scale != 100:
        shown = "could not be read" if scale is None else f"is {scale}%"
        return (
            f"the display scale {shown} and could not be set to 100%: the sandbox takes the host's, and qb64pe.exe's "
            "_SCREENCLICK misses under any but 100% (QB64PE-DIVERGENCES.md D-056); set Settings > System > Display > "
            "Scale to 100% to record"
        )
    return None


def programs(glob: str | None, family: Path = FAMILY) -> list[Path]:
    """The family's programs, by name, those matching glob (on the name without .bas) only when given."""
    out = sorted(family.glob("*.bas"))
    if glob:
        out = [p for p in out if fnmatch.fnmatch(p.stem, glob)]
    return out


def build_new(compiler: Path, src: Path, out: Path, log: Path) -> bool:
    """Builds src with qb64rust --coverage, the fixes off as every suite builds (spec testing/verification-suite)."""
    cmd = [
        str(compiler),
        "--coverage",
        "-q",
        "-m",
        "-x",
        "--include-root",
        str(INCLUDE_ROOT),
        "--fixes",
        "off",
    ]
    return compile_to(cmd + [src.name, "-o", str(out)], src.parent, out, log)


def build_old(qb_root: Path, src: Path, out: Path, log: Path) -> bool:
    """Builds src with qb64pe.exe in the clone, whose lock the caller holds (it builds in internal/temp)."""
    clear_temp(qb_root)
    return compile_to(
        [str(qb_root / "qb64pe.exe"), "-q", "-m", "-x", src.name, "-o", str(out)],
        src.parent,
        out,
        log,
    )


def compile_to(cmd: list[str], cwd: Path, out: Path, log: Path) -> bool:
    out.unlink(missing_ok=True)
    try:
        r = subprocess.run(
            cmd,
            cwd=cwd,
            capture_output=True,
            text=True,
            errors="replace",
            timeout=COMPILE_TIMEOUT,
            check=False,
        )
        log.write_text(r.stdout + r.stderr, encoding="utf-8")
        return r.returncode == 0 and out.is_file()
    except subprocess.TimeoutExpired:
        log.write_text(f"no build within {COMPILE_TIMEOUT} s\n", encoding="utf-8")
        return False


def prepare(run: Path, names: list[str], timeout: int) -> Path:
    """Writes the run's folder but for the executables: results/ emptied, list.txt, timeout.txt, inside.ps1 and the
    .wsb. Returns the .wsb."""
    results = run / "results"
    if results.exists():
        shutil.rmtree(results)
    results.mkdir(parents=True)
    (run / "list.txt").write_text("".join(f"{n}\n" for n in names), encoding="ascii")
    (run / "timeout.txt").write_text(f"{timeout}\n", encoding="ascii")
    shutil.copy2(INSIDE, run / "inside.ps1")
    path = run / "sandbox.wsb"
    path.write_text(wsb(run), encoding="utf-8")
    return path


def end_sandbox(which: tuple[str, ...] = PROCESSES) -> None:
    """Ends the sandbox's processes of `which`: the run's own, since a run refuses to start beside a sandbox."""
    for p in which:
        subprocess.run(["taskkill", "/f", "/im", p], capture_output=True, check=False)


def wait(
    results: Path,
    limit: float,
    running=lambda: running_processes(VM_PROCESSES),
    clock=time.monotonic,
    sleep=time.sleep,
) -> bool:
    """Waits for results/done and for the virtual machine's processes to end; False past limit seconds."""
    end = clock() + limit
    while clock() < end:
        if (results / "done").is_file() and not running():
            return True
        sleep(1)
    return False


@dataclass
class Verdict:
    name: str
    ok: bool
    why: str
    diff: str = ""


def read_text(path: Path) -> str:
    """A program's output or recording with LF line ends: git stores the recordings so (.gitattributes), and a
    program writes CRLF."""
    return path.read_bytes().decode("latin-1").replace("\r\n", "\n")


def judge(name: str, results: Path, family: Path = FAMILY) -> Verdict:
    """One program's result against its recording."""
    expected = family / f"{name}.out.txt"
    if not expected.is_file():
        return Verdict(name, False, f"no recording ({expected.name}): record it with --record")
    done = results / name / "exit"
    if not done.is_file():
        return Verdict(name, False, "no result: the sandbox did not run it")
    code = done.read_text(encoding="ascii", errors="replace").strip()
    if code == "timeout":
        return Verdict(name, False, "timed out: ended at the per-program limit")
    exit_file = family / f"{name}.exit.txt"
    want = exit_file.read_text(encoding="ascii").strip() if exit_file.is_file() else "0"
    out = results / name / "out.txt"
    if not out.is_file():
        return Verdict(name, False, f"no out.txt (exit {code})")
    got, rec = read_text(out), read_text(expected)
    problems = []
    if code != want:
        problems.append(f"exit {code}, recorded {want}")
    diff = ""
    if got != rec:
        problems.append("output differs")
        diff = "".join(
            difflib.unified_diff(
                rec.splitlines(keepends=True),
                got.splitlines(keepends=True),
                f"{expected.name}",
                f"{name}/out.txt",
            )
        )
    if problems:
        return Verdict(name, False, "; ".join(problems), diff)
    return Verdict(name, True, "pass")


def record(name: str, results: Path, family: Path = FAMILY) -> Verdict:
    """Writes one program's recording from qb64pe's run: out.txt, and its exit code when it is not 0."""
    done = results / name / "exit"
    if not done.is_file():
        return Verdict(name, False, "no result: the sandbox did not run it; nothing recorded")
    code = done.read_text(encoding="ascii", errors="replace").strip()
    out = results / name / "out.txt"
    if code == "timeout" or not out.is_file():
        why = "timed out" if code == "timeout" else f"no out.txt (exit {code})"
        return Verdict(name, False, f"{why}; nothing recorded")
    (family / f"{name}.out.txt").write_bytes(read_text(out).encode("latin-1"))
    exit_file = family / f"{name}.exit.txt"
    if code != "0":
        exit_file.write_text(f"{code}\n", encoding="ascii")
        return Verdict(name, True, f"recorded, exit {code}")
    exit_file.unlink(missing_ok=True)
    return Verdict(name, True, "recorded")


def coverage_check(counts: list[Path], list_path: Path = LIST) -> list[str]:
    """What fails the coverage check (design D6): a built-in listed as `sandbox` that ran in no program, or a list
    that cannot be read."""
    b = Builtins()
    problems = []
    for c in counts:
        try:
            b.add(c)
        except (CoverageError, OSError, ValueError) as e:
            # A counts file cut short (a program ended at its limit): named, and the check fails.
            problems.append(f"{c.parent.name}: counts not read: {e}")
    listed, bad = read_list(list_path)
    problems += bad
    for name, reason in sorted(listed.items()):
        if reason == "sandbox" and not b.ran.get(name):
            problems.append(
                f"`{name}` is listed for the sandbox but did not run in the sandbox family"
            )
    return problems


def coverage(
    recording: bool, glob: str | None, results: Path, list_path: Path = LIST
) -> tuple[list[str], str]:
    """The coverage check of a run, or why it is skipped, and its summary line."""
    if recording:
        return (
            [],
            "coverage: not checked (a recording run is built by qb64pe.exe and has no counts)",
        )
    if glob:
        return [], "coverage: not checked (--glob runs a part of the family)"
    problems = coverage_check(sorted(results.glob("*/counts")), list_path)
    return problems, f"coverage: {'FAIL' if problems else 'PASS'} ({len(problems)} problem(s))"


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument(
        "--record", action="store_true", help="build with qb64pe.exe and write the recordings"
    )
    ap.add_argument("--glob", help="only the programs whose name (without .bas) matches")
    ap.add_argument(
        "--timeout", type=int, default=20, help="the limit of one program, in seconds (default 20)"
    )
    ap.add_argument(
        "--scale",
        type=int,
        choices=SCALES,
        help="the host's display scale to set for the run (put back after it); a recording run sets 100",
    )
    ap.add_argument(
        "--qb64", type=Path, default=EXE, help="the new compiler (default: %(default)s)"
    )
    ap.add_argument(
        "--qb-root", type=Path, default=QB_ROOT, help="the reference clone (default: %(default)s)"
    )
    a = ap.parse_args(argv)

    # The scale the run asks for (a recording: 100%), set for the run and put back after it (README, "The display
    # scale"); the scale found stands when none is asked for.
    wanted = 100 if a.record else a.scale
    before = display_scale()
    if wanted is not None and before != wanted:
        if set_display_scale(wanted):
            print(f"display scale: {before}% set to {wanted}% for the run", flush=True)
        elif not a.record:
            # A recording refuses below; a judging run goes on at the scale found, and says so.
            print(f"display scale: {wanted}% could not be set; the run is at the scale found", flush=True)
    try:
        return run_family(a)
    finally:
        if wanted is not None and before is not None and before != wanted and display_scale() != before:
            print(f"display scale: {'put back to' if set_display_scale(before) else 'NOT put back to'} {before}%")


def run_family(a: argparse.Namespace) -> int:
    scale = display_scale()
    why = refusal(SANDBOX.is_file(), running_processes(), scale, a.record)
    if why:
        print(f"sandbox: refused: {why}", file=sys.stderr)
        return 2
    # Measured at 100% and 125% (README): a failure at another scale may be the scale's.
    print(f"display scale: {'not read' if scale is None else f'{scale}%'}", flush=True)
    progs = programs(a.glob)
    if not progs:
        print(f"sandbox: no program of {FAMILY.relative_to(REPO)} matches", file=sys.stderr)
        return 2

    bin_dir = RUN / "bin"
    # The results too: a run whose builds all fail starts no sandbox, and its coverage check reads no earlier run's.
    for d in (bin_dir, RUN / "results"):
        if d.exists():
            shutil.rmtree(d)
    bin_dir.mkdir(parents=True)
    logs = RUN / "logs"
    logs.mkdir(exist_ok=True)
    verdicts: dict[str, Verdict] = {}
    if a.record:
        print(f"[compiler] {a.qb_root.name}/qb64pe.exe (recording)", flush=True)
        lock = lock_clone(wait=True)
        try:
            for src in progs:
                if not build_old(
                    a.qb_root.resolve(), src, bin_dir / f"{src.stem}.exe", logs / f"{src.stem}.txt"
                ):
                    verdicts[src.stem] = Verdict(
                        src.stem, False, f"build failed (target/sandbox/run/logs/{src.stem}.txt)"
                    )
        finally:
            lock.close()
    else:
        compiler, digest = run_copy(a.qb64.resolve())
        try:
            shown = a.qb64.resolve().relative_to(REPO).as_posix()
        except ValueError:
            shown = a.qb64.name
        print(header(shown, digest, time.time()), flush=True)
        name_objects_lock()
        for src in progs:
            if not build_new(compiler, src, bin_dir / f"{src.stem}.exe", logs / f"{src.stem}.txt"):
                verdicts[src.stem] = Verdict(
                    src.stem, False, f"build failed (target/sandbox/run/logs/{src.stem}.txt)"
                )

    names = [p.stem for p in progs if p.stem not in verdicts]
    results = RUN / "results"
    if names:
        wsb_path = prepare(RUN, names, a.timeout)
        limit = START_LIMIT + a.timeout * len(names)
        print(f"starting the sandbox: {len(names)} program(s), up to {limit} s", flush=True)
        started = time.monotonic()
        os.startfile(wsb_path)  # type: ignore[attr-defined]  # pylint: disable=no-member  # Windows only
        if not wait(results, limit):
            end_sandbox()
            have = [n for n in names if (results / n / "exit").is_file()]
            print(f"sandbox: did not shut down within {limit} s; closed", flush=True)
            print(f"  with results: {', '.join(have) or 'none'}")
            print(f"  without: {', '.join(n for n in names if n not in have) or 'none'}")
            return 1
        # The client's window stays open on the host after the shutdown.
        end_sandbox((CLIENT,))
        print(f"the sandbox ended after {time.monotonic() - started:.0f} s", flush=True)

    for n in names:
        verdicts[n] = record(n, results) if a.record else judge(n, results)
    failed = 0
    for p in progs:
        v = verdicts[p.stem]
        print(f"{'PASS' if v.ok else 'FAIL'}  {v.name}: {v.why}")
        if v.diff:
            print(v.diff, end="" if v.diff.endswith("\n") else "\n")
        failed += not v.ok

    problems, summary = coverage(a.record, a.glob, results)
    for pr in problems:
        print(f"[coverage] {pr}")
    print(summary)
    print(f"sandbox: {len(progs) - failed} pass, {failed} fail")
    return 1 if failed or problems else 0


if __name__ == "__main__":
    sys.exit(main())
