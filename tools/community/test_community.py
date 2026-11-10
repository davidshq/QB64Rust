"""Tests of community.py: python -m unittest discover -s tools/community

A fake manifest, local git repositories as the remotes and a fake qb64rust script. No network, no real clone.
"""

from __future__ import annotations

import contextlib
import io
import json
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path
from types import SimpleNamespace

sys.path.insert(0, str(Path(__file__).resolve().parent))

import community as c  # noqa: E402

FAKE_QB = r'''
import sys
import time

args = sys.argv[1:]
src = args[args.index("-z") + 1]
text = open(src, encoding="utf-8", errors="replace").read()
if "HANG" in text:
    time.sleep(60)
if "LONG" in text:
    sys.stderr.write("f.bas:1:1: error: " + "x" * 500 + "\n")
    sys.exit(1)
if "OUTSIDE" in text:
    sys.stderr.write("note: first\nf.bas:1:1: error: cannot read C:\\secret\\dir\\x.bi, " + src + " and /home/me/y.bi\n")
    sys.exit(1)
if "FAIL" in text:
    sys.stderr.write("f.bas:2:3: error: boom\nf.bas:3:3: error: second\n")
    sys.exit(1)
sys.stdout.write("temp folder\n")
'''


def git(*args: str, cwd: Path) -> str:
    cmd = ["git", "-c", "user.name=t", "-c", "user.email=t@example.invalid", "-c", "commit.gpgsign=false", *args]
    r = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, check=True)
    return r.stdout.strip()


def make_remote(parent: Path, name: str, files: dict[str, str | bytes]) -> Path:
    folder = parent / name
    folder.mkdir(parents=True)
    git("init", "-q", cwd=folder)
    git("config", "uploadpack.allowFilter", "true", cwd=folder)
    git("config", "uploadpack.allowAnySHA1InWant", "true", cwd=folder)
    commit(folder, files, "first")
    return folder


def commit(folder: Path, files: dict[str, str | bytes], message: str) -> str:
    for rel, data in files.items():
        p = folder / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_bytes(data if isinstance(data, bytes) else data.encode("utf-8"))
    git("add", "-A", cwd=folder)
    git("commit", "-q", "-m", message, cwd=folder)
    return git("rev-parse", "HEAD", cwd=folder)


def index_entry(folder: Path, mode: str, path: str, content: str) -> None:
    """An entry made without a checkout: a symlink, a gitlink, or a name Windows cannot hold."""
    sha = subprocess.run(
        ["git", "hash-object", "-w", "--stdin"], cwd=folder, input=content, capture_output=True, text=True, check=True
    ).stdout.strip()
    git("-c", "core.protectNTFS=false", "update-index", "--add", "--cacheinfo", f"{mode},{sha},{path}", cwd=folder)


def commit_index(folder: Path, message: str) -> str:
    git("commit", "-q", "-m", message, cwd=folder)
    return git("rev-parse", "HEAD", cwd=folder)


def run_quiet(fn, *a, **kw):
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        code = fn(*a, **kw)
    return code, buf.getvalue()


