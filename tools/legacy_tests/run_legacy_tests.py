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
import fnmatch
import glob
import json
import os
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, asdict
from pathlib import Path

COMPILE_TIMEOUT = 900   # seconds; first compiles build libqb for each feature set
RUN_TIMEOUT = 120


@dataclass
class Result:
    suite: str
    test: str              # category/name, or the qbasic path
    kind: str              # output | error | compile-only | format
    status: str = "PASS"   # PASS | FAIL | KFAIL (failed, listed in known_failures.txt)
    stage: str = ""        # stage that failed
    detail: str = ""
    seconds: float = 0.0


def strip_trailing_newlines(data: bytes) -> bytes:
    # Mirrors filesize_without_trailing_newlines: remove LF, and a CR before it.
    while data.endswith(b"\n"):
        data = data[:-1]
        if data.endswith(b"\r"):
            data = data[:-1]
    return data


def norm_output(data: bytes) -> bytes:
    # Windows programs write CRLF; the clone's .output files may hold LF. Treat CRLF as LF,
    # keep bare CRs significant, then drop trailing line endings like the bash runner.
    return strip_trailing_newlines(data.replace(b"\r\n", b"\n"))


def norm_text(data: bytes) -> list[str]:
    return data.decode("latin-1").replace("\r", "").rstrip("\n").split("\n")


def clear_temp(qb_root: Path) -> None:
    temp = qb_root / "internal" / "temp"
    if not temp.is_dir():
        return
    for p in temp.iterdir():
        if p.is_dir():
            shutil.rmtree(p, ignore_errors=True)
        else:
            try:
                p.unlink()
            except OSError:
                pass


def copy_if_exists(src: Path, dst: Path) -> None:
    if src.is_file():
        shutil.copyfile(src, dst)


