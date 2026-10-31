#!/usr/bin/env python3
"""Tests of clean.py on a made-up target/ folder.

python -m unittest discover -s tools/clean -p "test_*.py"
"""

from __future__ import annotations

import os
import sys
import tempfile
import time
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import clean

OLD = time.time() - 30 * 86400


def touch(p: Path, text: str = "x", mtime: float | None = None) -> Path:
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text)
    if mtime is not None:
        os.utime(p, (mtime, mtime))
    return p


class Clean(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.target = Path(self.tmp.name) / "target"
        # Two compiler copies; the release executable matches the first.
        exe = touch(self.target / "release" / clean.EXE, "compiler bytes")
        h = clean.current_hash(self.target)
        touch(self.target / "test-bin" / h[:16] / clean.EXE, "compiler bytes")
        touch(self.target / "test-bin" / "0000000000000000" / clean.EXE, "older", mtime=OLD)
        self.assertTrue(exe.is_file())
        # Result folders: one old, one fresh.
        touch(self.target / "legacy-tests-old" / "results.json", mtime=OLD)
        os.utime(self.target / "legacy-tests-old", (OLD, OLD))
        touch(self.target / "legacy-tests" / "results.json")
        # Scratch at the root and in scratch/.
        touch(self.target / "t1.log")
        touch(self.target / "probe.py")
        touch(self.target / "CACHEDIR.TAG")
        touch(self.target / ".rustc_info.json")
        touch(self.target / "scratch" / "old-day" / "a.log", mtime=OLD)
        os.utime(self.target / "scratch" / "old-day", (OLD, OLD))
        touch(self.target / "scratch" / "hooks" / "stop-x.count", mtime=OLD)
        os.utime(self.target / "scratch" / "hooks", (OLD, OLD))
        (self.target / "qb64pe-copies" / "w1").mkdir(parents=True)

    def tearDown(self):
        self.tmp.cleanup()

    def test_dry_run_changes_nothing(self):
        _, lines = clean.run(self.target, dry=True)
        self.assertTrue((self.target / "t1.log").is_file())
        self.assertTrue((self.target / "test-bin" / "0000000000000000").is_dir())
        self.assertTrue(any(l.startswith("would free") for l in lines))

    def test_run(self):
        _, lines = clean.run(self.target, dry=False)
        kept = [d.name for d in (self.target / "test-bin").iterdir()]
        self.assertEqual(len(kept), 1)
        self.assertTrue(clean.current_hash(self.target).startswith(kept[0]))
        self.assertFalse((self.target / "legacy-tests-old").exists())
        self.assertTrue((self.target / "legacy-tests").exists())
        self.assertFalse((self.target / "t1.log").exists())
        self.assertFalse((self.target / "probe.py").exists())
        self.assertTrue((self.target / "CACHEDIR.TAG").exists())
        self.assertTrue((self.target / ".rustc_info.json").exists())
        swept = [d for d in (self.target / "scratch").iterdir() if d.name.startswith("root-sweep-")]
        self.assertEqual(len(swept), 1)
        self.assertTrue((swept[0] / "t1.log").is_file())
        self.assertFalse((self.target / "scratch" / "old-day").exists())
        self.assertTrue((self.target / "scratch" / "hooks" / "stop-x.count").is_file())
        self.assertTrue(any("qb64pe-copies: 1 copies" in l for l in lines))

    def test_no_target(self):
        _, lines = clean.run(self.target / "none", dry=False)
        self.assertEqual(lines, ["no target/ folder"])


if __name__ == "__main__":
    unittest.main()
