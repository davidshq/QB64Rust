#!/usr/bin/env python3
"""Tests of close.py: how each kind of log is judged, when the run without folding is asked for, and what the
plan holds.

    python -m unittest discover -s tools/close -p "test_*.py"

A change to a closing step or to how its log is judged comes with a change here.
"""

from __future__ import annotations

import io
import sys
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from typing import ClassVar
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))

import close
from close import judge, needs_nofold, steps

CARGO_OK = """test a ... ok
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
"""
CARGO_FAIL = """test resource_files_equal_the_old_compilers ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
test result: ok. 46 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
error: 1 target failed:
"""


def run_part(s: close.Step) -> str:
    """A close's step's command after the record of the commit it runs at (`head_record`)."""
    prefix = f"git rev-parse HEAD > {close.head_record(s.name)} && "
    assert s.command.startswith(prefix), s.command
    return s.command[len(prefix) :]


class Judges(unittest.TestCase):
    def test_cargo_pass_and_counts(self):
        self.assertEqual(judge("cargo", CARGO_OK, set()), (True, "18 passed, 0 failed"))

    def test_cargo_failure_unless_known(self):
        ok, summary = judge("cargo", CARGO_FAIL, set())
        self.assertFalse(ok)
        self.assertIn("not known: resource_files_equal_the_old_compilers", summary)
        ok, summary = judge("cargo", CARGO_FAIL, {"resource_files_equal_the_old_compilers"})
        self.assertTrue(ok)
        self.assertIn("known: resource_files_equal_the_old_compilers", summary)

    def test_cargo_failing_doctest(self):
        log = (
            "   Doc-tests qb64rust_base\n"
            "test crates\\base\\src\\lib.rs - show_bytes (line 3) ... FAILED\n"
            "test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s\n"
        )
        ok, summary = judge("cargo", log, set())
        self.assertFalse(ok)
        self.assertIn("not known: crates\\base\\src\\lib.rs - show_bytes (line 3)", summary)

    def test_cargo_failure_without_its_name(self):
        log = "test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s\n"
        ok, summary = judge("cargo", log, set())
        self.assertFalse(ok)
        self.assertIn("1 of them not named", summary)

    def test_cargo_binary_that_stopped(self):
        # The second binary crashed: a header, no result; cargo goes on (`--no-fail-fast`).
        log = (
            "     Running unittests src\\lib.rs (target\\debug\\deps\\a.exe)\n"
            + CARGO_OK.splitlines(keepends=True)[1]
            + "     Running tests\\inputs.rs (target\\debug\\deps\\inputs.exe)\n"
            "error: test failed, to rerun pass `-p qb64rust-driver --test inputs`\n"
            "error: 1 target failed:\n"
        )
        ok, summary = judge("cargo", log, set())
        self.assertFalse(ok)
        self.assertIn("1 test binaries stopped without a result", summary)

    def test_cargo_build_failure_and_no_result(self):
        self.assertFalse(judge("cargo", "error[E0425]: cannot find value\n" + CARGO_OK, set())[0])
        self.assertFalse(judge("cargo", "error: could not compile `x`\n", set())[0])
        self.assertFalse(judge("cargo", "", set())[0])

    def test_runner(self):
        ok_log = '[1/2] PASS a\n{"corpus": {"PASS": 449, "FAIL": 0, "KFAIL": 0}}\nresults: x.json\n'
        self.assertEqual(judge("runner", ok_log, set()), (True, "449 pass, 0 fail, 0 known"))
        self.assertFalse(
            judge("runner", '{"compile_tests": {"PASS": 3, "FAIL": 4, "KFAIL": 0}}\n', set())[0]
        )
        # Refused for the clone's lock: no result line.
        self.assertFalse(judge("runner", "Another run holds the reference clone\n", set())[0])

    def test_a_known_failure_on_a_pass_list(self):
        # D-052: the runner counts a program of known_failures.txt as KFAIL, which does not fail its run; a run of a
        # pass list fails on it, a full suite does not.
        log = (
            "[1/2] PASS corpus:slice/a (output, 1.0s)\n"
            "[2/2] KFAIL corpus:slice/b (output, 1.0s)  run: not run: known failure\n"
            '{"corpus": {"PASS": 1, "FAIL": 0, "KFAIL": 1}}\n'
        )
        self.assertTrue(judge("runner", log, set())[0])
        self.assertEqual(
            judge("list", log, set()),
            (False, "1 pass, 0 fail, 1 known; a pass list holds no known failure: corpus:slice/b"),
        )
        self.assertTrue(judge("list", '{"corpus": {"PASS": 2, "FAIL": 0, "KFAIL": 0}}\n', set())[0])
        for s in steps(True):
            if "--list" in s.command:
                self.assertEqual(s.judge, "list", s.name)

    def test_checklist_and_wording(self):
        self.assertEqual(
            judge("checklist", "slice.list: 493 of 493 passed in results-corpus.json\n", set()),
            (True, "slice.list: 493 of 493 passed"),
        )
        self.assertFalse(
            judge("checklist", "x\nslice.list: 492 of 493 passed in results-corpus.json\n", set())[
                0
            ]
        )
        self.assertEqual(
            judge("checklist", "no summary of a full run\n", set()), (False, "no check line")
        )
        self.assertTrue(
            judge(
                "wording", "1550 notes on recorded rejections, 0 not the recorded message\n", set()
            )[0]
        )
        self.assertFalse(
            judge(
                "wording", "1550 notes on recorded rejections, 2 not the recorded message\n", set()
            )[0]
        )

    def test_exit(self):
        # The sync and a merge's reports are judged by the exit code their command adds to the log.
        self.assertEqual(judge("exit", "synced 3 files\nexit: 0\n", set()), (True, "exit 0"))
        self.assertEqual(judge("exit", "rsync: failed\nexit: 23\n", set()), (False, "exit 23"))
        self.assertEqual(judge("exit", "synced 3 files\n", set()), (False, "no exit line"))

    def test_counts(self):
        # At a merge every block is written; one left as it is means a full suite did not run as it should.
        self.assertEqual(
            judge("counts", "8 blocks, 2 rewritten\n", set()), (True, "8 blocks, 2 rewritten")
        )
        left = "corpus-results: left as it is: no full run (results-corpus.json)\n8 blocks, 0 rewritten\n"
        self.assertEqual(
            judge("counts", left, set()),
            (False, "8 blocks, 0 rewritten; left as it is: corpus-results"),
        )
        # A malformed README: an error and exit 2, no summary line.
        self.assertEqual(
            judge("counts", "tests/corpus/README.md: the markers ...\n", set()),
            (False, "no summary line"),
        )

    def test_callsite(self):
        self.assertTrue(
            judge("callsite", "1871 programs: 1871 equal, 0 differ, 0 not compared\n", set())[0]
        )
        self.assertFalse(
            judge("callsite", "7 programs: 6 equal, 1 differ, 0 not compared\n", set())[0]
        )
        self.assertFalse(judge("callsite", "Another run holds the reference clone\n", set())[0])

    def test_clippy_fmt_build(self):
        self.assertTrue(judge("clippy", "    Finished `dev` profile\n", set())[0])
        self.assertFalse(judge("clippy", "warning: unused variable\n", set())[0])
        self.assertTrue(judge("empty", "", set())[0])
        self.assertFalse(judge("empty", "Diff in x.rs\n", set())[0])
        self.assertFalse(judge("build", "error: could not compile\n", set())[0])
        self.assertTrue(judge("build", "   Compiling a\n    Finished `release` profile\n", set())[0])
        self.assertEqual(judge(None, "", set()), (True, "ran"))
        # A run stopped before its end shows no error, and no Finished line (2026-10-31).
        self.assertEqual(
            judge("clippy", "    Checking a v0.1.0\n", set()),
            (False, "no Finished line: clippy stopped"),
        )
        self.assertEqual(
            judge("build", "   Compiling a v0.1.0\n", set()),
            (False, "no Finished line: the build stopped"),
        )


