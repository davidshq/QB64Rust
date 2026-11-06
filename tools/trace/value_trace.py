#!/usr/bin/env python3
"""The values a program's variables took, by variable (spec compiler/value-trace; OpenSpec change value-trace).

    python tools/trace/value_trace.py <records>...                 # merge the records and report
    python tools/trace/value_trace.py --json <records>...          # the same report as JSON
    python tools/trace/value_trace.py --var NAME <records>...      # the entries of one name
    python tools/trace/value_trace.py --program p.bas <records>... # the main file, when it is not in this folder

A program built with `qb64rust --trace-values` appends one record to the file QB64RUST_TRACE names each
time it runs (crates/codegen-cpp/src/trace.rs; the format is in README.md beside this file). This tool
merges any number of records of one program and refuses a record whose files are not the ones on disk.

For each entry (a variable, an array, or a member path of a user type) it prints the stores, the
after-call records, the range, whether the entry was cut at its limit of distinct values, and the
distinct values by count. Under an array it prints its aliases (the parameters it was passed to, whose
entries hold the values stored through them) and its bulk stores. Then it prints the stores at an address.
"""

from __future__ import annotations

import argparse
import json
import struct
import sys
from dataclasses import dataclass, field
from decimal import Decimal, InvalidOperation
from pathlib import Path

# The record's file lines and the check of a file's hash are the coverage tool's, so that the two tools
# cannot disagree on how a file's hash is computed (design D8).
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "coverage"))
import coverage

FORMAT = "trace 1"


class TraceError(Exception):
    """A record that cannot be read, or one of another program."""


@dataclass
class Value:
    """A distinct value of an entry: its text as recorded, how often it was recorded, and the file and
    line that first recorded it. A string above 200 bytes has its first 200 bytes as its text, and
    beside it its full length and the FNV-1a 64 hash of all its bytes (16 hex digits)."""

    text: str
    count: int
    file: str
    line: int
    length: int | None = None
    hash: str | None = None


@dataclass
class Entry:
    name: str
    type: str
    procedure: str | None
    stores: int = 0
    after_calls: int = 0
    cut: bool = False
    low: str | None = None
    high: str | None = None
    # By the value's text, length and hash, in the order first recorded. A string above 200 bytes is
    # told apart by its hash, so two that share their first 200 bytes stay two values.
    values: dict[tuple[str, int | None, str | None], Value] = field(default_factory=dict)

    @property
    def is_string(self) -> bool:
        return self.type.startswith("STRING")


@dataclass
class Site:
    """A statement counted and not traced: an alias or an untraced store, by its key, with its count."""

    file: str
    line: int
    count: int


def number(text: str) -> Decimal | None:
    """A bound of a range as a number, to compare two of them; None when it is none."""
    try:
        return Decimal(text)
    except InvalidOperation:
        return None


