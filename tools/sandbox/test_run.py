"""Tests of tools/sandbox/run.py without a sandbox or the test VM: python -m unittest tools/sandbox/test_run.py"""

from __future__ import annotations

import io
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))
import run  # pylint: disable=wrong-import-position,import-error

# No test starts the test VM or runs tools/winvm/run.ps1: both are replaced for the whole module, and a test that
# reaches either fails. The tests of vm_check and vm_run call the originals with a fake runner.
VM_CHECK, VM_RUN_ONCE = run.vm_check, run.vm_run
PROGRAMS, JUDGE = run.programs, run.judge


def _no_vm(*_args, **_kwargs):
    raise AssertionError("a test reached the test VM")


run.vm_check = _no_vm
run.vm_run = _no_vm


def _no_scale(*_args, **_kwargs):
    raise AssertionError("a run in the test VM read or set the host's display scale")


COUNTS = "coverage 2\nfile\t0\t0123456789abcdef\t{prog}.bas\n0\t3\t1\t{n}\t{names}\nend\n"
LIST_TEXT = (
    "# a comment\n_SCREENMOVE\tsandbox moves the window\n_MOUSEMOVE\tsandbox\nLPRINT\tprinter\n"
)


class Folder(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        self.family = self.root / "family"
        self.results = self.root / "results"
        self.family.mkdir()
        self.results.mkdir()

    def tearDown(self):
        self._tmp.cleanup()

    def result(self, name: str, exit_code: str | None, out: str | None = None) -> None:
        d = self.results / name
        d.mkdir(parents=True, exist_ok=True)
        if exit_code is not None:
            (d / "exit").write_text(exit_code + "\n", encoding="ascii")
        if out is not None:
            (d / "out.txt").write_bytes(out.encode("latin-1"))


class TestWsb(unittest.TestCase):
    def test_nothing_shared_but_the_one_folder(self):
        host = Path(r"X:\some & where\target\sandbox\run")
        text = run.wsb(host)
        root = ET.fromstring(text)
        for setting in (
            "Networking",
            "ClipboardRedirection",
            "PrinterRedirection",
            "AudioInput",
            "VideoInput",
        ):
            self.assertEqual(root.findtext(setting), "Disable", setting)
        folders = root.findall("MappedFolders/MappedFolder")
        self.assertEqual(len(folders), 1)
        self.assertEqual(folders[0].findtext("HostFolder"), str(host))
        self.assertEqual(folders[0].findtext("SandboxFolder"), r"C:\run")
        self.assertEqual(folders[0].findtext("ReadOnly"), "false")
        self.assertIn(r"C:\run\inside.ps1", root.findtext("LogonCommand/Command"))

    def test_the_sandbox_shuts_itself_down(self):
        # inside.ps1 shuts the machine down only when asked: the sandbox asks, the test VM does not.
        command = ET.fromstring(run.wsb(Path(r"X:\r"))).findtext("LogonCommand/Command")
        self.assertTrue(command.endswith(r"C:\run\inside.ps1 -Shutdown"), command)

    def test_no_host_path_but_the_one_passed(self):
        text = run.wsb(Path(r"Q:\only\this"))
        self.assertEqual(text.count(":\\"), text.count(r"Q:\only\this") + text.count(r"C:\run"))
        self.assertNotIn(str(run.REPO), text)


class TestRefusal(unittest.TestCase):
    def test_not_installed(self):
        why = run.refusal(False, [])
        self.assertIn(run.FEATURE, why)

    def test_a_sandbox_running(self):
        why = run.refusal(True, ["WindowsSandboxRemoteSession.exe"])
        self.assertIn("a sandbox is running", why)
        self.assertIn("WindowsSandboxRemoteSession.exe", why)

    def test_free_to_run(self):
        self.assertIsNone(run.refusal(True, []))
        self.assertIsNone(run.refusal(True, [], 100))

    def test_a_scaled_display_refuses_a_recording_only(self):
        # 2026-10-30: at 125% qb64pe.exe's _SCREENCLICK lands at 1.25 times its place (D-056); this compiler's does not.
        why = run.refusal(True, [], 125, recording=True)
        self.assertIn("display scale is 125%", why)
        self.assertIn("could not be read", run.refusal(True, [], None, recording=True))
        self.assertIsNone(run.refusal(True, [], 125))
        self.assertIsNone(run.refusal(True, [], None))

    def test_the_scale_is_read_or_none(self):
        scale = run.display_scale()
        self.assertTrue(scale is None or 50 <= scale <= 500, scale)

    def test_the_override_for_a_scale(self):
        # 2026-10-30: 125% recommended; 100% is one step below it, 150% one above.
        self.assertEqual(run.scale_offset(125, 100), -1)
        self.assertEqual(run.scale_offset(125, 125), 0)
        self.assertEqual(run.scale_offset(125, 150), 1)
        self.assertEqual(run.scale_offset(100, 200), 4)
        self.assertIsNone(run.scale_offset(110, 100))

    def test_setting_the_scale_sets_step_0_then_the_targets(self):
        # The machine is not touched: the API call and the reads are stood in for. A fake monitor recommending 125%.
        calls: list[int] = []
        state = {"step": -1}

        def api(_spi, step, _p, _f):
            calls.append(step)
            state["step"] = step
            return 1

        def read():
            return run.SCALES[run.SCALES.index(125) + state["step"]]

        with mock.patch("ctypes.windll.user32.SystemParametersInfoW", api):
            # At 100% (step -1), asked for 100%: nothing is called.
            self.assertTrue(run.set_display_scale(100, read=read, sleep=lambda s: None))
            self.assertEqual(calls, [])
            # Asked for 125%: step 0 shows it is the recommended scale; done.
            self.assertTrue(run.set_display_scale(125, read=read, sleep=lambda s: None))
            self.assertEqual(calls, [0])
            # Asked for 150%: step 0 again (already there), then +1.
            calls.clear()
            self.assertTrue(run.set_display_scale(150, read=read, sleep=lambda s: None))
            self.assertEqual(calls, [0, 1])
            # Back to 100%: 0, then -1. (The registry was not asked: it said 0 while -1 was in effect, 2026-10-30.)
            calls.clear()
            self.assertTrue(run.set_display_scale(100, read=read, sleep=lambda s: None))
            self.assertEqual(calls, [0, -1])
            # A scale that is not read back: False.
            self.assertFalse(run.set_display_scale(100, read=lambda: 175, sleep=lambda s: None))

    def test_a_recording_sets_the_scale_and_puts_it_back(self):
        readings = iter([125, 100, 100])
        log: list[str] = []
        patches = [
            mock.patch.object(run, "display_scale", lambda: next(readings)),
            mock.patch.object(run, "set_display_scale", lambda t: log.append(f"set {t}") or True),
            mock.patch.object(run, "run_family", lambda a: log.append("ran") or 0),
        ]
        for p in patches:
            p.start()
        try:
            out = io.StringIO()
            with redirect_stdout(out):
                self.assertEqual(run.main(["--record", "--where", "sandbox"]), 0)
        finally:
            mock.patch.stopall()
        self.assertEqual(log, ["set 100", "ran", "set 125"])
        self.assertIn("display scale: 125% set to 100% for the run", out.getvalue())
        self.assertIn("display scale: put back to 125%", out.getvalue())

    def test_main_refuses_before_building(self):
        # set_display_scale is stood in for too: on 2026-10-30 this test, without that, set the machine's display
        # to 100% (a recording run sets it before anything else).
        old = run.SANDBOX, run.running_processes, run.programs, run.display_scale, run.set_display_scale
        built = []
        set_to = []
        try:
            run.SANDBOX = Path(r"Z:\no\WindowsSandbox.exe")
            run.running_processes = list
            run.programs = lambda glob: built.append(glob) or []
            run.display_scale = lambda: 100
            run.set_display_scale = lambda t: set_to.append(t) or False
            sandbox = ["--where", "sandbox"]
            self.assertEqual(run.main(sandbox), 2)
            run.SANDBOX = Path(sys.executable)
            run.running_processes = lambda: ["WindowsSandboxRemoteSession.exe"]
            self.assertEqual(run.main(sandbox), 2)
            run.running_processes = list
            run.display_scale = lambda: 150
            self.assertEqual(run.main(["--record", *sandbox]), 2)
        finally:
            run.SANDBOX, run.running_processes, run.programs, run.display_scale, run.set_display_scale = old
        self.assertEqual(built, [])
        self.assertEqual(set_to, [100])


class TestMain(Folder):
    def test_builds_all_failed_read_no_earlier_results(self):
        # An earlier run's counts in results/: a run whose builds all fail starts no sandbox and must not read them.
        self.result("a", "0", "x\n")
        (self.results / "a" / "counts").write_text(COUNTS.format(prog="a", n=1, names="_SCREENMOVE"))
        patches = [
            mock.patch.object(run, "SANDBOX", Path(sys.executable)),
            mock.patch.object(run, "running_processes", list),
            mock.patch.object(run, "display_scale", lambda: 100),
            mock.patch.object(run, "programs", lambda glob: [self.family / "a.bas"]),
            mock.patch.object(run, "RUN", self.root),
            mock.patch.object(run, "run_copy", lambda exe: (exe, "ab")),
            mock.patch.object(run, "name_objects_lock", lambda: None),
            mock.patch.object(run, "build_new", lambda *args: False),
            mock.patch.object(run.os, "startfile", None, create=True),
        ]
        out = io.StringIO()
        for p in patches:
            p.start()
        try:
            with redirect_stdout(out):
                self.assertEqual(run.main(["--where", "sandbox"]), 1)
        finally:
            mock.patch.stopall()
        self.assertFalse(self.results.exists(), out.getvalue())
        self.assertIn("FAIL  a: build failed", out.getvalue())
        self.assertIn("display scale: 100%", out.getvalue())


class TestWhere(unittest.TestCase):
    """--where vm (the default) never reads or sets the host's display scale; --scale is refused there."""

    def main(self, argv: list[str]) -> tuple[int, list[str], str]:
        ran: list[str] = []
        patches = [
            mock.patch.object(run, "display_scale", _no_scale),
            mock.patch.object(run, "set_display_scale", _no_scale),
            mock.patch.object(run, "run_family", lambda a: ran.append(a.where) or 0),
        ]
        for p in patches:
            p.start()
        err = io.StringIO()
        try:
            with mock.patch.object(sys, "stderr", err):
                code = run.main(argv)
        finally:
            mock.patch.stopall()
        return code, ran, err.getvalue()

    def test_the_vm_by_default(self):
        self.assertEqual(self.main([]), (0, ["vm"], ""))

    def test_a_recording_in_the_vm_leaves_the_scale(self):
        self.assertEqual(self.main(["--record"]), (0, ["vm"], ""))

    def test_scale_refused_in_the_vm(self):
        code, ran, err = self.main(["--scale", "125"])
        self.assertEqual((code, ran), (2, []))
        self.assertIn("--scale", err)
        self.assertIn("--where sandbox", err)

    def test_the_sandbox_when_asked(self):
        with mock.patch.object(run, "display_scale", lambda: 100), mock.patch.object(
            run, "run_family", lambda a: a.where == "sandbox" and 0
        ):
            self.assertEqual(run.main(["--where", "sandbox"]), 0)


class TestVmRunner(unittest.TestCase):
    """run.ps1's exits mapped to a run, a timeout and a refusal; run.ps1 itself is never started."""

    def runner(self, code: int | None, calls: list):
        def fake(cmd, **kwargs):
            calls.append((cmd, kwargs))
            if code is None:
                raise run.subprocess.TimeoutExpired(cmd, kwargs.get("timeout"))
            return run.subprocess.CompletedProcess(cmd, code)

        return fake

    def test_the_command(self):
        calls: list = []
        VM_RUN_ONCE(Path(r"X:\run"), 200, self.runner(0, calls))
        cmd, kwargs = calls[0]
        self.assertEqual(cmd[0], "powershell.exe")
        self.assertEqual(cmd[cmd.index("-File") + 1], str(run.VM_RUN))
        self.assertEqual(cmd[cmd.index("-Folder") + 1], r"X:\run")
        self.assertEqual(cmd[cmd.index("-Limit") + 1], "200")
        self.assertGreater(kwargs["timeout"], 200)

    def test_its_exits(self):
        for code, want in ((0, run.VM_DONE), (1, run.VM_LIMIT), (2, run.VM_REFUSED), (7, run.VM_LIMIT)):
            self.assertEqual(VM_RUN_ONCE(Path("r"), 10, self.runner(code, [])), want, code)
        with redirect_stdout(io.StringIO()):
            self.assertEqual(VM_RUN_ONCE(Path("r"), 10, self.runner(None, [])), run.VM_LIMIT)

    def test_a_killed_run_turns_the_vm_off(self):
        # Killing run.ps1 skips its finally: run.ps1 -Stop turns the VM off in its place.
        calls: list = []

        def fake(cmd, **kwargs):
            calls.append(cmd)
            if "-Folder" in cmd:
                raise run.subprocess.TimeoutExpired(cmd, kwargs.get("timeout"))
            return run.subprocess.CompletedProcess(cmd, 0)

        out = io.StringIO()
        with redirect_stdout(out):
            self.assertEqual(VM_RUN_ONCE(Path("r"), 10, fake), run.VM_LIMIT)
        self.assertEqual(len(calls), 2)
        self.assertIn("-Stop", calls[1])
        self.assertNotIn("may still be on", out.getvalue())

    def test_a_vm_that_will_not_turn_off_is_said(self):
        out = io.StringIO()
        with redirect_stdout(out):
            self.assertEqual(VM_RUN_ONCE(Path("r"), 10, self.runner(None, [])), run.VM_LIMIT)
        self.assertIn("may still be on", out.getvalue())

    def test_the_check(self):
        calls: list = []
        self.assertEqual(VM_CHECK(self.runner(0, calls)), run.VM_DONE)
        self.assertIn("-Check", calls[0][0])
        self.assertEqual(calls[0][1]["timeout"], run.VM_CHECK_LIMIT)
        self.assertEqual(VM_CHECK(self.runner(2, [])), run.VM_REFUSED)
        # Anything but 0 is no go.
        self.assertEqual(VM_CHECK(self.runner(1, [])), run.VM_REFUSED)

    def test_a_check_that_hangs_is_a_refusal(self):
        out = io.StringIO()
        with redirect_stdout(out):
            self.assertEqual(VM_CHECK(self.runner(None, [])), run.VM_REFUSED)
        self.assertIn("did not end", out.getvalue())


class TestMainInTheVm(Folder):
    """run_family with --where vm, run.ps1 stood in for by vm_check and vm_run."""

    def main(self, vm_code: int, check: int = 0, results=None):
        """`results` is one run's {name: (exit, out)}, or a list of them, one for each start of the VM."""
        for n in ("a", "b"):
            (self.family / f"{n}.bas").write_text("", encoding="ascii")
            (self.family / f"{n}.out.txt").write_text("x\n", encoding="ascii")
        runs: list = []
        per_run = results if isinstance(results, list) else [results or {}]

        def build(_compiler, src, out, _log):
            out.write_bytes(b"MZ")
            return True

        def vm_run(folder, limit):
            runs.append((folder, limit))
            listed = (folder / "list.txt").read_text(encoding="ascii").split()
            for name, (code, out) in per_run[min(len(runs), len(per_run)) - 1].items():
                if name in listed:
                    self.result(name, code, out)
            return vm_code

        patches = [
            mock.patch.object(run, "display_scale", _no_scale),
            mock.patch.object(run, "set_display_scale", _no_scale),
            mock.patch.object(run, "vm_check", lambda: check),
            mock.patch.object(run, "vm_run", vm_run),
            mock.patch.object(run, "programs", lambda glob: PROGRAMS(glob, self.family)),
            mock.patch.object(run, "RUN", self.root),
            mock.patch.object(run, "judge", lambda name, results: JUDGE(name, results, self.family)),
            mock.patch.object(run, "run_copy", lambda exe: (exe, "ab")),
            mock.patch.object(run, "name_objects_lock", lambda: None),
            mock.patch.object(run, "build_new", build),
            mock.patch.object(run, "coverage", lambda *a: ([], "coverage: not checked (test)")),
            mock.patch.object(run.os, "startfile", _no_vm, create=True),
        ]
        out = io.StringIO()
        for p in patches:
            p.start()
        try:
            with redirect_stdout(out):
                code = run.main(["--timeout", "5"])
        finally:
            mock.patch.stopall()
        return code, runs, out.getvalue()

    def test_a_run(self):
        code, runs, out = self.main(run.VM_DONE, results={"a": ("0", "x\n"), "b": ("0", "x\n")})
        self.assertEqual(code, 0, out)
        self.assertEqual(runs, [(self.root, 5 * 2 + 60)])
        self.assertFalse((self.root / "sandbox.wsb").exists())
        self.assertIn("sandbox: 2 pass, 0 fail", out)

    def test_a_program_that_timed_out_runs_once_more(self):
        code, runs, out = self.main(
            run.VM_DONE,
            results=[{"a": ("0", "x\n"), "b": ("timeout", None)}, {"b": ("0", "x\n")}],
        )
        self.assertEqual(code, 0, out)
        self.assertEqual(runs, [(self.root, 5 * 2 + 60), (self.root, 5 + 60)])
        self.assertIn("timed out, run once more: b", out)
        self.assertIn("PASS  a: pass\n", out)
        self.assertIn("PASS  b: pass (on its second run: the first timed out)", out)
        self.assertFalse((self.root / "results.first").exists())

    def test_a_second_timeout_fails(self):
        code, runs, out = self.main(run.VM_DONE, results={"a": ("0", "x\n"), "b": ("timeout", None)})
        self.assertEqual((code, len(runs)), (1, 2), out)
        self.assertIn("PASS  a: pass\n", out)
        self.assertIn("FAIL  b: timed out", out)

    def test_a_wrong_output_is_not_run_again(self):
        code, runs, out = self.main(run.VM_DONE, results={"a": ("0", "x\n"), "b": ("0", "y\n")})
        self.assertEqual((code, len(runs)), (1, 1), out)
        self.assertNotIn("run once more", out)

    def test_a_timeout_names_the_programs(self):
        code, _, out = self.main(run.VM_LIMIT, results={"a": ("0", "x\n")})
        self.assertEqual(code, 1)
        self.assertIn("with results: a", out)
        self.assertIn("without: b", out)

    def test_a_refusal_in_the_vm(self):
        code, _, out = self.main(run.VM_REFUSED)
        self.assertEqual(code, 2)
        self.assertNotIn("PASS", out)

    def test_a_refusal_before_building(self):
        code, runs, out = self.main(run.VM_DONE, check=run.VM_REFUSED)
        self.assertEqual((code, runs), (2, []))
        self.assertFalse((self.root / "bin").exists(), out)


class TestPrepare(Folder):
    def test_no_wsb_for_the_vm(self):
        (self.root / "sandbox.wsb").write_text("old", encoding="utf-8")
        self.assertIsNone(run.prepare(self.root, ["a"], 5, sandbox=False))
        self.assertFalse((self.root / "sandbox.wsb").exists())
        self.assertEqual((self.root / "list.txt").read_text(encoding="ascii"), "a\n")
        self.assertTrue((self.root / "inside.ps1").is_file())


class TestWait(Folder):
    def clock(self):
        t = [0.0]

        def now():
            return t[0]

        def sleep(s):
            t[0] += s

        return now, sleep

    def test_done_and_ended(self):
        now, sleep = self.clock()
        (self.results / "done").write_text("done", encoding="ascii")
        alive = iter([["WindowsSandboxRemoteSession.exe"], []])
        self.assertTrue(run.wait(self.results, 10, lambda: next(alive), now, sleep))

    def test_past_the_limit(self):
        now, sleep = self.clock()
        self.assertFalse(
            run.wait(self.results, 5, lambda: ["WindowsSandboxRemoteSession.exe"], now, sleep)
        )


class TestJudge(Folder):
    def test_equal(self):
        (self.family / "a.out.txt").write_text("x 1\n", encoding="ascii")
        self.result("a", "0", "x 1\n")
        v = run.judge("a", self.results, self.family)
        self.assertTrue(v.ok, v.why)

    def test_crlf_output_against_an_lf_recording(self):
        (self.family / "a.out.txt").write_bytes(b"x 1\ny 2\n")
        self.result("a", "0", "x 1\r\ny 2\r\n")
        self.assertTrue(run.judge("a", self.results, self.family).ok)

    def test_a_difference_with_its_diff(self):
        (self.family / "a.out.txt").write_text("x 1\ny 2\n", encoding="ascii")
        self.result("a", "0", "x 1\ny 3\n")
        v = run.judge("a", self.results, self.family)
        self.assertFalse(v.ok)
        self.assertIn("output differs", v.why)
        self.assertIn("-y 2", v.diff)
        self.assertIn("+y 3", v.diff)

    def test_exit_code_against_exit_txt(self):
        (self.family / "a.out.txt").write_text("x\n", encoding="ascii")
        self.result("a", "1", "x\n")
        self.assertIn("exit 1, recorded 0", run.judge("a", self.results, self.family).why)
        (self.family / "a.exit.txt").write_text("1\n", encoding="ascii")
        self.assertTrue(run.judge("a", self.results, self.family).ok)

    def test_a_missing_recording(self):
        self.result("a", "0", "x\n")
        v = run.judge("a", self.results, self.family)
        self.assertFalse(v.ok)
        self.assertIn("no recording", v.why)

    def test_a_timeout(self):
        (self.family / "a.out.txt").write_text("x\n", encoding="ascii")
        self.result("a", "timeout", "x\n")
        v = run.judge("a", self.results, self.family)
        self.assertFalse(v.ok)
        self.assertIn("timed out", v.why)

    def test_a_missing_result(self):
        (self.family / "a.out.txt").write_text("x\n", encoding="ascii")
        self.assertIn("no result", run.judge("a", self.results, self.family).why)
        self.result("a", "0")
        self.assertIn("no out.txt", run.judge("a", self.results, self.family).why)


class TestRecord(Folder):
    def test_writes_the_recording_and_a_nonzero_exit(self):
        self.result("a", "0", "x\r\n")
        self.result("b", "3", "y\n")
        (self.family / "a.exit.txt").write_text("5\n", encoding="ascii")
        self.assertTrue(run.record("a", self.results, self.family).ok)
        self.assertTrue(run.record("b", self.results, self.family).ok)
        self.assertEqual((self.family / "a.out.txt").read_bytes(), b"x\n")
        self.assertFalse((self.family / "a.exit.txt").exists())
        self.assertEqual((self.family / "b.exit.txt").read_text(encoding="ascii"), "3\n")

    def test_a_timeout_records_nothing(self):
        self.result("a", "timeout", "x\n")
        self.assertFalse(run.record("a", self.results, self.family).ok)
        self.assertFalse((self.family / "a.out.txt").exists())


class TestCoverage(Folder):
    def counts(self, prog: str, n: int, names: str) -> Path:
        path = self.results / prog / "counts"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(COUNTS.format(prog=prog, n=n, names=names), encoding="utf-8")
        return path

    def listing(self) -> Path:
        path = self.root / "never_run.list"
        path.write_text(LIST_TEXT, encoding="utf-8")
        return path

    def test_every_sandbox_row_ran(self):
        counts = [self.counts("a", 1, "_SCREENMOVE"), self.counts("b", 2, "_MOUSEMOVE _MOUSEX")]
        self.assertEqual(run.coverage_check(counts, self.listing()), [])

    def test_a_sandbox_row_not_run(self):
        counts = [self.counts("a", 1, "_SCREENMOVE"), self.counts("b", 0, "_MOUSEMOVE")]
        problems = run.coverage_check(counts, self.listing())
        self.assertEqual(len(problems), 1)
        self.assertIn("`_MOUSEMOVE` is listed for the sandbox but did not run", problems[0])

    def test_a_row_of_another_reason_is_not_asked_for(self):
        counts = [self.counts("a", 1, "_SCREENMOVE _MOUSEMOVE")]
        self.assertEqual(run.coverage_check(counts, self.listing()), [])

    def test_a_run_reads_the_counts_of_its_results(self):
        self.counts("a", 0, "_SCREENMOVE _MOUSEMOVE")
        problems, summary = run.coverage(False, None, self.results, self.listing())
        self.assertEqual(len(problems), 2)
        self.assertIn("coverage: FAIL (2 problem(s))", summary)

    def test_a_recording_run_skips_it(self):
        self.counts("a", 0, "_SCREENMOVE _MOUSEMOVE")
        problems, summary = run.coverage(True, None, self.results, self.listing())
        self.assertEqual(problems, [])
        self.assertIn("not checked (a recording run", summary)

    def test_a_glob_run_skips_it(self):
        problems, summary = run.coverage(False, "screen*", self.results, self.listing())
        self.assertEqual(problems, [])
        self.assertIn("not checked (--glob", summary)


class TestPrograms(Folder):
    def test_by_name_and_glob(self):
        for n in ("screenmove_readback", "mousemove_readback", "notes"):
            (self.family / f"{n}.bas").write_text("", encoding="ascii")
        (self.family / "screenmove_readback.out.txt").write_text("", encoding="ascii")
        names = [p.stem for p in run.programs(None, self.family)]
        self.assertEqual(names, ["mousemove_readback", "notes", "screenmove_readback"])
        self.assertEqual(
            [p.stem for p in run.programs("screen*", self.family)], ["screenmove_readback"]
        )


if __name__ == "__main__":
    unittest.main()
