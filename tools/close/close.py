#!/usr/bin/env python3
"""The closing runs of a change (CLAUDE.md working rule 6), as a plan and a check.

    python tools/close/close.py --plan     # the commands, in order, each to be started by its own command
    python tools/close/close.py --check    # did each run, on the current build, and pass?

The tool runs nothing: each test run is started by its own command, so that tools/testtime sees it (rule 6). It
prints the commands with the log each one writes under target/close/, and then reads those logs. A step whose log
is missing, older than the build or than a changed source, or does not show a pass is named, and the exit code is
1. A failure that is known and not the change's is accepted with --known <test name>, and shown as such. A passing
step older than a later edit that cannot affect it, or one stopped before its result, is accepted with
--not-repeated STEP[,STEP...]=REASON, shown (rule 6: "say which runs were not repeated and why"); a failing one
never is. A changed file makes stale only the steps it can reach (REACH: a program of tests/sandbox reaches the
sandbox family alone, a libqb edit naming built-ins of tests/never_run.list their families alone); a step no changed
file reaches keeps its last passing log, and the plan prints it as a comment. A run with the pass cache that ran
most of a long list is named too (something emptied the cache: README.md, "What empties the pass cache"); when
that is right (a cold cache, a change to most programs' C++) it is accepted with --cache-miss STEP[,STEP...]=REASON.

Whether the run without folding is needed follows from the files changed against HEAD (sema's constant evaluation
or a numeric conversion, rule 6); --nofold asks for it anyway. So does the coverage check of Civil War Strategy's
scripts (tools/coverage): only when the game's copy or its scripts, the screen-state file, cut_game.py, the coverage
tool, codegen-cpp, ir or driver changed; --coverage asks for it anyway. And the format runs (QB64pe's format tests
and the format oracle, tests/format): only when the formatter, its recordings, tool or settings, the parser, the
built-in table, sema's symbols or the driver's -y changed; --format asks for them anyway. And the sandbox family
(tools/sandbox, in Windows Sandbox): only when the family, its runner or tests/never_run.list changed; --sandbox asks
for it anyway (a change to one of its built-ins' emission or libqb's code for them).
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

# Files whose change asks for the run without folding (rule 6: sema's constant evaluation, a numeric conversion).
NOFOLD_FILES = re.compile(r"^crates/sema/src/(consteval\.rs|literal\.rs|check/constants\.rs|check/auto\.rs|check/ops\.rs)$")
# Changed files that make a run stale: what the compiler, the tests and the programs are built from. Documents and
# the test-time record do not.
INPUTS = re.compile(
    r"^(crates/|tests/|tools/(legacy_tests|callsite|diagnostics|builtins|screen|coverage|format|sandbox)/|"
    r"libqb/|tools/libqb/edits/|Cargo\.(toml|lock)$)"
)
# Which steps a changed input reaches (2026-10-30, after a close ran every list for an edit of tests/sandbox and a
# libqb edit of _SCREENCLICK, which no list runs): a file matching an entry makes stale only the steps it names; one
# matching no entry, every step. The entries are conservative: tier 1 and the lists step read most of tests/
# (crates/driver/tests/inputs.rs), and tests/upstream/root is every program's include root, so neither has an entry.
TIER1_STEPS = frozenset({"lists", "tier1", "linux-tier1"})
REACH: list[tuple[re.Pattern[str], frozenset[str]]] = [
    # A snapshot is read by the test that made it (tier 1, or the command-line tests of crates/driver/tests/cli.rs),
    # not compiled: the release build is not older for it.
    (re.compile(r"^crates/[^/]+/tests/snapshots/"), TIER1_STEPS | {"cli"}),
    (re.compile(r"^tests/sandbox/"), frozenset({"sandbox"})),
    (re.compile(r"^tools/sandbox/"), frozenset({"sandbox"})),
    (re.compile(r"^tests/never_run\.list$"), TIER1_STEPS | {"sandbox"}),
    (re.compile(r"^tests/verification/"), TIER1_STEPS | {"verification"}),
    # tests/format is read by crates/driver/tests/format_oracle.rs too.
    (re.compile(r"^tests/format/"), TIER1_STEPS | {"format", "format-corpus"}),
    # tests/callsite by crates/sema/src/opengl.rs's test too.
    (re.compile(r"^tests/callsite/"), TIER1_STEPS | {"callsite"}),
]
# A libqb edit (tools/libqb/edits/*.edit) or the copy it is applied to: the steps that build programs (not the
# sandbox family, which a merge runs anyway: design D7 of desktop-sandbox), unless every changed edit names the
# built-ins it changes (`builtins: NAME ...`) and each is on tests/never_run.list, when it reaches the family of
# each one's reason only (FAMILY_STEP; a reason with no family: the command-line tests that build, so the edit is at
# least compiled).
LIBQB_FILES = re.compile(r"^(libqb/|tools/libqb/edits/)")
PROGRAM_STEPS = frozenset(
    {"slice", "upstream", "verification", "addprefix", "qbasic", "nofold", "coverage", "cli", "linux-slice"}
)
EDIT_BUILTINS = re.compile(r"^builtins:[ \t]*(.+?)[ \t]*$", re.M)
NEVER_RUN = "tests/never_run.list"
FAMILY_STEP = {"sandbox": "sandbox"}
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
# Files whose change asks for the Linux runs, in WSL after tools/linux/sync.sh (design D11 of m7-linux): whatever the
# release build is made from (BUILT_FROM: any crate can emit C++ that g++ takes otherwise, as SWAP's order and the end
# of a procedure in a debug build did; a list of the platform's files fell behind within one change), the runner, the
# copy of libqb, the Linux tools, and the Linux recordings and lists.
LINUX_FILES = re.compile(
    BUILT_FROM + r"|^(tools/legacy_tests/run_legacy_tests\.py$|libqb/|tools/libqb/edits/|tools/linux/|"
    r".*\.linux\.(output|err|out\.txt)$|.*known\.linux\.list$)"
)
# Files whose change asks for the sandbox family (design D7 of desktop-sandbox): the family, its runner, and the list
# whose `sandbox` rows it holds.
SANDBOX_FILES = re.compile(r"^(tests/sandbox/|tools/sandbox/|tests/never_run\.list$)")
# A command run in WSL's copy of the tree (tools/linux/sync.sh; crates/README.md, "Building on Linux").
WSL = "wsl -e bash -lc"
# A run with the pass cache that ran most of its list (README, "What empties the pass cache"; 2026-10-30, when a
# folder taken as support of every verification program ran all 4,121): with at least CACHE_LIST programs and fewer
# than CACHE_SKIPPED of them not run, it is named unless --cache-miss gives the reason.
CACHE_LIST = 50
CACHE_SKIPPED = 0.5
# A runner's line of a program with no executable, which the pass cache never skips: `[3/9] PASS name (error, 0.1s)`.
UNCACHED = re.compile(r"^\[\d+/\d+\] [A-Z]+ .*\((?:error|compile-only), [\d.]+s\)", re.M)


@dataclass
class Step:
    name: str
    command: str
    # How the log shows a pass; None: only that it ran (a report, a regeneration).
    judge: str | None
    # Before the build: a step that writes inputs of the build or of the runs after it (README, "The test cadence", "Order").
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


def linux_steps() -> list[Step]:
    """The Linux family (design D11 of m7-linux): the tree copied into WSL, then tier 1 and the slice list there."""
    return [
        # The folder in Windows' form (Git Bash's `pwd -W`): `--cd .` failed with Wsl/E_INVALIDARG from the Bash tool
        # on 2026-10-30, hours after it had worked from the same shell; the Windows form worked then.
        Step("linux-sync", f'wsl --cd "$(pwd -W)" -e bash tools/linux/sync.sh > {log("linux-sync")} 2>&1', None),
        Step(
            "linux-tier1",
            f"{WSL} 'cd ~/qb64rust && cargo test --workspace --locked --no-fail-fast' > {log('linux-tier1')} 2>&1",
            "cargo",
        ),
        Step(
            "linux-slice",
            f"{WSL} 'cd ~/qb64rust && cargo build --release --locked && python3 tools/legacy_tests/run_legacy_tests.py "
            "--suite corpus --qb64 target/release/qb64rust --jobs 8 --build-cache target/build-cache "
            f"--pass-cache target/pass-cache --list tests/corpus/slice.list' > {log('linux-slice')} 2>&1",
            "runner",
        ),
    ]


def steps(
    nofold: bool, coverage: bool = False, layout: bool = False, linux: bool = False, sandbox: bool = False
) -> list[Step]:
    # A thin close (DECISIONS.md 2026-10-28): what can show the change's fault. The wording table and its check, the
    # demand reports and the counts are regenerated at a merge into main, not here (README, "The test cadence").
    out = [
        # The shrink-only lists are written before the build: tier 1 reads them.
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
        # With the pass cache, as the other lists (DECISIONS.md 2026-10-27): builds are deterministic, and its key is
        # the executable, so only the programs whose C++ folding changes run again.
        out.append(
            Step(
                "nofold",
                f"QB64RUST_NO_FOLD=1 {RUNNER} --suite corpus {NEW} {CACHE} --wait --list tests/corpus/slice.list "
                f"> {log('nofold')} 2>&1",
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
    ]
    if linux:
        out += linux_steps()
    if sandbox:
        # Last and alone: the sandbox's window takes the screen (design D7 of desktop-sandbox).
        out.append(
            Step("sandbox", f"python tools/sandbox/run.py > {log('sandbox')} 2>&1", "sandbox", alone=True, copy=True)
        )
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


def is_input(f: str) -> bool:
    return bool(INPUTS.match(f)) and not NOT_INPUTS.search(f)


def never_run() -> dict[str, str]:
    """{built-in: reason} of tests/never_run.list (`<name>\\t<reason>[ <note>]`, `#` a comment)."""
    path = REPO / NEVER_RUN
    out = {}
    if path.is_file():
        for line in path.read_text(encoding="utf-8").splitlines():
            if line.startswith("#") or not line.strip():
                continue
            name, _, rest = line.partition("\t")
            out[name] = rest.split()[0] if rest.split() else ""
    return out


def libqb_reach(files: list[str]) -> frozenset[str] | None:
    """The steps the change's libqb edits reach (LIBQB_FILES, REACH), or None when no libqb file changed."""
    changed = [f for f in files if LIBQB_FILES.match(f) and is_input(f)]
    if not changed:
        return None
    edits = [f for f in changed if f.startswith("tools/libqb/edits/") and f.endswith(".edit")]
    if not edits:
        return PROGRAM_STEPS
    listed = never_run()
    out: set[str] = set()
    for f in edits:
        path = REPO / f
        if not path.is_file():
            return PROGRAM_STEPS
        m = EDIT_BUILTINS.search(path.read_text(encoding="utf-8", errors="replace"))
        if not m:
            return PROGRAM_STEPS
        for name in m.group(1).split():
            if name not in listed:
                return PROGRAM_STEPS
            out.add(FAMILY_STEP.get(listed[name], "cli"))
    return frozenset(out)