@dataclass
class Trace:
    """Merged records of one program."""

    main: Path
    records: int = 0
    # By (procedure, name, type).
    entries: dict[tuple[str | None, str, str], Entry] = field(default_factory=dict)
    # By (file, line, name, procedure, callee, parameter).
    aliases: dict[tuple, Site] = field(default_factory=dict)
    # By (file, line, what, name, procedure).
    untraced: dict[tuple, Site] = field(default_factory=dict)
    # The hash check of each file, hashed once however many records name it.
    checker: coverage.Coverage | None = None

    def check_file(self, name: str, digest: str) -> None:
        """Stops when the file a record names is not on disk with the text the program was built from."""
        if self.checker is None:
            self.checker = coverage.Coverage(self.main)
        try:
            self.checker.check_file(name, digest)
        except coverage.CoverageError as e:
            raise TraceError(str(e).replace("the counts", "the records")) from e

    def add(self, path: Path) -> None:
        """Merges every record of a file, each checked against the files on disk."""
        lines = path.read_bytes().decode("utf-8", "replace").splitlines()
        i = 0
        while i < len(lines):
            if not lines[i].strip():
                i += 1
                continue
            if lines[i].strip() != FORMAT:
                raise TraceError(f"{path} line {i + 1}: not a value-trace record ({FORMAT})")
            end = next((k for k in range(i + 1, len(lines)) if lines[k] == "end"), None)
            if end is None:
                raise TraceError(f"{path}: a record without its end (a run cut short)")
            self.add_record(path, i + 2, lines[i + 1 : end])
            i = end + 1

    def add_record(self, path: Path, first: int, lines: list[str]) -> None:
        """One record: its lines between the format line and `end`; `first` is the first one's number."""
        files: dict[int, str] = {}
        # The entry of each number of this record.
        numbered: dict[int, Entry] = {}

        def bad(k: int) -> TraceError:
            return TraceError(f"{path} line {first + k}: not a line of a record")

        def file_of(index: str, k: int) -> str:
            if not index.isdigit() or int(index) not in files:
                raise TraceError(f"{path} line {first + k}: a file the record does not name")
            return files[int(index)]

        for k, line in enumerate(lines):
            parts = line.split("\t")
            named = coverage.file_line(parts)
            try:
                if named:
                    index, (name, digest) = named
                    self.check_file(name, digest)
                    files[index] = name
                elif parts[0] == "entry" and len(parts) == 11:
                    n, name, ty, procedure, stores, after, _distinct, cut, low, high = parts[1:]
                    key = (None if procedure == "-" else procedure, name, ty)
                    entry = self.entries.setdefault(key, Entry(name, ty, key[0]))
                    numbered[int(n)] = entry
                    entry.stores += int(stores)
                    entry.after_calls += int(after)
                    entry.cut = entry.cut or cut == "1"
                    widen(entry, low, high)
                elif parts[0] == "value" and len(parts) in (6, 8):
                    entry = numbered[int(parts[1])]
                    length, digest = (int(parts[6]), parts[7]) if len(parts) == 8 else (None, None)
                    # 16 lower-case hex digits, as the run time writes them, so that one value
                    # always has one key.
                    if digest is not None and not (
                        len(digest) == 16 and all(c in "0123456789abcdef" for c in digest)
                    ):
                        raise bad(k)
                    key = (parts[5], length, digest)
                    value = entry.values.get(key)
                    if value:
                        value.count += int(parts[2])
                    else:
                        site = (file_of(parts[3], k), int(parts[4]))
                        entry.values[key] = Value(
                            parts[5], int(parts[2]), *site, length, digest
                        )
                elif parts[0] == "alias" and len(parts) == 8:
                    key = (file_of(parts[1], k), int(parts[2]), *parts[4:])
                    count(self.aliases, key, int(parts[3]))
                elif parts[0] == "untraced" and len(parts) == 7:
                    key = (file_of(parts[1], k), int(parts[2]), *parts[4:])
                    count(self.untraced, key, int(parts[3]))
                else:
                    raise bad(k)
            except (ValueError, KeyError) as e:
                raise bad(k) from e
        self.records += 1


def count(sites: dict[tuple, Site], key: tuple, n: int) -> None:
    site = sites.setdefault(key, Site(key[0], key[1], 0))
    site.count += n


def widen(entry: Entry, low: str, high: str) -> None:
    """The entry's range made the widest of its own and a record's. `-` is a record with no range (every
    value a NaN)."""
    for text, attr, keep in ((low, "low", min), (high, "high", max)):
        new = number(text)
        if new is None:
            continue
        old = getattr(entry, attr)
        if old is None or keep(number(old), new) != number(old):
            setattr(entry, attr, text)


