#!/usr/bin/env python3
"""Windows-friendly runner for the QB64pe test suites.

Re-implements tests/compile_tests.sh, tests/qbasic_tests.sh and tests/format_tests.sh
from the QB64pe repository so they can run without bash. It works with any compiler
that accepts the qb64pe command line (-x, -y, -q, -m, -o, -f:...), so the same runner
can measure the old compiler (baseline) and, later, the new one.

Rules followed from compile_tests.sh:
  * every *.bas under tests/compile_tests is a test; category = parent folder
  * <name>.err present      -> compile must fail, no exe, compiler stdout == .err
  * <name>.output present   -> compile, run, merged stdout+stderr == .output
  * neither                 -> compile only
  * sidecars: .flags, .noprompt, .compile-from-base, .<os>.license
  * internal/temp is emptied before each compile
  * output comparison ignores trailing LF / CRLF line endings
Rules followed from format_tests.sh:
  * every *.bas under tests/format_tests is a source; <name>.flagmap lists one
    variant per line: <expected file> <compiler flags...>
  * each variant: qb64 -y -m <flags> <name>.bas -o <out>, run in the test's folder;
    must succeed, write the output, and match the expected file with all CRs
    removed and trailing newlines ignored
For a compiler other than qb64pe (both the compile and the corpus suite), an .err program passes
when the compile fails, writes no exe and the compiler's last line is its error summary with at
least one error not marked "not supported yet" ("2 errors (1 not supported yet)"); the .err text
is not compared.
The corpus suite (tests/corpus of this repo, recorded with the old compiler; no bash original):
  * every *.bas under <corpus-root>/<group>/ is a test; it has exactly one of
    <name>.output (compile, run, merged stdout+stderr == .output),
    <name>.err (compile must fail, no exe, compiler stdout == .err ignoring CRs) or
    <name>.norun (compile only, the exe is never started; the file holds the reason)
  * the .bas is copied into a fresh folder <results>/corpus/<group>-<name>/ and compiled
    there with -q -m -x only; the exe runs there with no arguments, stdin from the null
    device, QB64PE_NOPROMPT=y (or the contents of <name>.noprompt, e.g. 'continue') and a 60 s
    timeout; the folder is deleted on a pass
  * <name>.stdin: the exe's stdin is a copy of that file, byte for byte, instead (for programs
    that read the console with INPUT or LINE INPUT), when checking and when recording
  * <name>.normalize: one rule per line, <regex><TAB><replacement> ('#' lines are comments),
    applied in order to each line of the actual output before comparing or recording
  * --record compiles once, runs twice, and writes .output (or .err for a failed compile)
    only if both normalised runs are equal; otherwise the program is reported
    non-deterministic and nothing is written. With a compiler other than qb64pe, a failed compile
    is recorded only when it is a rejection by the rule above (not a crash or a failed C++ build)
  * --cpp-opt adds -f:OptimizeCppProgram=true and compares against the same files
  * --list <file> runs only the programs named in the file: one <group>/<name> per line
    (no .bas), '#' starts a comment; also for --suite compile, with <category>/<name>
    (e.g. tests/upstream/pass.list); a list that names no program runs nothing and passes
    (a list whose programs --category or --glob leave out is still an error)
The verification suite (verification/ of this repo; verification/run.sh only starts it). It records
and compares nothing:
  * the programs named on the command line (no .bas), or every *.bas of the folder, are compiled
    where they are with qb64pe -x -q -m, so that their included files are found; <name>.exe is
    written beside the program and runs there with stdin from the null device (or <name>.stdin),
    QB64PE_NOPROMPT=y (or the contents of <name>.noprompt), a 60 s timeout and no key-press helper
  * writes <name>.compile.txt (the compiler's stdout and stderr, then "exit=<code>") and
    <name>.out.txt (the program's stdout and stderr, "(timed out after 60 s)" after a timeout, then
    "exit=<code>"; or "(not compiled)" when there is no exe), and prints the latter
  * exit codes are written as Git Bash reports them (bash_exit_code): the files were recorded by a
    shell script and stay comparable. qb64pe only, one program at a time
Speed (corpus and compile suites; the C++ build is most of each program's time):
  * --jobs N runs N programs at a time, after the first one alone (it builds any libqb object the
    clone lacks); results are printed as they finish. Compile tests of one folder run (not build)
    one at a time, since it is their working folder. --record runs one at a time
  * --jobs N with qb64pe (corpus suite only): qb64pe builds inside its own folder (internal/temp,
    and the libqb objects in internal/c), so each worker compiles with a private copy of the
    compiler under --copies-dir (default target/qb64pe-copies/w1..wN, about 800 MB each: qb64pe.exe,
    the Makefile, internal/, settings/). Nothing is shared between workers and a copy compiles one
    program at a time. A copy is made again when the clone's qb64pe.exe, Makefile or internal/c
    differ from what it was made from. A program that fails runs again alone in the clone and both
    outcomes are reported. With one job, and for --record, the clone itself compiles, as before
  * --build-cache DIR reuses an executable when a program's C++ build has exactly the inputs of an
    earlier one (QB64RUST_BUILD_CACHE, crates/driver/src/build.rs: the fragments, qbx.cpp, the
    Makefile, the options and every file of the clone's internal/c by size and time); the program
    is still compiled by qb64rust, run and compared. For local runs (e.g. target/build-cache, about
    2.4 MB per program); CI builds every program
Differences from the bash runners (all deliberate):
  * .err and .license comparisons ignore CR characters (line-ending normalisation)
  * .output comparison treats CRLF as LF (bare CR still significant); the bash
    runner relies on git converting expected files to CRLF on Windows checkouts
  * per-test timeouts so a hung program cannot stall the run; on timeout the whole
    process tree (compiler, make, C++ compiler, test program) is killed
  * tests listed in known_failures.txt are reported as KFAIL and do not fail the run
"""

from __future__ import annotations

import argparse
import concurrent.futures
import contextlib
import fnmatch
import functools
import glob
import hashlib
import json
import os
import queue
import re
import shutil
import signal
import subprocess
import sys
import threading
import time
from dataclasses import asdict, dataclass
from pathlib import Path

from clone_lock import BUSY_EXIT, CLONE_BUSY, lock_clone, lock_folder

COMPILE_TIMEOUT = 900  # seconds; first compiles build libqb for each feature set
RUN_TIMEOUT = 120
CORPUS_RUN_TIMEOUT = 60
VERIFICATION_RUN_TIMEOUT = 60


@dataclass
class Result:
    """The outcome of one test (one variant for the format suite), as written to results.json."""

    suite: str
    test: str  # category/name, or the qbasic path
    kind: str  # output | error | compile-only | format
    status: str = "PASS"  # PASS | FAIL | KFAIL (failed, listed in known_failures.txt)
    stage: str = ""  # stage that failed
    detail: str = ""
    seconds: float = 0.0


def strip_trailing_newlines(data: bytes) -> bytes:
    """Mirrors filesize_without_trailing_newlines: removes trailing LFs, and a CR before each."""
    while data.endswith(b"\n"):
        data = data[:-1]
        if data.endswith(b"\r"):
            data = data[:-1]
    return data


def norm_output(data: bytes) -> bytes:
    """Program output in the form compared against .output.

    Windows programs write CRLF; the clone's .output files may hold LF. Treats CRLF as LF, keeps
    bare CRs significant, then drops trailing line endings like the bash runner."""
    return strip_trailing_newlines(data.replace(b"\r\n", b"\n"))


