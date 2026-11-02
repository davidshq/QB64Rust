"""Tests of package_download.py. Run: python -m unittest discover -s vscode/scripts -p "test_*.py" """

import pathlib
import re
import tarfile
import tempfile
import unittest
import zipfile

import package_download as pz


class FakeBundle(unittest.TestCase):
    """A staged bundle and a repository in a temporary folder, as `npm run package` leaves them."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        root = pathlib.Path(self.tmp.name)
        self.repo = root / "repo"
        self.bundle = root / "bundle"
        self.out = root / "out"
        for path, text in {
            self.bundle / "bin" / "qb64rust.exe": "MZ",
            self.bundle / "libqb" / "Makefile": "all:\n",
            self.bundle / "libqb" / "internal" / "c" / "qbx.cpp": "// qbx\n",
            self.bundle / "libqb" / "COPYING.txt": "copying\n",
            self.bundle / "libqb.id": "0123\n",
            self.bundle / "THIRD-PARTY-NOTICES.txt": "notices\n",
            self.repo / "LICENSE": "MIT\n",
            self.repo / "docs" / "MANUAL.md": "# manual\n",
        }.items():
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, newline="\n")

    def write(self):
        return pz.write(self.out, "0.2.0", self.bundle, self.repo)

    def test_layout(self):
        path, count = self.write()
        self.assertEqual(path.name, "qb64rust-win-x64-0.2.0.zip")
        with zipfile.ZipFile(path) as z:
            names = z.namelist()
            self.assertEqual(z.read("qb64rust-0.2.0/LICENSE.txt"), b"MIT\n")
        self.assertEqual(count, len(names))
        self.assertEqual(names, sorted(names))
        self.assertEqual(
            set(names),
            {
                "qb64rust-0.2.0/qb64rust.exe",
                "qb64rust-0.2.0/libqb/Makefile",
                "qb64rust-0.2.0/libqb/internal/c/qbx.cpp",
                "qb64rust-0.2.0/libqb/COPYING.txt",
                "qb64rust-0.2.0/THIRD-PARTY-NOTICES.txt",
                "qb64rust-0.2.0/LICENSE.txt",
                "qb64rust-0.2.0/MANUAL.md",
            },
        )
        # libqb.id is the extension's alone.
        self.assertEqual(pz.check_names(path.name, set(names)), [])

    def linux_bundle(self):
        (self.bundle / "bin" / "qb64rust.exe").rename(self.bundle / "bin" / "qb64rust")

    def test_linux_layout(self):
        # Spec compiler/download, "Listing the Linux download": the binary executable, nothing for Windows.
        self.linux_bundle()
        path, count = pz.write(self.out, "0.4.0", self.bundle, self.repo, platform="linux-x64")
        self.assertEqual(path.name, "qb64rust-linux-x64-0.4.0.tar.gz")
        with tarfile.open(path) as t:
            members = {m.name: m for m in t.getmembers()}
        self.assertEqual(count, len(members))
        binary = members["qb64rust-0.4.0/qb64rust"]
        self.assertEqual(binary.mode, 0o755)
        self.assertEqual(members["qb64rust-0.4.0/MANUAL.md"].mode, 0o644)
        self.assertTrue(all((m.uid, m.gid, m.uname, m.gname) == (0, 0, "", "") for m in members.values()))
        modes = {n: m.mode for n, m in members.items()}
        self.assertEqual(pz.check_names(path.name, set(members), modes), [])

    def test_two_linux_packages_of_one_bundle_are_equal(self):
        self.linux_bundle()
        first, _ = pz.write(self.out / "a", "0.4.0", self.bundle, self.repo, platform="linux-x64")
        second, _ = pz.write(self.out / "b", "0.4.0", self.bundle, self.repo, platform="linux-x64")
        self.assertEqual(first.read_bytes(), second.read_bytes())

    def test_linux_check(self):
        top = "qb64rust-0.4.0"
        names = {f"{top}/qb64rust", *(f"{top}/{r}" for r in pz.COMMON)}
        name = "qb64rust-linux-x64-0.4.0.tar.gz"
        self.assertEqual(pz.check_names(name, names, {f"{top}/qb64rust": 0o755}), [])
        self.assertEqual(
            pz.check_names(name, names, {f"{top}/qb64rust": 0o644}),
            [f"not executable: {top}/qb64rust (mode 644)"],
        )
        self.assertEqual(
            pz.check_names(name, names | {f"{top}/qb64rust.exe"}),
            [f"must not be packaged: {top}/qb64rust.exe"],
        )
        self.assertEqual(pz.check_names(name, names - {f"{top}/qb64rust"}), [f"missing: {top}/qb64rust"])

    def test_not_staged(self):
        (self.bundle / "bin" / "qb64rust.exe").unlink()
        with self.assertRaisesRegex(pz.PackageError, "npm run package"):
            self.write()

    def test_required_file_missing(self):
        path, _ = self.write()
        with zipfile.ZipFile(path) as z:
            names = set(z.namelist()) - {"qb64rust-0.2.0/libqb/COPYING.txt"}
        self.assertEqual(
            pz.check_names(path.name, names), ["missing: qb64rust-0.2.0/libqb/COPYING.txt"]
        )

    def test_missing_notice_fails_the_licence_check(self):
        # The real check against this repository: the zip lacks libqb's licences/ notices.
        path, _ = self.write()
        problems, _ = pz.check(path)
        self.assertIn("repository/package license inventory check failed (see above)", problems)

    def test_forbidden_files(self):
        names = {f"qb64rust-0.2.0/{r}" for r in pz.REQUIRED} | {
            "qb64rust-0.2.0/libqb/internal/c/libqb_make_000.o",
            "qb64rust-0.2.0/libqb/internal/c/c_compiler/bin/gcc.exe",
            "qb64rust-0.2.0/bundle/x.txt",
        }
        problems = pz.check_names("qb64rust-win-x64-0.2.0.zip", names)
        self.assertEqual(len(problems), 3, problems)
        self.assertTrue(all(p.startswith("must not be packaged: ") for p in problems))

    def test_top_folder_must_match_the_name(self):
        names = {f"qb64rust-0.1.0/{r}" for r in pz.REQUIRED}
        problems = pz.check_names("qb64rust-win-x64-0.2.0.zip", names)
        self.assertTrue(any(p.startswith("not under qb64rust-0.2.0/") for p in problems), problems)
        self.assertEqual(len(pz.check_names("compiler.zip", names)), 1)


class LinuxPackagesInTheManual(unittest.TestCase):
    """Spec compiler/download, "The Linux packages in the manual": MANUAL.md's lines are toolchain.rs's table."""

    def setUp(self):
        self.manual = (pz.REPO / "docs" / "MANUAL.md").read_text(encoding="utf-8")
        self.rust = (pz.REPO / "crates" / "driver" / "src" / "toolchain.rs").read_text(encoding="utf-8")

    def test_the_manual_names_the_table(self):
        lines = pz.toolchain_packages(self.rust)
        self.assertEqual(len(lines), 3)
        self.assertTrue(lines[0].startswith("sudo apt-get install build-essential "), lines[0])
        self.assertEqual(pz.manual_problems(self.manual, self.rust), [])

    def test_the_extensions_readme_names_the_debian_line(self):
        readme = (pz.REPO / "vscode" / "README.md").read_text(encoding="utf-8")
        apt = pz.toolchain_packages(self.rust)[0]
        self.assertIn(apt, {line.strip() for line in readme.splitlines()})

    def test_a_changed_line_fails(self):
        apt = pz.toolchain_packages(self.rust)[0]
        changed = self.manual.replace(apt, apt.replace(" libasound2-dev", ""))
        self.assertEqual(pz.manual_problems(changed, self.rust), [f"MANUAL.md lacks the install line of toolchain.rs: {apt}"])


def usage_text():
    """The usage text of `qb64rust --help`, read from the driver's main.rs."""
    source = (pz.REPO / "crates" / "driver" / "src" / "main.rs").read_text(encoding="utf-8")
    m = re.search(r'const USAGE: &str = "(.*?)";', source, re.DOTALL)
    return m.group(1)


