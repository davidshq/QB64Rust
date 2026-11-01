#!/usr/bin/env python3
"""Tests of the hook: what guard_tool refuses and lets through.

python -m unittest discover -s tools/hooks -p "test_*.py"
"""

from __future__ import annotations

import sys
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))

import common
import guard_tool


class Staging(unittest.TestCase):
    def refused(self, cmd: str) -> bool:
        with mock.patch.object(guard_tool, "tracked", return_value=False):
            return guard_tool.check_command(cmd) is not None

    def test_git_add_and_friends(self):
        for cmd in (
            "git add -A",
            "cd /x && git add docs/STATUS.md",
            "git stash",
            "git reset HEAD~1",
            "git restore --staged a.rs",
            "git rm --cached a.rs",
            "git commit -am 'x'",
            "git commit -a -m 'x'",
            "git -C sub add .",
        ):
            self.assertTrue(self.refused(cmd), cmd)

    def test_allowed_git(self):
        for cmd in (
            "git status",
            "git diff HEAD",
            "git mv a.md b.md",
            "git commit -m 'msg'",
            "git log -3",
            "git restore a.rs",
        ):
            self.assertFalse(self.refused(cmd), cmd)


class MoveDelete(unittest.TestCase):
    def test_tracked_file_moved_or_deleted(self):
        with mock.patch.object(guard_tool, "tracked", return_value=True):
            self.assertIn("git mv", guard_tool.check_command("mv docs/a.md docs/b.md") or "")
            self.assertIn("git mv", guard_tool.check_command("rm -f docs/a.md") or "")
            self.assertIn("git mv", guard_tool.check_command("Move-Item docs/a.md docs/b.md") or "")
            self.assertIn(
                "git mv", guard_tool.check_command("ls; Remove-Item -Force docs/a.md") or ""
            )

    def test_untracked_file_free(self):
        with mock.patch.object(guard_tool, "tracked", return_value=False):
            self.assertIsNone(
                guard_tool.check_command("mv target/scratch/a.log target/scratch/b.log")
            )
            self.assertIsNone(guard_tool.check_command("rm -rf target/scratch/old"))


class TargetRoot(unittest.TestCase):
    def check(self, cmd: str):
        with mock.patch.object(guard_tool, "tracked", return_value=False):
            return guard_tool.check_command(cmd)

    def test_redirect_to_target_root_refused(self):
        self.assertIn("target/scratch", self.check("cargo test > target/t1.log 2>&1") or "")
        self.assertIn("target/scratch", self.check("cargo test >target\\t1.log 2>&1") or "")
        self.assertIn("target/scratch", self.check("python x.py >> target/notes.txt") or "")

    def test_redirect_below_target_allowed(self):
        self.assertIsNone(self.check("cargo test > target/scratch/2026-10-31/t1.log 2>&1"))
        self.assertIsNone(self.check("cargo test > target/check/logs/tier1.log 2>&1"))
        self.assertIsNone(self.check("echo x > /tmp/a.log"))

    def test_write_tool_paths(self):
        root = common.ROOT
        self.assertIsNotNone(guard_tool.check_path(str(root / "target" / "probe.py")))
        self.assertIsNotNone(guard_tool.check_path(str(root / "target" / "cpp-before" / "a.cpp")))
        self.assertIsNone(
            guard_tool.check_path(str(root / "target" / "scratch" / "d" / "probe.py"))
        )
        self.assertIsNotNone(guard_tool.check_path(str(root / "target" / "close" / "x.log")))
        self.assertIsNone(guard_tool.check_path(str(root / "target" / "check" / "slice.json")))
        self.assertIsNone(guard_tool.check_path(str(root / "docs" / "STATUS.md")))
        self.assertIsNotNone(
            guard_tool.check_path(str(root.parent / "QB64pe" / "source" / "x.bas"))
        )


class TestRuns(unittest.TestCase):
    def test_no_test_run_is_refused(self):
        with mock.patch.object(guard_tool, "tracked", return_value=False):
            for cmd in (
                "cargo test -p qb64rust-sema",
                "python tools/legacy_tests/run_legacy_tests.py --suite corpus --list tests/corpus/slice.list",
                "python tools/legacy_tests/run_legacy_tests.py --suite verification --pass-cache target/pass-cache-2",
                "python tools/callsite/callsite.py --jobs 8",
                "python tools/check/check.py quick",
            ):
                self.assertIsNone(guard_tool.check_command(cmd), cmd)


if __name__ == "__main__":
    unittest.main()
