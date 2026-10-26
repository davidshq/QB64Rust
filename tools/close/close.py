#!/usr/bin/env python3
"""The closing runs of a change (CLAUDE.md working rule 9), as a plan and a check.

    python tools/close/close.py --plan     # the commands, in order, each to be started by its own command
    python tools/close/close.py --check    # did each run, on the current build, and pass?

The tool runs nothing: each test run is started by its own command, so that tools/testtime sees it (rule 9). It
prints the commands with the log each one writes under target/close/, and then reads those logs. A step whose log
is missing, older than the build or than a changed source, or does not show a pass is named, and the exit code is
1. A failure that is known and not the change's is accepted with --known <test name>, and shown as such. A passing
step older than a later edit that cannot affect it is accepted with --not-repeated STEP[,STEP...]=REASON, shown
(rule 9: "say which runs were not repeated and why"); a failing one never is.

Whether the run without folding is needed follows from the files changed against HEAD (sema's constant evaluation
or a numeric conversion, rule 9); --nofold asks for it anyway. So does the coverage check of Civil War Strategy's
scripts (tools/coverage): only when the game's copy or its scripts, the screen-state file, cut_game.py, the coverage
tool, codegen-cpp, ir or driver changed; --coverage asks for it anyway. And the format runs (QB64pe's format tests
and the format oracle, tests/format): only when the formatter, its recordings, tool or settings, the parser, the
built-in table, sema's symbols or the driver's -y changed; --format asks for them anyway.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO / "tools" / "legacy_tests"))
from compiler_copy import read_header, sha256_of  # noqa: E402  # pylint: disable=wrong-import-position,import-error

LOGS = "target/close"
EXE = "target/release/qb64rust.exe"
RUNNER = "python tools/legacy_tests/run_legacy_tests.py"
NEW = f"--qb64 {EXE} --jobs 8 --build-cache target/build-cache"
CACHE = "--pass-cache target/pass-cache"

# Files whose change asks for the run without folding (rule 9: sema's constant evaluation, a numeric conversion).
NOFOLD_FILES = re.compile(r"^crates/sema/src/(consteval\.rs|literal\.rs|check/constants\.rs|check/auto\.rs|check/ops\.rs)$")
# Changed files that make a run stale: what the compiler and the tests are built from. Documents and the
# test-time record are written during the close and do not.
INPUTS = re.compile(r"^(crates/|tests/|tools/(legacy_tests|callsite|diagnostics|builtins|screen|coverage|format)/|Cargo\.(toml|lock)$)")
# insta's pending snapshots (`.pending-snap`, `.snap.new`) are written by the test runs themselves.
NOT_INPUTS = re.compile(r"(\.md$|^tools/testtime/|\.pending-snap$|\.snap\.new$)")
# Files whose change asks for the coverage check of Civil War Strategy's scripts (design D2 of cws-whole-game).
COVERAGE_FILES = re.compile(
    r"^(tests/corpus/screen/g12_game[./]|tests/programs/civil-war-strategy\.unreached$|tools/screen/(cut_game\.py|"
    r"screen_state\.cpp)$|tools/coverage/|crates/(codegen-cpp|ir|driver)/)"
)
# Files whose change asks for the formatter's runs, QB64pe's format tests and the format oracle (change
# m5b-formatter, task 7.1): the formatter, its recordings and tool, its settings, the tree it walks, the built-in
# spellings, the symbols it spells names from (and the checks that record them), and `-y`.
FORMAT_FILES = re.compile(
    r"^(crates/format/|tests/format/|tools/format/|crates/(syntax|builtins)/|crates/base/src/layout\.rs$|"
    r"crates/sema/src/(symbols\.rs|check/(places|decl)\.rs)$|crates/driver/src/(main|lib)\.rs$)"
)
# Changed inputs that reach a run only through the release build: a run of a copy is judged by the build's hash.
BUILT_FROM = r"^(crates/|Cargo\.(toml|lock)$)"


@dataclass
class Step:
    name: str
    command: str
    # How the log shows a pass; None: only that it ran (a report, a regeneration).
    judge: str | None
    # Before the build: a step that writes inputs of the build or of the runs after it (rule 9, "Order").
    before_build: bool = False
    # Run alone, not beside other runs of the new compiler.
    alone: bool = False
    # The files a step before the build writes: they make no log of a step before the build stale.
    writes: str | None = None
    # Compiles with a copy of the release build and names its hash and start in the log
    # (tools/legacy_tests/compiler_copy.py): judged by that hash, not by the log's time.
    copy: bool = False


def log(name: str) -> str:
    return f"{LOGS}/{name}.log"


def steps(nofold: bool, coverage: bool = False, layout: bool = False) -> list[Step]:
    out = [
        # The table of the old wording (from the messages of the release build there is) and the shrink-only lists
        # are written before the build: the build and tier 1 read them.
        Step(
            "wording",
            f"python tools/diagnostics/old_wording.py > {log('wording')} 2>&1",
            None,
            before_build=True,
            writes=r"^crates/base/src/old_wording\.tsv$",
        ),
        Step(
            "lists",
            f"QB64RUST_UPDATE_LISTS=1 cargo test -p qb64rust-driver --test inputs > {log('lists')} 2>&1",
            "cargo",
            before_build=True,
            writes=r"^tests/known_\w+\.list$",
        ),
        Step("build", f"cargo build --release > {log('build')} 2>&1", "build", before_build=True),
        # As CI runs it (`--locked`: a `Cargo.lock` that drifted fails), and past a failing test binary.
        Step("tier1", f"cargo test --workspace --locked --no-fail-fast > {log('tier1')} 2>&1", "cargo"),
        Step("fmt", f"cargo fmt --all --check > {log('fmt')} 2>&1", "empty"),
        Step("clippy", f"cargo clippy --all-targets > {log('clippy')} 2>&1", "clippy"),
        Step("wording-check", f"python tools/diagnostics/old_wording.py --check > {log('wording-check')} 2>&1", "wording"),
        Step(
            "slice",
            f"{RUNNER} --suite corpus {NEW} {CACHE} --list tests/corpus/slice.list > {log('slice')} 2>&1",
            "runner",
            copy=True,
        ),
        Step(
            "upstream",
            f"{RUNNER} --suite compile {NEW} {CACHE} --list tests/upstream/pass.list > {log('upstream')} 2>&1",
            "runner",
            copy=True,
        ),
        # The verification programs the new compiler passes (tests/verification), in place of the hand runs of a
        # family's accepted programs.
        Step(
            "verification",
            f"{RUNNER} --suite verification {NEW} {CACHE} --list tests/verification/pass.list > {log('verification')} 2>&1",
            "runner",
            copy=True,
        ),
        # QB64pe's add_prefix_test.sh: its $NOPREFIX converter built and run on its case (a few seconds). The
        # converter is in the clone, so the run takes its shared lock: --wait, beside a recording with qb64pe.
        Step(
            "addprefix", f"{RUNNER} --suite addprefix --qb64 {EXE} --wait > {log('addprefix')} 2>&1", "runner", copy=True
        ),
        # QB64pe's qbasic_tests.sh: every program of tests/qbasic_testcases compiled, all of them to build
        # (qbasic-testcases). They are read from the clone: its shared lock, --wait as above.
        Step(
            "qbasic",
            f"{RUNNER} --suite qbasic --qb64 {EXE} --jobs 8 --wait > {log('qbasic')} 2>&1",
            "runner",
            copy=True,
        ),
    ]
    if nofold:
        out.append(
            Step(
                "nofold",
                f"QB64RUST_NO_FOLD=1 {RUNNER} --suite corpus {NEW} --wait --list tests/corpus/slice.list > {log('nofold')} 2>&1",
                "runner",
                copy=True,
            )
        )
    if coverage:
        # Every script of g12_game built with --coverage and run, then the check against the list of unreached lines.
        out.append(
            Step("coverage", f"python tools/coverage/coverage.py --check > {log('coverage')} 2>&1", "coverage", copy=True)
        )
    if layout:
        # QB64pe's own format tests, and every recording of the format oracle (tests/format) compared.
        out += [
            Step("format", f"{RUNNER} --suite format --qb64 {EXE} > {log('format')} 2>&1", "runner", copy=True),
            Step(
                "format-corpus",
                f"{RUNNER} --suite format-corpus --qb64 {EXE} --jobs 8 > {log('format-corpus')} 2>&1",
                "runner",
                copy=True,
            ),
        ]
    out += [
        Step("callsite", f"python tools/callsite/callsite.py --wait > {log('callsite')} 2>&1", "callsite", copy=True),
        Step("cli", f"cargo test -p qb64rust-driver --test cli -- --ignored > {log('cli')} 2>&1", "cargo", alone=True),
        Step("demand-corpus", f"{EXE} --report unsupported tests/corpus tests/programs > {log('demand-corpus')} 2>&1", None),
        Step(
            "demand-upstream",
            f"{EXE} --report unsupported --include-root tests/upstream/root tests/upstream/compile_tests > {log('demand-upstream')} 2>&1",
            None,
        ),
        Step("counts", f"python tools/legacy_tests/counts.py > {log('counts')} 2>&1", None),
    ]
    return out


def changed_files() -> list[str]:
    """Files changed against HEAD, staged or not, and untracked ones, repo-relative with forward slashes."""
    out = subprocess.run(
        ["git", "status", "--porcelain", "--untracked-files=all"], cwd=REPO, capture_output=True, text=True, check=True
    ).stdout
    files = []
    for line in out.splitlines():
        path = line[3:]
        if " -> " in path:
            path = path.split(" -> ", 1)[1]
        files.append(path.strip('"'))
    return files


def needs_nofold(files: list[str]) -> bool:
    return any(NOFOLD_FILES.match(f) for f in files)


def needs_coverage(files: list[str]) -> bool:
    return any(COVERAGE_FILES.match(f) for f in files)


def needs_format(files: list[str]) -> bool:
    return any(FORMAT_FILES.match(f) for f in files)


def judge(kind: str | None, text: str, known: set[str]) -> tuple[bool, str]:
    """Whether a log shows a pass, and a short summary."""
    if kind is None:
        return True, "ran"
    if kind == "empty":
        return (text.strip() == "", "clean" if text.strip() == "" else "differences")
    if kind == "build":
        bad = re.search(r"^error", text, re.M)
        return (not bad, "built" if not bad else "build failed")
    if kind == "clippy":
        n = len(re.findall(r"^(warning|error)", text, re.M))
        return (n == 0, f"{n} warnings or errors")
    if kind == "cargo":
        results = re.findall(r"^test result: \w+\. (\d+) passed; (\d+) failed", text, re.M)
        # A doctest's name holds spaces (`src\lib.rs - f (line 3)`).
        failed = set(re.findall(r"^test (.+?) \.\.\. FAILED$", text, re.M))
        if re.search(r"^error(\[E\d+\]|: could not compile)", text, re.M):
            return False, "a test target failed to build"
        if not results:
            return False, "no test result"
        # Each test binary and each crate's doctests print a header and, unless the process stopped (a crash, an
        # abort), a result.
        headers = len(re.findall(r"^\s+(Running|Doc-tests) ", text, re.M))
        if headers > len(results):
            return False, f"{headers - len(results)} test binaries stopped without a result"
        passed = sum(int(p) for p, _ in results)
        count = sum(int(f) for _, f in results)
        if count > len(failed):
            return False, f"{passed} passed, {count} failed, {count - len(failed)} of them not named in the log"
        unknown = failed - known
        summary = f"{passed} passed, {len(failed)} failed"
        if failed & known:
            summary += f" (known: {', '.join(sorted(failed & known))})"
        return (not unknown, summary + (f"; not known: {', '.join(sorted(unknown))}" if unknown else ""))
    if kind == "runner":
        lines = [l for l in text.splitlines() if l.startswith("{")]
        if not lines:
            return False, "no result line"
        counts = next(iter(json.loads(lines[-1]).values()))
        ok = counts.get("FAIL", 1) == 0
        return ok, f"{counts.get('PASS', 0)} pass, {counts.get('FAIL', 0)} fail, {counts.get('KFAIL', 0)} known"
    if kind == "callsite":
        m = re.search(r"^(\d+) programs: (\d+) equal, (\d+) differ, (\d+) not compared", text, re.M)
        if not m:
            return False, "no summary"
        return (m.group(3) == "0" and m.group(4) == "0", m.group(0))
    if kind == "coverage":
        m = re.search(r"^check: (PASS|FAIL) \((\d+) problem", text, re.M)
        if not m:
            return False, "no check line"
        return (m.group(1) == "PASS", f"check {m.group(1)}, {m.group(2)} problems")
    if kind == "wording":
        m = re.search(r"(\d+) notes on recorded rejections, (\d+) not the recorded message", text)
        if not m:
            return False, "no summary"
        return (m.group(2) == "0", m.group(0))
    raise ValueError(kind)


def stamp(files: list[str], exe_too: bool = True, written: str | None = None) -> tuple[float, str]:
    """The time a run must be newer than: the release build's (unless not `exe_too`), or a later changed input's
    other than the files the steps before the build write (`written`); and what that is."""
    best = (0.0, "")
    for f in files:
        p = REPO / f
        if written and re.match(written, f):
            continue
        if INPUTS.match(f) and not NOT_INPUTS.search(f) and p.is_file():
            best = max(best, (p.stat().st_mtime, f))
    exe = REPO / EXE
    if exe_too and exe.is_file():
        best = max(best, (exe.stat().st_mtime, EXE))
    return best


def unlisted(files: list[str]) -> list[str]:
    """New or changed corpus programs that no closing run reaches: on no list, with no `.err` or `.norun`."""
    corpus = REPO / "tests/corpus"
    listed: set[str] = set()
    for name in ("slice.list",):
        path = corpus / name
        if path.is_file():
            listed |= {l.split("#")[0].strip() for l in path.read_text(encoding="utf-8").splitlines()}
    out = []
    for f in files:
        if not (f.startswith("tests/corpus/") and f.endswith(".bas")) or ".files/" in f:
            continue
        rel = f[len("tests/corpus/") : -len(".bas")]
        base = corpus / rel
        if rel in listed or base.with_suffix(".err").is_file() or base.with_suffix(".norun").is_file():
            continue
        if (REPO / f).is_file():
            out.append(f)
    return out


def when(t: float) -> str:
    return time.strftime("%Y-%m-%d %H:%M", time.localtime(t))


def check(
    nofold: bool,
    known: set[str],
    not_repeated: dict[str, str] | None = None,
    coverage: bool = False,
    layout: bool = False,
) -> int:
    not_repeated = not_repeated or {}
    files = changed_files()
    need = nofold or needs_nofold(files)
    cov = coverage or needs_coverage(files)
    lay = layout or needs_format(files)
    after, why = stamp(files)
    exe = REPO / EXE
    problems = 0
    # Each reason given with --not-repeated, numbered, and the steps it was used for.
    reasons: dict[str, tuple[int, list[str]]] = {}
    # What the steps before the build write (`lists` writes after `wording` has run).
    written = "|".join(f"(?:{s.writes})" for s in steps(need, cov, lay) if s.writes)
    for s in steps(need, cov, lay):
        path = REPO / log(s.name)
        if not path.is_file():
            print(f"MISSING  {s.name}: {log(s.name)} not written")
            problems += 1
            continue
        t = path.stat().st_mtime
        summary_at = f" [{when(t)}]"
        text = path.read_text(encoding="utf-8", errors="replace")
        tested = read_header(text) if s.copy else None
        # A step before the build must come before it, and after every changed input but those it writes (a log
        # left by an earlier change is not this one's); every other one after the build and every changed input.
        # A run of a copy of the compiler: of the current build by its hash, and started after every changed input
        # that does not reach it through the build (a log written before the copies, without the line, by time).
        if tested:
            digest, started = tested
            current = sha256_of(exe) if exe.is_file() else ""
            source, source_why = stamp(files, exe_too=False, written=BUILT_FROM)
            if digest != current:
                stale, reason = True, f"of build {digest[:12]}, not the current {current[:12] or '(none)'}"
            else:
                stale, reason = started < source, f"started before {source_why}"
        elif s.before_build and s.name != "build":
            source, source_why = stamp(files, exe_too=False, written=written)
            if exe.is_file() and t > exe.stat().st_mtime:
                stale, reason = True, "after the build: build again, then run what follows"
            else:
                stale, reason = t < source, f"older than {source_why}: run it again, then build"
        elif s.name == "build" and (not exe.is_file() or exe.stat().st_mtime < after):
            # The log may be newer than the edit while the executable is not (a build that stopped, another profile).
            stale = True
            reason = f"{EXE} is older than {why}"
        else:
            stale = t < after
            reason = f"older than {why}"
        ok, summary = judge(s.judge, text, known)
        summary += summary_at
        if stale and ok and s.name in not_repeated:
            # Rule 9: an edit after the closing runs is rerun only for what it can affect, saying why.
            r = not_repeated[s.name]
            number, used = reasons.setdefault(r, (len(reasons) + 1, []))
            used.append(s.name)
            print(f"ok       {s.name}: {summary}; not repeated ({number})")
        elif stale:
            print(f"STALE    {s.name}: {reason} ({summary})")
            problems += 1
        elif not ok:
            print(f"FAIL     {s.name}: {summary}")
            problems += 1
        else:
            print(f"ok       {s.name}: {summary}")
    for r, (number, used) in reasons.items():
        print(f"({number}) not repeated after {why} ({', '.join(used)}): {r}")
    for f in unlisted(files):
        print(f"UNLISTED {f}: on no list and without .err or .norun, so no closing run reaches it")
        problems += 1
    print("Then: python tools/testtime/test_time.py, and name its five files among those to stage.")
    return 1 if problems else 0


def plan(nofold: bool, coverage: bool = False, layout: bool = False) -> int:
    files = changed_files()
    need = nofold or needs_nofold(files)
    cov = coverage or needs_coverage(files)
    lay = layout or needs_format(files)
    print("# Bash (Git Bash, the Bash tool), not PowerShell: `VAR=1 command` and `mkdir -p` are Bash. One command each.")
    print(f"mkdir -p {LOGS}")
    for s in steps(need, cov, lay):
        note = []
        if s.before_build:
            note.append("before the build")
        if s.alone:
            note.append("alone")
        if s.name == "slice":
            note.append("beside upstream and verification")
        print(f"{s.command}" + (f"    # {', '.join(note)}" if note else ""))
    if not need:
        print(
            "# no run without folding: no file of sema's constant evaluation, literals or operators changed;"
            " pass --nofold when the change touched a numeric conversion elsewhere"
        )
    if not cov:
        print("# no coverage check: no file of Civil War Strategy's copy, its scripts or the tools they need changed")
    if not lay:
        print("# no format runs: no file of the formatter, its recordings, the symbols or -y changed")
    print("python tools/close/close.py --check")
    print("python tools/testtime/test_time.py")
    return 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    mode = ap.add_mutually_exclusive_group(required=True)
    mode.add_argument("--plan", action="store_true", help="print the closing commands in order")
    mode.add_argument("--check", action="store_true", help="judge the logs of the closing commands")
    ap.add_argument("--nofold", action="store_true", help="ask for the run without folding whatever changed")
    ap.add_argument("--coverage", action="store_true", help="ask for the coverage check whatever changed")
    ap.add_argument("--format", action="store_true", help="ask for the format runs whatever changed")
    ap.add_argument("--known", action="append", default=[], help="a failing test that is known and not the change's")
    ap.add_argument(
        "--not-repeated",
        action="append",
        default=[],
        metavar="STEP[,STEP...]=REASON",
        help="passing steps older than a later edit that cannot affect them, and why (rule 9)",
    )
    a = ap.parse_args(argv)
    if a.plan:
        return plan(a.nofold, a.coverage, a.format)
    not_repeated = {}
    for item in a.not_repeated:
        names, sep, reason = item.partition("=")
        if not sep or not reason.strip():
            ap.error(f"--not-repeated {item!r}: give STEP[,STEP...]=REASON")
        known_steps = {s.name for s in steps(True, True, True)}
        for step in names.split(","):
            if step not in known_steps:
                ap.error(f"--not-repeated: no step {step!r} (the steps: {', '.join(sorted(known_steps))})")
            not_repeated[step] = reason.strip()
    return check(a.nofold, set(a.known), not_repeated, a.coverage, a.format)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