def reaching(files: list[str], step: str, libqb: frozenset[str] | None) -> list[str]:
    """The changed files that can reach `step` (REACH; `libqb`: [`libqb_reach`])."""
    out = []
    for f in files:
        if LIBQB_FILES.match(f):
            if libqb is not None and step in libqb:
                out.append(f)
            continue
        for pattern, names in REACH:
            if pattern.match(f):
                if step in names:
                    out.append(f)
                break
        else:
            out.append(f)
    return out


def needs_nofold(files: list[str]) -> bool:
    return any(NOFOLD_FILES.match(f) for f in files)


def needs_coverage(files: list[str]) -> bool:
    return any(COVERAGE_FILES.match(f) for f in files)


def needs_format(files: list[str]) -> bool:
    return any(FORMAT_FILES.match(f) for f in files)


def needs_linux(files: list[str]) -> bool:
    return any(LINUX_FILES.match(f) for f in files)


def needs_sandbox(files: list[str]) -> bool:
    libqb = libqb_reach(files)
    return any(SANDBOX_FILES.match(f) for f in files) or (libqb is not None and "sandbox" in libqb)


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
    if kind == "sandbox":
        refused = re.search(r"^sandbox: refused: (.*)$", text, re.M)
        if refused:
            return False, f"refused: {refused.group(1)}"
        m = re.search(r"^sandbox: (\d+) pass, (\d+) fail$", text, re.M)
        cov = re.search(r"^coverage: (PASS|FAIL)", text, re.M)
        if not m:
            return False, "no result line"
        if not cov:
            return False, f"{m.group(1)} pass, {m.group(2)} fail; no coverage check"
        ok = m.group(2) == "0" and cov.group(1) == "PASS"
        return ok, f"{m.group(1)} pass, {m.group(2)} fail, coverage {cov.group(1)}"
    raise ValueError(kind)


