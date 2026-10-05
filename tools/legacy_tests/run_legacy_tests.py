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
For a compiler other than qb64pe (both the compile and the corpus suite), an .err program passes when
the compile fails, writes no exe and the compiler's last line is its error summary with at least one
error not marked "not supported yet" ("2 errors (1 not supported yet)"); the .err text is not compared.
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
    non-deterministic and nothing is written
  * --cpp-opt adds -f:OptimizeCppProgram=true and compares against the same files
  * --list <file> runs only the programs named in the file: one <group>/<name> per line
    (no .bas), '#' starts a comment; also for --suite compile, with <category>/<name>
    (e.g. tests/upstream/pass.list)
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
import re
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, asdict
from pathlib import Path

COMPILE_TIMEOUT = 900   # seconds; first compiles build libqb for each feature set
RUN_TIMEOUT = 120
CORPUS_RUN_TIMEOUT = 60


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


def is_old_compiler(qb64: Path) -> bool:
    """True for qb64pe, whose .err files hold its exact message text."""
    return qb64.stem.lower() == "qb64pe"


# The new compiler's last line after errors in the program (spec compiler/cli): "1 error", "3 errors",
# "3 errors (2 not supported yet)". Only the front end prints it, so a failed C++ build or an internal
# compiler error (exit code 3) never matches, even when clang's output holds "error:" lines.
SUMMARY_RE = re.compile(r"^(\d+) errors?(?: \((\d+) not supported yet\))?$")


def new_compiler_rejection(rc: int, out: bytes) -> str:
    """Why a failed compile by a compiler other than qb64pe does not count as a rejection, or "" if it does.

    An .err program passes when the compile fails, writes no executable (checked by the caller) and reports
    at least one error not marked "not supported yet" (CLAUDE.md, 2026-10-04); the message text is not
    compared."""
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
    temp = qb_root / "internal" / "temp"
    if not temp.is_dir():
        return
    for p in temp.iterdir():
        # temp.bin is tracked in the clone (the Makefile's clean keeps it too); qb64pe.exe rewrites it,
        # another compiler would not, and deleting it would change the clone.
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