class Plan(unittest.TestCase):
    def test_nofold_by_files(self):
        self.assertTrue(needs_nofold(["crates/sema/src/consteval.rs"]))
        self.assertTrue(needs_nofold(["crates/sema/src/check/constants.rs"]))
        self.assertFalse(needs_nofold(["crates/sema/src/check/mod.rs", "STATUS.md"]))

    def test_coverage_by_files(self):
        # Civil War Strategy's coverage check (design D2 of cws-whole-game), only when what it rests on changed.
        for f in (
            "tests/corpus/screen/g12_game.bas",
            "tests/corpus/screen/g12_game.navy.keys",
            "tests/corpus/screen/g12_game.files/CWS.CFG",
            "tests/programs/civil-war-strategy.unreached",
            "tools/screen/cut_game.py",
            "tools/screen/screen_state.cpp",
            "tools/coverage/coverage.py",
            "crates/codegen-cpp/src/procs.rs",
            "crates/ir/src/lower.rs",
            "crates/driver/src/main.rs",
        ):
            self.assertTrue(close.needs_coverage([f]), f)
        for f in (
            "tests/corpus/screen/g12_gamex.bas",
            "tools/screen/state_to_png.py",
            "crates/sema/src/lib.rs",
        ):
            self.assertFalse(close.needs_coverage([f]), f)
        names = [s.name for s in steps(False, True)]
        self.assertLess(names.index("verification"), names.index("coverage"))
        self.assertNotIn("coverage", [s.name for s in steps(False)])
        step = next(s for s in steps(False, True) if s.name == "coverage")
        self.assertEqual(
            run_part(step),
            "python tools/coverage/coverage.py --check > target/close/coverage.log 2>&1",
        )
        self.assertTrue(step.copy)
        self.assertEqual(
            judge("coverage", "...\ncheck: PASS (0 problem(s))\n", set()),
            (True, "check PASS, 0 problems"),
        )
        self.assertFalse(judge("coverage", "check: FAIL (3 problem(s))\n", set())[0])
        self.assertFalse(judge("coverage", "coverage: the counts are of another text\n", set())[0])

    def test_format_by_files(self):
        # The format runs (change m5b-formatter, task 7.1), only when what they rest on changed.
        for f in (
            "crates/format/src/expr.rs",
            "tests/format/corpus/slice/s001.bas",
            "tests/format/known_differences.list",
            "tools/format/layout_words.py",
            "crates/sema/src/symbols.rs",
            "crates/sema/src/check/places.rs",
            "crates/sema/src/check/decl.rs",
            "crates/syntax/src/parser.rs",
            "crates/builtins/src/lib.rs",
            "crates/base/src/layout.rs",
            "crates/driver/src/main.rs",
            "crates/driver/src/lib.rs",
        ):
            self.assertTrue(close.needs_format([f]), f)
        for f in (
            "crates/sema/src/lib.rs",
            "crates/driver/tests/cli.rs",
            "crates/formatx/a.rs",
            "tests/corpus/a.bas",
        ):
            self.assertFalse(close.needs_format([f]), f)
        self.assertTrue(close.INPUTS.match("tools/format/layout_words.py"))
        self.assertNotIn("format", [s.name for s in steps(True, True)])
        names = [s.name for s in steps(False, False, True)]
        self.assertLess(names.index("verification"), names.index("format"))
        self.assertLess(names.index("format"), names.index("format-corpus"))
        by = {s.name: s for s in steps(False, False, True)}
        self.assertEqual(
            run_part(by["format"]),
            "python tools/legacy_tests/run_legacy_tests.py --suite format --qb64 target/release/qb64rust.exe"
            " > target/close/format.log 2>&1",
        )
        self.assertIn(
            "--suite format-corpus --qb64 target/release/qb64rust.exe", by["format-corpus"].command
        )
        self.assertTrue(by["format"].copy and by["format-corpus"].copy)
        self.assertEqual(by["format-corpus"].judge, "runner")

    def test_linux_by_files(self):
        # The Linux family (design D11 of m7-linux): a change to what the release build is made from asks for it, a
        # document does not.
        for f in (
            "crates/driver/src/build.rs",
            "crates/codegen-cpp/src/procs.rs",
            "crates/sema/src/lib.rs",
            "Cargo.lock",
            "crates/driver/src/toolchain.rs",
            "crates/driver/src/library.rs",
            "crates/codegen-cpp/src/library.rs",
            "crates/dap/src/lib.rs",
            "tools/legacy_tests/run_legacy_tests.py",
            "libqb/internal/c/libqb.cpp",
            "tools/libqb/edits/d048-console-eof-input-string.edit",
            "tools/linux/sync.sh",
            "tests/corpus/slice/s127_sound_handles.linux.output",
            "verification/v01_x.linux.out.txt",
            "tests/verification/known.linux.list",
        ):
            self.assertTrue(close.needs_linux([f]), f)
        for f in (
            "STATUS.md",
            "openspec/changes/m7-linux/design.md",
            "vscode/src/extension.ts",
            "tests/corpus/a.output",
        ):
            self.assertFalse(close.needs_linux([f]), f)
        names = [s.name for s in steps(False, False, False, True)]
        self.assertEqual(names[-3:], ["linux-sync", "linux-tier1", "linux-slice"])
        self.assertNotIn("linux-sync", [s.name for s in steps(True, True, True)])
        by = {s.name: s for s in steps(False, False, False, True)}
        self.assertEqual(
            run_part(by["linux-sync"]),
            'wsl --cd "$(pwd -W)" -e bash tools/linux/sync.sh > target/close/linux-sync.log 2>&1; '
            'echo "exit: $?" >> target/close/linux-sync.log',
        )
        self.assertEqual(by["linux-sync"].judge, "exit")
        self.assertIn("cargo test --workspace --locked --no-fail-fast", by["linux-tier1"].command)
        self.assertIn("--list tests/corpus/slice.list", by["linux-slice"].command)
        # Not "list": known.linux.list names programs of slice.list that fail on Linux by design.
        self.assertEqual((by["linux-tier1"].judge, by["linux-slice"].judge), ("cargo", "runner"))
        for files, present in ((["crates/driver/src/build.rs"], True), (["STATUS.md"], False)):
            out = io.StringIO()
            with mock.patch.object(close, "changed_files", return_value=files):
                with redirect_stdout(out):
                    close.plan(False)
            self.assertEqual("tools/linux/sync.sh" in out.getvalue(), present, files)
            self.assertEqual("# no Linux runs" in out.getvalue(), not present, files)

    def test_sandbox_by_files(self):
        # The sandbox family (design D7 of desktop-sandbox): the family, its runner or the list ask for it; a crate
        # does not (--sandbox does).
        for f in (
            "tests/sandbox/screenmove_readback.bas",
            "tools/sandbox/run.py",
            "tests/never_run.list",
        ):
            self.assertTrue(close.needs_sandbox([f]), f)
        for f in (
            "crates/codegen-cpp/src/procs.rs",
            "libqb/internal/c/libqb.cpp",
            "STATUS.md",
            "tests/never_run.lists",
        ):
            self.assertFalse(close.needs_sandbox([f]), f)
        names = [s.name for s in steps(False, False, False, True, True)]
        self.assertEqual(names[-1], "sandbox")
        self.assertNotIn("sandbox", [s.name for s in steps(True, True, True, True)])
        by = {s.name: s for s in steps(False, sandbox=True)}
        self.assertEqual(
            run_part(by["sandbox"]), "python tools/sandbox/run.py > target/close/sandbox.log 2>&1"
        )
        self.assertTrue(by["sandbox"].alone and by["sandbox"].copy)
        for files, flag, present in (
            (["tools/sandbox/run.py"], False, True),
            (["STATUS.md"], False, False),
            (["STATUS.md"], True, True),
        ):
            out = io.StringIO()
            with mock.patch.object(close, "changed_files", return_value=files):
                with redirect_stdout(out):
                    close.plan(False, sandbox=flag)
            self.assertEqual("tools/sandbox/run.py" in out.getvalue(), present, files)
            self.assertEqual("# no sandbox run" in out.getvalue(), not present, files)

    def test_sandbox_judge(self):
        passed = "[compiler] x sha256 ab started 1.0 fixes off\nPASS  a: pass\ncoverage: PASS (0 problem(s))\n"
        self.assertEqual(
            judge("sandbox", passed + "sandbox: 7 pass, 0 fail\n", set()),
            (True, "7 pass, 0 fail, coverage PASS"),
        )
        self.assertFalse(judge("sandbox", passed + "sandbox: 6 pass, 1 fail\n", set())[0])
        cov_failed = "coverage: FAIL (1 problem(s))\nsandbox: 7 pass, 0 fail\n"
        self.assertEqual(
            judge("sandbox", cov_failed, set()), (False, "7 pass, 0 fail, coverage FAIL")
        )
        refused = judge(
            "sandbox",
            "sandbox: refused: a sandbox is running (WindowsSandboxRemoteSession.exe)\n",
            set(),
        )
        self.assertEqual(
            refused, (False, "refused: a sandbox is running (WindowsSandboxRemoteSession.exe)")
        )
        self.assertEqual(
            judge("sandbox", "sandbox: did not shut down within 260 s; closed\n", set()),
            (False, "no result line"),
        )

    def test_reach(self):
        # A changed file reaches the steps of its REACH entry; one of no entry, every step.
        files = [
            "tests/sandbox/a.bas",
            "crates/x.rs",
            "tests/verification/fixed/v1.out.txt",
            "tests/format/a.bas",
        ]
        self.assertEqual(close.reaching(files, "slice", None), ["crates/x.rs"])
        self.assertEqual(
            close.reaching(files, "verification", None),
            ["crates/x.rs", "tests/verification/fixed/v1.out.txt"],
        )
        self.assertEqual(
            close.reaching(files, "sandbox", None), ["tests/sandbox/a.bas", "crates/x.rs"]
        )
        # tests/format is read by a cargo test (format_oracle.rs) as well as the format runs.
        self.assertEqual(
            close.reaching(files, "tier1", None),
            ["crates/x.rs", "tests/verification/fixed/v1.out.txt", "tests/format/a.bas"],
        )
        self.assertEqual(
            close.reaching(files, "format", None), ["crates/x.rs", "tests/format/a.bas"]
        )
        # A snapshot: the tests that read it, not the build.
        snap = ["crates/driver/tests/snapshots/a.snap"]
        self.assertEqual(close.reaching(snap, "tier1", None), snap)
        self.assertEqual(close.reaching(snap, "cli", None), snap)
        self.assertEqual(close.reaching(snap, "build", None), [])
        self.assertEqual(close.reaching(snap, "slice", None), [])
        # A libqb file: the steps libqb_reach found, none when it found none.
        libqb = ["libqb/internal/c/libqb.cpp", "tools/libqb/edits/d1.edit", "crates/x.rs"]
        self.assertEqual(close.reaching(libqb, "slice", frozenset({"sandbox"})), ["crates/x.rs"])
        self.assertEqual(close.reaching(libqb, "sandbox", frozenset({"sandbox"})), libqb)
        self.assertEqual(close.reaching(libqb, "slice", close.PROGRAM_STEPS), libqb)

    def plan_of(self, files: list[str], *flags: bool, other: str | None = None) -> str:
        """The plan for `files` changed, every step's last log of HEAD unless `other` says why not."""
        out = io.StringIO()
        with (
            mock.patch.object(close, "changed_files", return_value=files),
            mock.patch.object(close, "open_tasks", return_value=[]),
            mock.patch.object(close, "head", return_value="h1"),
            mock.patch.object(close, "other_commit", return_value=other),
            redirect_stdout(out),
        ):
            close.plan(*(flags or (False,)))
        return out.getvalue()

    def test_plan_leaves_out_the_steps_no_changed_file_reaches(self):
        text = self.plan_of(["tests/sandbox/a.bas"])
        self.assertIn(
            "# slice: no changed file reaches it; not run, its last log stands (target/close/slice.log)",
            text,
        )
        self.assertIn("# tier1: no changed file reaches it", text)
        self.assertNotIn("cargo test --workspace", text.replace("# tier1", ""))
        self.assertIn(
            "\ngit rev-parse HEAD > target/close/sandbox.head && "
            "python tools/sandbox/run.py > target/close/sandbox.log 2>&1",
            text,
        )
        self.assertIn("python tools/close/close.py --check", text)

    def test_plan_runs_a_step_whose_last_log_is_of_another_commit(self):
        text = self.plan_of(["tests/sandbox/a.bas"], other="run at commit abc, not HEAD's h1")
        self.assertNotIn("no changed file reaches it; not run", text)
        self.assertIn(
            "> target/close/slice.log 2>&1    # no changed file reaches it, but its last log is not of HEAD: "
            "run at commit abc",
            text,
        )

    def test_plan_of_a_linux_tool(self):
        # 2026-10-31: tools/linux asked for the Linux family but was no input, so the plan ran none of it.
        text = self.plan_of(["tools/linux/sync.sh"])
        for name in ("linux-sync", "linux-tier1", "linux-slice"):
            self.assertIn(f"> target/close/{name}.log 2>&1", text.replace(f"({close.log(name)})", ""))
            self.assertNotIn(f"# {name}: no changed file", text)

    def test_a_step_needed_before_another_is_pulled_in(self):
        # A libqb edit reaches linux-slice, which runs on WSL's copy: the sync comes with it (2026-10-31).
        text = self.plan_of(["libqb/internal/c/libqb.cpp"])
        self.assertNotIn("# linux-sync: no changed file", text)
        self.assertIn('echo "exit: $?" >> target/close/linux-sync.log    # for linux-slice,', text)
        self.assertIn("# linux-tier1: no changed file", text)
        # A snapshot reaches the lists step, written before the build: the build comes with it. And linux-tier1, with
        # the sync.
        text = self.plan_of(["crates/driver/tests/snapshots/a.snap"])
        self.assertIn("cargo build --release > target/close/build.log 2>&1    # for lists,", text)
        self.assertIn("target/close/linux-sync.log    # for linux-tier1,", text)
        # A file that reaches the build itself: no note.
        text = self.plan_of(["crates/a.rs"])
        self.assertNotIn("# for", text)
        self.assertNotIn("    # for lists", text)

    def test_plan_names_the_format_runs(self):
        for files, present in (
            (["crates/format/src/lib.rs"], True),
            (["crates/sema/src/lib.rs"], False),
        ):
            out = io.StringIO()
            with mock.patch.object(close, "changed_files", return_value=files):
                with redirect_stdout(out):
                    close.plan(False)
            self.assertEqual("--suite format-corpus" in out.getvalue(), present, files)
            self.assertEqual("# no format runs" in out.getvalue(), not present, files)

    def test_order(self):
        names = [s.name for s in steps(True)]
        # The lists come before the build, the build before every run.
        self.assertLess(names.index("lists"), names.index("build"))
        # A thin close: the reports wait for a merge into main (DECISIONS.md 2026-10-28).
        for name in ("wording", "wording-check", "demand-corpus", "demand-upstream", "counts"):
            self.assertNotIn(name, [s.name for s in steps(True, True, True)])
        self.assertEqual(names[names.index("build") + 1], "tier1")
        self.assertIn("nofold", names)
        self.assertNotIn("nofold", [s.name for s in steps(False)])

    def test_commands(self):
        by = {s.name: s.command for s in steps(True)}
        self.assertIn("--no-fail-fast", by["tier1"])
        self.assertIn("--locked", by["tier1"])
        self.assertIn("--pass-cache", by["slice"])
        self.assertIn("--pass-cache", by["upstream"])
        self.assertIn("--pass-cache", by["verification"])
        self.assertIn("--suite verification", by["verification"])
        self.assertIn("--list tests/verification/pass.list", by["verification"])
        self.assertIn("--suite addprefix", by["addprefix"])
        self.assertIn("--wait", by["addprefix"])
        self.assertIn("--suite qbasic", by["qbasic"])
        self.assertIn("--jobs 8", by["qbasic"])
        self.assertIn("--wait", by["qbasic"])
        # Builds are deterministic, so a program whose C++ folding does not change is the same executable, and its
        # pass stands (DECISIONS.md 2026-10-27).
        self.assertIn("--pass-cache", by["nofold"])
        self.assertIn("--wait", by["callsite"])
        for s in steps(True):
            # Each command writes its own log, and no command holds a backslash (CLAUDE.md working rule 5).
            self.assertIn(f"> target/close/{s.name}.log 2>&1", s.command)
            self.assertNotIn("\\", s.command)
            # And first records the commit it runs at.
            self.assertTrue(
                s.command.startswith(f"git rev-parse HEAD > target/close/{s.name}.head && "),
                s.command,
            )

    def test_plan_prints_every_step(self):
        out = io.StringIO()
        with mock.patch.object(
            close, "changed_files", return_value=["crates/sema/src/consteval.rs"]
        ):
            with redirect_stdout(out):
                close.plan(False)
        text = out.getvalue()
        for s in steps(True):
            self.assertIn(s.command, text)

    def test_the_toolchain_pin_reaches_the_rust_steps(self):
        # 2026-10-30: a move of the pin in rust-toolchain.toml reached no step; then every step, and a close ran every
        # program list for 1.99.0 to 1.99.1 (the panel, 2026-10-31: the Rust steps only).
        files = ["rust-toolchain.toml"]
        self.assertTrue(close.is_input("rust-toolchain.toml"))
        for step in (
            "lists",
            "build",
            "tier1",
            "fmt",
            "clippy",
            "cli",
            "linux-sync",
            "linux-tier1",
        ):
            self.assertEqual(close.reaching(files, step, None), files, step)
        for step in (
            "slice",
            "upstream",
            "verification",
            "addprefix",
            "qbasic",
            "callsite",
            "linux-slice",
        ):
            self.assertEqual(close.reaching(files, step, None), [], step)
        self.assertTrue(close.needs_linux(files))

    def test_merge_plan(self):
        out = io.StringIO()
        with redirect_stdout(out):
            close.plan_merge()
        text = out.getvalue()
        names = [s.name for s in close.merge_steps()]
        for s in close.merge_steps():
            self.assertIn(s.command, text)
            # Its own log, beside a close's logs and not over them; no pass cache; no backslash (working rule 5).
            self.assertIn(f"> target/close/merge-{s.name}.log 2>&1", s.command)
            self.assertNotIn("--pass-cache", s.command)
            self.assertNotIn("\\", s.command)
        self.assertEqual(len(names), len(set(names)))
        # The wording table before the build that builds it in, and a build before it to compile with.
        self.assertLess(text.index("cargo build --release    #"), text.index("old_wording.py >"))
        self.assertLess(names.index("wording"), names.index("build"))
        self.assertEqual(names[names.index("build") + 1], "tier1")
        # The full suites, then what reads their results: the pass lists, the built-ins' check, the counts.
        last_suite = max(names.index(n) for n in ("corpus", "upstream", "verification"))
        for after in ("corpus-list", "upstream-list", "verification-list", "builtins", "counts"):
            self.assertLess(last_suite, names.index(after), after)
        # The lists of the corpus suite keep the full run's results apart.
        by = {s.name: s.command for s in close.merge_steps()}
        for name in ("nofold", "differential"):
            self.assertIn("--results target/legacy-tests-", by[name])
        self.assertIn("--coverage target/coverage/merge/corpus.txt", by["corpus"])
        self.assertIn("rm -rf target/coverage/merge", text)
        self.assertEqual(names[-1], "sandbox")
        for family in (
            "linux-corpus",
            "linux-upstream",
            "linux-verification",
            "callsite",
            "coverage",
            "cli",
        ):
            self.assertIn(family, names)
        self.assertTrue(text.rstrip().endswith("python tools/close/close.py --check --merge"))
        # Each Linux suite builds first, so that none runs beside another's build on an older one.
        for name in ("linux-corpus", "linux-upstream", "linux-verification"):
            self.assertIn("cargo build --release --locked && ", by[name], name)
        self.assertIn("beside linux-upstream and linux-verification", text)
        # The steps with no result of their own are judged by their exit code; counts by its blocks.
        judges = {s.name: s.judge for s in close.merge_steps()}
        self.assertEqual(judges["counts"], "counts")
        for name in ("wording", "demand-corpus", "demand-upstream", "linux-sync"):
            s = next(s for s in close.merge_steps() if s.name == name)
            self.assertEqual(s.judge, "exit", name)
            self.assertTrue(
                s.command.endswith(f'; echo "exit: $?" >> target/close/merge-{name}.log'), name
            )

    def test_merge_takes_no_family_option(self):
        with (
            redirect_stdout(io.StringIO()),
            mock.patch("sys.stderr", io.StringIO()),
            self.assertRaises(SystemExit),
        ):
            close.main(["--plan", "--merge", "--linux"])

    def test_inputs(self):
        self.assertTrue(close.INPUTS.match("crates/sema/src/lib.rs"))
        self.assertTrue(close.INPUTS.match("tests/corpus/slice.list"))
        self.assertTrue(close.NOT_INPUTS.search("tests/upstream/README.md"))
        self.assertTrue(close.NOT_INPUTS.search("tools/testtime/runs.csv"))
        self.assertTrue(close.NOT_INPUTS.search("crates/sema/tests/.symbols.rs.pending-snap"))
        self.assertTrue(close.NOT_INPUTS.search("crates/sema/tests/snapshots/symbols__a.snap.new"))
        self.assertFalse(close.NOT_INPUTS.search("crates/sema/tests/snapshots/symbols__a.snap"))
        self.assertFalse(close.INPUTS.match("STATUS.md"))
        # Every tool folder a family runs from (2026-10-31: tools/linux was not, so its edits reached no step).
        self.assertTrue(close.is_input("tools/linux/sync.sh"))