def cache_skips(text: str) -> tuple[int, int] | None:
    """The runner's last line of a run with the pass cache: how many programs were not run, of how many."""
    found = re.findall(r"^--pass-cache: (\d+) of (\d+) programs not run", text, re.M)
    return (int(found[-1][0]), int(found[-1][1])) if found else None


def cache_missed(text: str) -> str | None:
    """Why a run with the pass cache ran most of the programs it could skip, or None when the cache did its work (or
    the log says nothing of it). A program the runner judges by a compile error or by its build alone has no
    executable, so the cache never skips it: it counts in neither number (2026-10-30, when 2,470 of a verification
    run's 2,471 were such)."""
    skips = cache_skips(text)
    if skips is None:
        return None
    skipped, total = skips
    uncached = len(UNCACHED.findall(text))
    total -= min(uncached, total - skipped)
    if total < CACHE_LIST or skipped >= CACHE_SKIPPED * total:
        return None
    left = f" ({uncached} more without an executable)" if uncached else ""
    return f"the pass cache let {total - skipped} of {total} programs run{left}"


def stamp(files: list[str], exe_too: bool = True, written: str | None = None) -> tuple[float, str]:
    """The time a run must be newer than: the release build's (unless not `exe_too`), or a later changed input's
    other than the files the steps before the build write (`written`); and what that is."""
    best = (0.0, "")
    for f in files:
        p = REPO / f
        if written and re.match(written, f):
            continue
        if is_input(f) and p.is_file():
            best = max(best, (p.stat().st_mtime, f))
    exe = REPO / EXE
    if exe_too and exe.is_file():
        best = max(best, (exe.stat().st_mtime, EXE))
    return best


