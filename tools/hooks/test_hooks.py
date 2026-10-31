#!/usr/bin/env python3
"""Tests of the hooks: what guard_tool refuses and lets through, and when stop_continue keeps Claude going.

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
import stop_continue

TASKS = [("m8-thing", "2.1 Build the thing")]


class Staging(unittest.TestCase):
    def refused(self, cmd: str) -> bool:
        with (
            mock.patch.object(guard_tool, "tracked", return_value=False),
            mock.patch.object(guard_tool, "open_tasks", return_value=[]),
        ):
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
        with (
            mock.patch.object(guard_tool, "tracked", return_value=False),
            mock.patch.object(guard_tool, "open_tasks", return_value=[]),
        ):
            self.assertIsNone(
                guard_tool.check_command("mv target/scratch/a.log target/scratch/b.log")
            )
            self.assertIsNone(guard_tool.check_command("rm -rf target/scratch/old"))


class TargetRoot(unittest.TestCase):
    def check(self, cmd: str):
        with (
            mock.patch.object(guard_tool, "tracked", return_value=False),
            mock.patch.object(guard_tool, "open_tasks", return_value=[]),
        ):
            return guard_tool.check_command(cmd)

    def test_redirect_to_target_root_refused(self):
        self.assertIn("target/scratch", self.check("cargo test > target/t1.log 2>&1") or "")
        self.assertIn("target/scratch", self.check("cargo test >target\\t1.log 2>&1") or "")
        self.assertIn("target/scratch", self.check("python x.py >> target/notes.txt") or "")

    def test_redirect_below_target_allowed(self):
        self.assertIsNone(self.check("cargo test > target/scratch/2026-10-31/t1.log 2>&1"))
        self.assertIsNone(self.check("cargo test > target/close/tier1.log 2>&1"))
        self.assertIsNone(self.check("echo x > /tmp/a.log"))

    def test_write_tool_paths(self):
        root = common.ROOT
        self.assertIsNotNone(guard_tool.check_path(str(root / "target" / "probe.py")))
        self.assertIsNotNone(guard_tool.check_path(str(root / "target" / "cpp-before" / "a.cpp")))
        self.assertIsNone(
            guard_tool.check_path(str(root / "target" / "scratch" / "d" / "probe.py"))
        )
        self.assertIsNone(guard_tool.check_path(str(root / "target" / "close" / "x.log")))
        self.assertIsNone(guard_tool.check_path(str(root / "docs" / "STATUS.md")))
        self.assertIsNotNone(
            guard_tool.check_path(str(root.parent / "QB64pe" / "source" / "x.bas"))
        )


class CloseGate(unittest.TestCase):
    def check(self, cmd: str, tasks):
        with (
            mock.patch.object(guard_tool, "tracked", return_value=False),
            mock.patch.object(guard_tool, "open_tasks", return_value=tasks),
        ):
            return guard_tool.check_command(cmd)

    def test_closing_runs_wait_for_open_tasks(self):
        for cmd in (
            "python tools/close/close.py --plan",
            "python tools/close/close.py --check",
            "python tools/legacy_tests/run_legacy_tests.py --suite corpus --list tests/corpus/slice.list",
            "python tools/legacy_tests/run_legacy_tests.py --suite verification --pass-cache target/pass-cache",
            "python tools/callsite/check.py",
        ):
            self.assertIn("open task", self.check(cmd, TASKS) or "", cmd)

    def test_closing_runs_free_when_done(self):
        self.assertIsNone(self.check("python tools/close/close.py --plan", []))
        self.assertIsNone(self.check("python tools/close/close.py --plan --merge", TASKS))

    def test_working_runs_free(self):
        for cmd in (
            "cargo test -p qb64rust-sema",
            "python tools/legacy_tests/run_legacy_tests.py --suite corpus --glob 'tests/corpus/s16*'",
            "python -m unittest discover -s tools/close -p 'test_*.py'",
        ):
            self.assertIsNone(self.check(cmd, TASKS), cmd)


class StopHook(unittest.TestCase):
    def test_open_tasks_keep_going(self):
        reason = stop_continue.decide("Done with 2.1.", TASKS, 0)
        self.assertIn("2.1 Build the thing", reason or "")

    def test_question_or_handover_allows_stop(self):
        self.assertIsNone(stop_continue.decide("Which of the two do you want?", TASKS, 0))
        self.assertIsNone(stop_continue.decide("Files to stage: a.rs, b.rs.", TASKS, 0))
        self.assertIsNone(stop_continue.decide("I am blocked on the WSL network.", TASKS, 0))

    def test_input_without_message_lets_the_stop_through(self):
        import io

        with (
            mock.patch.object(stop_continue, "open_tasks", return_value=TASKS),
            mock.patch.object(sys, "stdin", io.StringIO('{"session_id": "t"}')),
        ):
            self.assertEqual(stop_continue.main(), 0)

    def test_no_tasks_or_cap(self):
        self.assertIsNone(stop_continue.decide("Done.", [], 0))
        self.assertIsNone(stop_continue.decide("Done.", TASKS, stop_continue.MAX_CONTINUES))


class OpenTasks(unittest.TestCase):
    def test_reads_unchecked_and_skips_closing_and_archive(self):
        import tempfile
        import time

        with tempfile.TemporaryDirectory() as d:
            changes = Path(d)
            (changes / "archive" / "old").mkdir(parents=True)
            (changes / "archive" / "old" / "tasks.md").write_text("- [ ] 1.1 never\n")
            c = changes / "m8-x"
            c.mkdir()
            (c / "tasks.md").write_text(
                "- [x] 1.1 done\n- [ ] 1.2 open one\n- [ ] 9.9 The closing runs: close.py --check passes\n"
            )
            with mock.patch.object(common, "CHANGES", changes):
                self.assertEqual(common.open_tasks(), [("m8-x", "1.2 open one")])
                self.assertEqual(common.open_tasks(now=time.time() + 2 * common.ACTIVE_SECONDS), [])


if __name__ == "__main__":
    unittest.main()