class Check(unittest.TestCase):
    """`check` over a temporary repo: logs written at chosen times against a build and a changed source."""

    def setUp(self):
        import tempfile

        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "target/release").mkdir(parents=True)
        (self.root / "target/close").mkdir(parents=True)
        (self.root / "crates").mkdir()
        self.exe = self.root / close.EXE
        self.exe.write_text("exe")
        self.source = self.root / "crates/a.rs"
        self.source.write_text("fn a() {}")
        self.at(self.source, 100)
        self.at(self.exe, 200)
        good = {
            "build": "    Finished\n",
            "tier1": CARGO_OK,
            "fmt": "",
            "clippy": "    Finished `dev` profile\n",
            "lists": CARGO_OK,
            "slice": '{"corpus": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
            "upstream": '{"compile_tests": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
            "verification": '{"verification": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
            "addprefix": '{"addprefix": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
            "qbasic": '{"qbasic_testcases": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
            "callsite": "1 programs: 1 equal, 0 differ, 0 not compared\n",
            "cli": CARGO_OK,
            "linux-sync": "synced 1 files into ~/qb64rust\nexit: 0\n",
            "linux-tier1": CARGO_OK,
            "linux-slice": '{"corpus": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
        }
        # A change to a crate asks for the Linux family too (LINUX_FILES).
        for s in steps(False, linux=True):
            path = self.root / close.log(s.name)
            path.write_text(good.get(s.name, ""))
            self.at(path, 150 if s.before_build and s.name != "build" else 300)
        # Every step's last run of HEAD's commit (`head_record`).
        for s in steps(True, True, True, True, True):
            (self.root / close.head_record(s.name)).write_text("h1\n")
        self.patches = [
            mock.patch.object(close, "REPO", self.root),
            mock.patch.object(close, "changed_files", return_value=["crates/a.rs"]),
            mock.patch.object(close, "head", return_value="h1"),
        ]
        for p in self.patches:
            p.start()

    def tearDown(self):
        for p in self.patches:
            p.stop()
        self.tmp.cleanup()

    @staticmethod
    def at(path: Path, t: float):
        import os

        os.utime(path, (t, t))

    def run_check(self, **kw) -> tuple[int, str]:
        out = io.StringIO()
        with redirect_stdout(out):
            code = close.check(False, set(), **kw)
        return code, out.getvalue()

    def test_all_good(self):
        code, text = self.run_check()
        self.assertEqual(code, 0, text)

    def test_missing(self):
        (self.root / close.log("callsite")).unlink()
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("MISSING  callsite", text)

    def test_stale_after_an_edit_and_not_repeated(self):
        self.at(self.source, 400)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    slice: older than crates/a.rs", text)
        reasons = {s.name: "a test-only edit" for s in steps(False, linux=True)}
        code, text = self.run_check(not_repeated=reasons)
        self.assertEqual(code, 0, text)
        # The reason once, with the steps it covers; each step points to it.
        self.assertEqual(text.count("a test-only edit"), 1, text)
        self.assertIn("(1) not repeated (lists, build, tier1,", text)
        self.assertIn("ok       slice: 1 pass, 0 fail, 0 known [", text)
        self.assertIn("; not repeated (1)", text)

    def test_a_change_reaches_only_some_steps(self):
        # 2026-10-30: an edit of tests/sandbox made every list stale, and a close ran them all. It reaches the
        # sandbox family alone; the other steps keep their logs.
        prog = self.root / "tests/sandbox/a.bas"
        prog.parent.mkdir(parents=True)
        prog.write_text("PRINT\n")
        self.at(prog, 400)
        with mock.patch.object(close, "changed_files", return_value=["tests/sandbox/a.bas"]):
            code, text = self.run_check()
            self.assertEqual(code, 1, text)
            self.assertNotIn("STALE", text)
            self.assertIn("MISSING  sandbox", text)
            self.assertIn("ok       slice: 1 pass", text)
            sandbox = self.root / close.log("sandbox")
            sandbox.write_text("coverage: PASS (0 problem(s))\nsandbox: 7 pass, 0 fail\n")
            self.at(sandbox, 500)
            code, text = self.run_check()
            self.assertEqual(code, 0, text)
            # A later edit of the family makes only that step stale.
            self.at(prog, 600)
            code, text = self.run_check()
            self.assertEqual(code, 1)
            self.assertIn("STALE    sandbox: older than tests/sandbox/a.bas", text)
            self.assertIn("ok       verification", text)

    def libqb_edit(self, text: str, t: float = 400) -> list[str]:
        """A changed libqb edit and the copy's file it changes, both at `t`; the changed files."""
        edit = self.root / "tools/libqb/edits/d1.edit"
        copy = self.root / "libqb/internal/c/libqb.cpp"
        for p in (edit, copy):
            p.parent.mkdir(parents=True, exist_ok=True)
        edit.write_text(text)
        copy.write_text("int x;\n")
        self.at(edit, t)
        self.at(copy, t)
        (self.root / "tests").mkdir(exist_ok=True)
        (self.root / close.NEVER_RUN).write_text(
            "# c\n_SCREENCLICK\tsandbox clicks\nLPRINT\tprinter\n"
        )
        return ["tools/libqb/edits/d1.edit", "libqb/internal/c/libqb.cpp"]

    def test_a_libqb_edit_naming_a_sandbox_builtin_reaches_the_sandbox_alone(self):
        # D-056: an edit of _SCREENCLICK, which no list runs. Only the sandbox family is asked for.
        files = self.libqb_edit(
            "# why\nfile: internal/c/libqb.cpp\nbuiltins: _SCREENCLICK\n--- old\nx\n--- new\ny\n--- end\n"
        )
        self.assertEqual(close.libqb_reach(files), {"sandbox"})
        self.assertTrue(close.needs_sandbox(files))
        with mock.patch.object(close, "changed_files", return_value=files):
            code, text = self.run_check()
            self.assertNotIn("STALE", text)
            self.assertIn("MISSING  sandbox", text)
            self.assertIn("ok       linux-slice", text)
        # One of a reason with no family: the command-line tests that build, so the edit is compiled.
        files = self.libqb_edit(
            "file: internal/c/libqb.cpp\nbuiltins: LPRINT _SCREENCLICK\n--- old\nx\n--- new\ny\n--- end\n"
        )
        self.assertEqual(close.libqb_reach(files), {"sandbox", "cli"})

    def test_a_libqb_edit_naming_no_builtin_reaches_every_run_that_builds_programs(self):
        files = self.libqb_edit("file: internal/c/libqb.cpp\n--- old\nx\n--- new\ny\n--- end\n")
        self.assertEqual(close.libqb_reach(files), close.PROGRAM_STEPS)
        self.assertFalse(close.needs_sandbox(files))
        with mock.patch.object(close, "changed_files", return_value=files):
            code, text = self.run_check()
            self.assertEqual(code, 1)
            self.assertIn("STALE    slice: older than tools/libqb/edits/d1.edit", text)
            self.assertIn("STALE    cli:", text)
            self.assertIn("ok       tier1", text)
            self.assertIn("ok       callsite", text)
        # A built-in that a list runs, or a name not on the list: the same.
        files = self.libqb_edit(
            "file: internal/c/libqb.cpp\nbuiltins: PRINT\n--- old\nx\n--- new\ny\n--- end\n"
        )
        self.assertEqual(close.libqb_reach(files), close.PROGRAM_STEPS)
        # The copy changed with no edit (a copy from another commit): the same.
        self.assertEqual(close.libqb_reach(["libqb/internal/c/libqb.cpp"]), close.PROGRAM_STEPS)
        self.assertIsNone(close.libqb_reach(["crates/a.rs", "libqb/SOURCE.md"]))

    def test_a_run_stopped_before_its_result_is_accepted_only_with_a_reason(self):
        verification = self.root / close.log("verification")
        verification.write_text("[1/9] PASS verification:a (output, 1.0s)\n")
        self.at(verification, 300)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("FAIL     verification: no result line", text)
        code, text = self.run_check(
            not_repeated={"verification": "stopped after a libqb edit of _SCREENCLICK"}
        )
        self.assertEqual(code, 0, text)
        self.assertIn("ok       verification: no result line [", text)
        self.assertIn("; stopped, not repeated (1)", text)
        # A log that shows a failure is never accepted so.
        verification.write_text('{"verification": {"PASS": 0, "FAIL": 1, "KFAIL": 0}}\n')
        self.at(verification, 300)
        code, text = self.run_check(not_repeated={"verification": "stopped"})
        self.assertEqual(code, 1)
        self.assertIn("FAIL     verification", text)

    def test_a_run_after_an_edit_but_before_the_build(self):
        # Built at 200, the source edited at 250, the runs at 300, never built again.
        self.at(self.root / close.log("build"), 200)
        self.at(self.source, 250)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    build: target/release/qb64rust.exe is older than crates/a.rs", text)

    def test_a_build_log_newer_than_its_executable(self):
        # The log written after the edit, the executable not (a build that stopped, another profile).
        self.at(self.source, 250)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    build: target/release/qb64rust.exe is older than crates/a.rs", text)

    def test_a_new_corpus_program_on_no_list(self):
        corpus = self.root / "tests/corpus"
        (corpus / "slice").mkdir(parents=True)
        (corpus / "slice.list").write_text("# a comment\nslice/s1_listed\n")
        for name in ("s1_listed", "s2_new", "s3_rejected"):
            (corpus / f"slice/{name}.bas").write_text("PRINT 1\n")
        (corpus / "slice/s3_rejected.err").write_text("error\n")
        (corpus / "slice/s1_listed.files").mkdir()
        (corpus / "slice/s1_listed.files/inc.bas").write_text("PRINT 2\n")
        changed = [f"tests/corpus/slice/{n}.bas" for n in ("s1_listed", "s2_new", "s3_rejected")]
        changed.append("tests/corpus/slice/s1_listed.files/inc.bas")
        with mock.patch.object(close, "changed_files", return_value=changed):
            code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("UNLISTED tests/corpus/slice/s2_new.bas", text)
        self.assertEqual(text.count("UNLISTED"), 1, text)

    def test_a_failure_is_never_excused_as_not_repeated(self):
        self.at(self.source, 400)
        (self.root / close.log("cli")).write_text(CARGO_FAIL)
        self.at(self.root / close.log("cli"), 300)
        code, text = self.run_check(not_repeated={"cli": "x"})
        self.assertEqual(code, 1)
        self.assertIn("STALE    cli", text)

    def test_a_pass_cache_that_ran_most_of_a_long_list(self):
        verification = self.root / close.log("verification")

        def write(skipped: int, total: int):
            verification.write_text(
                '{"verification": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n'
                f"--pass-cache: {skipped} of {total} programs not run (passed before)\n"
            )
            self.at(verification, 300)

        # Most of the list not run: the cache did its work.
        write(4100, 4121)
        code, text = self.run_check()
        self.assertEqual(code, 0, text)
        # All of a long list run (2026-10-30: a support folder in every program's key).
        write(0, 4121)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("CACHE    verification: the pass cache let 4121 of 4121 programs run", text)
        self.assertIn("--cache-miss verification=REASON", text)
        # Accepted with a reason, which is shown.
        code, text = self.run_check(cache_miss={"verification": "a cold cache"})
        self.assertEqual(code, 0, text)
        self.assertIn("ok       verification: 1 pass, 0 fail, 0 known [", text)
        self.assertIn("4121 of 4121 programs run: a cold cache", text)
        # A short list run whole, and exactly half not run, are not named.
        write(0, close.CACHE_LIST - 1)
        self.assertEqual(self.run_check()[0], 0)
        write(50, 100)
        self.assertEqual(self.run_check()[0], 0)
        # Not repeated after a later edit does not hide it.
        write(0, 4121)
        self.at(self.source, 400)
        everything = {s.name: "a test-only edit" for s in steps(False, linux=True)}
        code, text = self.run_check(not_repeated=everything)
        self.assertEqual(code, 1)
        self.assertIn("CACHE    verification", text)
        code, text = self.run_check(
            not_repeated=everything, cache_miss={"verification": "a cold cache"}
        )
        self.assertEqual(code, 0, text)
        self.assertIn("programs run: a cold cache; not repeated (1)", text)

    def test_programs_without_an_executable_count_in_no_cache_number(self):
        # 2026-10-30: 2,470 of the 2,471 programs a verification run ran expect a compile error, which the cache
        # never skips; one program with output ran.
        verification = self.root / close.log("verification")

        def write(skipped: int, output_run: int, errors: int):
            lines = [f"[{i}/9] PASS verification:e{i} (error, 0.1s)" for i in range(errors)]
            lines += [
                f"[{i}/9] PASS verification:c{i} [fixes on] (compile-only, 0.2s)" for i in range(2)
            ]
            lines += [f"[{i}/9] PASS verification:o{i} (output, 1.0s)" for i in range(output_run)]
            lines += [
                f"[{i}/9] PASS verification:n{i} (output, 1.0s)  not run: passed before with this executable"
                for i in range(skipped)
            ]
            total = skipped + output_run + errors + 2
            lines.append(f"--pass-cache: {skipped} of {total} programs not run (passed before)")
            lines.append('{"verification": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}')
            verification.write_text("\n".join(lines) + "\n")
            self.at(verification, 300)

        write(1652, 1, 2470)
        code, text = self.run_check()
        self.assertEqual(code, 0, text)
        # Most of the programs it can skip run: named, with those it cannot.
        write(10, 90, 300)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn(
            "the pass cache let 90 of 100 programs run (302 more without an executable)", text
        )

    def test_a_failing_run_is_named_as_failing_not_by_its_cache(self):
        verification = self.root / close.log("verification")
        verification.write_text(
            '{"verification": {"PASS": 1, "FAIL": 2, "KFAIL": 0}}\n--pass-cache: 0 of 4121 programs not run\n'
        )
        self.at(verification, 300)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("FAIL     verification", text)
        self.assertNotIn("CACHE", text)

    def test_cache_miss_names_a_step(self):
        with redirect_stdout(io.StringIO()), mock.patch("sys.stderr", io.StringIO()):
            with self.assertRaises(SystemExit):
                close.main(["--check", "--cache-miss", "nosuchstep=x"])
            with self.assertRaises(SystemExit):
                close.main(["--check", "--cache-miss", "verification"])

    def test_a_step_before_the_build_written_after_it(self):
        self.at(self.root / close.log("lists"), 250)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    lists: after the build", text)

    def test_a_step_before_a_build_that_compiled_nothing(self):
        # Built at 200; `lists` at 250; the build step at 260 found nothing to compile and left the executable at 200.
        lists, build = self.root / close.log("lists"), self.root / close.log("build")
        self.at(lists, 250)
        build.write_text("    Finished `release` profile [optimized] target(s) in 0.33s\n")
        self.at(build, 260)
        code, text = self.run_check()
        self.assertEqual(code, 0, text)
        # A build that compiled, or one before the step, still leaves it stale.
        build.write_text("   Compiling qb64rust v0.1.0\n    Finished `release` profile\n")
        self.at(build, 260)
        self.assertIn("STALE    lists: after the build", self.run_check()[1])
        build.write_text("    Finished `release` profile\n")
        self.at(build, 240)
        self.assertIn("STALE    lists: after the build", self.run_check()[1])

    def test_a_step_before_the_build_left_by_an_earlier_change(self):
        # Before the build, but older than this change's edit.
        self.at(self.root / close.log("lists"), 50)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    lists: older than crates/a.rs: run it again, then build", text)

    def copy_log(self, name: str, digest: str, started: float, end: float = 300):
        """The log of a run of a copy (compiler_copy.header) that started at `started` and ended at `end`."""
        path = self.root / close.log(name)
        path.write_text(
            f"[compiler] target/release/qb64rust.exe sha256 {digest} started {started:.3f}\n"
            '{"corpus": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n'
        )
        self.at(path, end)

    def test_a_run_of_a_copy_of_the_current_build(self):
        # Built again with the same bytes after the run ended: the run still counts, its log being older.
        self.copy_log("slice", close.sha256_of(self.exe), 250, end=260)
        self.at(self.exe, 280)
        code, text = self.run_check()
        self.assertIn("ok       slice:", text)
        self.assertNotIn("STALE    slice", text)

    def test_a_run_of_a_copy_of_another_build(self):
        # Started on an earlier build and ended after the current one was built (study/archive/34 2.1).
        self.copy_log("slice", "ab" * 32, 150, end=300)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    slice: of build abababababab, not the current", text)

    def test_a_run_of_a_copy_and_an_input_edited_while_it_ran(self):
        # A test input changed after the run started reaches the run by no build: stale, though the log is newer.
        listed = self.root / "tests/corpus/slice.list"
        listed.parent.mkdir(parents=True)
        listed.write_text("x\n")
        self.at(listed, 270)
        self.copy_log("slice", close.sha256_of(self.exe), 250, end=300)
        with mock.patch.object(
            close, "changed_files", return_value=["crates/a.rs", "tests/corpus/slice.list"]
        ):
            code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    slice: started before tests/corpus/slice.list", text)

    def test_a_run_of_a_copy_is_not_stale_by_a_source_edit_alone(self):
        # A source edited while the run went on does not make the run stale by itself: the build does, if it changes.
        self.copy_log("slice", close.sha256_of(self.exe), 250, end=300)
        self.at(self.source, 270)
        code, text = self.run_check()
        self.assertIn("ok       slice:", text)
        self.assertIn("STALE    build", text)

    def test_a_run_of_another_build_that_no_changed_file_reaches(self):
        # A move of the toolchain's pin builds the compiler again but reaches no program list: the slice list of the
        # build before it stands, where a change to a crate makes it stale.
        self.copy_log("slice", "ab" * 32, 150, end=300)
        pin = self.root / "rust-toolchain.toml"
        pin.write_text('[toolchain]\nchannel = "1.99.1"\n')
        self.at(pin, 100)
        with mock.patch.object(close, "changed_files", return_value=["rust-toolchain.toml"]):
            code, text = self.run_check()
        self.assertIn("ok       slice:", text)
        code, text = self.run_check()
        self.assertIn("STALE    slice: of build abababababab", text)

    def test_copy_steps(self):
        copies = {s.name for s in steps(True) if s.copy}
        self.assertEqual(
            copies,
            {"slice", "upstream", "verification", "addprefix", "qbasic", "nofold", "callsite"},
        )

    def test_a_step_before_the_build_is_not_stale_by_what_one_writes(self):
        # `lists` rewrote a list at 160 and ended at 170, then the build at 200.
        listed = self.root / "tests/known_false_errors.list"
        listed.parent.mkdir()
        listed.write_text("x\n")
        self.at(listed, 160)
        self.at(self.root / close.log("lists"), 170)
        with mock.patch.object(
            close, "changed_files", return_value=["crates/a.rs", "tests/known_false_errors.list"]
        ):
            code, text = self.run_check()
        self.assertEqual(code, 0, text)

    def test_a_snapshot_pulls_in_the_build(self):
        # 2026-10-31: a snapshot reached the lists step and not the build, so the lists' log, newer than the
        # executable, was stale with no build in the plan to make it current.
        snap = self.root / "crates/sema/tests/snapshots/a.snap"
        snap.parent.mkdir(parents=True)
        snap.write_text("x\n")
        self.at(snap, 400)
        for name in ("tier1", "cli", "linux-sync", "linux-tier1"):
            self.at(self.root / close.log(name), 470)
        self.at(self.root / close.log("lists"), 450)
        with mock.patch.object(close, "changed_files", return_value=[snap.relative_to(self.root).as_posix()]):
            code, text = self.run_check()
            self.assertEqual(code, 1)
            self.assertIn("STALE    lists: after the build", text)
            self.assertIn("STALE    build: older than crates/sema/tests/snapshots/a.snap", text)
            # The build the plan now prints, which compiles nothing: the executable stays older than the snapshot.
            build = self.root / close.log("build")
            build.write_text("    Finished `release` profile [optimized] target(s) in 0.33s\n")
            self.at(build, 460)
            code, text = self.run_check()
            self.assertEqual(code, 0, text)

    def test_a_family_option_asks_for_a_run_after_the_change(self):
        # --sandbox for a libqb edit that names no built-in (which reaches no sandbox run by itself): the sandbox
        # log from before the edit was accepted (2026-10-31).
        files = self.libqb_edit("file: internal/c/libqb.cpp\n--- old\nx\n--- new\ny\n--- end\n")
        sandbox = self.root / close.log("sandbox")
        sandbox.write_text("coverage: PASS (0 problem(s))\nsandbox: 7 pass, 0 fail\n")
        self.at(sandbox, 300)
        with mock.patch.object(close, "changed_files", return_value=files):
            code, text = self.run_check(sandbox=True)
            self.assertIn("STALE    sandbox: older than", text)
            self.at(sandbox, 500)
            code, text = self.run_check(sandbox=True)
            self.assertIn("ok       sandbox", text)

    def test_a_log_of_another_commit(self):
        # A step no changed file reaches keeps its log only when that log is of HEAD's commit (2026-10-31: after a
        # change of branch, or a commit made without a close, the logs are of another tree).
        with mock.patch.object(close, "changed_files", return_value=[]):
            self.assertEqual(self.run_check()[0], 0)
            (self.root / close.head_record("slice")).write_text("h0\n")
            (self.root / close.head_record("tier1")).unlink()
            code, text = self.run_check()
            self.assertEqual(code, 1)
            self.assertIn("STALE    slice: run at commit h0, not HEAD's h1", text)
            self.assertIn("STALE    tier1: no record of the commit it ran at (target/close/tier1.head)", text)
            code, text = self.run_check(not_repeated={"slice": "x", "tier1": "x"})
            self.assertEqual(code, 0, text)