def norm_text(data: bytes) -> list[str]:
    """The lines of a text file with all CRs and trailing newlines removed (.err, format output)."""
    return data.decode("latin-1").replace("\r", "").rstrip("\n").split("\n")


def is_old_compiler(qb64: Path) -> bool:
    """True for qb64pe, whose .err files hold its exact message text."""
    return qb64.stem.lower() == "qb64pe"


# The compiler root for included files of the upstream tests (m2-parser-breadth design D9): it holds
# tests/compile_tests/extra, which two tests include relative to the compiler's own folder.
INCLUDE_ROOT = Path(__file__).resolve().parents[2] / "tests" / "upstream" / "root"


def include_root_args(qb64: Path) -> list[str]:
    """The new compiler's --include-root; qb64pe finds included files under its own folder."""
    return [] if is_old_compiler(qb64) else ["--include-root", str(INCLUDE_ROOT)]


# The new compiler's last line after errors in the program (spec compiler/cli): "1 error",
# "3 errors", "3 errors (2 not supported yet)". Only the front end prints it, so a failed C++ build
# or an internal compiler error (exit code 3) never matches, even when clang's output holds "error:"
# lines.
SUMMARY_RE = re.compile(r"^(\d+) errors?(?: \((\d+) not supported yet\))?$")


def new_compiler_rejection(rc: int, out: bytes) -> str:
    """Why a failed compile by a compiler other than qb64pe does not count as a rejection, or "" if
    it does.

    An .err program passes when the compile fails, writes no executable (checked by the caller) and
    reports at least one error not marked "not supported yet" (DECISIONS.md, 2026-10-04); the
    message text is not compared."""
    if rc == 3:
        return "internal compiler error (exit code 3)"
    lines = [ln.strip() for ln in out.decode("latin-1").splitlines() if ln.strip()]
    m = SUMMARY_RE.match(lines[-1]) if lines else None
    if not m:
        return f"exit code {rc} without an error summary (not an error in the program)"
    total, marked = int(m.group(1)), int(m.group(2) or 0)
    if total <= marked:
        return f"only errors marked not supported yet ({lines[-1]})"
    return ""


def clear_temp(qb_root: Path) -> None:
    """Empties the clone's internal/temp before a qb64pe compile, as the bash runners do."""
    temp = qb_root / "internal" / "temp"
    if not temp.is_dir():
        return
    for p in temp.iterdir():
        # temp.bin is tracked in the clone (the Makefile's clean keeps it too); qb64pe.exe rewrites
        # it, another compiler would not, and deleting it would change the clone.
        if p.name == "temp.bin":
            continue
        if p.is_dir():
            shutil.rmtree(p, ignore_errors=True)
        else:
            try:
                p.unlink()
            except OSError:
                pass


# qb64pe several at a time (--jobs with qb64pe, corpus suite). qb64pe builds inside its own folder:
# the generated C++ in internal/temp, the libqb objects in internal/c. Instances started from one
# folder would share the objects, and the old compiler's answer is what gets recorded as truth, so
# each worker compiles with a private copy of the compiler instead: nothing is shared, and a copy
# compiles one program at a time, exactly as the clone does with one job.
COPY_STAMP = "copy-stamp.txt"


def written_by_builds(parts: tuple[str, ...], name: str) -> bool:
    """True for a file of internal/c (parts: its folder below internal/c) that a build writes, by
    either compiler: the libqb objects, qbx<N>.cpp and qbx*.o of the program being built, the
    generated header in parts/core/gl_header_for_parsing/temp. Such a file may change or vanish
    while another run builds in the clone, so a copy neither depends on it nor takes it: a copy
    builds its own libqb objects, once, with its first programs."""
    if "c_compiler" in parts:  # the C++ compiler's own objects and libraries: never written
        return False
    if "temp" in parts or name.endswith((".o", ".a", ".tmp")):
        return True
    return not parts and name.startswith("qbx") and name != "qbx.cpp"


def clone_settings(qb_root: Path) -> dict[str, bytes]:
    """The files of the clone's settings/, read once. config.ini holds options a compile follows
    (OptimizeCppProgram, ExtraCppFlags, ...), so a copy must have the clone's; qb64pe may rewrite the
    file, so the stamp goes by its bytes and the copy is written from the same bytes."""
    folder = qb_root / "settings"
    if not folder.is_dir():
        return {}
    return {
        p.relative_to(folder).as_posix(): p.read_bytes()
        for p in sorted(folder.rglob("*"))
        if p.is_file()
    }


def clone_stamp(qb_root: Path, settings: dict[str, bytes]) -> str:
    """A digest of what a copy is made from: qb64pe.exe, the Makefile and every file of internal/c
    by path, size and time (not what a build writes there), and the settings by content. A copy made
    from another state of the clone has another stamp and is made again."""
    internal_c = qb_root / "internal" / "c"
    files = [qb_root / "qb64pe.exe", qb_root / "Makefile"]
    for dirpath, dirnames, filenames in os.walk(internal_c):
        dirnames.sort()
        folder = Path(dirpath)
        parts = folder.relative_to(internal_c).parts
        files += [folder / f for f in sorted(filenames) if not written_by_builds(parts, f)]
    h = hashlib.sha256()
    for p in files:
        st = p.stat()
        h.update(f"{p.relative_to(qb_root).as_posix()}\t{st.st_size}\t{st.st_mtime_ns}\n".encode())
    for name, data in settings.items():
        h.update(f"settings/{name}\t{hashlib.sha256(data).hexdigest()}\n".encode())
    return h.hexdigest()


def make_copy(qb_root: Path, root: Path, stamp: str, settings: dict[str, bytes]) -> None:
    """Makes root a private copy of the compiler: qb64pe.exe, the Makefile, internal/ (without what
    builds write, written_by_builds) and settings/. The stamp is written last: a copy that was
    interrupted has none and is made again."""
    if root.exists():
        shutil.rmtree(root)
    root.mkdir(parents=True)
    for name in ("qb64pe.exe", "Makefile"):
        shutil.copy2(qb_root / name, root / name)
    internal = qb_root / "internal"

    def skip(folder: str, names: list[str]) -> list[str]:
        if Path(folder) == internal:  # temp, temp2, ...: build folders of earlier compiles
            return [n for n in names if n.startswith("temp")]
        try:
            parts = Path(folder).relative_to(internal / "c").parts
        except ValueError:  # internal/source, internal/support, ...
            return []
        return [n for n in names if written_by_builds(parts, n) and not (Path(folder) / n).is_dir()]

    shutil.copytree(internal, root / "internal", ignore=skip)
    for name, data in settings.items():
        target = root / "settings" / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
    (root / "internal" / "temp").mkdir()
    (root / COPY_STAMP).write_text(stamp, encoding="ascii")


class CopiesInUse(Exception):
    """Another run holds the folder of copies."""


