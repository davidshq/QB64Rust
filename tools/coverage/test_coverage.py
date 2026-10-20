#!/usr/bin/env python3
"""Tests of coverage.py: one per scenario of the spec testing/coverage, and the run of the scripts.

Run:  python -m unittest discover -s tools/coverage -p "test_*.py"

The counts files are written here in the form a coverage build writes them
(crates/codegen-cpp/src/coverage.rs); no program is built.
"""

from __future__ import annotations

import fnmatch
import io
import tempfile
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path
from unittest import mock

import coverage as cov

# Line 10 holds two statements: the IF's test at column 1 and the assignment of its THEN at column 11.
SOURCE = b"\r\n" * 9 + b"IF a THEN b = 1\r\nPRINT a\r\nPRINT b\r\n"


class Fixture(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.dir = Path(self.tmp.name)
        self.main = self.dir / "p.bas"
        self.main.write_bytes(SOURCE)
        self.list = self.dir / "p.unreached"
        self.n = 0

    def counts(self, ran: dict[tuple[int, int], int], text: bytes = SOURCE) -> Path:
        """A counts file of one run: the statements of SOURCE with the counts in `ran` (0 if absent)."""
        self.n += 1
        lines = ["coverage 1", f"file\t0\t{cov.fnv1a64(text):016x}\tp.bas"]
        for at in [(10, 1), (10, 11), (11, 1), (12, 1)]:
            lines.append(f"0\t{at[0]}\t{at[1]}\t{ran.get(at, 0)}")
        path = self.dir / f"c{self.n}.txt"
        # LF line ends, as the build writes them ("ab").
        path.write_bytes("\n".join(lines + ["end", ""]).encode())
        return path

    def merged(self, *paths: Path) -> cov.Coverage:
        c = cov.Coverage(self.main)
        for p in paths:
            c.add(p)
        return c

    def run_main(self, *argv: str) -> tuple[int, str, str]:
        out, err = io.StringIO(), io.StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            rc = cov.main([*argv, "--program", str(self.main), "--unreached", str(self.list)])
        return rc, out.getvalue(), err.getvalue()


class CoverageOfRuns(Fixture):
    def test_a_line_with_two_statements(self):
        # The spec's scenario: the runs took the IF but never its THEN.
        c = self.merged(self.counts({(10, 1): 3, (11, 1): 1, (12, 1): 1}))
        self.assertEqual(c.lines("p.bas")[10], ("partly", [11]))
        self.assertIn("line 10: partly run; not run: column 11", cov.report(c))

    def test_two_runs_merged(self):
        one = self.counts({(10, 1): 1, (10, 11): 1, (11, 1): 1})
        two = self.counts({(10, 1): 1, (12, 1): 2})
        lines = self.merged(one).lines("p.bas")
        self.assertEqual((lines[11][0], lines[12][0]), ("run", "not run"))
        lines = self.merged(one, two).lines("p.bas")
        self.assertEqual([lines[n][0] for n in (10, 11, 12)], ["run", "run", "run"])
        # Two records in one file merge as two files do.
        both = self.dir / "both.txt"
        both.write_bytes(one.read_bytes() + two.read_bytes())
        self.assertEqual(self.merged(both).lines("p.bas"), lines)

    def test_counts_of_another_program(self):
        # The spec's scenario: the tool stops with a message naming the file, and reports nothing.
        stale = self.counts({(10, 1): 1}, text=b"PRINT 1\r\n")
        rc, out, err = self.run_main(str(stale))
        self.assertEqual((rc, out), (2, ""))
        self.assertIn("p.bas", err)
        self.assertIn("another text", err)

    def test_a_record_cut_short_is_refused(self):
        path = self.counts({})
        path.write_bytes(path.read_bytes().replace(b"end\n", b""))
        with self.assertRaisesRegex(cov.CoverageError, "without its end"):
            self.merged(path)


class TheList(Fixture):
    def test_a_reason_outside_the_set(self):
        # The spec's scenario: the check fails, naming the line of the list.
        self.list.write_text("# reasons\n12\tforgotten\n", encoding="utf-8")
        problems = cov.check(self.merged(self.counts({(10, 1): 1, (10, 11): 1, (11, 1): 1})), self.list)
        self.assertEqual(len(problems), 2, problems)
        self.assertIn("p.unreached line 2: the reason 'forgotten' is not one of", problems[0])

    def test_a_list_that_matches_passes(self):
        self.list.write_text("10:11\tdead the THEN needs a\n12\tinput\n", encoding="utf-8")
        rc, out, _ = self.run_main("--check", str(self.counts({(10, 1): 1, (11, 1): 1})))
        self.assertEqual(rc, 0, out)
        self.assertIn("check: PASS", out)

    def test_a_partly_run_line_lists_its_columns(self):
        self.list.write_text("10\tdead\n", encoding="utf-8")
        problems = cov.check(self.merged(self.counts({(10, 1): 1, (11, 1): 1, (12, 1): 1})), self.list)
        self.assertEqual(len(problems), 1, problems)
        self.assertIn("the columns not run are 11", problems[0])

    def test_only_an_edit_may_list_a_line_without_a_statement(self):
        self.list.write_text("3\tedit\n4\tdead\n", encoding="utf-8")
        ran = {(10, 1): 1, (10, 11): 1, (11, 1): 1, (12, 1): 1}
        problems = cov.check(self.merged(self.counts(ran)), self.list)
        self.assertEqual(problems, ["line 4: listed (dead) but holds no statement; only an edit may remove one"])


class TheCheck(Fixture):
    def test_a_new_script_covers_a_listed_line(self):
        # The spec's scenario: the check fails, naming the line as listed but run.
        self.list.write_text("12\tdead\n", encoding="utf-8")
        before = self.counts({(10, 1): 1, (10, 11): 1, (11, 1): 1})
        self.assertEqual(self.run_main("--check", str(before))[0], 0)
        rc, out, _ = self.run_main("--check", str(before), str(self.counts({(12, 1): 1})))
        self.assertEqual(rc, 1)
        self.assertIn("[check] line 12: listed (dead) but run in full", out)

    def test_a_line_no_script_reaches(self):
        # The spec's scenario: the check fails, naming the line.
        rc, out, _ = self.run_main("--check", str(self.counts({(10, 1): 1, (10, 11): 1, (12, 1): 1})))
        self.assertEqual(rc, 1)
        self.assertIn("[check] line 11: not run, and not in p.unreached", out)

    def test_without_counts_the_scripts_run_with_coverage(self):
        made = []

        def runner(cmd, check):
            made.append(cmd)
            counts = Path(cmd[cmd.index("--coverage") + 1])
            counts.write_bytes(self.counts({(10, 1): 1, (10, 11): 1, (11, 1): 1, (12, 1): 1}).read_bytes())
            return mock.Mock(returncode=0)

        with mock.patch.object(cov, "REPO", self.dir), mock.patch.object(cov.subprocess, "run", runner):
            rc, out, _ = self.run_main("--check")
        self.assertEqual(rc, 0, out)
        cmd = made[0]
        # The program and its scripts (p.<script>.bas), not another program whose name starts with p.
        glob = cmd[cmd.index("--glob") + 1]
        self.assertEqual(glob, "p.*")
        for name, match in (("p.bas", True), ("p.navy.bas", True), ("p2.bas", False), ("px.navy.bas", False)):
            self.assertEqual(fnmatch.fnmatch(name, glob), match, name)
        self.assertEqual(cmd[cmd.index("--suite") + 1], "corpus")
        self.assertIn("--build-cache", cmd)

    def test_a_script_that_fails_fails_the_check(self):
        def runner(cmd, check):
            counts = Path(cmd[cmd.index("--coverage") + 1])
            counts.write_bytes(self.counts({(10, 1): 1, (10, 11): 1, (11, 1): 1, (12, 1): 1}).read_bytes())
            return mock.Mock(returncode=1)

        with mock.patch.object(cov, "REPO", self.dir), mock.patch.object(cov.subprocess, "run", runner):
            rc, out, _ = self.run_main("--check")
        self.assertEqual(rc, 1)
        self.assertIn("a script failed", out)


if __name__ == "__main__":
    unittest.main()
