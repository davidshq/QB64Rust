"""Tests of compiler_copy.py: the copy of the new compiler a run uses (study/archive/34 section 3, item 2).

    python -m unittest discover -s tools/legacy_tests -p "test_*.py"

No compiler: a file of a few bytes stands for it.
"""

from __future__ import annotations

import os
import tempfile
import time
import unittest
from pathlib import Path

import compiler_copy


class Copy(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()  # pylint: disable=consider-using-with
        self.addCleanup(self.dir.cleanup)
        self.root = Path(self.dir.name)
        self.exe = self.root / "release" / "qb64rust.exe"
        self.exe.parent.mkdir()
        self.exe.write_bytes(b"first build")
        self.bin = self.root / "test-bin"

    def test_a_build_after_the_copy_changes_nothing_the_run_reads(self):
        copy, digest = compiler_copy.run_copy(self.exe, self.bin)
        self.assertEqual(copy.parent.name, digest[:16])
        self.assertEqual(copy.name, "qb64rust.exe")
        self.exe.write_bytes(b"second build")
        self.assertEqual(copy.read_bytes(), b"first build")
        second, other = compiler_copy.run_copy(self.exe, self.bin)
        self.assertNotEqual(digest, other)
        self.assertNotEqual(copy, second)
        self.assertEqual(copy.read_bytes(), b"first build")

    def test_the_same_build_reuses_its_copy(self):
        copy, digest = compiler_copy.run_copy(self.exe, self.bin)
        again, same = compiler_copy.run_copy(self.exe, self.bin)
        self.assertEqual((copy, digest), (again, same))
        self.assertEqual([p.name for p in copy.parent.iterdir()], ["qb64rust.exe"])

    def test_a_copy_left_damaged_is_made_again(self):
        copy, _ = compiler_copy.run_copy(self.exe, self.bin)
        copy.write_bytes(b"half")
        again, _ = compiler_copy.run_copy(self.exe, self.bin)
        self.assertEqual(again.read_bytes(), b"first build")

    def test_old_copies_are_removed_and_the_current_kept(self):
        copy, digest = compiler_copy.run_copy(self.exe, self.bin)
        old = self.bin / "0123456789abcdef"
        old.mkdir()
        (old / "qb64rust.exe").write_bytes(b"old")
        recent = self.bin / "fedcba9876543210"
        recent.mkdir()
        day_ago = time.time() - compiler_copy.KEEP_SECONDS - 60
        os.utime(old, (day_ago, day_ago))
        os.utime(copy.parent, (day_ago, day_ago))
        compiler_copy.prune(self.bin, digest[:16])
        self.assertFalse(old.exists())
        self.assertTrue(recent.exists())
        self.assertTrue(copy.exists())

    def test_an_old_copy_taken_up_is_marked_before_it_is_read(self):
        # A run beside this one prunes by the folder's time: run_copy sets it before it reads the copy.
        copy, _ = compiler_copy.run_copy(self.exe, self.bin)
        day_ago = time.time() - compiler_copy.KEEP_SECONDS - 60
        os.utime(copy.parent, (day_ago, day_ago))
        seen = []
        real = compiler_copy.sha256_of

        def sha256_of(path):
            if path == copy:
                seen.append(copy.parent.stat().st_mtime)
            return real(path)

        compiler_copy.sha256_of = sha256_of
        self.addCleanup(setattr, compiler_copy, "sha256_of", real)
        again, _ = compiler_copy.run_copy(self.exe, self.bin)
        self.assertEqual(again, copy)
        self.assertGreater(seen[0], day_ago + 60)
        compiler_copy.prune(self.bin, "other")
        self.assertTrue(copy.exists())

    def test_a_copy_removed_while_it_is_made_is_made_again(self):
        real = compiler_copy.shutil.copy2
        calls = []

        def copy2(src, dst):
            calls.append(dst)
            if len(calls) == 1:
                # The prune of a run beside this one removes the folder before the rename.
                compiler_copy.shutil.rmtree(Path(dst).parent)
                raise FileNotFoundError(dst)
            return real(src, dst)

        compiler_copy.shutil.copy2 = copy2
        self.addCleanup(setattr, compiler_copy.shutil, "copy2", real)
        copy, _ = compiler_copy.run_copy(self.exe, self.bin)
        self.assertEqual(len(calls), 2)
        self.assertEqual(copy.read_bytes(), b"first build")

    def test_header_round_trip(self):
        line = compiler_copy.header("target/release/qb64rust.exe", "ab" * 32, 1760000000.25)
        self.assertEqual(
            line,
            f"[compiler] target/release/qb64rust.exe sha256 {'ab' * 32} started 1760000000.250 fixes off",
        )
        text = f"waiting for another run to leave the reference clone\n{line}\n[1/2] PASS a\n"
        self.assertEqual(compiler_copy.read_header(text), ("ab" * 32, 1760000000.25))
        self.assertIsNone(compiler_copy.read_header("[1/2] PASS a\n"))
        self.assertIsNone(compiler_copy.read_header("[compiler] x sha256\n"))


if __name__ == "__main__":
    unittest.main()