class CompilerCopies:
    """The private copies of qb64pe for --jobs: <folder>/w1 ... w<count>, one per worker.

    One run at a time per folder (CopiesInUse otherwise): a second run would compile in the same
    copies, or remake them under the first."""

    def __init__(self, qb_root: Path, folder: Path, count: int, wait: bool):
        self._lock = lock_folder(
            folder, wait, f"waiting for another run to leave the copies in {folder}"
        )
        if self._lock is None:
            raise CopiesInUse(
                f"another run is using the copies in {folder}; start again when it has ended, "
                "pass --wait, or give this run its own --copies-dir"
            )
        settings = clone_settings(qb_root)
        stamp = clone_stamp(qb_root, settings)
        roots = [folder / f"w{k}" for k in range(1, count + 1)]
        stale = [
            r
            for r in roots
            if not (r / COPY_STAMP).is_file()
            or (r / COPY_STAMP).read_text(encoding="ascii") != stamp
        ]
        if stale:
            print(f"copying qb64pe for {len(stale)} of {count} workers to {folder}", flush=True)
            with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
                list(pool.map(lambda r: make_copy(qb_root, r, stamp, settings), stale))
        self._free: queue.SimpleQueue[Path] = queue.SimpleQueue()
        for r in roots:
            self._free.put(r)

    @contextlib.contextmanager
    def lease(self):
        """A copy no other worker is using, for the time of one compile."""
        root = self._free.get()
        try:
            yield root
        finally:
            self._free.put(root)

    def close(self) -> None:
        """Gives the folder back to other runs."""
        self._lock.close()


def copy_if_exists(src: Path, dst: Path) -> None:
    """Copies src to dst if src is a file; does nothing otherwise."""
    if src.is_file():
        shutil.copyfile(src, dst)