def manual_names(manual):
    """The options (`--x`, `-x`, `-f:`) and `qb64rust` subcommands the manual names in code spans."""
    names = set()
    for span in re.findall(r"`([^`\n]+)`", manual):
        for word in span.split():
            m = re.match(r"^(-f:|--?[A-Za-z][A-Za-z-]*)", word)
            if m:
                names.add(m.group(1))
        words = span.split()
        if len(words) >= 3 and words[0] == "qb64rust" and words[1].isalpha():
            names.add(f"qb64rust {words[1]} {words[2]}")
    return names


class Manual(unittest.TestCase):
    """Every option and subcommand the manual names is one `qb64rust --help` names (spec compiler/download, "An
    option in the manual")."""

    def test_manual_names_only_usage_options(self):
        usage = usage_text()
        names = manual_names((pz.REPO / "docs" / "MANUAL.md").read_text(encoding="utf-8"))
        self.assertIn("--include-root", names)
        self.assertIn("qb64rust toolchain path", names)
        for name in sorted(names):
            if name.startswith("qb64rust "):
                self.assertIn(name, usage)
            else:
                self.assertRegex(usage, rf"(?<![\w-]){re.escape(name)}(?![\w-])", name)

    def test_an_unknown_option_is_found(self):
        self.assertIn("--frobnicate", manual_names("use `qb64rust --frobnicate <x>` here"))


if __name__ == "__main__":
    unittest.main()