def shortest(text: str, ty: str) -> str:
    """The shortest text of a SINGLE or DOUBLE that reads back to the same value (the record gives 9 and
    17 digits, which always do). Any other text as it is."""
    try:
        if ty == "SINGLE":
            bits = struct.pack("<f", float(text))
            for digits in range(1, 10):
                short = f"{float(text):.{digits}g}"
                if struct.pack("<f", float(short)) == bits:
                    # Written as Python writes a float: `20`, not `2e+01`.
                    return repr(float(short)).removesuffix(".0")
        elif ty == "DOUBLE":
            # Python's own text of a float is the shortest that reads back.
            return repr(float(text)).removesuffix(".0")
    except (ValueError, OverflowError):
        pass
    return text


def shown(entry: Entry, value: Value) -> str:
    """A value as the report prints it."""
    if entry.is_string:
        if value.length is not None:
            return f"{value.text} ({value.length} bytes, the first 200 shown)"
        return value.text
    return shortest(value.text, entry.type)


def root(name: str) -> str:
    """The variable an entry's name starts with: `SHIPS` of `SHIPS().HP`."""
    return name.split(".")[0].removesuffix("()")


def matches(name: str, wanted: str) -> bool:
    """Whether `--var wanted` keeps an entry named `name`: the entry's own name, with or without the `()`
    of its arrays, or the variable it starts with. Any letter case."""
    wanted = wanted.upper()
    return wanted in (name, name.replace("()", ""), root(name))


@dataclass
class Block:
    """What the report prints for one name: an entry, or an array that has only aliases and bulk stores."""

    name: str
    procedure: str | None
    entry: Entry | None = None
    aliases: list[tuple[tuple, Site]] = field(default_factory=list)
    bulk: list[tuple[tuple, Site]] = field(default_factory=list)


def blocks(trace: Trace, var: str | None = None) -> list[Block]:
    """The blocks of the report, sorted by procedure (the main module first) and name. An alias or a bulk
    store goes under each entry of its array's name and procedure, or under a block of its own when the
    array has no entry."""
    out = [Block(e.name, e.procedure, e) for e in trace.entries.values()]

    def under(name: str, procedure: str) -> list[Block]:
        procedure = None if procedure == "-" else procedure
        found = [b for b in out if (b.name, b.procedure) == (name, procedure)]
        if not found:
            found = [Block(name, procedure)]
            out.extend(found)
        return found

    for key, site in trace.aliases.items():
        for b in under(key[2], key[3]):
            b.aliases.append((key, site))
    for key, site in trace.untraced.items():
        if key[3] != "-":
            for b in under(key[3], key[4]):
                b.bulk.append((key, site))
    if var:
        out = [b for b in out if matches(b.name, var)]
    return sorted(out, key=lambda b: (b.procedure or "", b.name, b.entry.type if b.entry else ""))


def address_stores(trace: Trace) -> list[tuple[tuple, Site]]:
    """The stores at an address: the untraced stores with no name."""
    return sorted((k, s) for k, s in trace.untraced.items() if k[3] == "-")


def by_count(entry: Entry) -> list[Value]:
    """The values, most recorded first; of equal counts, the one recorded first."""
    return sorted(entry.values.values(), key=lambda v: -v.count)


def times(n: int) -> str:
    return "1 time" if n == 1 else f"{n} times"


