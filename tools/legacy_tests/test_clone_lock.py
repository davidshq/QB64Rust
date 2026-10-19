"""Tests of clone_lock.py: the readers-writer lock of the reference clone (test-parallel-runs).

    python -m unittest discover -s tools/legacy_tests -p "test_*.py"

Each lock is taken through a file opened of its own, which the system treats as another holder
(LockFileEx on Windows is per handle, flock elsewhere per open file), so one process can play two
runs. No clone, no compiler.
"""

from __future__ import annotations

import os
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import clone_lock


class Lock(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()  # pylint: disable=consider-using-with
        self.addCleanup(self.dir.cleanup)
        self.path = Path(self.dir.name) / "in-use.lock"
        self.held = []
        self.addCleanup(lambda: [fh.close() for fh in self.held])

    def take(self, shared: bool):
        fh = clone_lock.lock_file(self.path, shared=shared)
        if fh is not None:
            self.held.append(fh)
        return fh

    def test_two_shared_holders(self):
        self.assertIsNotNone(self.take(shared=True))
        self.assertIsNotNone(self.take(shared=True))

    def test_exclusive_waits_for_shared(self):
        self.assertIsNotNone(self.take(shared=True))
        self.assertIsNone(self.take(shared=False))

    def test_shared_waits_for_exclusive(self):
        self.assertIsNotNone(self.take(shared=False))
        self.assertIsNone(self.take(shared=True))
        self.assertIsNone(self.take(shared=False))

    def test_closing_gives_the_lock_back(self):
        first = self.take(shared=True)
        second = self.take(shared=True)
        self.assertIsNone(self.take(shared=False))
        first.close()
        second.close()
        self.assertIsNotNone(self.take(shared=False))

    def test_folder_lock_is_exclusive(self):
        folder = Path(self.dir.name) / "results"
        fh = clone_lock.lock_folder(folder)
        self.held.append(fh)
        self.assertIsNotNone(fh)
        self.assertIsNone(clone_lock.lock_folder(folder))
        self.assertIsNone(clone_lock.lock_file(folder / "in-use.lock", shared=True))


class CloneLockKind(unittest.TestCase):
    """Which runs take the clone's lock, and how (design D5 of libqb-copy)."""

    def test_qb64pe_alone(self):
        self.assertEqual(clone_lock.clone_lock_kind(True, 1, False), "exclusive")
        self.assertEqual(clone_lock.clone_lock_kind(True, 1, True), "exclusive")

    def test_qb64pe_with_jobs_takes_it_later(self):
        self.assertIsNone(clone_lock.clone_lock_kind(True, 8, True))

    def test_qb64rust_only_for_programs_in_the_clone(self):
        self.assertIsNone(clone_lock.clone_lock_kind(False, 1, False))
        self.assertIsNone(clone_lock.clone_lock_kind(False, 8, False))
        self.assertEqual(clone_lock.clone_lock_kind(False, 8, True), "shared")


class CloneLock(unittest.TestCase):
    """name_objects_lock names the objects' lock for a run's compiles; lock_clone names nothing."""

    def setUp(self):
        d = tempfile.TemporaryDirectory()  # pylint: disable=consider-using-with
        self.addCleanup(d.cleanup)
        patch = mock.patch.object(clone_lock, "CLONE_LOCK_DIR", Path(d.name))
        patch.start()
        self.addCleanup(patch.stop)
        env = mock.patch.dict(os.environ)
        env.start()
        self.addCleanup(env.stop)
        os.environ.pop(clone_lock.OBJECTS_LOCK_ENV, None)

    def test_name_objects_lock(self):
        clone_lock.name_objects_lock()
        self.assertEqual(os.environ[clone_lock.OBJECTS_LOCK_ENV], str(clone_lock.OBJECTS_LOCK))

    def test_lock_clone_names_nothing(self):
        for shared in (True, False):
            fh = clone_lock.lock_clone(shared=shared)
            fh.close()
        self.assertNotIn(clone_lock.OBJECTS_LOCK_ENV, os.environ)

    def test_a_qb64pe_run_waits_for_a_qb64rust_run(self):
        fh = clone_lock.lock_clone(shared=True)
        self.addCleanup(fh.close)
        self.assertIsNone(clone_lock.lock_clone())


class MainCheckout(unittest.TestCase):
    """A worktree takes the main checkout's locks (study/archive/34 section 3, item 3): the layout git writes."""

    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()  # pylint: disable=consider-using-with
        self.addCleanup(self.dir.cleanup)
        self.root = Path(self.dir.name).resolve()

    def test_the_main_checkout_is_itself(self):
        main = self.root / "QB64Rust"
        (main / ".git").mkdir(parents=True)
        self.assertEqual(clone_lock.main_checkout(main), main)

    def test_a_worktree_finds_the_main_checkout(self):
        main = self.root / "QB64Rust"
        gitdir = main / ".git" / "worktrees" / "merge"
        gitdir.mkdir(parents=True)
        (gitdir / "commondir").write_text("../..\n")
        tree = self.root / "QB64Rust-merge"
        tree.mkdir()
        (tree / ".git").write_text(f"gitdir: {gitdir.as_posix()}\n")
        self.assertEqual(clone_lock.main_checkout(tree), main)

    def test_no_git_at_all(self):
        self.assertEqual(clone_lock.main_checkout(self.root), self.root)


if __name__ == "__main__":
    unittest.main()
