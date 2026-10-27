"""Tests of check_vsix.py. Run: python -m unittest discover -s vscode/scripts -p "test_*.py" """

import pathlib
import tempfile
import unittest
import zipfile

import check_vsix as cv

MANIFEST = '<PackageManifest><Metadata><Identity TargetPlatform="win32-x64" /></Metadata></PackageManifest>'


class CheckVsix(unittest.TestCase):
    def test_complete_package(self):
        self.assertEqual(cv.check_vsix(set(cv.REQUIRED), MANIFEST), [])

    def test_missing_binary(self):
        names = set(cv.REQUIRED) - {"extension/bundle/bin/qb64rust.exe"}
        self.assertEqual(
            cv.check_vsix(names, MANIFEST), ["missing: extension/bundle/bin/qb64rust.exe"]
        )

    def test_forbidden_files(self):
        names = set(cv.REQUIRED) | {
            "extension/src/build.ts",
            "extension/bundle/libqb/internal/c/libqb_make_000.o",
            "extension/bundle/libqb/internal/c/c_compiler/bin/gcc.exe",
        }
        problems = cv.check_vsix(names, MANIFEST)
        self.assertEqual(len(problems), 3)
        self.assertTrue(any("build.ts" in p for p in problems))
        self.assertTrue(any("toolchain" in p for p in problems))

    def test_universal_package(self):
        self.assertEqual(
            cv.check_vsix(set(cv.REQUIRED), "<PackageManifest />"),
            ["the manifest does not target win32-x64"],
        )

    def test_main_reads_a_zip(self):
        with tempfile.TemporaryDirectory() as d:
            path = pathlib.Path(d) / "x.vsix"
            with zipfile.ZipFile(path, "w") as z:
                z.writestr("extension.vsixmanifest", MANIFEST)
                for n in cv.REQUIRED:
                    if n not in {"extension/LICENSE.txt", "extension/bundle/libqb/COPYING.txt"}:
                        z.writestr(n, "")
                root = cv.PACKAGE_JSON.parents[1]
                z.write(root / "LICENSE", "extension/LICENSE.txt")
                z.write(root / "libqb/COPYING.txt", "extension/bundle/libqb/COPYING.txt")
                for notice in (root / "libqb" / "licenses").iterdir():
                    if notice.is_file():
                        z.write(notice, "extension/bundle/" + notice.relative_to(root).as_posix())
            self.assertEqual(cv.main([str(path)]), 0)


class CheckTag(unittest.TestCase):
    def test_matching_tag(self):
        self.assertEqual(cv.check_tag("vscode-v0.2.0", "0.2.0"), [])

    def test_mismatched_tag(self):
        self.assertEqual(
            cv.check_tag("vscode-v0.2.1", "0.2.0"),
            ["tag vscode-v0.2.1 names version 0.2.1, but package.json says 0.2.0"],
        )

    def test_malformed_tag(self):
        self.assertEqual(len(cv.check_tag("v0.2.0", "0.2.0")), 1)

    def test_cargo_version_equal(self):
        self.assertEqual(cv.check_tag("vscode-v0.2.0", "0.2.0", "0.2.0"), [])

    def test_cargo_version_differs(self):
        problems = cv.check_tag("vscode-v0.3.0", "0.3.0", "0.2.0")
        self.assertEqual(len(problems), 1)
        self.assertIn("0.2.0", problems[0])
        self.assertIn("0.3.0", problems[0])

    def test_cargo_version_read(self):
        text = '[workspace]\nmembers = []\n\n[workspace.package]\nversion = "1.2.3"\n'
        self.assertEqual(cv.cargo_version(text), "1.2.3")
        cv.cargo_version(cv.CARGO_TOML.read_text(encoding="utf-8"))  # the repository's own parses


class ReleaseNotes(unittest.TestCase):
    def test_notes_end_with_the_assets_paragraph(self):
        notes = cv.release_notes("New things.\n")
        self.assertTrue(notes.startswith("New things.\n\n"))
        self.assertTrue(notes.endswith(cv.ASSETS_PARAGRAPH))
        self.assertIn("MANUAL.md", cv.ASSETS_PARAGRAPH)


class ChangelogSection(unittest.TestCase):
    TEXT = "# Changelog\n\n## 0.2.0\n\nNew things.\n\n- a\n\n## 0.1.0\n\nOld things.\n"

    def test_section_of_the_version(self):
        self.assertEqual(cv.changelog_section(self.TEXT, "0.2.0"), "New things.\n\n- a\n")
        self.assertEqual(cv.changelog_section(self.TEXT, "0.1.0"), "Old things.\n")

    def test_no_section(self):
        self.assertIsNone(cv.changelog_section(self.TEXT, "0.3.0"))

    def test_the_real_changelog_has_the_package_version(self):
        version = cv.json.loads(cv.PACKAGE_JSON.read_text(encoding="utf-8"))["version"]
        self.assertIsNotNone(
            cv.changelog_section(cv.CHANGELOG.read_text(encoding="utf-8"), version)
        )


if __name__ == "__main__":
    unittest.main()