def run_proc(args, cwd, stdout_path: Path, timeout, env=None, merge_stderr=False, creationflags=0):
    with open(stdout_path, "wb") as out:
        p = subprocess.Popen(
            args, cwd=cwd, stdout=out,
            stderr=subprocess.STDOUT if merge_stderr else subprocess.DEVNULL,
            stdin=subprocess.DEVNULL, env=env, creationflags=creationflags,
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

    base = ["-f:OptimizeCppProgram=true", "-f:StripDebugSymbols=false", *flags, "-q", "-m", "-x"]
    if (tdir / f"{name}.compile-from-base").is_file():
        # relpath, not relative_to: with --compile-tests the program is outside qb_root.
        cmd = [str(args.qb64), *base, os.path.relpath(bas, qb_root), "-o", str(exe)]
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


def load_normalize(path: Path) -> list[tuple[re.Pattern, str]]:
    # Lines: <regex><TAB><replacement>; '#' lines and blank lines are skipped.
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
    # Applied per line; a line's CR (CRLF line ends) stays outside the text the rules see,
    # so '$' anchors work and the recorded bytes keep their line ends.
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
    # Removes everything a previous run of the program left behind.
    for p in folder.iterdir():
        if p.name in keep:
            continue
        if p.is_dir():
            shutil.rmtree(p)
        else:
            p.unlink()


def write_expected(path: Path, data: bytes, other: Path) -> str:
    # Writes a recorded result; removes the other kind of expected file. Returns what changed.
    old = path.read_bytes() if path.is_file() else None
    replaced_other = other.is_file()
    if replaced_other:
        other.unlink()
    if old == data and not replaced_other:
        return ""
    path.write_bytes(data)
    if old is None and not replaced_other:
        return f"recorded new {path.name}"
    return f"warning: {path.name} changed" if old is not None else f"warning: {other.name} replaced by {path.name}"


def corpus_test(bas: Path, args, results: Path, qb_root: Path, corpus_root: Path,
                known: dict[str, str]) -> Result:
    group = bas.parent.relative_to(corpus_root).as_posix()
    name = bas.stem
    tdir = bas.parent
    key = f"{group.replace('/', '-')}-{name}"
    err_file = tdir / f"{name}.err"
    out_file = tdir / f"{name}.output"
    norun = (tdir / f"{name}.norun").is_file()
    present = [p.suffix for p in (out_file, err_file) if p.is_file()] + ([".norun"] if norun else [])
    kind = "compile-only" if norun else ("error" if err_file.is_file() else "output")
    r = Result("corpus", f"{group}/{name}", kind)
    t0 = time.time()

    def finish(status="PASS", stage="", detail=""):
        r.status, r.stage, r.detail = status, stage, detail
        r.seconds = round(time.time() - t0, 1)
        return r

    def fail(stage, detail=""):
        return finish("FAIL", stage, detail)

    # A known failure without an expected file (it hangs, or loops forever) cannot pass: compile it, never run it.
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

    clear_temp(qb_root)
    flags = ["-f:OptimizeCppProgram=true"] if args.cpp_opt else []
    rc, timed_out = run_proc([str(args.qb64), *flags, "-q", "-m", "-x", bas.name, "-o", str(exe)],
                             work, compile_out, COMPILE_TIMEOUT)
    copy_if_exists(qb_root / "internal" / "temp" / "compilelog.txt", results / f"{key}-compilelog.txt")
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
    env.update({
        "QB64PE_NOPROMPT": noprompt,
        "QB64PE_LOG_HANDLERS": "file",
        "QB64PE_LOG_SCOPES": "qb64,libqb,libqb-image,libqb-audio",
        "QB64PE_LOG_FILE_PATH": str(results / f"{key}-log.txt"),
    })
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
        rc, timed_out = run_proc(cmd, work, run_out, CORPUS_RUN_TIMEOUT, env=env, merge_stderr=True,
                                 creationflags=flags)
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
    # Lines: <group>/<name> (no .bas) relative to root (the corpus, or the compile suite's tests/compile_tests);
    # '#' starts a comment. Every named program must exist.
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
    ap.add_argument("--suite", choices=["compile", "qbasic", "format", "corpus", "all"], default="compile")
    ap.add_argument("--category", help="only this category (compile and format suites) or group (corpus)")
    ap.add_argument("--glob", default="*.bas", help="file-name pattern within the category")
    ap.add_argument("--results", type=Path, default=here.parents[2] / "target" / "legacy-tests",
                    help="where exes, logs and results.json go (default: %(default)s)")
    ap.add_argument("--os-tag", default="win", help="OS tag for .<os>.license files")
    ap.add_argument("--known-failures", type=Path, default=here.parent / "known_failures.txt",
                    help="tests whose failure does not fail the run (default: %(default)s)")
    ap.add_argument("--corpus-root", type=Path, default=here.parents[2] / "tests" / "corpus",
                    help="golden corpus for --suite corpus (default: %(default)s)")
    ap.add_argument("--record", action="store_true",
                    help="corpus: write .output/.err from this compiler instead of comparing")
    ap.add_argument("--cpp-opt", action="store_true",
                    help="corpus: build with -f:OptimizeCppProgram=true (the -O2 report)")
    ap.add_argument("--compile-tests", type=Path,
                    help="compile suite's test folder (default: <qb-root>/tests/compile_tests; CI passes the copy "
                         "tests/upstream/compile_tests, which has no binary assets)")
    ap.add_argument("--list", type=Path,
                    help="corpus and compile: run only the programs named in this file (<group>/<name> per line)")
    args = ap.parse_args()
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
        print("--record uses the default build; it cannot be combined with --cpp-opt", file=sys.stderr)
        return 2
    known = load_known_failures(args.known_failures)

    qb_root = args.qb_root.resolve()
    args.qb64 = (args.qb64 or qb_root / "qb64pe.exe").resolve()
    if not args.qb64.is_file():
        print(f"compiler not found: {args.qb64}", file=sys.stderr)
        return 2

    compile_root = (args.compile_tests or qb_root / "tests" / "compile_tests").resolve()
    all_results: list[Result] = []
    known_passed: list[str] = []
    corpus_root = args.corpus_root.resolve()
    suites = ["compile", "qbasic", "format", "corpus"] if args.suite == "all" else [args.suite]
    for suite in suites:
        results = (args.results / (suite + ("-cpp-opt" if args.cpp_opt else ""))).resolve()
        results.mkdir(parents=True, exist_ok=True)
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
        if not tests:
            print(f"no tests selected in suite {suite}", file=sys.stderr)
            return 2
        for i, bas in enumerate(tests, 1):
            try:
                if suite == "compile":
                    rs = [compile_test(bas, args, results, qb_root, args.os_tag)]
                elif suite == "format":
                    rs = format_tests(bas, args, results, qb_root)
                elif suite == "corpus":
                    rs = [corpus_test(bas, args, results, qb_root, corpus_root, known)]
                else:
                    rs = [qbasic_test(bas, args, results, qb_root)]
            except Exception as e:  # one broken test must not lose a long run's results
                if suite == "corpus":
                    rs = [Result("corpus", bas.relative_to(corpus_root).with_suffix("").as_posix(), "?",
                                 "FAIL", "runner", f"{type(e).__name__}: {e}")]
                elif suite == "compile":
                    # Named like compile_test's results: <category>/<name>.
                    rs = [Result("compile_tests", f"{bas.parent.name}/{bas.stem}", "?", "FAIL", "runner",
                                 f"{type(e).__name__}: {e}")]
                else:
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
                elif r.detail:  # corpus --record: what was written
                    line += f"  {r.detail}"
                print(line, flush=True)

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
