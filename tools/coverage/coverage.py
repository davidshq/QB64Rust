#!/usr/bin/env python3
"""The coverage of a program's runs, line by line (spec testing/coverage; OpenSpec change cws-whole-game).

    python tools/coverage/coverage.py <counts>...            # merge the records and report
    python tools/coverage/coverage.py --check                # run g12_game's scripts with coverage, then check
    python tools/coverage/coverage.py --check <counts>...    # check records made earlier

A program built with `qb64rust --coverage` appends one record to the file QB64RUST_COVERAGE names each
time it runs (crates/codegen-cpp/src/coverage.rs): its source files, each with the FNV-1a 64 hash of its
text and its path relative to the main file's folder, then each statement as file, line, column and count.
This tool merges any number of records of one program, refuses a record whose files are not the ones on
disk, and gives each line of a file one status: run (every statement that starts on it ran), partly run
(some did), not run (none did), or none (no statement starts on it).

--check holds the main file to a list of the lines its runs do not reach, each with a reason
(tests/programs/civil-war-strategy.unreached for Civil War Strategy, whose copy g12_game keeps the game's
line numbers):

    <line>[:<column>,<column>...]<TAB><reason>[ <more>]

one line per entry, '#' starting a comment. The columns are given for a partly run line: those of the
statements that do not run. The reasons are a fixed set (REASONS). The check fails on a line with a
statement that is not run in full and not listed, and on a listed line that ran in full or whose columns
are not the ones that did not run, so the list only shrinks. An `edit` line may hold no statement (the edit
of the copy removed it); a line of any other reason must hold one.

Without counts files, --check makes them: it runs every script of the program (the corpus runner's
--glob "<name>.*": the program and its scripts, no other program; it builds the program once, with --coverage and its screen-state file) and
checks what they ran; a script that fails is reported, and the check fails.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
PROGRAM = REPO / "tests" / "corpus" / "screen" / "g12_game.bas"
UNREACHED = REPO / "tests" / "programs" / "civil-war-strategy.unreached"
RUNNER = REPO / "tools" / "legacy_tests" / "run_legacy_tests.py"
COMPILER = REPO / "target" / "release" / "qb64rust.exe"

# The reasons a line may be left unreached for.
REASONS = {
    "edit": "removed by an edit of the copy",
    "file": "a file the program does not ship",
    "dead": "no path reaches it",
    "input": "input the screen-state check cannot give",
    "bug": "a bug of the program that ends or hangs it",
    "rare": "reachable only in a state no script creates (name it after the reason)",
}


class CoverageError(Exception):
    """A counts file that cannot be read, or one of another program."""


def fnv1a64(data: bytes) -> int:
    h = 0xCBF29CE484222325
    for b in data:
        h = ((h ^ b) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


@dataclass
class Coverage:
    """Merged counts: by file path (relative to the main file's folder), {(line, column): count}."""

    main: Path
    files: dict[str, dict[tuple[int, int], int]] = field(default_factory=dict)
    records: int = 0
    # The digest of each file on disk, by name: hashed once however many records name it.
    digests: dict[str, str] = field(default_factory=dict)

    def add(self, path: Path) -> None:
        """Merges every record of a counts file, each checked against the files on disk."""
        lines = path.read_bytes().decode("utf-8", "replace").splitlines()
        i = 0
        while i < len(lines):
            if not lines[i].strip():
                i += 1
                continue
            if lines[i].strip() != "coverage 1":
                raise CoverageError(f"{path} line {i + 1}: not a coverage record")
            i += 1
            names: dict[int, str] = {}
            counts: list[tuple[int, int, int, int]] = []
            while i < len(lines) and lines[i] != "end":
                parts = lines[i].split("\t")
                if parts[0] == "file" and len(parts) == 4:
                    names[int(parts[1])] = parts[3]
                    self.check_file(parts[3], parts[2])
                elif len(parts) == 4 and all(p.isdigit() for p in parts):
                    f, line, col, n = (int(p) for p in parts)
                    counts.append((f, line, col, n))
                else:
                    raise CoverageError(f"{path} line {i + 1}: not a line of a record")
                i += 1
            if i == len(lines):
                raise CoverageError(f"{path}: a record without its end (a run cut short)")
            i += 1
            for f, line, col, n in counts:
                if f not in names:
                    raise CoverageError(f"{path}: a count of file {f}, which the record does not name")
                at = self.files.setdefault(names[f], {})
                at[(line, col)] = at.get((line, col), 0) + n
            for name in names.values():
                self.files.setdefault(name, {})
            self.records += 1

    def check_file(self, name: str, digest: str) -> None:
        """Stops when the file the record names is not on disk with the text it was built from."""
        if name not in self.digests:
            path = self.main.parent / name
            if not path.is_file():
                raise CoverageError(f"the counts name {name}, which is not in {self.main.parent}")
            self.digests[name] = f"{fnv1a64(path.read_bytes()):016x}"
        if self.digests[name] != digest.lower():
            raise CoverageError(
                f"the counts are of another text of {name} than the one on disk: build and run "
                "it again"
            )

    def lines(self, name: str) -> dict[int, tuple[str, list[int]]]:
        """Each line of file `name` that holds a statement: (status, the columns of its statements
        that did not run). Status: run, partly, not run."""
        by_line: dict[int, list[tuple[int, int]]] = {}
        for (line, col), n in self.files.get(name, {}).items():
            by_line.setdefault(line, []).append((col, n))
        out = {}
        for line, stmts in sorted(by_line.items()):
            unrun = sorted(col for col, n in stmts if n == 0)
            if not unrun:
                out[line] = ("run", [])
            elif len(unrun) == len(stmts):
                out[line] = ("not run", unrun)
            else:
                out[line] = ("partly", unrun)
        return out


def ranges(numbers: list[int]) -> str:
    """1, 2, 3, 7 as "1-3, 7"."""
    parts: list[list[int]] = []
    for n in numbers:
        if parts and parts[-1][1] == n - 1:
            parts[-1][1] = n
        else:
            parts.append([n, n])
    return ", ".join(f"{a}" if a == b else f"{a}-{b}" for a, b in parts)


def report(cov: Coverage) -> str:
    """The totals per file, the lines not run, and the partly run lines with their unrun columns."""
    out = [f"{cov.records} record(s)"]
    for name in sorted(cov.files):
        lines = cov.lines(name)
        status = [s for s, _ in lines.values()]
        out.append(
            f"{name}: {len(lines)} lines with a statement: {status.count('run')} run, "
            f"{status.count('partly')} partly run, {status.count('not run')} not run"
        )
        not_run = [n for n, (s, _) in lines.items() if s == "not run"]
        if not_run:
            out.append(f"  not run: {ranges(not_run)}")
        for n, (s, cols) in lines.items():
            if s == "partly":
                out.append(f"  line {n}: partly run; not run: column {', '.join(map(str, cols))}")
    return "\n".join(out)


@dataclass
class Entry:
    line: int
    columns: list[int]
    reason: str
    at: int  # the line of the list


def read_unreached(path: Path) -> tuple[list[Entry], list[str]]:
    """The entries of a list of unreached lines, and its mistakes (a line of the list named)."""
    entries: list[Entry] = []
    problems: list[str] = []
    if not path.is_file():
        return entries, problems
    for n, text in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not text.strip() or text.lstrip().startswith("#"):
            continue
        where, tab, rest = text.partition("\t")
        reason = rest.split()[0] if rest.split() else ""
        line, colon, cols = where.strip().partition(":")
        try:
            columns = [int(c) for c in cols.split(",")] if colon else []
            number = int(line)
        except ValueError:
            problems.append(f"{path.name} line {n}: not <line>[:<column>,...]<TAB><reason>")
            continue
        if not tab or reason not in REASONS:
            problems.append(
                f"{path.name} line {n}: the reason {reason!r} is not one of {', '.join(REASONS)}"
            )
            continue
        entries.append(Entry(number, columns, reason, n))
    return entries, problems


def check(cov: Coverage, unreached: Path) -> list[str]:
    """What fails the check of the main file against the list (nothing when it passes)."""
    entries, problems = read_unreached(unreached)
    lines = cov.lines(cov.main.name)
    listed: dict[int, Entry] = {}
    for e in entries:
        if e.line in listed:
            problems.append(f"{unreached.name} line {e.at}: line {e.line} is listed twice")
        listed[e.line] = e
    for n, (status, cols) in lines.items():
        e = listed.get(n)
        if status == "run":
            if e:
                problems.append(f"line {n}: listed ({e.reason}) but run in full: take it out of the list")
            continue
        if not e:
            what = "not run" if status == "not run" else f"partly run (column {', '.join(map(str, cols))})"
            problems.append(f"line {n}: {what}, and not in {unreached.name}")
        elif status == "partly" and sorted(e.columns) != cols:
            problems.append(
                f"line {n}: partly run; the columns not run are {', '.join(map(str, cols))}, "
                f"{unreached.name} line {e.at} gives {', '.join(map(str, e.columns)) or 'none'}"
            )
        elif status == "not run" and e.columns:
            problems.append(f"line {n}: not run at all; {unreached.name} line {e.at} gives columns")
    for n, e in listed.items():
        if n not in lines and e.reason != "edit":
            problems.append(
                f"line {n}: listed ({e.reason}) but holds no statement; only an edit may remove one"
            )
    return problems


def run_scripts(program: Path, counts: Path, jobs: int) -> int:
    """Runs every test of the program (its own keys file and its scripts) with --coverage into counts;
    returns the runner's exit code."""
    counts.parent.mkdir(parents=True, exist_ok=True)
    counts.unlink(missing_ok=True)
    cmd = [
        sys.executable,
        str(RUNNER),
        "--suite",
        "corpus",
        "--qb64",
        str(COMPILER),
        "--category",
        program.parent.name,
        "--glob",
        f"{program.stem}.*",
        "--coverage",
        str(counts),
        "--build-cache",
        str(REPO / "target" / "build-cache"),
        "--jobs",
        str(jobs),
        "--results",
        str(REPO / "target" / "coverage" / "runs"),
    ]
    print(" ".join(cmd), flush=True)
    return subprocess.run(cmd, check=False).returncode


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("counts", nargs="*", type=Path, help="counts files (QB64RUST_COVERAGE) to merge")
    ap.add_argument("--check", action="store_true", help="check the main file against the list")
    ap.add_argument("--program", type=Path, default=PROGRAM, help="the main file (default: %(default)s)")
    ap.add_argument("--unreached", type=Path, default=UNREACHED, help="the list (default: %(default)s)")
    ap.add_argument("--jobs", type=int, default=4, help="--check without counts: scripts at a time")
    args = ap.parse_args(argv)
    counts = list(args.counts)
    runner_rc = 0
    if not counts:
        if not args.check:
            ap.error("give counts files, or --check to make them")
        made = REPO / "target" / "coverage" / f"{args.program.stem}.txt"
        runner_rc = run_scripts(args.program.resolve(), made, args.jobs)
        counts = [made]
    cov = Coverage(args.program.resolve())
    try:
        for c in counts:
            cov.add(c)
    except (CoverageError, OSError) as e:
        print(f"coverage: {e}", file=sys.stderr)
        return 2
    print(report(cov))
    if not args.check:
        return 0
    problems = check(cov, args.unreached)
    if runner_rc:
        problems.append(f"a script failed (the runner's exit code {runner_rc}); its counts may be short")
    for p in problems:
        print(f"[check] {p}")
    print(f"check: {'FAIL' if problems else 'PASS'} ({len(problems)} problem(s))")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