def kill_tree(p: subprocess.Popen) -> None:
    """Kills a process and all its children, then waits for it (at most 30 s).

    Popen.kill() ends only the direct child; the compiler's make/clang children or a program's own
    children would survive and keep files in internal/temp open."""
    if os.name == "nt":
        subprocess.run(
            ["taskkill", "/T", "/F", "/PID", str(p.pid)],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
    else:
        p.kill()
    try:
        p.wait(timeout=30)
    except subprocess.TimeoutExpired:
        pass


def run_proc(
    args,
    cwd,
    stdout_path: Path,
    timeout,
    env=None,
    merge_stderr=False,
    creationflags=0,
    stdin_path: Path | None = None,
    ask_first=False,
):
    """Runs a command with stdin from the null device (or from stdin_path) and stdout (and stderr
    if merge_stderr) to stdout_path; stderr is discarded otherwise.

    Returns (exit code, False), or (None, True) when the timeout killed the process tree. With
    ask_first (Windows) the process gets Ctrl+Break first and 5 s to end: it then writes out what
    it has printed so far (a prompt without a line end, say), which a killed process loses."""
    ask_first = ask_first and os.name == "nt"
    if ask_first:
        creationflags |= subprocess.CREATE_NEW_PROCESS_GROUP  # Ctrl+Break reaches only this group
    with contextlib.ExitStack() as stack:
        out = stack.enter_context(open(stdout_path, "wb"))
        stdin = stack.enter_context(open(stdin_path, "rb")) if stdin_path else subprocess.DEVNULL
        p = subprocess.Popen(
            args,
            cwd=cwd,
            stdout=out,
            stderr=subprocess.STDOUT if merge_stderr else subprocess.DEVNULL,
            stdin=stdin,
            env=env,
            creationflags=creationflags,
        )
        try:
            return p.wait(timeout=timeout), False
        except subprocess.TimeoutExpired:
            if ask_first:
                try:
                    p.send_signal(signal.CTRL_BREAK_EVENT)
                    p.wait(timeout=5)
                except (OSError, subprocess.TimeoutExpired):
                    pass
            kill_tree(p)
            return None, True


# One lock per test folder: a compile test's executable runs in its folder, as the bash runners run
# it, and some programs there use the same file names (tests/upstream/compile_tests/arrays: three
# OPEN "test"). With --jobs the builds run side by side, the programs of one folder one at a time.
_folder_locks: dict[Path, threading.Lock] = {}
_folder_locks_guard = threading.Lock()


def folder_lock(folder: Path) -> threading.Lock:
    """The lock of a test folder, made on first use."""
    with _folder_locks_guard:
        return _folder_locks.setdefault(folder, threading.Lock())


def compile_test(bas: Path, args, results: Path, qb_root: Path, os_tag: str) -> Result:
    """Runs one program of the compile suite by the rules of compile_tests.sh (module docstring)."""
    category = bas.parent.name
    name = bas.stem
    tdir = bas.parent
    exe = results / f"{category}-{name} - output.exe"
    compile_out = results / f"{category}-{name}-compile_result.txt"
    err_file = tdir / f"{name}.err"
    out_file = tdir / f"{name}.output"
    kind = "error" if err_file.is_file() else ("output" if out_file.is_file() else "compile-only")
    r = Result("compile_tests", f"{category}/{name}", kind)
    t0 = time.time()

    # Only qb64pe builds in the clone's internal/temp (qb64rust builds in a folder of its own, which
    # --jobs needs).
    if is_old_compiler(args.qb64):
        clear_temp(qb_root)
    # glob.escape: test and category names may contain glob characters (e.g. "parens()", "[").
    # A folder is a build folder qb64rust kept after a failed build.
    for stale in results.glob(glob.escape(f"{category}-{name} - output.exe") + "*"):
        if stale.is_dir():
            shutil.rmtree(stale, ignore_errors=True)
        else:
            stale.unlink(missing_ok=True)

    flags: list[str] = []
    flags_file = tdir / f"{name}.flags"
    if flags_file.is_file():
        flags += flags_file.read_text(encoding="latin-1").split()
    license_file = tdir / f"{name}.{os_tag}.license"
    check_license = license_file.is_file()
    if check_license:
        flags.append("-f:GenerateLicenseFile=true")
    noprompt = "y"
    np_file = tdir / f"{name}.noprompt"
    if np_file.is_file():
        noprompt = np_file.read_text(encoding="latin-1").strip()

    base = [
        "-f:OptimizeCppProgram=true",
        "-f:StripDebugSymbols=false",
        *flags,
        "-q",
        "-m",
        "-x",
        *include_root_args(args.qb64),
    ]
    if (tdir / f"{name}.compile-from-base").is_file():
        # relpath, not relative_to: with --compile-tests the program is outside qb_root.
        cmd = [str(args.qb64), *base, os.path.relpath(bas, qb_root), "-o", str(exe)]
        cwd = qb_root
    else:
        cmd = [str(args.qb64), *base, f"{name}.bas", "-o", str(exe)]
        cwd = tdir
    rc, timed_out = run_proc(cmd, cwd, compile_out, COMPILE_TIMEOUT)
    if is_old_compiler(args.qb64):
        copy_if_exists(
            qb_root / "internal" / "temp" / "compilelog.txt",
            results / f"{category}-{name}-compilelog.txt",
        )

    def fail(stage, detail=""):
        r.status, r.stage, r.detail = "FAIL", stage, detail
        r.seconds = round(time.time() - t0, 1)
        return r

    if timed_out:
        return fail("compile", "timeout")

    if kind == "error":
        if rc == 0:
            return fail("compile", "compiled successfully, expected an error")
        if exe.exists():
            return fail("exe exists", "exe produced although an error was expected")
        if not is_old_compiler(args.qb64):
            why = new_compiler_rejection(rc, compile_out.read_bytes())
            if why:
                return fail("error result", why)
        elif norm_text(err_file.read_bytes()) != norm_text(compile_out.read_bytes()):
            return fail("error result", "compiler output differs from .err")
        r.seconds = round(time.time() - t0, 1)
        return r

    if rc != 0:
        return fail("compile", f"exit code {rc}")
    if not exe.exists():
        return fail("exe exists", "no executable produced")
    if check_license:
        lic = Path(str(exe) + ".license.txt")
        got = lic.read_bytes() if lic.is_file() else b""
        if strip_trailing_newlines(license_file.read_bytes()).replace(
            b"\r", b""
        ) != strip_trailing_newlines(got).replace(b"\r", b""):
            return fail("license", "generated license text differs")
    if kind == "compile-only":
        r.seconds = round(time.time() - t0, 1)
        return r

    run_out = results / f"{category}-{name}-run-output.txt"
    env = dict(os.environ)
    env.update(
        {
            "QB64PE_NOPROMPT": noprompt,
            "QB64PE_LOG_HANDLERS": "file",
            "QB64PE_LOG_SCOPES": "qb64,libqb,libqb-image,libqb-audio",
            "QB64PE_LOG_FILE_PATH": str(results / f"{category}-{name}-log.txt"),
        }
    )
    with folder_lock(tdir):
        rc, timed_out = run_proc(
            [str(exe), str(results), f"{category}-{name}"],
            tdir,
            run_out,
            RUN_TIMEOUT,
            env=env,
            merge_stderr=True,
        )
    if timed_out:
        return fail("run", "timeout")
    if rc != 0:
        return fail("run", f"exit code {rc}")
    if norm_output(out_file.read_bytes()) != norm_output(run_out.read_bytes()):
        return fail("result", "program output differs from .output")
    r.seconds = round(time.time() - t0, 1)
    return r


def qbasic_test(bas: Path, args, results: Path, qb_root: Path) -> Result:
    """Compiles one program of the qbasic suite; it passes when the compile succeeds."""
    rel = bas.relative_to(qb_root / "tests" / "qbasic_testcases").as_posix()
    r = Result("qbasic_testcases", rel, "compile-only")
    t0 = time.time()
    clear_temp(qb_root)
    key = rel.replace("/", "-")[:-4]
    exe = results / f"{key}-output.exe"
    exe.unlink(missing_ok=True)
    rc, timed_out = run_proc(
        [str(args.qb64), "-x", str(bas.relative_to(qb_root)), "-o", str(exe)],
        qb_root,
        results / f"{key}-compile_result.txt",
        COMPILE_TIMEOUT,
    )
    copy_if_exists(
        qb_root / "internal" / "temp" / "compilelog.txt", results / f"{key}-compilelog.txt"
    )
    r.seconds = round(time.time() - t0, 1)
    if timed_out or rc != 0:
        r.status, r.stage = "FAIL", "compile"
        r.detail = "timeout" if timed_out else f"exit code {rc}"
    return r


def format_tests(bas: Path, args, results: Path, qb_root: Path) -> list[Result]:
    """Runs every variant in a format test's .flagmap; one result per variant."""
    category = bas.parent.name
    name = bas.stem
    tdir = bas.parent
    out: list[Result] = []
    flagmap = (tdir / f"{name}.flagmap").read_text(encoding="latin-1").replace("\r", "")
    for line in flagmap.split("\n"):
        if not line.strip():
            continue
        variant, *flags = line.split()
        r = Result("format_tests", f"{category}/{name}/{variant}", "format")
        t0 = time.time()
        clear_temp(qb_root)
        key = f"{category}-{name}-{variant}"
        output = results / f"{key}-output"
        output.unlink(missing_ok=True)
        rc, timed_out = run_proc(
            [str(args.qb64), "-y", "-m", *flags, f"{name}.bas", "-o", str(output)],
            tdir,
            results / f"{key}-compile_result.txt",
            COMPILE_TIMEOUT,
        )
        if timed_out or rc != 0:
            r.status, r.stage = "FAIL", "format"
            r.detail = "timeout" if timed_out else f"exit code {rc}"
        elif not output.is_file():
            r.status, r.stage, r.detail = "FAIL", "output exists", "no formatted output written"
        elif not (tdir / variant).is_file():
            r.status, r.stage, r.detail = "FAIL", "result", f"expected file {variant} missing"
        elif norm_text((tdir / variant).read_bytes()) != norm_text(output.read_bytes()):
            r.status, r.stage, r.detail = "FAIL", "result", "formatted output differs from expected"
        r.seconds = round(time.time() - t0, 1)
        out.append(r)
    return out


def load_normalize(path: Path) -> list[tuple[re.Pattern, str]]:
    """Reads a .normalize file (none if it does not exist).

    Lines: <regex><TAB><replacement>; '#' lines and blank lines are skipped."""
    rules: list[tuple[re.Pattern, str]] = []
    if path.is_file():
        for n, line in enumerate(path.read_text(encoding="latin-1").splitlines(), 1):
            if not line.strip() or line.startswith("#"):
                continue
            if "\t" not in line:
                raise ValueError(f"{path.name} line {n}: no tab between pattern and replacement")
            pattern, repl = line.split("\t", 1)
            rules.append((re.compile(pattern), repl))
    return rules


def apply_normalize(data: bytes, rules: list[tuple[re.Pattern, str]]) -> bytes:
    """Applies .normalize rules in order to each line of data.

    A line's CR (CRLF line ends) stays outside the text the rules see, so '$' anchors work and the
    recorded bytes keep their line ends."""
    if not rules:
        return data
    lines = []
    for line in data.split(b"\n"):
        cr = line.endswith(b"\r")
        text = (line[:-1] if cr else line).decode("latin-1")
        for pattern, repl in rules:
            text = pattern.sub(repl, text)
        lines.append(text.encode("latin-1") + (b"\r" if cr else b""))
    return b"\n".join(lines)


def reset_folder(folder: Path, keep: set[str]) -> None:
    """Removes everything a previous run of the program left behind, except the names in keep."""
    for p in folder.iterdir():
        if p.name in keep:
            continue
        if p.is_dir():
            shutil.rmtree(p)
        else:
            p.unlink()


def write_expected(path: Path, data: bytes, other: Path) -> str:
    """Writes a recorded result and removes the other kind of expected file.

    Returns what changed ("" if nothing did)."""
    old = path.read_bytes() if path.is_file() else None
    replaced_other = other.is_file()
    if replaced_other:
        other.unlink()
    if old == data and not replaced_other:
        return ""
    path.write_bytes(data)
    if old is None and not replaced_other:
        return f"recorded new {path.name}"
    if old is not None:
        return f"warning: {path.name} changed"
    return f"warning: {other.name} replaced by {path.name}"


def corpus_test(
    bas: Path, args, results: Path, qb_root: Path, corpus_root: Path, known: dict[str, str]
) -> Result:
    """Runs (or with --record, records) one program of the corpus suite (module docstring)."""
    group = bas.parent.relative_to(corpus_root).as_posix()
    name = bas.stem
    tdir = bas.parent
    key = f"{group.replace('/', '-')}-{name}"
    err_file = tdir / f"{name}.err"
    out_file = tdir / f"{name}.output"
    norun = (tdir / f"{name}.norun").is_file()
    present = [p.suffix for p in (out_file, err_file) if p.is_file()]
    if norun:
        present.append(".norun")
    kind = "compile-only" if norun else ("error" if err_file.is_file() else "output")
    r = Result("corpus", f"{group}/{name}", kind)
    t0 = time.time()

    def finish(status="PASS", stage="", detail=""):
        r.status, r.stage, r.detail = status, stage, detail
        r.seconds = round(time.time() - t0, 1)
        return r

    def fail(stage, detail=""):
        return finish("FAIL", stage, detail)

    # A known failure without an expected file (it hangs, or loops forever) cannot pass: compile it,
    # never run it.
    skip_run = not present and f"corpus:{group}/{name}" in known
    if len(present) > 1:
        return fail("expected", f"more than one of .output, .err, .norun: {', '.join(present)}")
    if not present and not args.record and not skip_run:
        return fail("expected", "no .output, .err or .norun (record it with --record)")
    rules = load_normalize(tdir / f"{name}.normalize")

    # A fresh folder per program: files it creates stay out of the repo and out of other tests.
    work = results / key
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True)
    shutil.copyfile(bas, work / bas.name)
    exe = work / f"{name}.exe"
    compile_out = results / f"{key}-compile_result.txt"

    old = is_old_compiler(args.qb64)
    flags = ["-f:OptimizeCppProgram=true"] if args.cpp_opt else []
    with contextlib.ExitStack() as stack:
        # Only qb64pe builds in its own internal/temp: the clone's, or with --jobs this worker's
        # private copy's (CompilerCopies).
        qb64, build_root = args.qb64, qb_root
        if old and args.copies:
            build_root = stack.enter_context(args.copies.lease())
            qb64 = build_root / "qb64pe.exe"
        if old:
            clear_temp(build_root)
        rc, timed_out = run_proc(
            [
                str(qb64),
                *flags,
                "-q",
                "-m",
                "-x",
                *include_root_args(args.qb64),
                bas.name,
                "-o",
                str(exe),
            ],
            work,
            compile_out,
            COMPILE_TIMEOUT,
        )
        if old:
            copy_if_exists(
                build_root / "internal" / "temp" / "compilelog.txt",
                results / f"{key}-compilelog.txt",
            )
    if timed_out:
        return fail("compile", "timeout")

    def passed(detail=""):
        shutil.rmtree(work, ignore_errors=True)
        return finish(detail=detail)

    if rc != 0:
        if exe.exists():
            return fail("exe exists", "exe produced although the compile failed")
        got = apply_normalize(compile_out.read_bytes(), rules)
        if skip_run:
            return fail("compile", f"exit code {rc}")
        if args.record and not norun:
            r.kind = "error"
            if not is_old_compiler(args.qb64):
                # A crash or a failed C++ build is not a rejection of the program: record only a
                # summary with a real error.
                why = new_compiler_rejection(rc, compile_out.read_bytes())
                if why:
                    return fail("record", f"not recorded: {why}")
            return passed(write_expected(err_file, got, out_file))
        if kind != "error":
            return fail("compile", f"exit code {rc}")
        if not is_old_compiler(args.qb64):
            why = new_compiler_rejection(rc, compile_out.read_bytes())
            return fail("error result", why) if why else passed()
        if norm_text(err_file.read_bytes()) != norm_text(got):
            return fail("error result", "compiler output differs from .err")
        return passed()

    if kind == "error" and not args.record:
        return fail("compile", "compiled successfully, expected an error")
    if not exe.exists():
        return fail("exe exists", "no executable produced")
    if norun:
        return passed()
    if skip_run:
        shutil.rmtree(work, ignore_errors=True)
        return fail("run", "not run: known failure with no expected file")

    r.kind = "output"
    noprompt = "y"
    np_file = tdir / f"{name}.noprompt"
    if np_file.is_file():
        noprompt = np_file.read_text(encoding="latin-1").strip()
    env = dict(os.environ)
    env.update(
        {
            "QB64PE_NOPROMPT": noprompt,
            "QB64PE_LOG_HANDLERS": "file",
            "QB64PE_LOG_SCOPES": "qb64,libqb,libqb-image,libqb-audio",
            "QB64PE_LOG_FILE_PATH": str(results / f"{key}-log.txt"),
        }
    )
    # <name>.stdin: the program's standard input is a copy of that file, kept beside the results so
    # that the program's own folder holds only what the program finds without the sidecar.
    stdin_file = tdir / f"{name}.stdin"
    stdin_copy = None
    if stdin_file.is_file():
        stdin_copy = results / f"{key}-stdin.txt"
        shutil.copyfile(stdin_file, stdin_copy)
    if os.name == "nt":
        # END waits for a console key event, which the null device never sends (press_any_key.py).
        cmd = [sys.executable, str(Path(__file__).with_name("press_any_key.py")), str(exe)]
        if stdin_copy:
            cmd.append(str(stdin_copy))
        flags = subprocess.CREATE_NO_WINDOW
    else:
        cmd, flags = [str(exe)], 0
    outputs: list[bytes] = []
    for run in range(1, 3 if args.record else 2):
        reset_folder(work, {bas.name, exe.name})
        run_out = results / f"{key}-run{run}-output.txt"
        rc, timed_out = run_proc(
            cmd,
            work,
            run_out,
            CORPUS_RUN_TIMEOUT,
            env=env,
            merge_stderr=True,
            creationflags=flags,
            stdin_path=stdin_copy if os.name != "nt" else None,
        )
        if timed_out:
            return fail("run", "timeout")
        if rc != 0:
            return fail("run", f"exit code {rc}")
        outputs.append(apply_normalize(run_out.read_bytes(), rules))

    if args.record:
        if outputs[0] != outputs[1]:
            return fail("record", "non-deterministic: the two runs differ after normalisation")
        return passed(write_expected(out_file, outputs[0], err_file))
    if norm_output(out_file.read_bytes()) != norm_output(outputs[0]):
        return fail("result", "program output differs from .output")
    return passed()