class Base(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory(ignore_cleanup_errors=True)
        self.addCleanup(self._tmp.cleanup)
        self.tmp = Path(self._tmp.name)
        self.root = self.tmp / "clones"
        self.root.mkdir()
        self.git = c.Git(self.root / ".no-hooks", allow_file=True)
        self.qb = self.tmp / "fake_qb.py"
        self.qb.write_text(FAKE_QB, encoding="utf-8")


class FetchTests(Base):
    def test_fetch_moves_a_clone_to_a_moved_pin_and_leaves_the_others(self):
        a = make_remote(self.tmp, "ra", {"a.bas": "PRINT 1\n"})
        b = make_remote(self.tmp, "rb", {"b.bas": "PRINT 2\n"})
        a1, b1 = git("rev-parse", "HEAD", cwd=a), git("rev-parse", "HEAD", cwd=b)
        repo_a, repo_b = c.Repo("a", str(a), a1), c.Repo("b", str(b), b1)
        self.assertEqual(c.fetch_repo(self.git, repo_a, self.root), "cloned")
        self.assertEqual(c.fetch_repo(self.git, repo_b, self.root), "cloned")
        a2 = commit(a, {"a2.bas": "PRINT 3\n"}, "second")
        self.assertEqual(c.fetch_repo(self.git, c.Repo("a", str(a), a2), self.root), "moved")
        self.assertEqual(c.head_of(self.git, self.root / "a"), a2)
        self.assertTrue((self.root / "a" / "a2.bas").is_file())
        self.assertEqual(c.fetch_repo(self.git, repo_b, self.root), "unchanged")
        # and back to the old pin
        self.assertEqual(c.fetch_repo(self.git, repo_a, self.root), "moved")
        self.assertEqual(c.head_of(self.git, self.root / "a"), a1)

    def test_a_submodule_comes_from_the_manifest_not_from_gitmodules(self):
        lib = make_remote(self.tmp, "lib", {"lib.bi": "' library\n"})
        lib1 = git("rev-parse", "HEAD", cwd=lib)
        main = make_remote(self.tmp, "main", {"m.bas": "PRINT 1\n"})
        (main / ".gitmodules").write_text(f'[submodule "inc"]\n\tpath = inc\n\turl = {self.tmp / "nowhere"}\n')
        git("add", ".gitmodules", cwd=main)
        # a gitlink's sha is the submodule's commit itself
        git("update-index", "--add", "--cacheinfo", f"160000,{lib1},inc", cwd=main)
        pin = commit_index(main, "with submodule")
        repo = c.Repo("m", str(main), pin, submodules={"inc": (str(lib), lib1)})
        c.fetch_repo(self.git, repo, self.root)
        self.assertEqual(c.head_of(self.git, self.root / "m" / "inc"), lib1)
        self.assertTrue((self.root / "m" / "inc" / "lib.bi").is_file())

    def test_a_symlink_is_checked_out_as_a_plain_file(self):
        r = make_remote(self.tmp, "rl", {"real.txt": "x\n"})
        index_entry(r, "120000", "link", "real.txt")
        pin = commit_index(r, "symlink")
        c.fetch_repo(self.git, c.Repo("l", str(r), pin), self.root)
        link = self.root / "l" / "link"
        self.assertTrue(link.is_file())
        self.assertFalse(link.is_symlink())
        self.assertEqual(link.read_text(), "real.txt")

    def test_a_hook_in_the_clone_does_not_run(self):
        r = make_remote(self.tmp, "rh", {"a.bas": "PRINT 1\n"})
        p1 = git("rev-parse", "HEAD", cwd=r)
        p2 = commit(r, {"b.bas": "PRINT 2\n"}, "second")
        c.fetch_repo(self.git, c.Repo("h", str(r), p1), self.root)
        marker = self.tmp / "hook-ran"
        hook = self.root / "h" / ".git" / "hooks" / "post-checkout"
        hook.parent.mkdir(exist_ok=True)
        hook.write_text(f'#!/bin/sh\ntouch "{marker.as_posix()}"\n')
        hook.chmod(0o755)
        c.fetch_repo(self.git, c.Repo("h", str(r), p2), self.root)
        self.assertFalse(marker.exists(), "the hook ran during fetch")
        # control: a plain git checkout does run it, so the test could have failed
        subprocess.run(["git", "checkout", "-q", "--detach", p1], cwd=self.root / "h", capture_output=True)
        if not marker.exists():
            self.skipTest("this git does not run the sample hook, so the test above proves nothing")

    def test_every_git_command_carries_the_hardened_settings(self):
        git_hard = c.Git(self.root / ".no-hooks", allow_file=False)
        cmd = " ".join(git_hard.command("clone", "x"))
        for setting in ("core.symlinks=false", "protocol.file.allow=never", "transfer.fsckObjects=true", "core.hooksPath="):
            self.assertIn(setting, cmd)
        self.assertEqual(git_hard.env["GIT_LFS_SKIP_SMUDGE"], "1")
        self.assertEqual(git_hard.env["GIT_TERMINAL_PROMPT"], "0")
        self.assertNotIn("recurse-submodules", cmd)

    def test_file_remotes_are_refused_by_default(self):
        r = make_remote(self.tmp, "rf", {"a.bas": "PRINT 1\n"})
        pin = git("rev-parse", "HEAD", cwd=r)
        hard = c.Git(self.root / ".no-hooks", allow_file=False)
        with self.assertRaises(c.ToolError):
            c.fetch_repo(hard, c.Repo("f", str(r), pin), self.root)

    def test_an_old_git_is_refused(self):
        with self.assertRaises(c.ToolError) as cm:
            c.require_git("git version 2.45.0.windows.1")
        self.assertIn("2.45.1", str(cm.exception))
        c.require_git("git version 2.45.1")
        c.require_git("git version 2.46.0.windows.1")
        c.require_git("git version 3.0.0")

    def test_fetch_writes_heads_txt(self):
        r = make_remote(self.tmp, "rt", {"a.bas": "PRINT 1\n"})
        pin = git("rev-parse", "HEAD", cwd=r)
        repo = c.Repo("t", str(r), pin)
        args = SimpleNamespace(root=str(self.root), only=None, allow_file=True)
        code, out = run_quiet(c.cmd_fetch, args, [repo])
        self.assertEqual(code, 0, out)
        self.assertEqual(c.read_heads(self.root), {"t": pin})

    def test_a_failed_fetch_names_the_repository(self):
        repo = c.Repo("gone", str(self.tmp / "no-such-remote"), "a" * 40)
        args = SimpleNamespace(root=str(self.root), only=None, allow_file=True)
        code, out = run_quiet(c.cmd_fetch, args, [repo])
        self.assertEqual(code, 1)
        self.assertIn("gone", out)


class ManifestTests(unittest.TestCase):
    def load(self, text: str):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "m.toml"
            p.write_text(text, encoding="utf-8")
            return c.load_manifest(p)

    GOOD = '[[repo]]\nname = "a"\nurl = "https://example.invalid/a"\ncommit = "' + "a" * 40 + '"\n'

    def test_a_good_manifest(self):
        r = self.load(self.GOOD + 'license = "MIT"\nsubmodules = { "inc/x" = { url = "https://e.invalid/x", commit = "' + "b" * 40 + '" } }\n')
        self.assertEqual(r[0].submodules["inc/x"][1], "b" * 40)

    def test_an_unsafe_submodule_path_is_refused(self):
        for bad in ("../x", "/abs", "a/../b", ".git/x", "C:/x"):
            with self.assertRaises(c.ToolError, msg=bad):
                self.load(self.GOOD + 'submodules = { "' + bad + '" = { url = "u", commit = "' + "b" * 40 + '" } }\n')

    def test_a_short_commit_is_refused(self):
        with self.assertRaises(c.ToolError):
            self.load('[[repo]]\nname = "a"\nurl = "u"\ncommit = "abc123"\n')

    def test_an_include_root_must_name_a_repository(self):
        with self.assertRaises(c.ToolError):
            self.load(self.GOOD + 'include_root = "nope"\n')

    def test_the_real_manifest_loads(self):
        repos = c.load_manifest()
        self.assertGreater(len(repos), 50)
        draw = next(r for r in repos if r.name == "draw")
        self.assertIn("includes/QB64_GJ_LIB", draw.submodules)

    def test_the_real_manifest_holds_no_local_path(self):
        text = c.MANIFEST.read_text(encoding="utf-8")
        for needle in ("C:\\", "C:/", "/c/", "/Users/", "/home/"):
            self.assertNotIn(needle, text)


class SanitizeTests(unittest.TestCase):
    def test_cut_to_200_characters(self):
        self.assertEqual(len(c.sanitize("e" * 500, None)), 200)

    def test_an_outside_path_becomes_path_and_the_clone_path_goes(self):
        clone = Path("/tmp/community/draw") if sys.platform != "win32" else Path("C:/work/community/draw")
        inside = f"{clone}/GUI/x.bm"
        text = f"{inside}:3:1: error: cannot read C:\\secret\\dir\\x.bi, /home/me/y.bi missing"
        out = c.sanitize(text, clone)
        self.assertNotIn("secret", out)
        self.assertNotIn("/home", out)
        self.assertNotIn("community", out)
        self.assertIn("<path>", out)
        self.assertTrue(out.startswith("GUI/x.bm:3:1"))

    def test_control_characters_are_removed(self):
        self.assertEqual(c.sanitize("a\x1b[31mb\nc", None), "a [31mb c")

    def test_odd_windows_names(self):
        for bad in ("CON", "a/nul.txt", "x/COM1", "a/../b", "trail.", "a:b"):
            self.assertTrue(c.odd_windows_name(bad), bad)
        for good in ("a/b.bas", "console.bas", "communal/x"):
            self.assertFalse(c.odd_windows_name(good), good)


class CheckTests(Base):
    """`check` against plain folders and heads.txt, as the suites VM holds them: no .git, no git needed."""

    def setUp(self):
        super().setUp()
        self.pin = "a" * 40
        self.repos = [c.Repo("a", "u", self.pin), c.Repo("b", "u", "b" * 40)]
        for name, files in {"a": {"ok.bas": "PRINT 1", "Sub/Deep.BAS": "PRINT 2"}, "b": {"x.bas": "PRINT 3"}}.items():
            for rel, text in files.items():
                p = self.root / name / rel
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text(text)
        (self.root / c.HEADS).write_text(f"a {self.pin}\nb {'b' * 40}\n")
        self.list = self.tmp / "pass.list"
        self.results = self.tmp / "out" / "results.json"

    def args(self, **kw):
        base = dict(
            root=str(self.root), only=None, update_list=False, jobs=2, timeout=30.0,
            qb64rust=[sys.executable, str(self.qb)], list=str(self.list), results=str(self.results), from_results=None,
        )  # fmt: skip
        base.update(kw)
        return SimpleNamespace(**base)

    def write(self, repo: str, rel: str, text: str):
        p = self.root / repo / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text)

    def test_all_programs_found_in_any_case_and_results_written(self):
        code, out = run_quiet(c.cmd_check, self.args(), self.repos)
        self.assertEqual(code, 0, out)
        data = json.loads(self.results.read_text())
        self.assertEqual(sorted(p["program"] for p in data["programs"]), ["a/Sub/Deep.BAS", "a/ok.bas", "b/x.bas"])
        self.assertTrue(all(p["passed"] for p in data["programs"]))
        self.assertIn("NEW 3", out)

    def test_a_listed_failure_fails_the_run_with_the_program_and_its_error(self):
        self.write("a", "bad.bas", "FAIL")
        self.list.write_text("a/bad.bas\na/ok.bas\n")
        code, out = run_quiet(c.cmd_check, self.args(), self.repos)
        self.assertEqual(code, 1)
        self.assertIn("FAIL a/bad.bas: f.bas:2:3: error: boom", out)
        self.assertNotIn("second", out)  # only the first error

    def test_an_unlisted_failure_does_not_fail_the_run(self):
        self.write("a", "bad.bas", "FAIL")
        self.list.write_text("a/ok.bas\n")
        code, out = run_quiet(c.cmd_check, self.args(), self.repos)
        self.assertEqual(code, 0, out)

    def test_a_listed_program_missing_from_the_clone_fails_the_run(self):
        self.list.write_text("a/gone.bas\n")
        code, out = run_quiet(c.cmd_check, self.args(), self.repos)
        self.assertEqual(code, 1)
        self.assertIn("MISSING a/gone.bas", out)

    def test_a_new_pass_is_reported_and_update_list_only_adds(self):
        self.list.write_text("a/gone.bas\nzzz/other.bas\n")
        code, out = run_quiet(c.cmd_check, self.args(update_list=True), self.repos)
        self.assertEqual(code, 1)  # a/gone.bas is still missing
        lines = self.list.read_text().splitlines()
        self.assertEqual(lines, sorted(lines))
        for kept in ("a/gone.bas", "zzz/other.bas", "a/ok.bas", "a/Sub/Deep.BAS", "b/x.bas"):
            self.assertIn(kept, lines)

    def test_a_repository_not_named_by_only_is_not_judged(self):
        self.list.write_text("b/gone.bas\n")
        code, out = run_quiet(c.cmd_check, self.args(only=["a"]), self.repos)
        self.assertEqual(code, 0, out)
        data = json.loads(self.results.read_text())
        self.assertTrue(all(p["program"].startswith("a/") for p in data["programs"]))

    def test_from_results_runs_nothing(self):
        run_quiet(c.cmd_check, self.args(), self.repos)
        self.list.write_text("a/ok.bas\n")
        bad_qb = [sys.executable, "-c", "raise SystemExit('must not run')"]
        code, out = run_quiet(
            c.cmd_check, self.args(from_results=str(self.results), qb64rust=bad_qb, update_list=True), self.repos
        )
        self.assertEqual(code, 0, out)
        self.assertEqual(self.list.read_text().splitlines(), ["a/Sub/Deep.BAS", "a/ok.bas", "b/x.bas"])

    def test_a_clone_at_the_wrong_commit_fails_before_any_program(self):
        (self.root / c.HEADS).write_text(f"a {'c' * 40}\nb {'b' * 40}\n")
        with self.assertRaises(c.ToolError) as cm:
            c.cmd_check(self.args(), self.repos)
        msg = str(cm.exception)
        self.assertIn("a", msg)
        self.assertIn("fetch --only a", msg)
        self.assertFalse(self.results.exists())

    def test_a_missing_clone_fails_before_any_program(self):
        import shutil

        shutil.rmtree(self.root / "b")
        with self.assertRaises(c.ToolError) as cm:
            c.cmd_check(self.args(), self.repos)
        self.assertIn("fetch --only b", str(cm.exception))

    def test_a_submodule_at_the_wrong_commit_is_refused(self):
        repos = [c.Repo("a", "u", self.pin, submodules={"inc": ("u", "d" * 40)})]
        with self.assertRaises(c.ToolError) as cm:
            c.cmd_check(self.args(), repos)
        self.assertIn("a/inc", str(cm.exception))

    def test_the_first_error_is_cut_to_200_characters(self):
        self.write("a", "long.bas", "LONG")
        res = c.run_checks([sys.executable, str(self.qb)], self.repos[:1], self.root, 2, 30)
        long = next(r for r in res if r["program"] == "a/long.bas")
        self.assertFalse(long["passed"])
        self.assertEqual(len(long["error"]), 200)

    def test_an_outside_path_in_an_error_is_shown_as_path(self):
        self.write("a", "out.bas", "OUTSIDE")
        res = c.run_checks([sys.executable, str(self.qb)], self.repos[:1], self.root, 2, 30)
        err = next(r for r in res if r["program"] == "a/out.bas")["error"]
        self.assertIn("<path>", err)
        self.assertNotIn("secret", err)
        self.assertNotIn("/home/me", err)
        self.assertNotIn(str(self.root), err)
        self.assertTrue(err.startswith("f.bas:1:1: error"))

    def test_a_program_that_hangs_is_stopped_at_the_time_limit(self):
        self.write("a", "hang.bas", "HANG")
        started = time.monotonic()
        res = c.run_checks([sys.executable, str(self.qb)], self.repos[:1], self.root, 2, 1.5)
        self.assertLess(time.monotonic() - started, 20)
        hung = next(r for r in res if r["program"] == "a/hang.bas")
        self.assertFalse(hung["passed"])
        self.assertEqual(hung["error"], "timed out")
        self.assertTrue(next(r for r in res if r["program"] == "a/ok.bas")["passed"])

    def test_no_source_line_is_printed(self):
        self.write("a", "bad.bas", "FAIL SECRET-SOURCE-TEXT")
        self.list.write_text("a/bad.bas\n")
        code, out = run_quiet(c.cmd_check, self.args(), self.repos)
        self.assertNotIn("SECRET-SOURCE-TEXT", out)

    def test_the_include_root_is_passed_as_the_named_clone(self):
        seen = self.tmp / "seen.txt"
        spy = self.tmp / "spy.py"
        spy.write_text(f"import sys\nopen(r'{seen}', 'a').write(' '.join(sys.argv[1:]) + '\\n')\n")
        repos = [c.Repo("a", "u", self.pin, include_root="b"), self.repos[1]]
        c.run_checks([sys.executable, str(spy)], repos[:1], self.root, 1, 30)
        text = seen.read_text()
        self.assertIn("--include-root", text)
        self.assertIn(str(self.root / "b"), text)

    def test_names_with_control_characters_are_left_out(self):
        progs, skipped = c.list_programs(self.root / "a")
        self.assertEqual(skipped, 0)
        self.assertEqual(progs, ["Sub/Deep.BAS", "ok.bas"])


