#!/usr/bin/env python3
"""Tests of value_trace.py: one per scenario of "The report" of the spec compiler/value-trace, and the merge rules
of design D8 of the OpenSpec change value-trace.

Run:  python -m unittest discover -s tools/trace -p "test_*.py"

The record files are written here in the form a value-trace build writes them
(crates/codegen-cpp/src/trace_rt.h); no program is built.
"""

from __future__ import annotations

import io
import json
import tempfile
import value_trace as tr
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path

SOURCE = b"a = 1\r\nfill grid()\r\n"
INCLUDED = b"b = 2\r\n"


class Fixture(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.dir = Path(self.tmp.name)
        self.main = self.dir / "p.bas"
        self.main.write_bytes(SOURCE)
        (self.dir / "inc.bi").write_bytes(INCLUDED)
        self.n = 0

    def record(self, *lines: str, text: bytes = SOURCE) -> Path:
        """A file of one record: the two file lines, then `lines` with a tab for each ` | `."""
        self.n += 1
        head = [
            "trace 1",
            f"file\t0\t{tr.coverage.fnv1a64(text):016x}\tp.bas",
            f"file\t1\t{tr.coverage.fnv1a64(INCLUDED):016x}\tinc.bi",
        ]
        body = [line.replace(" | ", "\t") for line in lines]
        path = self.dir / f"t{self.n}.txt"
        # LF line ends, as the build writes them ("ab").
        path.write_bytes("\n".join(head + body + ["end", ""]).encode())
        return path

    def merged(self, *paths: Path) -> tr.Trace:
        t = tr.Trace(self.main)
        for p in paths:
            t.add(p)
        return t

    def run_main(self, *argv: str) -> tuple[int, str, str]:
        out, err = io.StringIO(), io.StringIO()
        with redirect_stdout(out), redirect_stderr(err):
            rc = tr.main([*argv, "--program", str(self.main)])
        return rc, out.getvalue(), err.getvalue()


class Merge(Fixture):
    def test_two_runs_merged(self):
        # Scenario "Two runs merged": one run stores 1 into `a`, another stores 2.
        one = self.record(
            "entry | 0 | A | SINGLE | - | 1 | 0 | 1 | 0 | 1 | 1", "value | 0 | 1 | 0 | 1 | 1"
        )
        two = self.record(
            "entry | 0 | A | SINGLE | - | 1 | 0 | 1 | 0 | 2 | 2", "value | 0 | 1 | 0 | 1 | 2"
        )
        t = self.merged(one, two)
        a = t.entries[(None, "A", "SINGLE")]
        self.assertEqual((t.records, a.stores, a.low, a.high), (2, 2, "1", "2"))
        self.assertEqual([(v.text, v.count) for v in a.values.values()], [("1", 1), ("2", 1)])
        rc, out, _ = self.run_main(str(one), str(two))
        self.assertEqual(rc, 0)
        self.assertIn("2 record(s) of p.bas", out)
        self.assertIn(
            "A  SINGLE\n  2 store(s), 0 after-call record(s), range 1 to 2, 2 distinct value(s)\n",
            out,
        )
        self.assertIn(
            "         1 x  1    first at p.bas:1\n         1 x  2    first at p.bas:1", out
        )

    def test_counts_add_and_the_first_site_is_the_earliest_records(self):
        # Design D8: the counts, stores and after-call numbers add up; a value's first site is the one of the
        # earliest record; the entry numbers of two records need not agree.
        one = self.record(
            "entry | 0 | A | SINGLE | - | 3 | 1 | 2 | 0 | 1 | 5",
            "value | 0 | 2 | 0 | 1 | 1",
            "value | 0 | 2 | 1 | 1 | 5",
        )
        two = self.record(
            "entry | 4 | B | LONG | - | 1 | 0 | 1 | 0 | 9 | 9",
            "value | 4 | 1 | 0 | 2 | 9",
            "entry | 7 | A | SINGLE | - | 2 | 0 | 2 | 0 | -3 | 5",
            "value | 7 | 1 | 0 | 2 | 5",
            "value | 7 | 1 | 0 | 2 | -3",
        )
        a = self.merged(one, two).entries[(None, "A", "SINGLE")]
        self.assertEqual((a.stores, a.after_calls, a.low, a.high), (5, 1, "-3", "5"))
        self.assertEqual(
            [(v.text, v.count, v.file, v.line) for v in tr.by_count(a)],
            [("5", 3, "inc.bi", 1), ("1", 2, "p.bas", 1), ("-3", 1, "p.bas", 2)],
        )

    def test_a_cut_entry(self):
        # An entry is cut when any of its records is; its range is the widest of the records.
        cut = self.record(
            "entry | 0 | I | SINGLE | - | 1001 | 0 | 2 | 1 | 1 | 1001", "value | 0 | 1 | 0 | 1 | 1"
        )
        whole = self.record(
            "entry | 0 | I | SINGLE | - | 1 | 0 | 1 | 0 | 0 | 0", "value | 0 | 1 | 0 | 1 | 0"
        )
        i = self.merged(whole, cut).entries[(None, "I", "SINGLE")]
        self.assertEqual((i.cut, i.stores, i.low, i.high), (True, 1002, "0", "1001"))
        rc, out, _ = self.run_main(str(whole), str(cut))
        self.assertEqual(rc, 0)
        self.assertIn(
            "range 0 to 1001, 2 distinct value(s), cut: it took more distinct values", out
        )
        _, text, _ = self.run_main("--json", str(whole), str(cut))
        self.assertTrue(json.loads(text)["entries"][0]["cut"])

    def test_one_name_in_two_procedures_and_of_two_types(self):
        t = self.merged(
            self.record(
                "entry | 0 | I | SINGLE | A | 1 | 0 | 1 | 0 | 1 | 1",
                "value | 0 | 1 | 0 | 1 | 1",
                "entry | 1 | I | SINGLE | B | 1 | 0 | 1 | 0 | 2 | 2",
                "value | 1 | 1 | 0 | 1 | 2",
                "entry | 2 | I | STRING | - | 1 | 0 | 1 | 0 | 1 | 1",
                'value | 2 | 1 | 0 | 1 | "x"',
            )
        )
        shown = [(b.procedure, b.name, b.entry.type) for b in tr.blocks(t)]
        self.assertEqual(shown, [(None, "I", "STRING"), ("A", "I", "SINGLE"), ("B", "I", "SINGLE")])
        self.assertIn("I  SINGLE  in B\n", tr.report(t))


class Arrays(Fixture):
    ALIAS = "alias | 0 | 2 | 3 | GRID() | - | FILL | A()"

    def test_an_alias_in_the_report(self):
        # Scenario "An alias in the report": the block for GRID() names A() of FILL.
        record = self.record(
            "entry | 0 | A() | SINGLE | FILL | 3 | 0 | 1 | 0 | 7 | 7",
            "value | 0 | 3 | 0 | 2 | 7",
            self.ALIAS,
        )
        rc, out, _ = self.run_main(str(record))
        self.assertEqual(rc, 0)
        self.assertIn(
            "GRID()\n  no value recorded by name\n  also stored through A() of FILL, p.bas:2, 3 calls\n",
            out,
        )
        _, text, _ = self.run_main("--json", str(record))
        grid = next(e for e in json.loads(text)["entries"] if e["name"] == "GRID()")
        self.assertEqual(
            grid["aliases"],
            [{"file": "p.bas", "line": 2, "count": 3, "callee": "FILL", "parameter": "A()"}],
        )

    def test_an_alias_and_a_bulk_store_go_under_the_arrays_entry(self):
        record = self.record(
            "entry | 0 | GRID() | SINGLE | - | 1 | 0 | 1 | 0 | 4 | 4",
            "value | 0 | 1 | 0 | 1 | 4",
            self.ALIAS,
            "untraced | 0 | 2 | 1 | REDIM | GRID() | -",
            "untraced | 0 | 1 | 2 | POKE | - | -",
            "untraced | 0 | 2 | 1 | ERASE | GRID() | WORK",
        )
        out = tr.report(self.merged(record, record))
        self.assertIn(
            "GRID()  SINGLE\n"
            "  2 store(s), 0 after-call record(s), range 4 to 4, 1 distinct value(s)\n"
            "         2 x  4    first at p.bas:1\n"
            "  also stored through A() of FILL, p.bas:2, 6 calls\n"
            "  bulk store: REDIM, p.bas:2, 2 times\n",
            out,
        )
        # The array of the same name in a procedure is another block.
        self.assertIn(
            "GRID()  in WORK\n  no value recorded by name\n  bulk store: ERASE, p.bas:2, 2 times\n",
            out,
        )
        self.assertTrue(
            out.endswith("Stores at an address (tied to no variable):\n  POKE, p.bas:1, 4 times"),
            out,
        )


class Texts(Fixture):
    def test_a_single_and_a_double_are_shown_at_their_shortest(self):
        self.assertEqual(tr.shortest("0.100000001", "SINGLE"), "0.1")
        self.assertEqual(tr.shortest("0.10000000000000001", "DOUBLE"), "0.1")
        self.assertEqual(tr.shortest("3", "DOUBLE"), "3")
        self.assertEqual(tr.shortest("16777216", "SINGLE"), "16777216")
        self.assertEqual(tr.shortest("20", "SINGLE"), "20")
        self.assertEqual(tr.shortest("1.00000002e+20", "SINGLE"), "1e+20")
        self.assertEqual(tr.shortest("nan", "SINGLE"), "nan")
        # Any other type keeps its text.
        self.assertEqual(
            tr.shortest("0.100000000000000000001", "_FLOAT"), "0.100000000000000000001"
        )
        self.assertEqual(
            tr.shortest("18446744073709551615", "_UNSIGNED _INTEGER64"), "18446744073709551615"
        )

    def test_a_string_with_a_tab_and_one_cut_at_200_bytes(self):
        long = '"' + "y" * 200 + '"'
        record = self.record(
            "entry | 0 | T | STRING | - | 2 | 0 | 2 | 0 | 3 | 300",
            'value | 0 | 1 | 0 | 1 | "a\\u0009b"',
            f"value | 0 | 1 | 0 | 1 | {long} | 300 | 00000000000000a1",
        )
        t = self.merged(record)
        out = tr.report(t)
        self.assertIn("length 3 to 300", out)
        self.assertIn('1 x  "a\\u0009b"    first at p.bas:1', out)
        self.assertIn(f"1 x  {long} (300 bytes, the first 200 shown)", out)
        values = tr.as_json(t)["entries"][0]["values"]
        self.assertEqual(values[0]["value"], "a\tb")
        self.assertEqual(
            (values[1]["value"], values[1]["length"], values[1]["hash"]),
            ("y" * 200, 300, "00000000000000a1"),
        )

    def test_long_strings_that_share_their_first_200_bytes_stay_two_values(self):
        long = '"' + "y" * 200 + '"'
        first = self.record(
            "entry | 0 | T | STRING | - | 2 | 0 | 2 | 0 | 300 | 300",
            f"value | 0 | 1 | 0 | 1 | {long} | 300 | 00000000000000a1",
            f"value | 0 | 1 | 0 | 1 | {long} | 300 | 00000000000000b2",
        )
        # A second run of the first value adds to its count.
        second = self.record(
            "entry | 0 | T | STRING | - | 1 | 0 | 1 | 0 | 300 | 300",
            f"value | 0 | 1 | 0 | 1 | {long} | 300 | 00000000000000a1",
        )
        values = tr.as_json(self.merged(first, second))["entries"][0]["values"]
        self.assertEqual(
            [(v["hash"], v["count"]) for v in values],
            [("00000000000000a1", 2), ("00000000000000b2", 1)],
        )

    def test_a_long_string_without_its_hash_is_refused(self):
        long = '"' + "y" * 200 + '"'
        record = self.record(
            "entry | 0 | T | STRING | - | 1 | 0 | 1 | 0 | 300 | 300",
            f"value | 0 | 1 | 0 | 1 | {long} | 300",
        )
        with self.assertRaises(tr.TraceError):
            self.merged(record)

    def test_a_hash_that_is_not_16_lower_case_hex_digits_is_refused(self):
        long = '"' + "y" * 200 + '"'
        for digest in ("00000000000000A1", "0x000000000000a1", "0000_000000000a1", "a1"):
            record = self.record(
                "entry | 0 | T | STRING | - | 1 | 0 | 1 | 0 | 300 | 300",
                f"value | 0 | 1 | 0 | 1 | {long} | 300 | {digest}",
            )
            with self.subTest(digest=digest), self.assertRaises(tr.TraceError):
                self.merged(record)

    def test_a_range_of_nans_is_none(self):
        record = self.record(
            "entry | 0 | D | DOUBLE | - | 1 | 0 | 1 | 0 | - | -", "value | 0 | 1 | 0 | 1 | nan"
        )
        out = tr.report(self.merged(record))
        self.assertIn("1 store(s), 0 after-call record(s), no range, 1 distinct value(s)", out)
        self.assertIn("1 x  nan    first at p.bas:1", out)


class Var(Fixture):
    def test_var_keeps_the_entries_of_one_name(self):
        record = self.record(
            "entry | 0 | A | SINGLE | - | 1 | 0 | 1 | 0 | 1 | 1",
            "value | 0 | 1 | 0 | 1 | 1",
            "entry | 1 | A | STRING | S | 1 | 0 | 1 | 0 | 1 | 1",
            'value | 1 | 1 | 0 | 1 | "x"',
            "entry | 2 | SHIPS().HP | INTEGER | - | 1 | 0 | 1 | 0 | 9 | 9",
            "value | 2 | 1 | 0 | 1 | 9",
            "entry | 3 | AB | SINGLE | - | 1 | 0 | 1 | 0 | 1 | 1",
            "value | 3 | 1 | 0 | 1 | 1",
            "untraced | 0 | 1 | 2 | POKE | - | -",
        )
        t = self.merged(record)
        self.assertEqual(
            [(b.name, b.procedure) for b in tr.blocks(t, "a")], [("A", None), ("A", "S")]
        )
        for name in ("ships", "SHIPS.HP", "ships().hp"):
            self.assertEqual([b.name for b in tr.blocks(t, name)], ["SHIPS().HP"], name)
        rc, out, _ = self.run_main("--var", "a", str(record))
        self.assertEqual(rc, 0)
        self.assertNotIn("AB  SINGLE", out)
        self.assertIn("1 statement(s) store at an address; run without --var to list them.", out)


class Refused(Fixture):
    def test_records_of_another_program(self):
        # Scenario "Records of another program": a message naming the file, and nothing reported.
        record = self.record(
            "entry | 0 | A | SINGLE | - | 1 | 0 | 1 | 0 | 1 | 1", text=b"a = 2\r\n"
        )
        rc, out, err = self.run_main(str(record))
        self.assertEqual((rc, out), (2, ""))
        self.assertIn("another text of p.bas", err)

    def test_a_file_that_is_not_there(self):
        (self.dir / "inc.bi").unlink()
        rc, out, err = self.run_main(str(self.record()))
        self.assertEqual((rc, out), (2, ""))
        self.assertIn("inc.bi", err)

    def test_a_record_cut_short_and_a_line_that_is_none(self):
        whole = self.record("entry | 0 | A | SINGLE | - | 1 | 0 | 1 | 0 | 1 | 1")
        cut = self.dir / "cut.txt"
        cut.write_bytes(whole.read_bytes().removesuffix(b"end\n"))
        rc, out, err = self.run_main(str(cut))
        self.assertEqual((rc, out), (2, ""))
        self.assertIn("without its end", err)
        for line in (
            "entry | 0 | A",
            "value | 9 | 1 | 0 | 1 | 1",
            "value | 0 | 1 | 5 | 1 | 1",
            "other",
        ):
            rc, out, err = self.run_main(
                str(self.record("entry | 0 | A | SINGLE | - | 1 | 0 | 1 | 0 | 1 | 1", line))
            )
            self.assertEqual((rc, out), (2, ""), line)
        other = self.dir / "counts.txt"
        other.write_bytes(b"coverage 2\nend\n")
        rc, _, err = self.run_main(str(other))
        self.assertEqual(rc, 2)
        self.assertIn("not a value-trace record", err)

    def test_the_main_file_is_the_records_first_file_by_default(self):
        record = self.record()
        self.assertEqual(tr.main_file([record], None).name, "p.bas")
        self.assertEqual(tr.main_file([record], self.main), self.main.resolve())


if __name__ == "__main__":
    unittest.main()
