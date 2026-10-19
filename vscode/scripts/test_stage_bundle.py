"""Tests of stage_bundle.py. Run: python -m unittest discover -s vscode/scripts -p "test_*.py" """

import pathlib
import subprocess
import tempfile
import unittest

import stage_bundle as sb


def git(repo, *args):
    subprocess.run(["git", *args], cwd=repo, check=True, capture_output=True)


class RepoCase(unittest.TestCase):
    """A temporary repository shaped like this one: libqb/ with qbx.cpp tracked, LICENSE, a binary."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.repo = pathlib.Path(self.tmp.name)
        c = self.repo / "libqb" / "internal" / "c"
        c.mkdir(parents=True)
        (c / "qbx.cpp").write_text("// qbx\n")
        (self.repo / "libqb" / "COPYING.txt").write_text("copying\n")
        (self.repo / "LICENSE").write_text("MIT\n")
        git(self.repo, "init", "-q")
        git(self.repo, "add", ".")
        # Left by a build in the working tree, never tracked.
        (c / "libqb_make_000.o").write_bytes(b"\0")
        self.binary = self.repo / "qb64rust.exe"
        self.binary.write_bytes(b"MZ")
        self.overrides = self.repo / "overrides"
        self.overrides.mkdir()
        self.out = self.repo / "vscode" / "bundle"
        self.licence_out = self.repo / "vscode" / "LICENSE"

    def tearDown(self):
        self.tmp.cleanup()

    def stage(self, crates=(), dirs=None):
        return sb.stage(
            self.repo,
            self.out,
            self.binary,
            self.licence_out,
            self.overrides,
            crates=list(crates),
            dirs=dirs or {},
        )


class Stage(RepoCase):
    def test_only_tracked_files_of_libqb(self):
        self.assertEqual(self.stage(), 2)
        self.assertTrue((self.out / "libqb" / "internal" / "c" / "qbx.cpp").is_file())
        self.assertTrue((self.out / "libqb" / "COPYING.txt").is_file())
        self.assertFalse((self.out / "libqb" / "internal" / "c" / "libqb_make_000.o").exists())
        self.assertTrue((self.out / "bin" / "qb64rust.exe").is_file())
        self.assertEqual(self.licence_out.read_text(), "MIT\n")
        self.assertTrue((self.out / "THIRD-PARTY-NOTICES.txt").is_file())

    def test_libqb_id_follows_the_tracked_files(self):
        self.stage()
        first = (self.out / sb.LIBQB_ID).read_text()
        self.assertRegex(first, r"^[0-9a-f]{16}\n$")
        (self.repo / "libqb" / "internal" / "c" / "other.o").write_bytes(b"\1")  # untracked: no change
        self.stage()
        self.assertEqual((self.out / sb.LIBQB_ID).read_text(), first)
        (self.repo / "libqb" / "internal" / "c" / "qbx.cpp").write_text("// qbx, edited\n")
        self.stage()
        self.assertNotEqual((self.out / sb.LIBQB_ID).read_text(), first)

    def test_stale_bundle_is_replaced(self):
        (self.out / "old").mkdir(parents=True)
        self.stage()
        self.assertFalse((self.out / "old").exists())

    def test_missing_binary_fails(self):
        self.binary.unlink()
        with self.assertRaisesRegex(sb.StageError, "binary is missing"):
            self.stage()
        self.assertFalse(self.out.exists())

    def test_missing_libqb_fails(self):
        (self.repo / "libqb" / "internal" / "c" / "qbx.cpp").unlink()
        with self.assertRaisesRegex(sb.StageError, "qbx.cpp"):
            self.stage()


class Notice(RepoCase):
    def crate_dir(self, name, *files):
        d = self.repo / "registry" / name
        d.mkdir(parents=True)
        for f in files:
            (d / f).write_text(f"text of {f}\n")
        return d

    def test_licence_files_of_each_crate(self):
        d = self.crate_dir("serde", "LICENSE-MIT", "LICENSE-APACHE", "README.md")
        self.stage([("serde", "1.0.0", "MIT OR Apache-2.0")], {("serde", "1.0.0"): d})
        text = (self.out / "THIRD-PARTY-NOTICES.txt").read_text()
        self.assertIn("serde 1.0.0 (MIT OR Apache-2.0)", text)
        self.assertIn("text of LICENSE-MIT", text)
        self.assertIn("text of LICENSE-APACHE", text)
        self.assertNotIn("README", text)

    def test_crate_without_licence_file_fails(self):
        d = self.crate_dir("astro-float-num", "README.md")
        with self.assertRaisesRegex(sb.StageError, "astro-float-num 0.3.7"):
            self.stage([("astro-float-num", "0.3.7", "MIT")], {("astro-float-num", "0.3.7"): d})
        self.assertFalse(self.out.exists())

    def test_override_stands_in(self):
        d = self.crate_dir("astro-float-num", "README.md")
        (self.overrides / "astro-float-num.txt").write_text("MIT, stencillogic\n")
        self.stage([("astro-float-num", "0.3.7", "MIT")], {("astro-float-num", "0.3.7"): d})
        self.assertIn("MIT, stencillogic", (self.out / "THIRD-PARTY-NOTICES.txt").read_text())


class ParseTree(unittest.TestCase):
    def test_path_dependencies_and_repeats(self):
        # As cargo tree prints it: `(*)` after the licence, `(proc-macro)` after the package, a path
        # dependency's folder (absolute there) in parentheses: a drive with either slash, or a POSIX path. The drive
        # is put in by replace(), so that the file holds no drive path (tools/repo_check).
        text = r"""qb64rust-driver v0.1.0 (<drive>/repo/crates/driver)|MIT
serde v1.0.229|MIT OR Apache-2.0
serde v1.0.229|MIT OR Apache-2.0 (*)
serde_derive v1.0.229 (proc-macro)|MIT OR Apache-2.0
qb64rust-ir v0.1.0 (/repo/crates/ir)|MIT (*)
qb64rust-sema v0.1.0 (<drive>\repo\crates\sema)|MIT
"""
        self.assertEqual(
            sb.parse_tree(text.replace("<drive>", "C:")),
            [
                ("serde", "1.0.229", "MIT OR Apache-2.0"),
                ("serde_derive", "1.0.229", "MIT OR Apache-2.0"),
            ],
        )

    def test_git_dependency_is_kept(self):
        # Not one of the workspace's crates: its licence must ship.
        text = "fancy v0.2.0 (https://github.com/someone/fancy#0123abcd)|MIT\n"
        self.assertEqual(sb.parse_tree(text), [("fancy", "0.2.0", "MIT")])


if __name__ == "__main__":
    unittest.main()