def bash_exit_code(rc: int) -> int:
    """A Windows exit code as Git Bash's $? reports it (measured 2026-10-10 with ExitProcess).

    The verification files were recorded by a shell script, so their "exit=" lines hold these
    values: the low byte of an ordinary code; 139, 132 and 130 for an access violation, an illegal
    instruction and Ctrl+C; 127 for every other code from 0xC0000000 (e.g. an integer division
    overflow, a stack overflow)."""
    rc &= 0xFFFFFFFF
    if rc < 0xC0000000:
        return rc & 0xFF
    return {0xC0000005: 139, 0xC000001D: 132, 0xC000013A: 130}.get(rc, 127)


def git_sees_text(data: bytes) -> bool:
    """Git's test for text=auto (convert.c, gather_stats and convert_is_binary): no NUL, no CR
    without LF, and at most one control character per 128 other characters."""
    if b"\0" in data or b"\r" in data.replace(b"\r\n", b""):
        return False
    body = data[:-1] if data.endswith(b"\x1a") else data  # a final Ctrl+Z is not counted
    odd = sum(1 for c in body if c == 127 or (c < 32 and c not in b"\b\t\n\r\f\x1b"))
    printable = len(body) - odd - body.count(b"\r") - body.count(b"\n")
    return (printable >> 7) >= odd


def write_recorded(path: Path, data: bytes) -> str:
    """Writes a recorded verification file unless it already holds data.

    A text file that differs only by CRLF against LF is left alone: the compiler and the programs
    write CRLF, and git stores these files with LF (.gitattributes: text=auto). A file git takes
    for binary is stored as it is, so it is compared byte for byte. Returns what changed ("" if
    nothing did)."""
    old = path.read_bytes() if path.is_file() else None
    if old == data:
        return ""
    if (
        old is not None
        and git_sees_text(old)
        and git_sees_text(data)
        and old.replace(b"\r\n", b"\n") == data.replace(b"\r\n", b"\n")
    ):
        return ""
    path.write_bytes(data)
    return f"recorded new {path.name}" if old is None else f"warning: {path.name} changed"