def built_after(t: float) -> bool:
    """Whether the build step's log, written after `t`, shows a build that compiled nothing: the executable, though
    older than `t`, was then already built from the sources (cargo leaves it untouched; 2026-10-30)."""
    path = REPO / log("build")
    if not path.is_file() or path.stat().st_mtime <= t:
        return False
    text = path.read_text(encoding="utf-8", errors="replace")
    return re.search(r"^\s*Finished ", text, re.M) is not None and re.search(r"^\s*Compiling ", text, re.M) is None


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
    linux: bool = False,
    sandbox: bool = False,
    cache_miss: dict[str, str] | None = None,
) -> int:
    not_repeated = not_repeated or {}
    cache_miss = cache_miss or {}
    files = changed_files()
    need = nofold or needs_nofold(files)
    cov = coverage or needs_coverage(files)
    lay = layout or needs_format(files)
    lin = linux or needs_linux(files)
    sbx = sandbox or needs_sandbox(files)
    libqb = libqb_reach(files)
    exe = REPO / EXE
    problems = 0
    # Each reason given with --not-repeated, numbered, and the steps it was used for.
    reasons: dict[str, tuple[int, list[str]]] = {}
    # What the steps before the build write (`lists`: the shrink-only lists).
    written = "|".join(f"(?:{s.writes})" for s in steps(need, cov, lay, lin, sbx) if s.writes)
    for s in steps(need, cov, lay, lin, sbx):
        path = REPO / log(s.name)
        if not path.is_file():
            print(f"MISSING  {s.name}: {log(s.name)} not written")
            problems += 1
            continue
        # Only the changed inputs that can reach this step make it stale (REACH); a step none reaches keeps its
        # last log, whatever its age, as long as it passed.
        mine = reaching(files, s.name, libqb)
        reached = any(is_input(f) for f in mine)
        after, why = stamp(mine)
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
            source, source_why = stamp(mine, exe_too=False, written=BUILT_FROM)
            if digest != current:
                stale, reason = True, f"of build {digest[:12]}, not the current {current[:12] or '(none)'}"
            else:
                stale, reason = reached and started < source, f"started before {source_why}"
        elif not reached:
            stale, reason = False, ""
        elif s.before_build and s.name != "build":
            source, source_why = stamp(mine, exe_too=False, written=written)
            if exe.is_file() and t > exe.stat().st_mtime and not built_after(t):
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
        # A run stopped before its result (no result line at all): accepted with --not-repeated like a stale one,
        # never a run whose log shows a failure.
        stopped = not ok and summary in ("no result line", "no test result", "no summary", "no check line")
        summary += summary_at
        # A passing run with the pass cache that ran most of its list, whether current or not repeated.
        missed = cache_missed(text) if ok and CACHE in s.command else None
        if missed and s.name in cache_miss:
            summary += f"; {missed}: {cache_miss[s.name]}"
            missed = None
        if (stale and ok or stopped) and s.name in not_repeated:
            # Rule 6: an edit after the closing runs is rerun only for what it can affect, saying why.
            r = not_repeated[s.name]
            number, used = reasons.setdefault(r, (len(reasons) + 1, []))
            used.append(s.name)
            summary += f"; {'stopped, ' if stopped else ''}not repeated ({number})"
        elif stale:
            print(f"STALE    {s.name}: {reason} ({summary})")
            problems += 1
            continue
        elif not ok:
            print(f"FAIL     {s.name}: {summary}")
            problems += 1
            continue
        if missed:
            print(
                f"CACHE    {s.name}: {missed}; find out why (tools/close/README.md, \"What empties the pass "
                f"cache\"), or give --cache-miss {s.name}=REASON when a run of most of them is right"
            )
            problems += 1
        else:
            print(f"ok       {s.name}: {summary}")
    for r, (number, used) in reasons.items():
        print(f"({number}) not repeated ({', '.join(used)}): {r}")
    for f in unlisted(files):
        print(f"UNLISTED {f}: on no list and without .err or .norun, so no closing run reaches it")
        problems += 1
    return 1 if problems else 0


