"""Tests of copy_upstream_tests.py: the files of the compiler root that upstream programs name (design D3 of
m4-devices-continue-paths).

Run: python -m unittest discover -s tools/upstream -p "test_*.py"
"""

import pathlib
import sys
import tempfile
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import copy_upstream_tests as cut  # noqa: E402


class RootFiles(unittest.TestCase):
    def test_each_root_file_is_named_by_an_upstream_program_of_the_copy(self):
        self.assertEqual(cut.unnamed_root_files(cut.DEST / "compile_tests"), [])

    def test_a_file_no_program_names_is_reported(self):
        with tempfile.TemporaryDirectory() as tmp:
            copy = pathlib.Path(tmp)
            (copy / "icon").mkdir()
            (copy / "icon" / "test.bas").write_bytes(b"$EXEICON:'tests/compile_tests/icon/qb64pe.ico'\n")
            files = ["tests/compile_tests/icon/qb64pe.ico", "tests/compile_tests/other/x.ico"]
            self.assertEqual(cut.unnamed_root_files(copy, files), ["tests/compile_tests/other/x.ico"])

    def test_a_missing_or_changed_copy_differs(self):
        with tempfile.TemporaryDirectory() as tmp:
            clone, root = pathlib.Path(tmp) / "clone", pathlib.Path(tmp) / "root"
            for d, data in ((clone, b"icon"), (root, b"icon")):
                (d / "a").mkdir(parents=True)
                (d / "a" / "same.ico").write_bytes(data)
            (clone / "a" / "changed.ico").write_bytes(b"new")
            (root / "a" / "changed.ico").write_bytes(b"old")
            (clone / "a" / "missing.ico").write_bytes(b"x")
            files = ["a/same.ico", "a/changed.ico", "a/missing.ico"]
            self.assertEqual(cut.differing_root_files(clone, root, files), ["a/changed.ico", "a/missing.ico"])


if __name__ == "__main__":
    unittest.main()
