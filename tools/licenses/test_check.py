"""Failure-path checks for the license inventory (standard library only)."""

import io
import pathlib
import tempfile
import unittest
import zipfile

import check as licenses


class Inventory(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = pathlib.Path(self.temp.name)
        self.files = {"LICENSE", "vendor/example/SOURCE.md", "vendor/example/LICENSE"}
        for path in self.files:
            target = self.root / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text("original notice\n", encoding="utf-8")
        self.components = [{
            "name": "example", "paths": ["vendor/example"],
            "source": "example upstream", "license": "MIT",
            "provenance": ["vendor/example/SOURCE.md"],
            "notices": ["vendor/example/LICENSE"], "package_prefix": "extension/bundle/",
        }]

    def test_registered_content_passes(self):
        self.assertEqual(licenses.check(self.root, self.files, self.components), [])

    def test_missing_or_empty_notice_fails(self):
        notice = self.root / "vendor/example/LICENSE"
        notice.write_text("")
        self.assertTrue(any("empty" in p for p in licenses.check(self.root, self.files, self.components)))
        notice.unlink()
        self.assertTrue(any("missing" in p for p in licenses.check(self.root, self.files, self.components)))

    def test_new_provenance_and_vendor_directory_fail(self):
        files = self.files | {"tests/new/SOURCE.md", "third_party/new/code.c"}
        problems = licenses.check(self.root, files, self.components)
        self.assertTrue(any("unregistered provenance" in p for p in problems))
        self.assertTrue(any("unregistered vendor" in p for p in problems))

    def test_package_missing_or_changed_notices_fail(self):
        buffer = io.BytesIO()
        with zipfile.ZipFile(buffer, "w") as package:
            package.writestr("extension/LICENSE.txt", "changed")
            problems = licenses.check(self.root, self.files, self.components, package)
        self.assertTrue(any("package missing" in p for p in problems))
        self.assertTrue(any("notice differs" in p for p in problems))

    def test_package_preserves_text_across_checkout_line_endings(self):
        buffer = io.BytesIO()
        with zipfile.ZipFile(buffer, "w") as package:
            package.writestr("extension/LICENSE.txt", "original notice\r\n")
            package.writestr("extension/bundle/vendor/example/LICENSE", "original notice\r\n")
            self.assertEqual(licenses.check(self.root, self.files, self.components, package), [])


if __name__ == "__main__":
    unittest.main()
