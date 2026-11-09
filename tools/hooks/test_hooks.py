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


class Backslashes(unittest.TestCase):
    """Working rule 5: no text with backslashes through Bash."""

    def test_refused(self):
        for cmd in (
            # The two of 2026-11-08: a quoted heredoc with a Python escape in it.
            "cat > x/edit.py <<'EOF'\np.write_text(t, newline=\"\\\\n\")\nEOF\npython -I x/edit.py",
            "python -I - <<'EOF'\nt = t.replace('f\"\\\\\"{FILLER}\\\\\"\"', 'x')\nEOF",
            "cat <<EOF > a.txt\nstudy\\02\nEOF",
            "python -c \"print('a\\nb')\"",
            "python -I -c \"b=open(f,'rb').read(); print(b.count(b'\\r'))\"",
            "sed -i 's/a\\/b/c/' f.md",
            "sed 's/\\(a\\)/\\1/' f.md > g.md",
            "awk -F'\\t' '{print $1}' a.tsv > b.tsv",
            "printf 'x\\n' > f",
            "echo -e 'a\\tb'",
            "perl -e 'print \"\\n\"'",
            # Flags written together, and a `|` inside the quoted script.
            "python -Ic \"print('a\\nb')\"",
            "py -3.12 -c \"print('a|b\\n')\"",
            "cd x && python -c \"print('\\n')\"",
        ):
            self.assertIsNotNone(guard_tool.check_backslashes(cmd), cmd)

    def test_allowed(self):
        for cmd in (
            "cat > a.py <<'EOF'\nprint('no backslash here')\nEOF",
            'grep -n "a\\|b" docs/x.md',
            "python -I tools/play/sessions.py tests/scripts/*.keys",
            "python -I -c \"print(1 + 1)\"",
            "sed -n 1,40p docs/STATUS.md",
            # Refused on 2026-11-09 before sed and awk counted only when they write: a read-only pipeline.
            'grep -rn "a\\|b" crates | grep -v "fn \\|///" ; echo ---; sed -n 1315,1340p f.rs',
            "sed -n 's/\\(x\\)/\\1/p' f.md 2>/dev/null",
            "awk -F'\\t' '{print $1}' a.tsv | head",
            # A script's own arguments are not the interpreter's -c or -e.
            "python -I tools/x.py study\\02 -e",
            'cd "/c/code/x" && ls',
            "git log --oneline -5",
        ):
            self.assertIsNone(guard_tool.check_backslashes(cmd), cmd)

    def test_powershell_not_checked(self):
        # PowerShell takes a backslash as a plain character; only Bash collapses them.
        with mock.patch.object(guard_tool, "read_input", return_value={
                "tool_name": "PowerShell", "tool_input": {"command": "Get-Content study\\02.md"}}):
            self.assertEqual(guard_tool.main(), 0)


if __name__ == "__main__":
    unittest.main()
