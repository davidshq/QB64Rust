"""Tests of find_chrome.mjs: where the browser launcher looks for Chrome, from made-up environments, so that both
lists are checked on either host and without a browser. Skipped where there is no Node on PATH."""
import json
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
NODE = shutil.which("node")


def call(function: str, platform: str, env: dict):
    """What find_chrome.mjs's `function(platform, env)` returns, through Node."""
    script = (
        f"import {{ {function} }} from " + json.dumps((HERE / "find_chrome.mjs").as_uri()) + ";"
        "const [platform, env] = JSON.parse(process.argv[1]);"
        f"console.log(JSON.stringify({function}(platform, env)));"
    )
    out = subprocess.run(
        [NODE, "--input-type=module", "-e", script, json.dumps([platform, env])],
        capture_output=True, text=True,
    )
    if out.returncode != 0:
        raise AssertionError(f"node exited {out.returncode}:\n{out.stderr}")
    return json.loads(out.stdout)


def candidates(platform: str, env: dict) -> list[str]:
    return call("chromeCandidates", platform, env)


@unittest.skipIf(NODE is None, "no node on PATH")
class ChromeCandidates(unittest.TestCase):
    def test_windows_from_the_environment(self):
        env = {"PROGRAMFILES": "X:\\pf", "PROGRAMFILES(X86)": "X:\\pf86", "LOCALAPPDATA": "X:\\u\\local"}
        self.assertEqual(candidates("win32", env), [
            "X:\\pf\\Google\\Chrome\\Application\\chrome.exe",
            "X:\\pf86\\Google\\Chrome\\Application\\chrome.exe",
            "X:\\u\\local\\Google\\Chrome\\Application\\chrome.exe",
            "X:\\pf86\\Microsoft\\Edge\\Application\\msedge.exe",
            "X:\\pf\\Microsoft\\Edge\\Application\\msedge.exe",
        ])

    def test_windows_without_variables(self):
        # A variable that is not set gives no candidate, not a path made from "undefined".
        self.assertEqual(candidates("win32", {"PROGRAMFILES": "X:\\pf"}), [
            "X:\\pf\\Google\\Chrome\\Application\\chrome.exe",
            "X:\\pf\\Microsoft\\Edge\\Application\\msedge.exe",
        ])
        self.assertEqual(candidates("win32", {}), [])

    def test_windows_names_in_any_letter_case(self):
        # Windows keeps a variable's name as it was set, e.g. ProgramFiles(x86); findInstalledChrome still finds the
        # Chrome under it.
        with tempfile.TemporaryDirectory() as tmp:
            exe = Path(tmp, "Google", "Chrome", "Application", "chrome.exe")
            exe.parent.mkdir(parents=True)
            exe.write_bytes(b"")
            self.assertEqual(call("findInstalledChrome", "win32", {"ProgramFiles(x86)": tmp}), str(exe))
            self.assertIsNone(call("findInstalledChrome", "win32", {}))

    def test_linux_on_path(self):
        self.assertEqual(candidates("linux", {"PATH": "/a/bin:/snap/bin"}), [
            "/a/bin/google-chrome-stable", "/snap/bin/google-chrome-stable",
            "/a/bin/google-chrome", "/snap/bin/google-chrome",
            "/a/bin/chromium-browser", "/snap/bin/chromium-browser",
            "/a/bin/chromium", "/snap/bin/chromium",
        ])
        self.assertEqual(candidates("linux", {}), [])


if __name__ == "__main__":
    unittest.main()