class AuditTests(Base):
    def make(self, files, extra=None):
        remote = self.root / "x"
        remote.mkdir()
        git("init", "-q", cwd=remote)
        pin = commit(remote, files, "first")
        return remote, pin

    def test_each_kind_of_file_is_listed_once(self):
        remote, _ = self.make({"ok.bas": "PRINT 1\n"})
        files = {
            "tool.exe": b"MZ\x90\x00",
            "disguised.dat": b"#!/bin/sh\necho hi\n",
            "big.bin": b"\0" * 3000,
            ".vscode/settings.json": "{}",
            ".vscode/tasks.json": "{}",
        }
        commit(remote, files, "second")
        index_entry(remote, "120000", "link", "real.txt")
        index_entry(remote, "100644", "CON", "x")
        index_entry(remote, "160000", "sub", "e" * 40)
        pin = commit_index(remote, "odd entries")
        repo = c.Repo("x", "u", pin)
        found = c.audit_repo(self.git, repo, self.root, big=2000)
        lines = [f"{cat}: {detail}" for cat, detail in found]
        for needle in (
            "tool.exe",
            "disguised.dat",
            "link -> real.txt",
            "big.bin (3000 bytes)",
            "CON",
            "sub -> NOT IN THE MANIFEST",
            ".vscode (2 files)",
        ):
            self.assertEqual(sum(needle in ln for ln in lines), 1, f"{needle!r} in {lines}")
        self.assertFalse(any("ok.bas" in ln for ln in lines))

    def test_since_lists_only_what_changed(self):
        remote, first = self.make({"old.exe": b"MZ\x90\x00", "a.bas": "PRINT 1\n"})
        pin = commit(remote, {"new.dll": b"MZ\x90\x00", "b.bas": "PRINT 2\n"}, "second")
        repo = c.Repo("x", "u", pin)
        everything = " ".join(d for _, d in c.audit_repo(self.git, repo, self.root))
        self.assertIn("old.exe", everything)
        self.assertIn("new.dll", everything)
        changed = " ".join(d for _, d in c.audit_repo(self.git, repo, self.root, since=first))
        self.assertIn("new.dll", changed)
        self.assertNotIn("old.exe", changed)

    def test_a_manifest_submodule_shows_its_url(self):
        remote, _ = self.make({"a.bas": "PRINT 1\n"})
        index_entry(remote, "160000", "inc", "f" * 40)
        pin = commit_index(remote, "gitlink")
        repo = c.Repo("x", "u", pin, submodules={"inc": ("https://example.invalid/lib", "f" * 40)})
        details = [d for cat, d in c.audit_repo(self.git, repo, self.root) if cat == "submodule"]
        self.assertEqual(details, ["inc -> https://example.invalid/lib"])

    def test_a_clone_not_at_its_pin_is_refused(self):
        self.make({"a.bas": "PRINT 1\n"})
        with self.assertRaises(c.ToolError):
            c.audit_repo(self.git, c.Repo("x", "u", "9" * 40), self.root)


if __name__ == "__main__":
    unittest.main()
