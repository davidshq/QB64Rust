#!/usr/bin/env python3
"""Tests of builtin_runs.py: one per scenario of the spec testing/coverage's requirements on built-ins.

Run:  python -m unittest discover -s tools/coverage -p "test_*.py"

The counts files are written here in the form a coverage build writes them (version 2,
crates/codegen-cpp/src/coverage.rs); no program is built.
"""

from __future__ import annotations

import io
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path

import builtin_runs as bi


class Fixture(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.dir = Path(self.tmp.name)
        self.list = self.dir / "never_run.list"
        self.list.write_text("# none\n", encoding="utf-8")
        # sema's tables, in the form of crates/sema/src/builtins.rs.
        self.table = self.dir / "builtins.rs"
        self.table.write_text(
            'pub const SUPPORTED: &[(&str, Rule)] = &[\n    ("LEN", Rule::Plain),\n];\n'
            '        ("sub_seek", 1) => Slot::Int64,\n'
            'pub const STATEMENTS: &[(&str, StmtRule)] = &[\n    ("_TITLE", StmtRule::Plain),\n];\n'
            'pub const OWN_NODE: &[&str] = &["LPRINT"];\n',
            encoding="utf-8",
        )
        self.n = 0

    def counts(self, program: str, entries: list[tuple[int, str]], version: int = 2) -> Path:
        """A counts file of one run of `program`: one entry per (count, names) on its own line."""
        self.n += 1
        lines = [f"coverage {version}", f"file\t0\t{0:016x}\t{program}"]
        for i, (n, names) in enumerate(entries, 1):
            lines.append(f"0\t{i}\t1\t{n}" + (f"\t{names}" if version == 2 else ""))
        path = self.dir / f"c{self.n}.txt"
        path.write_bytes("\n".join(lines + ["end", ""]).encode())
        return path

    def merged(self, *paths: Path) -> bi.Builtins:
        b = bi.Builtins()
        for p in paths:
            b.add(p)
        return b

    def listing(self, text: str) -> None:
        self.list.write_text(text, encoding="utf-8")

    def run_main(self, *argv: str) -> tuple[int, str]:
        out = io.StringIO()
        with redirect_stdout(out):
            rc = bi.main([*argv, "--list", str(self.list), "--table", str(self.table)])
        return rc, out.getvalue()


class BuiltinsReached(Fixture):
    def test_run_in_one_program_unreached_in_another(self):
        # The spec's scenario: LEN stands after SYSTEM in one program and runs in another.
        one = self.counts("a.bas", [(1, ""), (0, "LEN")])
        two = self.counts("b.bas", [(3, "LEFT$ LEN")])
        b = self.merged(one, two)
        self.assertEqual(b.not_run(), [])
        self.assertEqual(b.count["LEN"], 3)
        self.assertEqual(b.held["LEN"], {"a.bas", "b.bas"})

    def test_only_in_unreached_code(self):
        # The spec's scenario: every statement that names _TITLE has the count 0.
        one = self.counts("a.bas", [(1, "LEN"), (0, "_TITLE")])
        two = self.counts("b.bas", [(0, "_TITLE")])
        b = self.merged(one, two)
        self.assertEqual(b.not_run(), ["_TITLE"])
        self.assertIn("not run: _TITLE (held by a.bas, b.bas)", bi.report(b))

    def test_a_record_of_version_1_names_none(self):
        b = self.merged(self.counts("a.bas", [(1, "")], version=1))
        self.assertEqual((b.records, b.count), (1, {}))

    def test_many_programs_are_cut_short(self):
        paths = [self.counts(f"p{i}.bas", [(0, "_TITLE")]) for i in range(7)]
        self.assertIn("p4.bas and 2 more", bi.report(self.merged(*paths)))


class TheList(Fixture):
    def test_a_reason_outside_the_set(self):
        # The spec's scenario: the check fails, naming the line of the list.
        self.listing("# c\n_TITLE\tloud\n")
        rc, out = self.run_main("--check", str(self.counts("a.bas", [(0, "_TITLE")])))
        self.assertEqual(rc, 1)
        self.assertIn("never_run.list line 2: `loud` is no reason of the list", out)

    def test_a_list_that_matches_passes(self):
        self.listing("_TITLE\tdesktop the window's title\n")
        counts = self.counts("a.bas", [(1, "LEN"), (0, "_TITLE"), (1, "LPRINT")])
        rc, out = self.run_main("--check", str(counts))
        self.assertEqual(rc, 0, out)
        self.assertIn("check: PASS", out)

    def test_a_line_of_spaces_is_no_empty_line(self):
        # As tier 1 reads the list: only an empty line is skipped.
        self.listing("_TITLE\tdesktop\n  \n")
        rc, out = self.run_main("--check", str(self.counts("a.bas", [(1, "LEN"), (0, "_TITLE")])))
        self.assertEqual(rc, 1)
        self.assertIn("never_run.list line 2: no tab after the name", out)


class TheCheck(Fixture):
    def test_a_builtin_reached_only_in_unreached_code(self):
        # The spec's scenario: not in the list, so the check fails, naming it and its programs.
        rc, out = self.run_main("--check", str(self.counts("a.bas", [(1, "LEN"), (0, "_TITLE")])))
        self.assertEqual(rc, 1)
        self.assertIn("`_TITLE` is run by no test and is not in never_run.list (held by a.bas)", out)

    def test_a_listed_builtin_that_ran(self):
        # The spec's scenario: a new program runs _TITLE, which is listed.
        self.listing("_TITLE\tdesktop\n")
        counts = self.counts("w.bas", [(1, "_TITLE")])
        rc, out = self.run_main("--check", str(counts))
        self.assertEqual(rc, 1)
        self.assertIn("`_TITLE` is listed (desktop) but ran (in w.bas): take it out of the list", out)

    def test_a_listed_builtin_no_record_names(self):
        self.listing("_TITLE\tdesktop\n")
        rc, out = self.run_main("--check", str(self.counts("a.bas", [(1, "LEN")])))
        self.assertEqual(rc, 1)
        self.assertIn("`_TITLE` is listed (desktop) but no record names it", out)

    def test_a_builtin_of_the_tables_no_record_names(self):
        # The spec's scenario: LPRINT stands only where no statement is counted, so no record names it.
        rc, out = self.run_main("--check", str(self.counts("a.bas", [(1, "LEN _TITLE")])))
        self.assertEqual(rc, 1)
        self.assertIn("`LPRINT` is a built-in sema compiles but no record names it", out)
        self.assertNotIn("sub_seek", out)

    def test_the_tables_are_read_from_their_rows(self):
        self.assertEqual(bi.table_names(self.table), {"LEN", "_TITLE", "LPRINT"})
        self.table.write_text("// no tables\n", encoding="utf-8")
        with self.assertRaisesRegex(bi.CoverageError, "no rows of the built-in tables"):
            bi.table_names(self.table)

    def test_a_record_cut_short_stops_the_tool(self):
        path = self.counts("a.bas", [(1, "LEN")])
        path.write_bytes(path.read_bytes().replace(b"end\n", b""))
        self.assertEqual(self.run_main(str(path))[0], 2)


if __name__ == "__main__":
    unittest.main()