def verification_test(bas: Path, args, results: Path, qb_root: Path) -> Result:
    """Records one program of the verification suite (module docstring)."""
    name = bas.stem
    tdir = bas.parent
    r = Result("verification", name, "recorded")
    t0 = time.time()
    exe = tdir / f"{name}.exe"
    exe.unlink(missing_ok=True)
    clear_temp(qb_root)
    env = dict(os.environ)
    env["QB64PE_NOPROMPT"] = "y"
    # Both go to the results folder first: a compile that times out leaves the recorded files alone.
    compile_out = results / f"{name}-compile.txt"
    rc, timed_out = run_proc(
        [str(args.qb64), "-x", "-q", "-m", bas.name, "-o", str(exe)],
        tdir,
        compile_out,
        COMPILE_TIMEOUT,
        env=env,
        merge_stderr=True,
    )
    if timed_out:
        r.status, r.stage, r.detail = "FAIL", "compile", "timeout; nothing recorded"
        r.seconds = round(time.time() - t0, 1)
        return r

    # The compiler names an included file by its full path; no local path goes into the repo
    # (CLAUDE.md, rule 2). Any spelling of the folder: either slash, any letter case.
    folder = re.compile(
        rb"[\\/]+".join(re.escape(p.encode()) for p in re.split(r"[\\/]+", str(tdir))),
        re.IGNORECASE,
    )

    def masked(data: bytes) -> bytes:
        return folder.sub(b"<verification>", data)

    changes = [
        write_recorded(
            tdir / f"{name}.compile.txt",
            masked(compile_out.read_bytes()) + f"exit={bash_exit_code(rc)}\n".encode(),
        )
    ]
    if exe.is_file():
        np_file = tdir / f"{name}.noprompt"
        if np_file.is_file():
            env["QB64PE_NOPROMPT"] = np_file.read_text(encoding="latin-1").strip()
        stdin_file = tdir / f"{name}.stdin"
        run_out = results / f"{name}-out.txt"
        # A hung program (e.g. screen PRINT with a comma under a redirected $CONSOLE) would block
        # forever.
        rc, timed_out = run_proc(
            [str(exe)],
            tdir,
            run_out,
            VERIFICATION_RUN_TIMEOUT,
            env=env,
            merge_stderr=True,
            stdin_path=stdin_file if stdin_file.is_file() else None,
            ask_first=True,  # as `timeout` did under run.sh: the pending prompt is recorded
        )
        out = masked(run_out.read_bytes())
        if timed_out:
            out += f"(timed out after {VERIFICATION_RUN_TIMEOUT} s)\nexit=124\n".encode()
        else:
            out += f"exit={bash_exit_code(rc)}\n".encode()
    else:
        out = b"(not compiled)\n"
    changes.append(write_recorded(tdir / f"{name}.out.txt", out))
    r.detail = "; ".join(c for c in changes if c)
    r.seconds = round(time.time() - t0, 1)
    return r


def load_list(path: Path, root: Path) -> set[Path]:
    """Reads a --list file into the resolved paths of the programs it names.

    Lines: <group>/<name> (no .bas) relative to root (the corpus, or the compile suite's
    tests/compile_tests); '#' starts a comment. Every named program must exist (ValueError)."""
    wanted: set[Path] = set()
    for n, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        bas = (root / f"{line}.bas").resolve()
        if not bas.is_file():
            raise ValueError(f"{path.name} line {n}: no program {line}.bas in {root.name}")
        wanted.add(bas)
    return wanted


def load_known_failures(path: Path) -> dict[str, str]:
    """Reads known_failures.txt into {"<suite>:<test>": reason} (empty if it does not exist).

    Lines: <suite>:<test>  <reason...>; '#' starts a comment."""
    known: dict[str, str] = {}
    if path.is_file():
        for line in path.read_text(encoding="utf-8").splitlines():
            line = line.split("#", 1)[0].strip()
            if line:
                key, _, reason = line.partition(" ")
                known[key] = reason.strip()
    return known


def qbasic_sources(qb_root: Path) -> list[Path]:
    """The programs of the qbasic suite.

    Same file set as qbasic_tests.sh: recursive for n54, pete, qb45com, thebob; top level only for
    open_gl and misc."""
    base = qb_root / "tests" / "qbasic_testcases"
    files: list[Path] = []
    for d in ("n54", "pete", "qb45com", "thebob"):
        files += sorted((base / d).rglob("*.bas"))
    for d in ("open_gl", "misc"):
        files += sorted((base / d).glob("*.bas"))
    return files


def run_suite_test(
    suite: str,
    bas: Path,
    *,
    args,
    results: Path,
    qb_root: Path,
    corpus_root: Path,
    known: dict[str, str],
) -> list[Result]:
    """Runs one program of a suite; an exception in the runner becomes a FAIL result."""
    try:
        if suite == "compile":
            return [compile_test(bas, args, results, qb_root, args.os_tag)]
        if suite == "format":
            return format_tests(bas, args, results, qb_root)
        if suite == "corpus":
            return [corpus_test(bas, args, results, qb_root, corpus_root, known)]
        if suite == "verification":
            return [verification_test(bas, args, results, qb_root)]
        return [qbasic_test(bas, args, results, qb_root)]
    # One broken test must not lose a long run's results. Named like the suite's own results, so
    # that known_failures.txt entries match.
    except Exception as e:  # noqa: BLE001  # pylint: disable=broad-exception-caught
        if suite == "corpus":
            name = ("corpus", bas.relative_to(corpus_root).with_suffix("").as_posix())
        elif suite == "compile":
            name = ("compile_tests", f"{bas.parent.name}/{bas.stem}")
        elif suite == "format":
            # Per program, not per variant: the variants are unknown when the runner fails.
            name = ("format_tests", f"{bas.parent.name}/{bas.stem}")
        elif suite == "verification":
            name = ("verification", bas.stem)
        else:
            name = (
                "qbasic_testcases",
                bas.relative_to(qb_root / "tests" / "qbasic_testcases").as_posix(),
            )
        return [Result(*name, "?", "FAIL", "runner", f"{type(e).__name__}: {e}")]


def run_all(tests: list[Path], run_one, jobs: int):
    """Yields (index in tests, run_one's results): in order with one job; with more, as they finish
    (the first program alone, so that a libqb object the clone lacks is built once, not by several
    `make`s at a time)."""
    if jobs == 1 or len(tests) == 1:
        yield from enumerate(map(run_one, tests))
        return
    yield 0, run_one(tests[0])
    pool = concurrent.futures.ThreadPoolExecutor(max_workers=jobs)
    try:
        futures = {pool.submit(run_one, bas): i for i, bas in enumerate(tests[1:], 1)}
        for f in concurrent.futures.as_completed(futures):
            yield futures[f], f.result()
    finally:
        # On Ctrl+C (or the caller stopping early) drop the programs not yet started; only the
        # running ones finish.
        pool.shutdown(wait=True, cancel_futures=True)