class CheckMerge(unittest.TestCase):
    """`check --merge` over a temporary repo: every step's merge log, after the build and after HEAD's commit."""

    # A passing log of each kind of judge.
    GOOD: ClassVar[dict[str, str]] = {
        "build": "    Finished\n",
        "cargo": CARGO_OK,
        "empty": "",
        "clippy": "    Finished `dev` profile\n",
        "runner": '{"corpus": {"PASS": 1, "FAIL": 0, "KFAIL": 2}}\n',
        "list": '{"corpus": {"PASS": 1, "FAIL": 0, "KFAIL": 0}}\n',
        "checklist": "slice.list: 1 of 1 passed in results-corpus.json\n",
        "wording": "1 notes on recorded rejections, 0 not the recorded message\n",
        "coverage": "check: PASS (0 problem(s))\n",
        "callsite": "1 programs: 1 equal, 0 differ, 0 not compared\n",
        "sandbox": "sandbox: 7 pass, 0 fail\ncoverage: PASS\n",
        "exit": "exit: 0\n",
        "counts": "8 blocks, 3 rewritten\n",
    }

    def setUp(self):
        import tempfile

        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "target/release").mkdir(parents=True)
        (self.root / "target/close").mkdir(parents=True)
        self.exe = self.root / close.EXE
        self.exe.write_text("exe")
        Check.at(self.exe, 200)
        for s in close.merge_steps():
            path = self.root / s.log
            path.write_text(self.GOOD[s.judge])
            Check.at(path, 150 if s.before_build and s.name != "build" else 300)
        # The merge commit, with nothing changed since (the usual case).
        self.patches = [
            mock.patch.object(close, "REPO", self.root),
            mock.patch.object(close, "changed_files", return_value=[]),
            mock.patch.object(close, "head_time", return_value=100.0),
        ]
        for p in self.patches:
            p.start()

    def tearDown(self):
        for p in self.patches:
            p.stop()
        self.tmp.cleanup()

    def run_check(self) -> tuple[int, str]:
        out = io.StringIO()
        with redirect_stdout(out):
            code = close.check(False, set(), merge=True)
        return code, out.getvalue()

    def test_all_good(self):
        code, text = self.run_check()
        self.assertEqual(code, 0, text)
        # A full suite's known failures are no failure there.
        self.assertIn("ok       corpus: 1 pass, 0 fail, 2 known", text)

    def test_a_close_log_does_not_stand_for_a_merge_log(self):
        (self.root / close.log("callsite", True)).unlink()
        (self.root / close.log("callsite")).write_text(self.GOOD["callsite"])
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("MISSING  callsite: target/close/merge-callsite.log not written", text)

    def test_logs_of_an_earlier_merge(self):
        # Nothing changed and the build is the same, but the logs are older than HEAD's commit: every step is
        # stale, which a close (no changed file reaches a step) would not say.
        with mock.patch.object(close, "head_time", return_value=400.0):
            code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    tier1: written before HEAD's commit", text)
        self.assertIn("STALE    sandbox: written before HEAD's commit", text)

    def test_a_run_older_than_the_build(self):
        Check.at(self.root / close.log("tier1", True), 180)
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("STALE    tier1: older than target/release/qb64rust.exe", text)

    def test_a_failing_pass_list(self):
        (self.root / close.log("verification-list", True)).write_text(
            "pass.list: 3 of 4 passed in results-verification.json\n"
        )
        code, text = self.run_check()
        self.assertEqual(code, 1)
        self.assertIn("FAIL     verification-list", text)


class NotBeforeDone(unittest.TestCase):
    """A close with open tasks is refused (study 41, 2026-10-31); the merge set is not a change's close."""

    def test_plan_and_check_refuse_open_tasks(self):
        tasks = [("m8-x", "2.1 open one")]
        with mock.patch.object(close, "open_tasks", return_value=tasks):
            out = io.StringIO()
            with redirect_stdout(out):
                self.assertEqual(close.plan(False), 1)
            self.assertIn("not at the close: change m8-x has 1 open task(s)", out.getvalue())
            out = io.StringIO()
            with redirect_stdout(out):
                self.assertEqual(close.main(["--check"]), 1)
            self.assertIn("not at the close", out.getvalue())

    def test_plan_starts_with_clean_when_done(self):
        with (
            mock.patch.object(close, "open_tasks", return_value=[]),
            mock.patch.object(close, "changed_files", return_value=["STATUS.md"]),
        ):
            out = io.StringIO()
            with redirect_stdout(out):
                close.plan(False)
        lines = [l for l in out.getvalue().splitlines() if not l.startswith("#")]
        self.assertTrue(lines[1].startswith("python tools/clean/clean.py"), lines[:3])


if __name__ == "__main__":
    unittest.main()
