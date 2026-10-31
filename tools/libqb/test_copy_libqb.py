"""Tests of copy_libqb.py (design D1 of libqb-copy) on a small git repository made by each test.

Run: python -m unittest discover -s tools/libqb -p "test_*.py"
"""

import pathlib
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import copy_libqb


def git(repo, *args):
    subprocess.run(["git", "-C", str(repo), *args], check=True, capture_output=True)


class CopyTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        base = pathlib.Path(self.tmp.name)
        self.clone = base / "clone"
        self.dest = base / "dest"
        files = {
            "Makefile": b"all:\n",
            "internal/c/libqb.cpp": b"int x;\r\n",  # CR LF kept as committed
            "internal/c/parts/a/b.h": b"#pragma once\n",
            "internal/c/.gitignore": b"c_compiler\n",
            "licenses/license_qb64.txt": b"MIT\n",
            "COPYING.txt": b"MIT\n",
            "source/qb64pe.bas": b"PRINT\n",  # not copied
        }
        self.clone.mkdir()
        git(self.clone, "init", "-q")
        git(self.clone, "config", "core.autocrlf", "false")
        for rel, data in files.items():
            p = self.clone / rel
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_bytes(data)
        git(self.clone, "add", "-A")
        git(self.clone, "-c", "user.name=t", "-c", "user.email=t@t", "commit", "-q", "-m", "c")
        # A working-tree edit the copy must not see: it is taken from the commit.
        (self.clone / "Makefile").write_bytes(b"changed\n")

    def tearDown(self):
        self.tmp.cleanup()

    def run_tool(self, *args, edits=None):
        edits = edits or pathlib.Path(self.tmp.name) / "no-edits"
        return copy_libqb.main(
            ["--clone", str(self.clone), "--dest", str(self.dest), "--edits", str(edits), *args]
        )

    def edits_dir(self, **files):
        d = pathlib.Path(self.tmp.name) / "edits"
        d.mkdir(exist_ok=True)
        for name, text in files.items():
            (d / f"{name}.edit").write_bytes(text)
        return d

    def problems(self):
        commit = copy_libqb.pinned_commit(self.dest)
        return copy_libqb.check(self.dest, copy_libqb.commit_files(self.clone, commit))

    def test_a_fresh_copy_checks_clean(self):
        self.assertEqual(self.run_tool(), 0)
        self.assertEqual((self.dest / "Makefile").read_bytes(), b"all:\n")
        self.assertEqual((self.dest / "internal/c/libqb.cpp").read_bytes(), b"int x;\r\n")
        self.assertFalse((self.dest / "source").exists())
        self.assertIn("Commit: `", (self.dest / "SOURCE.md").read_text(encoding="utf-8"))
        self.assertEqual(self.run_tool("--check"), 0)

    def test_an_edited_removed_and_added_file_are_named(self):
        self.run_tool()
        (self.dest / "internal/c/libqb.cpp").write_bytes(b"int y;\r\n")
        (self.dest / "licenses/license_qb64.txt").unlink()
        (self.dest / "internal/c/parts/a/new.h").write_bytes(b"\n")
        self.assertEqual(
            self.problems(),
            [
                "differs: internal/c/libqb.cpp",
                "missing: licenses/license_qb64.txt",
                "extra: internal/c/parts/a/new.h",
            ],
        )
        self.assertEqual(self.run_tool("--check"), 1)

    def test_built_files_are_not_extra(self):
        self.run_tool()
        for name in [
            "internal/c/libqb_make_000.o",
            "internal/c/parts/a/x.a",
            "internal/c/parts/a/y.tmp",
        ]:
            (self.dest / name).write_bytes(b"\0")
        self.assertEqual(self.problems(), [])

    def test_writing_again_replaces_the_copy(self):
        self.run_tool()
        (self.dest / "internal/c/parts/a/stale.h").write_bytes(b"\n")
        (self.dest / "internal/c/libqb_make_000.o").write_bytes(b"\0")
        self.assertEqual(self.run_tool(), 0)
        self.assertFalse((self.dest / "internal/c/parts/a/stale.h").exists())
        self.assertEqual(self.problems(), [])

    def test_an_edit_is_applied_and_checked(self):
        # The edit file has LF line ends; the target's CR LF are kept.
        edits = self.edits_dir(
            a=b"# why\nfile: internal/c/libqb.cpp\n--- old\nint x;\n--- new\nint y;\nint z;\n--- end\n"
        )
        self.assertEqual(self.run_tool(edits=edits), 0)
        self.assertEqual(
            (self.dest / "internal/c/libqb.cpp").read_bytes(), b"int y;\r\nint z;\r\n"
        )
        self.assertIn("tools/libqb/edits/a.edit", (self.dest / "SOURCE.md").read_text("utf-8"))
        self.assertEqual(self.run_tool("--check", edits=edits), 0)
        # Without the edit the file differs from the commit's.
        self.assertEqual(self.run_tool("--check"), 1)
        self.assertEqual(self.problems(), ["differs: internal/c/libqb.cpp"])

    def test_an_edit_may_name_the_builtins_it_changes(self):
        # `builtins:` is read by tools/close/close.py; the tool takes it and applies the edit as without it.
        edits = self.edits_dir(
            a=b"# why\nfile: internal/c/libqb.cpp\nbuiltins: _SCREENCLICK\n--- old\nint x;\n--- new\nint y;\n--- end\n"
        )
        self.assertEqual(self.run_tool(edits=edits), 0)
        self.assertEqual((self.dest / "internal/c/libqb.cpp").read_bytes(), b"int y;\r\n")
        self.assertEqual(self.run_tool("--check", edits=edits), 0)

    def test_an_edit_that_does_not_fit_stops_the_tool(self):
        for text in [
            b"file: internal/c/libqb.cpp\n--- old\nint w;\n--- new\n--- end\n",  # not in the file
            b"file: internal/c/none.cpp\n--- old\nint x;\n--- new\n--- end\n",  # no such file
            b"file: internal/c/libqb.cpp\n--- old\nint x;\n",  # no end
        ]:
            with self.subTest(text=text):
                edits = self.edits_dir(a=text)
                self.assertEqual(self.run_tool(edits=edits), 1)
                self.assertFalse((self.dest / "Makefile").exists())


if __name__ == "__main__":
    unittest.main()