def report(trace: Trace, var: str | None = None) -> str:
    out = [
        f"{trace.records} record(s) of {trace.main.name}",
        "A variable's starting value (0 or an empty string) is not a store and is not listed.",
    ]
    for b in blocks(trace, var):
        where = f"  in {b.procedure}" if b.procedure else ""
        out.append("")
        if b.entry is None:
            out.append(f"{b.name}{where}")
            out.append("  no value recorded by name")
        else:
            e = b.entry
            out.append(f"{e.name}  {e.type}{where}")
            what = "length" if e.is_string else "range"
            span = (
                f"{what} {shortest(e.low, e.type)} to {shortest(e.high, e.type)}"
                if e.low
                else "no range"
            )
            cut = ", cut: it took more distinct values than the limit keeps" if e.cut else ""
            out.append(
                f"  {e.stores} store(s), {e.after_calls} after-call record(s), {span}, "
                f"{len(e.values)} distinct value(s){cut}"
            )
            for v in by_count(e):
                out.append(f"  {v.count:>8} x  {shown(e, v)}    first at {v.file}:{v.line}")
        for (file, line, _, _, callee, parameter), site in sorted(b.aliases):
            out.append(
                f"  also stored through {parameter} of {callee}, {file}:{line}, "
                f"{site.count} call{'' if site.count == 1 else 's'}"
            )
        for (file, line, what, _, _), site in sorted(b.bulk):
            out.append(f"  bulk store: {what}, {file}:{line}, {times(site.count)}")
    stores = address_stores(trace)
    if var:
        if stores:
            out.append("")
            out.append(
                f"{len(stores)} statement(s) store at an address; run without --var to list them."
            )
    else:
        out.append("")
        out.append(
            "Stores at an address (tied to no variable):" if stores else "No store at an address."
        )
        for (file, line, what, _, _), site in stores:
            out.append(f"  {what}, {file}:{line}, {times(site.count)}")
    return "\n".join(out)


def as_json(trace: Trace, var: str | None = None) -> dict:
    """The report's data."""

    def sites(found, with_what: bool) -> list[dict]:
        out = []
        for key, site in sorted(found):
            row = {"file": site.file, "line": site.line, "count": site.count}
            if with_what:
                row["what"] = key[2]
            else:
                row["callee"], row["parameter"] = key[4], key[5]
            out.append(row)
        return out

    entries = []
    for b in blocks(trace, var):
        e = b.entry
        row: dict = {"name": b.name, "procedure": b.procedure}
        if e:
            row.update(
                type=e.type,
                stores=e.stores,
                after_calls=e.after_calls,
                cut=e.cut,
                min=shortest(e.low, e.type) if e.low else None,
                max=shortest(e.high, e.type) if e.high else None,
                values=[
                    {
                        "value": json.loads(v.text) if e.is_string else shortest(v.text, e.type),
                        "recorded": v.text,
                        "count": v.count,
                        "file": v.file,
                        "line": v.line,
                        **({"length": v.length, "hash": v.hash} if v.length is not None else {}),
                    }
                    for v in by_count(e)
                ],
            )
        row["aliases"] = sites(b.aliases, False)
        row["bulk"] = sites(b.bulk, True)
        entries.append(row)
    return {
        "records": trace.records,
        "program": trace.main.name,
        "entries": entries,
        "address": sites(address_stores(trace), True),
    }


def main_file(records: list[Path], program: Path | None) -> Path:
    """The main file: `--program`, or the first file of the first record, in the working folder."""
    if program:
        return program.resolve()
    for path in records:
        for line in path.read_bytes().decode("utf-8", "replace").splitlines():
            named = coverage.file_line(line.split("\t"))
            if named and named[0] == 0:
                return Path(named[1][0]).resolve()
    raise TraceError("no record names a main file: give --program")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("records", nargs="+", type=Path, help="record files (QB64RUST_TRACE) to merge")
    ap.add_argument(
        "--program",
        type=Path,
        help="the main file; its folder holds the files the records name (default: the first file a "
        "record names, in the working folder)",
    )
    ap.add_argument("--json", action="store_true", help="print the report as JSON")
    ap.add_argument(
        "--var", metavar="NAME", help="only the entries of this name, in any letter case"
    )
    args = ap.parse_args(argv)
    try:
        trace = Trace(main_file(args.records, args.program))
        for path in args.records:
            trace.add(path)
    except (TraceError, OSError) as e:
        print(f"trace: {e}", file=sys.stderr)
        return 2
    if args.json:
        print(json.dumps(as_json(trace, args.var), indent=2))
    else:
        print(report(trace, args.var))
    return 0


if __name__ == "__main__":
    sys.exit(main())
