#!/usr/bin/env python3
"""Tests of counts.py (study/archive/31 item 2): a block is replaced and nothing else, and results that are
missing or not of this build are never written as current.

    python -m unittest discover -s tools/legacy_tests -p "test_*.py"
"""

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import counts
import run_legacy_tests as rlt


class Blocks(unittest.TestCase):
    def test_only_the_block_changes_in_the_text_s_own_line_ends(self):
        for nl in ("\n", "\r\n"):
            text = nl.join(
                ["# T", "prose", "<!-- counts:x -->", "old", "<!-- /counts:x -->", "after", ""]
            )
            new = counts.replace_block(text, "x", ["a", "b"])
            want = nl.join(
                ["# T", "prose", "<!-- counts:x -->", "a", "b", "<!-- /counts:x -->", "after", ""]
            )
            self.assertEqual(new, want, repr(nl))
            self.assertEqual(counts.replace_block(new, "x", ["a", "b"]), new)

    def test_an_empty_block_is_filled(self):
        text = "<!-- counts:x -->\n<!-- /counts:x -->\n"
        self.assertEqual(
            counts.replace_block(text, "x", ["a"]), "<!-- counts:x -->\na\n<!-- /counts:x -->\n"
        )

    def test_markers_missing_or_twice_are_an_error(self):
        with self.assertRaises(ValueError):
            counts.replace_block("no markers\n", "x", ["a"])
        twice = "<!-- counts:x -->\n<!-- /counts:x -->\n" * 2
        with self.assertRaises(ValueError):
            counts.replace_block(twice, "x", ["a"])


class VerificationMembership(unittest.TestCase):
    def test_the_lists_and_the_reasons_of_known_list(self):
        with tempfile.TemporaryDirectory() as tmp:
            root, lists = Path(tmp, "verification"), Path(tmp, "lists")
            root.mkdir()
            lists.mkdir()
            for name in ("v01_a", "v01_b", "v02_c", "v02_d"):
                (root / f"{name}.bas").write_bytes(b"")
            (lists / "pass.list").write_text("v01_a\n# a comment\n", encoding="utf-8")
            (lists / "known.list").write_text(
                "v01_b\tD-026\nv02_c\tbug: PRINT USING\nv02_d\tbug: LOCATE\n", encoding="utf-8"
            )
            with (
                mock.patch.object(counts, "VERIFICATION_ROOT", root),
                mock.patch.object(counts, "VERIFICATION", lists),
            ):
                lines = counts.verification_membership()
        self.assertEqual(
            lines,
            [
                "Of the 4 programs of `verification\\`, `pass.list` names **1** and `known.list` 3.",
                "",
                "| Reason in `known.list` | Programs |",
                "|---|---|",
                "| `D-026` | 1 |",
                "| `bug` | 2 |",
            ],
        )


class Results(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)
        self.addCleanup(self.tmp.cleanup)
        self.compiler = self.dir / "qb64rust.exe"
        self.compiler.write_bytes(b"build 1")
        for name, value in (("RESULTS", self.dir), ("COMPILER", self.compiler)):
            patcher = mock.patch.object(counts, name, value)
            patcher.start()
            self.addCleanup(patcher.stop)

    def write(self, summary: dict | None = None, **run) -> None:
        record = {
            "version": rlt.RUN_RECORD_VERSION,
            "selection": {},
            "no_fold": False,
            "compiler_sha256": rlt.file_sha256(self.compiler),
            "runner_sha256": rlt.file_sha256(Path(rlt.__file__).resolve()),
        }
        record.update(run)
        data = {"run": record, "summary": {"corpus": summary or {}}, "results": []}
        (self.dir / "results-corpus.json").write_text(json.dumps(data))

    def test_a_run_of_this_build_is_taken(self):
        self.write()
        data, why = counts.full_run("corpus")
        self.assertEqual(why, "")
        self.assertIsNotNone(data)

    def test_no_results_or_results_of_another_kind_are_refused(self):
        self.assertIn("no full run", counts.full_run("corpus")[1])
        for run, why in [
            ({"version": 0}, "no run record"),
            ({"compiler_sha256": "0"}, "not of the release build"),
            ({"runner_sha256": "0"}, "another version of the runner"),
            ({"pass_cache": True}, "not a plain full run"),
            ({"selection": {"glob": "s1*"}}, "not a plain full run"),
            ({"no_fold": True}, "not a plain full run"),
        ]:
            self.write(**run)
            data, reason = counts.full_run("corpus")
            self.assertIsNone(data, run)
            self.assertIn(why, reason, run)

    def test_the_counts_are_worded_in_the_singular_and_the_plural(self):
        run = {"started": 0.0, "ended": 60.0, "jobs": 8, "build_cache": True}
        with (
            mock.patch.object(rlt, "load_list", return_value=set()),
            mock.patch.object(rlt, "check_list", return_value=([], [], 0)),
        ):
            for fail, kfail, want in [
                (1, 1, ", 1 does not (a wrong result, "),
                (2, 5, ", 2 do not (a wrong result, "),
                (0, 0, ", 0 do not (a wrong result, "),
            ]:
                self.write({"PASS": 3, "FAIL": fail, "KFAIL": kfail}, **run)
                lines, why = counts.results_block("corpus", self.dir, self.dir / "slice.list")
                self.assertEqual(why, "")
                self.assertIn(want, lines[0])
                self.assertIn(f"{kfail} known failure{'' if kfail == 1 else 's'} (", lines[0])


if __name__ == "__main__":
    unittest.main()
