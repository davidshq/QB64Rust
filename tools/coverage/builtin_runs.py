#!/usr/bin/env python3
"""The built-ins the test runs reach (spec testing/coverage; OpenSpec change m4-coverage-review).

    python tools/coverage/builtin_runs.py <counts>...                 # merge the records and report
    python tools/coverage/builtin_runs.py --check <counts>...         # and hold them to tests/never_run.list

A coverage build's record of version 2 (crates/codegen-cpp/src/coverage.rs, tools/coverage/README.md) names,
for each counted statement, the built-ins the statement calls. This tool merges the records of any number of
runs of any programs (no source is read) and gives each built-in a record names as run (a statement that names
it ran at least once) or not run, with the programs that hold it. A record of version 1 names none.

--check holds the result to tests/never_run.list, one built-in per line:

    <name><TAB><reason>[ <note>]

'#' starting a comment, the reason one of REASONS. It fails on a built-in that is not run and not listed, on a
listed one that ran, on a listed one that no record names (the counts are of a part of the suites, or no slice
program holds it), on a line of the list that is not of that form, and on a built-in of sema's tables
(crates/sema/src/builtins.rs, read as text: one row per line, which tier 1 holds) that no record names, as
one held only where no statement is counted (a CONST) is neither run nor not run.
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

from coverage import CoverageError, read_records

REPO = Path(__file__).resolve().parents[2]
LIST = REPO / "tests" / "never_run.list"
TABLE = REPO / "crates" / "sema" / "src" / "builtins.rs"
# A row of SUPPORTED or STATEMENTS, and the line of OWN_NODE (crates/driver/tests/inputs.rs holds the source to
# these forms).
ROW = re.compile(r'^\s*\("([A-Z_$0-9]+)", (?:Rule|StmtRule)::', re.M)
OWN_NODE = re.compile(r"^pub const OWN_NODE: &\[&str\] = &\[(.*)\];$", re.M)
REASONS = ("desktop", "printer", "keyboard", "person", "compiler")
# How many programs a line of the report names before "and N more".
SHOWN = 5


@dataclass
class Builtins:
    """Merged counts by built-in: the summed count, the programs whose records name it, and those in
    which a statement naming it ran."""

    count: dict[str, int] = field(default_factory=dict)
    held: dict[str, set[str]] = field(default_factory=dict)
    ran: dict[str, set[str]] = field(default_factory=dict)
    records: int = 0
    programs: set[str] = field(default_factory=set)

    def add(self, path: Path) -> None:
        for record in read_records(path):
            # The program is its main file, the record's file 0.
            program = record.files.get(0, ("?", ""))[0]
            self.programs.add(program)
            for _, _, _, n, names in record.entries:
                for name in names:
                    self.count[name] = self.count.get(name, 0) + n
                    self.held.setdefault(name, set()).add(program)
                    if n:
                        self.ran.setdefault(name, set()).add(program)
            self.records += 1

    def not_run(self) -> list[str]:
        return sorted(n for n, c in self.count.items() if c == 0)


def some(programs: set[str]) -> str:
    """At most SHOWN of `programs`, sorted, then how many more."""
    names = sorted(programs)
    more = len(names) - SHOWN
    return ", ".join(names[:SHOWN]) + (f" and {more} more" if more > 0 else "")


def report(b: Builtins) -> str:
    lines = [
        f"{b.records} record(s) of {len(b.programs)} program(s); {len(b.count)} built-in(s) named, "
        f"{len(b.count) - len(b.not_run())} run, {len(b.not_run())} not run"
    ]
    for name in b.not_run():
        lines.append(f"  not run: {name} (held by {some(b.held[name])})")
    return "\n".join(lines)


def read_list(path: Path) -> tuple[dict[str, str], list[str]]:
    """The listed built-ins with their reasons, and the problems of the list's lines."""
    listed: dict[str, str] = {}
    problems: list[str] = []
    for i, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        # Only an empty line is skipped, as tier 1 reads the list (crates/driver/tests/inputs.rs).
        if not line or line.startswith("#"):
            continue
        name, tab, rest = line.partition("\t")
        reason = rest.split(" ")[0]
        if not tab:
            problems.append(f"{path.name} line {i}: no tab after the name")
        elif reason not in REASONS:
            problems.append(f"{path.name} line {i}: `{reason}` is no reason of the list")
        elif name in listed:
            problems.append(f"{path.name} line {i}: `{name}` is listed twice")
        else:
            listed[name] = reason
    return listed, problems


def table_names(path: Path) -> set[str]:
    """The built-ins of sema's tables as written: the rows of SUPPORTED and STATEMENTS, and OWN_NODE."""
    text = path.read_text(encoding="utf-8")
    names = set(ROW.findall(text))
    own = OWN_NODE.search(text)
    if not names or not own:
        raise CoverageError(f"{path}: no rows of the built-in tables found")
    names.update(re.findall(r'"([^"]+)"', own.group(1)))
    return names


def check(b: Builtins, path: Path, table: set[str]) -> list[str]:
    """What fails the check of the merged counts against the list and the tables (nothing when it passes)."""
    listed, problems = read_list(path)
    for name in sorted(table - b.count.keys() - listed.keys()):
        problems.append(
            f"`{name}` is a built-in sema compiles but no record names it: it is held only where no "
            "statement is counted, or the counts are of a part of the suites"
        )
    for name in b.not_run():
        if name not in listed:
            problems.append(
                f"`{name}` is run by no test and is not in {path.name} (held by {some(b.held[name])}): "
                "run it in a test, or list it with its reason"
            )
    for name, reason in sorted(listed.items()):
        if name not in b.count:
            problems.append(
                f"`{name}` is listed ({reason}) but no record names it: the counts may be of a part of "
                "the suites, or no program holds it"
            )
        elif b.count[name]:
            problems.append(
                f"`{name}` is listed ({reason}) but ran (in {some(b.ran[name])}): take it out of the list"
            )
    return problems


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("counts", nargs="+", type=Path, help="counts files (QB64RUST_COVERAGE) to merge")
    ap.add_argument("--check", action="store_true", help="hold the counts to the list")
    ap.add_argument("--list", type=Path, default=LIST, help="the list (default: %(default)s)")
    ap.add_argument("--table", type=Path, default=TABLE, help="sema's tables (default: %(default)s)")
    args = ap.parse_args(argv)
    b = Builtins()
    try:
        for c in args.counts:
            b.add(c)
        table = table_names(args.table) if args.check else set()
    except (CoverageError, OSError) as e:
        print(f"builtins: {e}", file=sys.stderr)
        return 2
    print(report(b))
    if not args.check:
        return 0
    problems = check(b, args.list, table)
    for p in problems:
        print(f"[check] {p}")
    print(f"check: {'FAIL' if problems else 'PASS'} ({len(problems)} problem(s))")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