def main() -> int:
    """Runs the selected suites and writes the results file.

    Returns 0 if no test failed (KFAIL does not count), 1 if one did, one passed only alone after
    failing with --jobs, or the copies for --jobs could not be made, 2 for a usage error, 3 when another run holds the results folder, the
    clone or the copies (BUSY_EXIT)."""
    here = Path(__file__).resolve()
    default_qb_root = here.parents[3] / "QB64pe"
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument(
        "--qb-root",
        type=Path,
        default=default_qb_root,
        help="QB64pe checkout holding tests/ and internal/ (default: %(default)s)",
    )
    ap.add_argument("--qb64", type=Path, help="compiler executable (default: <qb-root>/qb64pe.exe)")
    ap.add_argument(
        "--suite",
        choices=["compile", "qbasic", "format", "corpus", "verification", "all"],
        default="compile",
        help="'all' is the four test suites; verification records and is run on its own",
    )
    ap.add_argument(
        "names",
        nargs="*",
        help="verification: the programs to record, without .bas (default: all, or --glob)",
    )
    ap.add_argument(
        "--verification-root",
        type=Path,
        default=here.parents[2] / "verification",
        help="folder of the verification programs (default: %(default)s)",
    )
    ap.add_argument(
        "--category", help="only this category (compile and format suites) or group (corpus)"
    )
    ap.add_argument("--glob", default="*.bas", help="file-name pattern within the category")
    ap.add_argument(
        "--results",
        type=Path,
        default=here.parents[2] / "target" / "legacy-tests",
        help="where exes, logs and results.json go (default: %(default)s)",
    )
    ap.add_argument("--os-tag", default="win", help="OS tag for .<os>.license files")
    ap.add_argument(
        "--known-failures",
        type=Path,
        default=here.parent / "known_failures.txt",
        help="tests whose failure does not fail the run (default: %(default)s)",
    )
    ap.add_argument(
        "--corpus-root",
        type=Path,
        default=here.parents[2] / "tests" / "corpus",
        help="golden corpus for --suite corpus (default: %(default)s)",
    )
    ap.add_argument(
        "--record",
        action="store_true",
        help="corpus: write .output/.err from this compiler instead of comparing",
    )
    ap.add_argument(
        "--cpp-opt",
        action="store_true",
        help="corpus: build with -f:OptimizeCppProgram=true (the -O2 report)",
    )
    ap.add_argument(
        "--compile-tests",
        type=Path,
        help="compile suite's test folder (default: <qb-root>/tests/compile_tests; "
        "CI passes the copy tests/upstream/compile_tests, which has no binary "
        "assets)",
    )
    ap.add_argument(
        "--list",
        type=Path,
        help="corpus and compile: run only the programs named in this file "
        "(<group>/<name> per line)",
    )
    ap.add_argument(
        "--jobs",
        type=int,
        default=1,
        help="corpus and compile: run this many programs at a time. With qb64pe: corpus "
        "only, each worker compiling with a private copy of the compiler (--copies-dir)",
    )
    ap.add_argument(
        "--copies-dir",
        type=Path,
        default=here.parents[2] / "target" / "qb64pe-copies",
        help="where --jobs keeps the private copies of qb64pe, about 800 MB each "
        "(default: %(default)s)",
    )
    ap.add_argument(
        "--wait",
        action="store_true",
        help="when another run is using the results folder, building in the reference clone or "
        "using the copies, wait for it to end instead of stopping with exit code 3",
    )
    ap.add_argument(
        "--build-cache",
        type=Path,
        help="with qb64rust: a folder of built executables to reuse when a program's "
        "C++ build is the same as an earlier one's (QB64RUST_BUILD_CACHE; the "
        "program is still compiled, run and compared). Not for CI, which builds "
        "every program",
    )
    args = ap.parse_args()
    if args.jobs < 1:
        print("--jobs must be 1 or more", file=sys.stderr)
        return 2
    if args.category and args.suite in ("qbasic", "verification", "all"):
        print("--category applies to the compile, format and corpus suites only", file=sys.stderr)
        return 2
    if args.names and args.suite != "verification":
        print("program names apply to --suite verification only", file=sys.stderr)
        return 2
    if args.names and args.glob != "*.bas":
        print("give program names or --glob, not both", file=sys.stderr)
        return 2
    if (args.record or args.cpp_opt) and args.suite != "corpus":
        print("--record and --cpp-opt apply to --suite corpus only", file=sys.stderr)
        return 2
    if args.list and args.suite not in ("corpus", "compile"):
        print("--list applies to --suite corpus and --suite compile only", file=sys.stderr)
        return 2
    if args.record and args.cpp_opt:
        print(
            "--record uses the default build; it cannot be combined with --cpp-opt", file=sys.stderr
        )
        return 2
    known = load_known_failures(args.known_failures)

    qb_root = args.qb_root.resolve()
    args.qb64 = (args.qb64 or qb_root / "qb64pe.exe").resolve()
    if not args.qb64.is_file():
        print(f"compiler not found: {args.qb64}", file=sys.stderr)
        return 2
    if args.suite == "verification" and not is_old_compiler(args.qb64):
        print("--suite verification records the old compiler: it needs qb64pe", file=sys.stderr)
        return 2
    if args.jobs > 1 and (args.record or args.suite not in ("corpus", "compile")):
        print("--jobs above 1 needs --suite corpus or compile, and no --record", file=sys.stderr)
        return 2
    # The compile suite's programs include files relative to the compiler's folder and some compile
    # from the clone's root; a copy has neither.
    if (
        args.jobs > 1
        and is_old_compiler(args.qb64)
        and (args.suite != "corpus" or args.qb64 != qb_root / "qb64pe.exe")
    ):
        print(
            "--jobs above 1 with qb64pe needs --suite corpus and the qb64pe.exe of --qb-root",
            file=sys.stderr,
        )
        return 2
    args.copies = None  # set per suite below
    if args.build_cache:
        if is_old_compiler(args.qb64):
            print("--build-cache applies to qb64rust only", file=sys.stderr)
            return 2
        # Inherited by every compile this run starts (crates/driver/src/build.rs).
        os.environ["QB64RUST_BUILD_CACHE"] = str(args.build_cache.resolve())

    compile_root = (args.compile_tests or qb_root / "tests" / "compile_tests").resolve()
    all_results: list[Result] = []
    known_passed: list[str] = []
    retried_passed: list[str] = []
    corpus_root = args.corpus_root.resolve()
    suites = ["compile", "qbasic", "format", "corpus"] if args.suite == "all" else [args.suite]
    # The tests of every suite are selected before any lock is taken: a usage error is reported at
    # once, not after waiting for another run (--wait).
    selected: list[tuple[str, Path, list[Path]]] = []  # suite, its results folder, its tests
    for suite in suites:
        results = (args.results / (suite + ("-cpp-opt" if args.cpp_opt else ""))).resolve()
        wanted = None
        if suite == "corpus":
            root = corpus_root / args.category if args.category else corpus_root
            if not root.is_dir():
                print(f"no such corpus folder: {root}", file=sys.stderr)
                return 2
            tests = sorted(p for p in root.rglob("*.bas") if fnmatch.fnmatch(p.name, args.glob))
            if args.list:
                try:
                    wanted = load_list(args.list, corpus_root)
                except (ValueError, OSError) as e:
                    print(e, file=sys.stderr)
                    return 2
                tests = [p for p in tests if p in wanted]
        elif suite in ("compile", "format"):
            if suite == "compile":
                root = compile_root
            else:
                root = qb_root / "tests" / "format_tests"
            if args.category:
                root = root / args.category
            if not root.is_dir():
                print(f"no such test folder: {root}", file=sys.stderr)
                return 2
            tests = sorted(p for p in root.rglob("*.bas") if fnmatch.fnmatch(p.name, args.glob))
            if args.list and suite == "compile":
                try:
                    wanted = load_list(args.list, compile_root)
                except (ValueError, OSError) as e:
                    print(e, file=sys.stderr)
                    return 2
                tests = [p for p in tests if p.resolve() in wanted]
        elif suite == "verification":
            root = args.verification_root.resolve()
            if args.names:
                tests = [root / f"{n.removesuffix('.bas')}.bas" for n in args.names]
                missing = [p.name for p in tests if not p.is_file()]
                if missing:
                    print(f"no such verification program: {', '.join(missing)}", file=sys.stderr)
                    return 2
            else:
                tests = sorted(p for p in root.glob("*.bas") if fnmatch.fnmatch(p.name, args.glob))
        else:
            tests = qbasic_sources(qb_root)
        if not tests and wanted is not None and not wanted:
            # A pass list may start empty (tests/differential/pass.list grows as the compiler
            # catches up). Only a list that names nothing passes: one whose programs --category or
            # --glob filtered out is an error below.
            print(f"{args.list.name} names no program in suite {suite}: nothing to run", flush=True)
            continue
        if not tests:
            print(f"no tests selected in suite {suite}", file=sys.stderr)
            return 2
        selected.append((suite, results, tests))

    # One run at a time per results folder: a program is built and run in <results>/<its key>, which
    # the runner removes first, so two runs there would delete each other's folders and run each
    # other's executables. Taken before the clone's lock by every run, so no two runs wait for each
    # other. Held until the process ends.
    results_locks = []
    for _, results, _ in selected:
        lock = lock_folder(results, args.wait, f"waiting for another run to leave {results}")
        if lock is None:
            print(
                f"another run is using the results in {results}; start again when it has ended, "
                "pass --wait, or give this run its own --results",
                file=sys.stderr,
            )
            return BUSY_EXIT
        results_locks.append(lock)
    # One run at a time builds in the clone (clone_lock.py): qb64pe in its internal/temp, and
    # qb64rust's make runs there too and builds the libqb objects it lacks. Only a qb64pe run with
    # --jobs starts without the lock: it compiles in its copies and takes the lock when it comes back
    # to the clone.
    clone_lock = None
    if selected and not (is_old_compiler(args.qb64) and args.jobs > 1):
        clone_lock = lock_clone(args.wait)
        if clone_lock is None:
            print(CLONE_BUSY, file=sys.stderr)
            return BUSY_EXIT

    for suite, results, tests in selected:
        run_one = functools.partial(
            run_suite_test,
            suite,
            args=args,
            results=results,
            qb_root=qb_root,
            corpus_root=corpus_root,
            known=known,
        )
        # Printed as they finish, written in the order of tests: with --jobs the files of two runs
        # stay comparable.
        by_test: dict[int, list[Result]] = {}
        if args.jobs > 1 and len(tests) > 1 and is_old_compiler(args.qb64):
            try:
                args.copies = CompilerCopies(
                    qb_root, args.copies_dir.resolve(), min(args.jobs, len(tests)), args.wait
                )
            except CopiesInUse as e:
                print(e, file=sys.stderr)
                return BUSY_EXIT
            except (OSError, shutil.Error) as e:
                # Without its stamp the copy is made again by the next run. Not a usage error (2):
                # the command was right and the run failed.
                print(f"could not copy qb64pe to {args.copies_dir}: {e}", file=sys.stderr)
                return 1
        elif is_old_compiler(args.qb64) and clone_lock is None:  # --jobs with a single program
            clone_lock = lock_clone(args.wait)
            if clone_lock is None:
                print(CLONE_BUSY, file=sys.stderr)
                return BUSY_EXIT
        for i, (idx, rs) in enumerate(run_all(tests, run_one, args.jobs), 1):
            by_test[idx] = rs
            for r in rs:
                key = f"{r.suite}:{r.test}"
                if r.status == "FAIL" and key in known:
                    r.status = "KFAIL"
                    r.detail += f" (known: {known[key]})"
                elif r.status == "PASS" and key in known:
                    known_passed.append(key)
                line = f"[{i}/{len(tests)}] {r.status} {key} ({r.kind}, {r.seconds}s)"
                if r.status != "PASS":
                    line += f"  {r.stage}: {r.detail}"
                elif r.detail:  # corpus --record: what was written
                    line += f"  {r.detail}"
                print(line, flush=True)
                if suite == "verification" and r.status == "PASS":
                    # What was recorded, for reading along: the program's bytes as they are.
                    sys.stdout.buffer.write((tests[idx].with_suffix(".out.txt")).read_bytes())
                    sys.stdout.buffer.flush()
        if args.copies:
            # A failure beside other builds, in a copy, is not yet the old compiler's answer: each
            # one runs again alone in the clone, as with one job, and both outcomes are reported.
            args.copies.close()
            args.copies = None
            for idx in sorted(by_test):
                before = by_test[idx][0]
                if before.status != "FAIL":
                    continue
                if clone_lock is None:
                    # This late in a long run, wait for the clone (also without --wait).
                    clone_lock = lock_clone(wait=True)
                after = run_one(tests[idx])[0]
                first = f"{before.stage}: {before.detail}"
                if after.status == "PASS":
                    after.detail = f"passed alone in the clone; failed with --jobs ({first})"
                    retried_passed.append(f"{after.suite}:{after.test}")
                else:
                    after.detail += f" (alone in the clone; with --jobs: {first})"
                by_test[idx] = [after]
                print(
                    f"[again] {after.status} {after.suite}:{after.test} ({after.kind}, "
                    f"{after.seconds}s)  {after.stage + ': ' if after.stage else ''}{after.detail}",
                    flush=True,
                )
        for idx in sorted(by_test):
            all_results.extend(by_test[idx])

    summary: dict = {}
    for r in all_results:
        s = summary.setdefault(r.suite, {"PASS": 0, "FAIL": 0, "KFAIL": 0})
        s[r.status] += 1
    # A partial run must not overwrite the results of a fuller one.
    if args.category or args.glob != "*.bas" or args.list or args.names:
        name = "results-partial.json"
    else:
        mode = "-record" if args.record else ("-cpp-opt" if args.cpp_opt else "")
        name = "results.json" if args.suite == "all" else f"results-{args.suite}{mode}.json"
    out = args.results.resolve() / name
    out.parent.mkdir(parents=True, exist_ok=True)
    # Record the compiler relative to this repo so results do not reveal local absolute paths.
    # Only paths inside the repo's parent folder qualify; anything further away is reduced to
    # its file name (a relative path could still name user folders).
    try:
        rel = Path(os.path.relpath(args.qb64, here.parents[2]))
        compiler = rel.as_posix() if rel.parts.count("..") <= 1 else args.qb64.name
    except ValueError:  # different drive
        compiler = args.qb64.name
    out.write_text(
        json.dumps(
            {
                "compiler": compiler,
                "summary": summary,
                "results": [asdict(r) for r in all_results],
            },
            indent=1,
        )
    )
    print(json.dumps(summary))
    for key in known_passed:
        print(f"note: {key} passed but is listed in {args.known_failures.name}; remove the entry")
    for key in retried_passed:
        print(f"note: {key} failed with --jobs and passed alone in the clone; find out why")
    print(f"results: {out}")
    # A program that passed only alone counts as passed in the results, and still fails the run: the
    # copies gave another answer than the clone, which nobody reading only the exit code may miss.
    return 0 if all(s["FAIL"] == 0 for s in summary.values()) and not retried_passed else 1


if __name__ == "__main__":
    sys.exit(main())