def kill_tree(p: subprocess.Popen) -> None:
    # Popen.kill() ends only the direct child; the compiler's make/clang children or a
    # program's own children would survive and keep files in internal/temp open.
    if os.name == "nt":
        subprocess.run(["taskkill", "/T", "/F", "/PID", str(p.pid)],
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    else:
        p.kill()
    try:
        p.wait(timeout=30)
    except subprocess.TimeoutExpired:
        pass


def run_proc(args, cwd, stdout_path: Path, timeout, env=None, merge_stderr=False):
    with open(stdout_path, "wb") as out:
        p = subprocess.Popen(
            args, cwd=cwd, stdout=out,
            stderr=subprocess.STDOUT if merge_stderr else subprocess.DEVNULL,
            stdin=subprocess.DEVNULL, env=env,
        )
        try:
            return p.wait(timeout=timeout), False
        except subprocess.TimeoutExpired:
            kill_tree(p)
            return None, True


def compile_test(bas: Path, args, results: Path, qb_root: Path, os_tag: str) -> Result:
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

    clear_temp(qb_root)
    # glob.escape: test and category names may contain glob characters (e.g. "parens()", "[").
    for stale in results.glob(glob.escape(f"{category}-{name} - output.exe") + "*"):
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

    base = ["-f:OptimizeCppProgram=true", "-f:StripDebugSymbols=false", *flags, "-q", "-m", "-x"]
    if (tdir / f"{name}.compile-from-base").is_file():
        cmd = [str(args.qb64), *base, str(bas.relative_to(qb_root)), "-o", str(exe)]
        cwd = qb_root
    else:
        cmd = [str(args.qb64), *base, f"{name}.bas", "-o", str(exe)]
        cwd = tdir
    rc, timed_out = run_proc(cmd, cwd, compile_out, COMPILE_TIMEOUT)
    copy_if_exists(qb_root / "internal" / "temp" / "compilelog.txt",
                   results / f"{category}-{name}-compilelog.txt")

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
        if norm_text(err_file.read_bytes()) != norm_text(compile_out.read_bytes()):
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
        if strip_trailing_newlines(license_file.read_bytes()).replace(b"\r", b"") != \
                strip_trailing_newlines(got).replace(b"\r", b""):
            return fail("license", "generated license text differs")
    if kind == "compile-only":
        r.seconds = round(time.time() - t0, 1)
        return r

    run_out = results / f"{category}-{name}-run-output.txt"
    env = dict(os.environ)
    env.update({
        "QB64PE_NOPROMPT": noprompt,
        "QB64PE_LOG_HANDLERS": "file",
        "QB64PE_LOG_SCOPES": "qb64,libqb,libqb-image,libqb-audio",
        "QB64PE_LOG_FILE_PATH": str(results / f"{category}-{name}-log.txt"),
    })
    rc, timed_out = run_proc([str(exe), str(results), f"{category}-{name}"], tdir, run_out,
                             RUN_TIMEOUT, env=env, merge_stderr=True)
    if timed_out:
        return fail("run", "timeout")
    if rc != 0:
        return fail("run", f"exit code {rc}")
    if norm_output(out_file.read_bytes()) != norm_output(run_out.read_bytes()):
        return fail("result", "program output differs from .output")
    r.seconds = round(time.time() - t0, 1)
    return r


def qbasic_test(bas: Path, args, results: Path, qb_root: Path) -> Result:
    rel = bas.relative_to(qb_root / "tests" / "qbasic_testcases").as_posix()
    r = Result("qbasic_testcases", rel, "compile-only")
    t0 = time.time()
    clear_temp(qb_root)
    key = rel.replace("/", "-")[:-4]
    exe = results / f"{key}-output.exe"
    exe.unlink(missing_ok=True)
    rc, timed_out = run_proc([str(args.qb64), "-x", str(bas.relative_to(qb_root)), "-o", str(exe)],
                             qb_root, results / f"{key}-compile_result.txt", COMPILE_TIMEOUT)
    copy_if_exists(qb_root / "internal" / "temp" / "compilelog.txt", results / f"{key}-compilelog.txt")
    r.seconds = round(time.time() - t0, 1)
    if timed_out or rc != 0:
        r.status, r.stage, r.detail = "FAIL", "compile", "timeout" if timed_out else f"exit code {rc}"
    return r


def format_tests(bas: Path, args, results: Path, qb_root: Path) -> list[Result]:
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
        rc, timed_out = run_proc([str(args.qb64), "-y", "-m", *flags, f"{name}.bas", "-o", str(output)],
                                 tdir, results / f"{key}-compile_result.txt", COMPILE_TIMEOUT)
        if timed_out or rc != 0:
            r.status, r.stage, r.detail = "FAIL", "format", "timeout" if timed_out else f"exit code {rc}"
        elif not output.is_file():
            r.status, r.stage, r.detail = "FAIL", "output exists", "no formatted output written"
        elif not (tdir / variant).is_file():
            r.status, r.stage, r.detail = "FAIL", "result", f"expected file {variant} missing"
        elif norm_text((tdir / variant).read_bytes()) != norm_text(output.read_bytes()):
            r.status, r.stage, r.detail = "FAIL", "result", "formatted output differs from expected"
        r.seconds = round(time.time() - t0, 1)
        out.append(r)
    return out


def load_known_failures(path: Path) -> dict[str, str]:
    # Lines: <suite>:<test>  <reason...>; '#' starts a comment.
    known: dict[str, str] = {}
    if path.is_file():
        for line in path.read_text(encoding="utf-8").splitlines():
            line = line.split("#", 1)[0].strip()
            if line:
                key, _, reason = line.partition(" ")
                known[key] = reason.strip()
    return known


def qbasic_sources(qb_root: Path) -> list[Path]:
    # Same file set as qbasic_tests.sh: recursive for n54, pete, qb45com, thebob;
    # top level only for open_gl and misc.
    base = qb_root / "tests" / "qbasic_testcases"
    files: list[Path] = []
    for d in ("n54", "pete", "qb45com", "thebob"):
        files += sorted((base / d).rglob("*.bas"))
    for d in ("open_gl", "misc"):
        files += sorted((base / d).glob("*.bas"))
    return files


def main() -> int:
    here = Path(__file__).resolve()
    default_qb_root = here.parents[3] / "QB64pe"
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--qb-root", type=Path, default=default_qb_root,
                    help="QB64pe checkout holding tests/ and internal/ (default: %(default)s)")
    ap.add_argument("--qb64", type=Path, help="compiler executable (default: <qb-root>/qb64pe.exe)")
    ap.add_argument("--suite", choices=["compile", "qbasic", "format", "all"], default="compile")
    ap.add_argument("--category", help="only this category (compile and format suites)")
    ap.add_argument("--glob", default="*.bas", help="file-name pattern within the category")
    ap.add_argument("--results", type=Path, default=here.parents[2] / "target" / "legacy-tests",
                    help="where exes, logs and results.json go (default: %(default)s)")
    ap.add_argument("--os-tag", default="win", help="OS tag for .<os>.license files")
    ap.add_argument("--known-failures", type=Path, default=here.parent / "known_failures.txt",
                    help="tests whose failure does not fail the run (default: %(default)s)")
    args = ap.parse_args()
    if args.category and args.suite in ("qbasic", "all"):
        print("--category applies to the compile and format suites only", file=sys.stderr)
        return 2
    known = load_known_failures(args.known_failures)

    qb_root = args.qb_root.resolve()
    args.qb64 = (args.qb64 or qb_root / "qb64pe.exe").resolve()
    if not args.qb64.is_file():
        print(f"compiler not found: {args.qb64}", file=sys.stderr)
        return 2

    all_results: list[Result] = []
    known_passed: list[str] = []
    suites = ["compile", "qbasic", "format"] if args.suite == "all" else [args.suite]
    for suite in suites:
        results = (args.results / suite).resolve()
        results.mkdir(parents=True, exist_ok=True)
        if suite in ("compile", "format"):
            root = qb_root / "tests" / ("compile_tests" if suite == "compile" else "format_tests")
            if args.category:
                root = root / args.category
            if not root.is_dir():
                print(f"no such test folder: {root}", file=sys.stderr)
                return 2
            tests = sorted(p for p in root.rglob("*.bas") if fnmatch.fnmatch(p.name, args.glob))
        else:
            tests = qbasic_sources(qb_root)
        if not tests:
            print(f"no tests selected in suite {suite}", file=sys.stderr)
            return 2
        for i, bas in enumerate(tests, 1):
            try:
                if suite == "compile":
                    rs = [compile_test(bas, args, results, qb_root, args.os_tag)]
                elif suite == "format":
                    rs = format_tests(bas, args, results, qb_root)
                else:
                    rs = [qbasic_test(bas, args, results, qb_root)]
            except Exception as e:  # one broken test must not lose a long run's results
                rs = [Result(f"{suite}_tests" if suite != "qbasic" else "qbasic_testcases",
                             bas.relative_to(qb_root / "tests").as_posix(), "?", "FAIL", "runner",
                             f"{type(e).__name__}: {e}")]
            for r in rs:
                key = f"{r.suite}:{r.test}"
                if r.status == "FAIL" and key in known:
                    r.status = "KFAIL"
                    r.detail += f" (known: {known[key]})"
                elif r.status == "PASS" and key in known:
                    known_passed.append(key)
                all_results.append(r)
                line = f"[{i}/{len(tests)}] {r.status} {key} ({r.kind}, {r.seconds}s)"
                if r.status != "PASS":
                    line += f"  {r.stage}: {r.detail}"
                print(line, flush=True)

    summary: dict = {}
    for r in all_results:
        s = summary.setdefault(r.suite, {"PASS": 0, "FAIL": 0, "KFAIL": 0})
        s[r.status] += 1
    # A partial run must not overwrite the results of a fuller one.
    if args.category or args.glob != "*.bas":
        name = "results-partial.json"
    else:
        name = "results.json" if args.suite == "all" else f"results-{args.suite}.json"
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
    out.write_text(json.dumps({
        "compiler": compiler,
        "summary": summary,
        "results": [asdict(r) for r in all_results],
    }, indent=1))
    print(json.dumps(summary))
    for key in known_passed:
        print(f"note: {key} passed but is listed in {args.known_failures.name}; remove the entry")
    print(f"results: {out}")
    return 0 if all(s["FAIL"] == 0 for s in summary.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