def plan(nofold: bool, coverage: bool = False, layout: bool = False, linux: bool = False, sandbox: bool = False) -> int:
    files = changed_files()
    need = nofold or needs_nofold(files)
    cov = coverage or needs_coverage(files)
    lay = layout or needs_format(files)
    lin = linux or needs_linux(files)
    sbx = sandbox or needs_sandbox(files)
    libqb = libqb_reach(files)
    # A step an option asks for "whatever changed" is printed whatever reaches it.
    asked: set[str] = set()
    for flag, names in (
        (nofold, {"nofold"}),
        (coverage, {"coverage"}),
        (layout, {"format", "format-corpus"}),
        (linux, {s.name for s in linux_steps()}),
        (sandbox, {"sandbox"}),
    ):
        if flag:
            asked |= names
    print("# Bash (Git Bash, the Bash tool), not PowerShell: `VAR=1 command` and `mkdir -p` are Bash. One command each.")
    print(f"mkdir -p {LOGS}")
    for s in steps(need, cov, lay, lin, sbx):
        if s.name not in asked and not any(is_input(f) for f in reaching(files, s.name, libqb)):
            # Not run: its last log stands, and --check names it only when that log is missing or failing.
            print(f"# {s.name}: no changed file reaches it; not run, its last log stands ({log(s.name)})")
            continue
        note = []
        if s.before_build:
            note.append("before the build")
        if s.alone:
            note.append("alone")
        if s.name == "slice":
            note.append("beside upstream and verification")
        if s.name == "linux-sync":
            note.append("the Linux family, in WSL, after the Windows runs")
        if s.name == "sandbox":
            note.append("last: its window takes the screen for about a minute")
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
    if not lin:
        print(
            "# no Linux runs: nothing the release build is made from (crates/, Cargo.toml, Cargo.lock), the runner,"
            " libqb, tools/linux or a Linux recording or list changed; pass --linux to ask for them"
        )
    if not sbx:
        print(
            "# no sandbox run: no file of tests/sandbox, tools/sandbox or tests/never_run.list changed; pass --sandbox"
            " when the change touched one of its built-ins' emission or libqb's code for them"
        )
    print("python tools/close/close.py --check")
    return 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    mode = ap.add_mutually_exclusive_group(required=True)
    mode.add_argument("--plan", action="store_true", help="print the closing commands in order")
    mode.add_argument("--check", action="store_true", help="judge the logs of the closing commands")
    ap.add_argument("--nofold", action="store_true", help="ask for the run without folding whatever changed")
    ap.add_argument("--coverage", action="store_true", help="ask for the coverage check whatever changed")
    ap.add_argument("--format", action="store_true", help="ask for the format runs whatever changed")
    ap.add_argument("--linux", action="store_true", help="ask for the Linux runs (in WSL) whatever changed")
    ap.add_argument("--sandbox", action="store_true", help="ask for the sandbox family whatever changed")
    ap.add_argument("--known", action="append", default=[], help="a failing test that is known and not the change's")
    ap.add_argument(
        "--not-repeated",
        action="append",
        default=[],
        metavar="STEP[,STEP...]=REASON",
        help="passing steps older than a later edit that cannot affect them, and why (rule 6)",
    )
    ap.add_argument(
        "--cache-miss",
        action="append",
        default=[],
        metavar="STEP[,STEP...]=REASON",
        help="runs with the pass cache that rightly ran most of their list (a cold cache, most programs' C++ changed)",
    )
    a = ap.parse_args(argv)
    if a.plan:
        return plan(a.nofold, a.coverage, a.format, a.linux, a.sandbox)
    known_steps = {s.name for s in steps(True, True, True, True, True)}

    def reasons(option: str, items: list[str]) -> dict[str, str]:
        out = {}
        for item in items:
            names, sep, reason = item.partition("=")
            if not sep or not reason.strip():
                ap.error(f"{option} {item!r}: give STEP[,STEP...]=REASON")
            for step in names.split(","):
                if step not in known_steps:
                    ap.error(f"{option}: no step {step!r} (the steps: {', '.join(sorted(known_steps))})")
                out[step] = reason.strip()
        return out

    not_repeated = reasons("--not-repeated", a.not_repeated)
    cache_miss = reasons("--cache-miss", a.cache_miss)
    return check(a.nofold, set(a.known), not_repeated, a.coverage, a.format, a.linux, a.sandbox, cache_miss)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
