#!/usr/bin/env python3
"""Tests of outputs.py: what a step in the suites VM copies out (OpenSpec change vm-suites, design D8).

    python -m unittest discover -s tools/winvm -p "test_*.py"
"""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import outputs  # noqa: E402


class Outputs(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.tree = Path(self.tmp.name)

    def write(self, rel: str, data: str = "x") -> None:
        p = self.tree / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(data, encoding="utf-8")

    def test_a_results_file_brings_the_files_of_its_failures_and_no_executable(self):
        results = {
            "results": [
                {"suite": "corpus", "test": "g/p1", "status": "PASS"},
                {"suite": "corpus", "test": "g/p2", "status": "FAIL"},
                {"suite": "verification", "test": "v10", "status": "FAIL"},
            ]
        }
        self.write("target/legacy-tests/results-partial.json", json.dumps(results))
        for rel in (
            "corpus/g-p1-run1-output.txt",
            "corpus/g-p2-run1-output.txt",
            "corpus/g-p2-log.txt",
            "corpus/g-p2/p2.bas",
            "corpus/g-p2/p2.exe",
            "corpus/g-p20-run1-output.txt",
            "corpus/v10-out.txt",
            "verification/v10-out.txt",
            "verification/v100-out.txt",
            "verification/v10-output.exe",
        ):
            self.write(f"target/legacy-tests/{rel}")
        got = outputs.listed(self.tree, ["target/legacy-tests/results-partial.json", "target/coverage/merge/x.txt"])
        self.assertEqual(
            got,
            [
                "target/legacy-tests/corpus/g-p2-log.txt",
                "target/legacy-tests/corpus/g-p2-run1-output.txt",
                "target/legacy-tests/corpus/g-p2/p2.bas",
                "target/legacy-tests/results-partial.json",
                "target/legacy-tests/verification/v10-out.txt",
            ],
        )

    def test_a_plain_file_and_a_missing_one(self):
        self.write("target/coverage/merge/corpus.txt")
        self.assertEqual(
            outputs.listed(self.tree, ["target/coverage/merge/corpus.txt", "target/none.json"]),
            ["target/coverage/merge/corpus.txt"],
        )


if __name__ == "__main__":
    unittest.main()
