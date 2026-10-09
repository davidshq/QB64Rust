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
Speed (corpus and compile suites, a compiler other than qb64pe only; the C++ build is most of each
program's time):
  * --jobs N runs N programs at a time, after the first one alone (it builds any libqb object the
    clone lacks); results are printed as they finish. Compile tests of one folder run (not build)
    one at a time, since it is their working folder. qb64pe and --record run one at a time: qb64pe
    builds in the clone's shared internal/temp
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
import fnmatch
import functools
import glob
import json
import os
import re
import shutil
import subprocess
import sys
import threading
import time
from dataclasses import asdict, dataclass
from pathlib import Path

COMPILE_TIMEOUT = 900  # seconds; first compiles build libqb for each feature set
RUN_TIMEOUT = 120
CORPUS_RUN_TIMEOUT = 60


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


def run_proc(args, cwd, stdout_path: Path, timeout, env=None, merge_stderr=False, creationflags=0):
    """Runs a command with stdin from the null device and stdout (and stderr if merge_stderr) to
    stdout_path; stderr is discarded otherwise.

    Returns (exit code, False), or (None, True) when the timeout killed the process tree."""
    with open(stdout_path, "wb") as out:
        p = subprocess.Popen(
            args,
            cwd=cwd,
            stdout=out,
            stderr=subprocess.STDOUT if merge_stderr else subprocess.DEVNULL,
            stdin=subprocess.DEVNULL,
            env=env,
            creationflags=creationflags,
        )
        try:
            return p.wait(timeout=timeout), False
        except subprocess.TimeoutExpired:
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
    if old:  # only qb64pe builds in the clone's internal/temp
        clear_temp(qb_root)
    flags = ["-f:OptimizeCppProgram=true"] if args.cpp_opt else []
    rc, timed_out = run_proc(
        [
            str(args.qb64),
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
            qb_root / "internal" / "temp" / "compilelog.txt", results / f"{key}-compilelog.txt"
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
    if os.name == "nt":
        # END waits for a console key event, which the null device never sends (press_any_key.py).
        cmd = [sys.executable, str(Path(__file__).with_name("press_any_key.py")), str(exe)]
        flags = subprocess.CREATE_NO_WINDOW
    else:
        cmd, flags = [str(exe)], 0
    outputs: list[bytes] = []
    for run in range(1, 3 if args.record else 2):
        reset_folder(work, {bas.name, exe.name})
        run_out = results / f"{key}-run{run}-output.txt"
        rc, timed_out = run_proc(
            cmd, work, run_out, CORPUS_RUN_TIMEOUT, env=env, merge_stderr=True, creationflags=flags
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

    Returns 0 if no test failed (KFAIL does not count), 1 if one did, 2 for a usage error."""
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
        "--suite", choices=["compile", "qbasic", "format", "corpus", "all"], default="compile"
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
        help="corpus and compile, with a compiler other than qb64pe: run this many "
        "programs at a time (qb64pe builds in the clone's shared internal/temp, "
        "so it runs one at a time)",
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
    if args.category and args.suite in ("qbasic", "all"):
        print("--category applies to the compile, format and corpus suites only", file=sys.stderr)
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
    if args.jobs > 1 and (
        is_old_compiler(args.qb64) or args.record or args.suite not in ("corpus", "compile")
    ):
        print(
            "--jobs above 1 needs a compiler other than qb64pe, --suite corpus or compile, "
            "and no --record",
            file=sys.stderr,
        )
        return 2
    if args.build_cache:
        if is_old_compiler(args.qb64):
            print("--build-cache applies to qb64rust only", file=sys.stderr)
            return 2
        # Inherited by every compile this run starts (crates/driver/src/build.rs).
        os.environ["QB64RUST_BUILD_CACHE"] = str(args.build_cache.resolve())

    compile_root = (args.compile_tests or qb_root / "tests" / "compile_tests").resolve()
    all_results: list[Result] = []
    known_passed: list[str] = []
    corpus_root = args.corpus_root.resolve()
    suites = ["compile", "qbasic", "format", "corpus"] if args.suite == "all" else [args.suite]
    for suite in suites:
        results = (args.results / (suite + ("-cpp-opt" if args.cpp_opt else ""))).resolve()
        results.mkdir(parents=True, exist_ok=True)
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
        for idx in sorted(by_test):
            all_results.extend(by_test[idx])

    summary: dict = {}
    for r in all_results:
        s = summary.setdefault(r.suite, {"PASS": 0, "FAIL": 0, "KFAIL": 0})
        s[r.status] += 1
    # A partial run must not overwrite the results of a fuller one.
    if args.category or args.glob != "*.bas" or args.list:
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
    print(f"results: {out}")
    return 0 if all(s["FAIL"] == 0 for s in summary.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
