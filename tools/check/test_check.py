#!/usr/bin/env python3
"""Tests of tools/check: the globs, the fingerprints, what stands, and the cost of a check (spec
testing/check-workflow, "The cost of a check is held by tests").

    python -m unittest discover -s tools/check -p "test_*.py"

The cost tests plan in a fake repository (a git repository in a temporary folder holding one file for each kind of
input), so they assert the plan of the real steps (steps.py) without running anything.
"""

from __future__ import annotations

import contextlib
import dataclasses
import io
import itertools
import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import check  # noqa: E402
import steps  # noqa: E402
from steps import FULL, MERGE, QUICK  # noqa: E402

REPO = HERE.parents[1]

TREE = {
    "Cargo.toml": "[workspace]\n",
    "Cargo.lock": "",
    "rust-toolchain.toml": "",
    "clippy.toml": "",
    "rustfmt.toml": "",
    ".config/nextest.toml": "",
    "crates/sema/Cargo.toml": 'name = "qb64rust-sema"\n',
    "crates/sema/src/lib.rs": "fn a() {}\n",
    "crates/driver/tests/it/inputs.rs": "",
    "libqb/internal/c/qbx.cpp": "int x;\n",
    "tools/libqb/copy_libqb.py": "",
    "tools/libqb/edits/d1-sound.edit": "file: internal/c/libqb.cpp\nbuiltins: SOUND\n",
    "tools/legacy_tests/run_legacy_tests.py": "",
    "tools/check/check.py": "",
    "tests/corpus/slice.list": "g/p1\ng/p2\n",
    "tests/corpus/g/p1.bas": "PRINT 1\n",
    "tests/corpus/g/p1.output": "1\n",
    "tests/corpus/g/p2.bas": "SOUND 1,1\n",
    "tests/corpus/g/p2.output": "\n",
    "tests/frontend/a.bas": "",
    "tests/upstream/pass.list": "c/u1\n",
    "tests/upstream/compile_tests/c/u1.bas": "SOUND 1,1\n",
    "tests/upstream/root/x.bi": "",
    "tests/verification/pass.list": "v1\nv3\n",
    "tests/verification/known.list": "v9\tD-001\n",
    "verification/v1.bas": "PRINT 1\n",
    "verification/v1.out.txt": " 1\n",
    "verification/v3.bas": "SOUND 1,1\n",
    "verification/v3.out.txt": "\n",
    # A support item of the v1 family's programs (the runner's verification_support), no program of its own.
    "verification/v1_inc.bi": "x = 1\n",
    "docs/STATUS.md": "status\n",
}

# The coverage records of a merge: SOUND is held by p2, u1 and v3.
COUNTS = "".join(
    f"coverage 2\nfile\t0\t0\t{name}\n0\t1\t1\t1\t{builtin}\nend\n"
    for name, builtin in (
        ("p1.bas", "PRINT"),
        ("p2.bas", "SOUND"),
        ("u1.bas", "SOUND"),
        ("v1.bas", "PRINT"),
        ("v3.bas", "SOUND"),
    )
)


def git(root: Path, *args: str) -> None:
    subprocess.run(["git", *args], cwd=root, check=True, capture_output=True)


class FakeRepo:
    def __init__(self):
        self.tmp = tempfile.TemporaryDirectory()
        # A folder of its own inside the temporary one, so that no reference clone is found beside it.
        self.root = Path(self.tmp.name) / "repo"
        self.root.mkdir()
        for rel, text in TREE.items():
            self.write(rel, text)
        self.write(".gitignore", "target/\n")
        git(self.root, "init", "-q")

    def write(self, rel: str, text: str) -> None:
        p = self.root / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")

    def pass_all(self) -> None:
        """Every step passed on the tree as it is (the last passing check)."""
        plan = check.make_plan(self.root, FULL)
        for p in plan.run:
            check.write_record(
                self.root,
                p.step.name,
                {
                    "fingerprint": p.inputs.fingerprint,
                    "result": "pass",
                    "started": 0,
                    "seconds": 1,
                    "log": "x",
                    "note": "",
                    "files": p.inputs.files,
                    "lists": p.inputs.lists,
                },
            )

    def pass_merge(self, failed: tuple[str, ...] = ()) -> None:
        """Every step of a merge recorded as that merge records it, passed but for `failed`."""
        writers = check.make_plan(self.root, FULL, True, [s.name for s in steps.STEPS if s.first], merge=True)
        for p in writers.run + check.make_plan(self.root, MERGE).run:
            check.write_record(
                self.root,
                p.step.name,
                {
                    "fingerprint": p.inputs.fingerprint,
                    "run_fingerprint": p.inputs.run,
                    "merge": True,
                    "rejudged": False,
                    "result": "fail" if p.step.name in failed else "pass",
                    "files": p.inputs.files,
                    "lists": p.inputs.lists,
                },
            )

    def plan_reasons(self, layer: str, force: bool = False) -> dict[str, str]:
        return {p.step.name: p.reason for p in check.make_plan(self.root, layer, force).run}

    def planned(self, layer: str = QUICK, force: bool = False) -> dict[str, list[str] | None]:
        return {p.step.name: p.only for p in check.make_plan(self.root, layer, force).run}

    def run_plan(self, layer: str = QUICK) -> None:
        """Every planned step passes, recorded as run_step records it, without running anything."""
        for p in check.make_plan(self.root, layer).run:
            p.step = steps.Step(p.step.name, p.step.layer, (sys.executable, "-c", "pass"), p.step.inputs)
            check.run_step(p, self.root, False, True)

    def close(self):
        self.tmp.cleanup()


class Globs(unittest.TestCase):
    def test_star_stays_in_a_folder(self):
        self.assertTrue(check.glob_regex("tests/*.list").match("tests/a.list"))
        self.assertFalse(check.glob_regex("tests/*.list").match("tests/x/a.list"))
        self.assertTrue(check.glob_regex("crates/**").match("crates/a/b/c.rs"))
        self.assertTrue(check.glob_regex("tests/corpus/**/*.bas").match("tests/corpus/a.bas"))
        self.assertTrue(check.glob_regex("tests/corpus/**/*.bas").match("tests/corpus/g/h/a.bas"))

    def test_a_negative_glob_takes_back(self):
        files = sorted(["verification/a.bas", "verification/a.exe", "x/a.exe"])
        self.assertEqual(
            check.matching(("verification/**", "!verification/*.exe"), files), ["verification/a.bas"]
        )


class RealSteps(unittest.TestCase):
    """The step list against this repository."""

    @classmethod
    def setUpClass(cls):
        cls.files = check.repo_files(REPO)

    def test_every_glob_takes_a_file(self):
        for step in steps.STEPS:
            for g in step.inputs:
                if g.startswith("!"):
                    continue
                with self.subTest(step=step.name, glob=g):
                    self.assertTrue(check.matching((g,), self.files), f"{step.name}: {g} matches no file")

    def test_the_two_tools_steps_run_every_tools_tests_once(self):
        import tool_tests  # pylint: disable=import-outside-toplevel

        everything = tool_tests.folders()
        ran = []
        for name in ("tools", "tools-check"):
            command = steps.BY_NAME[name].command
            only = [command[i + 1] for i, a in enumerate(command) if a == "--only"]
            skip = [command[i + 1] for i, a in enumerate(command) if a == "--except"]
            ran += [d.name for d in tool_tests.chosen(everything, only, skip)]
        self.assertEqual(sorted(ran), [d.name for d in everything])
        with self.assertRaises(SystemExit):
            tool_tests.chosen(everything, ["no-such-folder"], [])

    def test_the_sandbox_step_takes_the_vm_scripts(self):
        # The family runs in the test VM: a change of its runner runs the step again.
        inputs = steps.BY_NAME["sandbox"].inputs
        self.assertIn("tools/winvm/**", inputs)
        self.assertTrue(check.matching(("tools/winvm/**",), ["tools/winvm/run.ps1"]))

    def test_the_v37_libraries_are_built_before_the_verification_programs(self):
        # The quick layer runs the verification programs a change reaches, so it builds them too.
        self.assertIn("v37-libs", steps.BY_NAME["verification"].needs)
        self.assertIn("v37-libs", steps.BY_NAME["verification-whole"].needs)
        self.assertEqual(steps.BY_NAME["v37-libs"].layer, QUICK)

    def test_names_are_unique_and_needs_exist(self):
        self.assertEqual(len(steps.BY_NAME), len(steps.STEPS))
        for step in steps.STEPS:
            for need in step.needs:
                self.assertIn(need, steps.BY_NAME, step.name)
                self.assertLess(steps.STEPS.index(steps.BY_NAME[need]), steps.STEPS.index(step))

    def test_every_file_a_suite_reads_is_an_input_of_its_step(self):
        exe = REPO / steps.EXE
        if not exe.is_file():
            self.skipTest("no release build")
        for suite, step, lst in (
            ("verification", "verification", "tests/verification/pass.list"),
            ("corpus", "slice", "tests/corpus/slice.list"),
            ("compile", "upstream", "tests/upstream/pass.list"),
        ):
            with self.subTest(step=step):
                out = subprocess.run(
                    [sys.executable, steps.RUNNER, "--suite", suite, "--qb64", str(exe), "--list", lst, "--list-only"],
                    cwd=REPO,
                    capture_output=True,
                    text=True,
                    check=True,
                ).stdout.split()
                taken = set(check.matching(steps.BY_NAME[step].inputs, sorted(out)))
                self.assertEqual([p for p in out if p not in taken], [], f"read by {suite}, not inputs of {step}")


class Standing(unittest.TestCase):
    def setUp(self):
        self.repo = FakeRepo()

    def tearDown(self):
        self.repo.close()

    def test_nothing_changed_stands_and_a_commit_changes_nothing(self):
        self.repo.pass_all()
        self.assertEqual(self.repo.planned(FULL), {})
        git(self.repo.root, "add", "-A")
        git(self.repo.root, "-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", "x")
        self.assertEqual(self.repo.planned(FULL), {})

    def test_an_input_change_runs_again_and_says_which(self):
        self.repo.pass_all()
        self.repo.write("crates/sema/src/lib.rs", "fn b() {}\n")
        plan = check.make_plan(self.repo.root, QUICK)
        fmt = next(p for p in plan.run if p.step.name == "fmt")
        self.assertIn("crates/sema/src/lib.rs", fmt.reason)

    def test_a_failure_never_stands(self):
        self.repo.pass_all()
        r = check.read_record(self.repo.root, "fmt")
        r["result"] = "fail"
        check.write_record(self.repo.root, "fmt", r)
        self.assertIn("fmt", self.repo.planned())

    def test_force_runs_everything(self):
        self.repo.pass_all()
        self.assertEqual(
            set(self.repo.planned(FULL, force=True)), {s.name for s in steps.STEPS if s.layer != MERGE}
        )

    def test_a_run_records_its_result(self):
        plan = check.make_plan(self.repo.root, QUICK)
        p = next(p for p in plan.run if p.step.name == "fmt")
        p.step = steps.Step("fmt", QUICK, (sys.executable, "-c", "pass"), p.step.inputs)
        r = check.run_step(p, self.repo.root, False, True)
        self.assertEqual(r["result"], "pass")
        self.assertNotIn("fmt", self.repo.planned())


class Cost(unittest.TestCase):
    """What a standard change plans (spec testing/check-workflow); a change that plans more fails here."""

    def setUp(self):
        self.repo = FakeRepo()
        self.repo.pass_all()

    def tearDown(self):
        self.repo.close()

    def test_documents_only_plan_nothing(self):
        self.repo.write("docs/STATUS.md", "changed\n")
        self.repo.write("docs/NEW.md", "new\n")
        self.assertEqual(self.repo.planned(), {})

    def test_recordings_plan_their_programs_and_the_list_check(self):
        self.repo.write("verification/v2.bas", "PRINT 2\n")
        self.repo.write("verification/v2.out.txt", " 2\n")
        self.repo.write("tests/verification/pass.list", "v1\nv2\nv3\n")
        self.repo.write("tests/verification/known.list", "v9\tD-001\nv3\tD-002\n")
        self.assertEqual(self.repo.planned(), {"lists": None, "verification": ["v2", "v3"]})

    def test_a_crate_change_plans_the_quick_layer(self):
        self.repo.write("crates/sema/src/lib.rs", "fn b() {}\n")
        self.assertEqual(
            set(self.repo.planned()), {"fmt", "build", "clippy", "nextest", "lists", "doc", "slice"}
        )
        self.assertIsNone(self.repo.planned()["slice"])

    def test_a_change_of_a_crates_tests_plans_no_build_no_lists_and_no_programs(self):
        self.repo.write("crates/sema/tests/it/symbols.rs", "#[test]\nfn t() {}\n")
        self.repo.write("crates/driver/tests/it/cli.rs", "#[test]\nfn t() {}\n")
        self.assertEqual(set(self.repo.planned()), {"fmt", "clippy", "nextest", "doc"})

    def test_a_change_of_inputs_rs_plans_lists(self):
        self.repo.write("crates/driver/tests/it/inputs.rs", "#[test]\nfn t() {}\n")
        self.assertEqual(set(self.repo.planned()), {"fmt", "clippy", "nextest", "lists", "doc"})

    def test_a_libqb_edit_naming_built_ins_plans_their_programs(self):
        self.repo.write(check.COUNTS + "/verification.txt", COUNTS)
        self.repo.write("tools/libqb/edits/d1-sound.edit", "file: internal/c/libqb.cpp\nbuiltins: SOUND\n# v2\n")
        self.repo.write("libqb/internal/c/qbx.cpp", "int y;\n")
        self.assertEqual(
            self.repo.planned(),
            {"nextest": None, "libqb-copy": None, "slice": ["g/p2"], "upstream": ["c/u1"], "verification": ["v3"]},
        )

    def test_a_support_file_of_several_programs_is_not_narrowed(self):
        # v1_inc.bi is no program's own file: the whole list, left to `full`, never a pass of no program.
        self.repo.write("verification/v1_inc.bi", "x = 2\n")
        plan = check.make_plan(self.repo.root, QUICK)
        self.assertNotIn("verification", {p.step.name for p in plan.run})
        self.assertIn("verification", {s.name for s, _ in plan.left})
        self.assertIsNone(self.repo.planned(FULL)["verification"])

    def test_a_pass_narrowed_by_coverage_stands_for_quick_not_full(self):
        self.repo.write(check.COUNTS + "/verification.txt", COUNTS)
        self.repo.write("libqb/internal/c/qbx.cpp", "int y;\n")
        self.repo.write("tools/libqb/edits/d1-sound.edit", "file: internal/c/libqb.cpp\nbuiltins: SOUND\n# v2\n")
        self.repo.run_plan()
        self.assertEqual(check.read_record(self.repo.root, "verification")["narrowed"], "coverage")
        self.assertEqual(self.repo.planned(), {})
        # A narrowing by files on top keeps the mark; only a whole run takes it away.
        self.repo.write("verification/v1.out.txt", " 1 \n")
        self.repo.run_plan()
        self.assertEqual(check.read_record(self.repo.root, "verification")["narrowed"], "coverage")
        full = self.repo.planned(FULL)
        for name in ("slice", "upstream", "verification"):
            self.assertIsNone(full[name], name)
        self.repo.run_plan(FULL)
        self.assertNotIn("narrowed", check.read_record(self.repo.root, "verification"))
        self.assertEqual(self.repo.planned(FULL), {})

    def test_a_libqb_edit_without_built_ins_plans_the_whole_slice(self):
        self.repo.write(check.COUNTS + "/verification.txt", COUNTS)
        self.repo.write("tools/libqb/edits/d2-x.edit", "file: internal/c/libqb.cpp\n")
        got = self.repo.planned()
        self.assertIsNone(got["slice"])
        self.assertNotIn("verification", got)

    def test_the_tools_plan_their_tests_alone(self):
        self.repo.write("tools/check/check.py", "# changed\n")
        self.assertEqual(self.repo.planned(), {"tools-check": None})

    def test_another_tool_does_not_plan_checks_own_tests(self):
        self.repo.write("tools/diagnostics/old_wording.py", "# changed\n")
        self.assertEqual(self.repo.planned(), {"tools": None})

    def test_a_folder_check_reads_plans_both(self):
        self.repo.write("tools/libqb/copy_libqb.py", "# changed\n")
        got = self.repo.planned()
        self.assertIn("tools", got)
        self.assertIn("tools-check", got)

    def test_the_runner_of_the_tools_tests_plans_both(self):
        self.repo.write("tools/check/tool_tests.py", "# changed\n")
        self.assertEqual(self.repo.planned(), {"tools": None, "tools-check": None})


class Full(unittest.TestCase):
    def test_full_refuses_uncommitted_changes(self):
        repo = FakeRepo()
        try:
            self.assertEqual(check.main(["full"], root=repo.root), 2)
        finally:
            repo.close()

    def test_full_runs_on_uncommitted_changes_when_asked_and_says_so(self):
        repo = FakeRepo()
        try:
            planned = []
            out = io.StringIO()
            with mock.patch.object(check, "no_clone", lambda root: None), mock.patch.object(
                check, "choose_place", lambda root, what, asked, *a: (check.HOST, None)
            ), mock.patch.object(
                check, "execute", lambda root, plan, force, what="": planned.append(plan) or 0
            ), contextlib.redirect_stdout(out):
                self.assertEqual(check.main(["full", "--wait"], root=repo.root), 2)
                self.assertEqual(planned, [])
                self.assertEqual(check.main(["full", "--uncommitted", "--wait"], root=repo.root), 0)
                self.assertEqual(check.main(["merge", "--uncommitted", "--wait"], root=repo.root), 2)
            self.assertEqual(len(planned), 1)
            self.assertEqual(out.getvalue().count("running on uncommitted changes"), 1)
            self.assertIn("--uncommitted applies to `check full` only: a merge's results", out.getvalue())
            # Elsewhere the refusal gives no merge's reason.
            quick = io.StringIO()
            with contextlib.redirect_stdout(quick):
                self.assertEqual(check.main(["quick", "--uncommitted"], root=repo.root), 2)
            self.assertEqual(quick.getvalue().strip(), "--uncommitted applies to `check full` only")
        finally:
            repo.close()

    def test_full_refuses_a_tree_without_the_reference_clone_beside_it(self):
        repo = FakeRepo()
        try:
            self.assertIn("worktree goes beside the checkout", check.no_clone(repo.root))
            with mock.patch.object(check, "dirty", lambda root: []), mock.patch.object(check, "execute") as run:
                self.assertEqual(check.main(["full", "--wait"], root=repo.root), 2)
            run.assert_not_called()
            self.assertIsNone(check.no_clone(REPO))
        finally:
            repo.close()

    def test_order_failures_first_then_longest_with_needs(self):
        repo = FakeRepo()
        try:
            for name, result, seconds in (("fmt", "pass", 1), ("clippy", "pass", 50), ("slice", "fail", 5)):
                check.write_record(repo.root, name, {"result": result, "seconds": seconds, "started": 0})
            plan = check.make_plan(repo.root, QUICK, force=True)
            names = [p.step.name for p in check.order(plan.run, repo.root)]
            self.assertLess(names.index("build"), names.index("slice"))
            self.assertLess(names.index("slice"), names.index("clippy"))
            self.assertLess(names.index("clippy"), names.index("fmt"))
            full = [p.step.name for p in check.order(check.make_plan(repo.root, FULL, force=True).run, repo.root)]
            self.assertEqual(full[-1], "sandbox")
        finally:
            repo.close()


class Merge(unittest.TestCase):
    """The merge into main: every step again but those its whole suites stand for, and its own."""

    def setUp(self):
        self.repo = FakeRepo()
        self.repo.pass_all()

    def tearDown(self):
        self.repo.close()

    def test_merge_runs_everything_again_but_the_lists_its_whole_suites_stand_for(self):
        # The writers are a plan of their own, run before this one is made (check.main).
        writers = {s.name for s in steps.STEPS if s.first}
        self.assertEqual(writers, {"update-lists", "wording"})
        got = self.repo.planned(MERGE)
        self.assertEqual(set(got), {s.name for s in steps.STEPS} - steps.MERGE_REPLACES - writers)
        self.assertTrue(all(only is None for only in got.values()))
        self.assertFalse({s.name for s in steps.STEPS if s.layer == MERGE} & set(self.repo.planned(FULL, force=True)))

    def test_merge_refuses_uncommitted_changes(self):
        self.assertEqual(check.main(["merge"], root=self.repo.root), 2)

    def test_merge_plans_after_the_writers_wrote(self):
        # Every fingerprint of the merge's plan is of the files the writers wrote, not of those before them.
        ran = []

        def execute(root, plan, force, what=""):
            ran.append((what, [p.step.name for p in plan.run]))
            if what == "merge: the writers":
                self.repo.write("crates/sema/src/lib.rs", "fn written() {}\n")
            return 0

        planned = []
        real = check.make_plan

        def make_plan(root, layer, *a, **kw):
            plan = real(root, layer, *a, **kw)
            if layer == MERGE:
                planned.append(next(p for p in plan.run if p.step.name == "fmt").inputs.files["crates/sema/src/lib.rs"])
            return plan

        with mock.patch.object(check, "dirty", lambda root: []), mock.patch.object(
            check, "no_clone", lambda root: None
        ), mock.patch.object(check, "execute", execute), mock.patch.object(check, "make_plan", make_plan):
            self.assertEqual(check.main(["merge", "--wait"], root=self.repo.root), 0)
        self.assertEqual([w for w, _ in ran], ["merge: the writers", "merge"])
        self.assertEqual(sorted(ran[0][1]), ["update-lists", "wording"])
        self.assertNotEqual(planned[0], planned[-1])
        self.assertEqual(planned[-1], check.Hashes(self.repo.root).of("crates/sema/src/lib.rs"))

    def test_the_pass_lists_hold_no_known_failure(self):
        for name in ("slice", "upstream", "verification", "differential", "nofold"):
            self.assertIn("--pass-list", steps.BY_NAME[name].command, name)

    def test_merge_order_readers_after_the_whole_suites(self):
        names = [p.step.name for p in check.order(check.make_plan(self.repo.root, MERGE).run, self.repo.root)]
        self.assertEqual(names[-1], "sandbox")
        for name, _, _ in steps.WHOLE:
            self.assertLess(names.index("build"), names.index(f"{name}-whole"))
            for reader in (f"{name}-list", "builtins", "counts"):
                self.assertLess(names.index(f"{name}-whole"), names.index(reader))

    def test_a_merge_again_lets_the_earlier_merges_passes_stand(self):
        # OpenSpec change check-rejudge, design D3: a pass of an earlier merge with the same fingerprint stands.
        self.repo.pass_merge(failed=("upstream-whole",))
        # Nothing changed: the failed suite runs again (a failure may be the machine's); its readers stand.
        self.assertEqual(self.repo.plan_reasons(MERGE), {"upstream-whole": "failed last time"})

    def test_a_merge_again_after_a_known_list_fix_re_judges(self):
        self.repo.pass_merge(failed=("upstream-whole", "linux-verification"))
        self.repo.write("tools/legacy_tests/known_failures.txt", "compile_tests:c/u1  D-052\n")
        self.repo.write("tests/verification/known.linux.list", "v3\tvaries\n")
        got = self.repo.plan_reasons(MERGE)
        self.assertEqual(got["upstream-whole"], check.REJUDGE_REASON)
        self.assertEqual(got["linux-verification"], check.REJUDGE_REASON)
        # Every whole suite that reads a changed list is judged again, none run; what needs them runs (it reads their
        # results), and the sync that copies the lists.
        for name, step in ((n, steps.BY_NAME[n]) for n in got):
            if step.rejudge is not None:
                self.assertEqual(got[name], check.REJUDGE_REASON, name)
        for name in ("upstream-list", "counts", "linux-sync"):
            self.assertIn(name, got)
        for name in ("build", "nextest", "callsite", "fmt"):
            self.assertNotIn(name, got, f"{name} does not read the lists that changed")

    def test_the_sync_checks_by_content_at_the_first_merge_and_under_force_only(self):
        # design D6: after an earlier merge's sync passed, the copy follows git; the first merge and --force check
        # every file by content.
        def sync(force: bool = False) -> str:
            p = next(p for p in check.make_plan(self.repo.root, MERGE, force).run if p.step.name == "linux-sync")
            return check.command_of(p, self.repo.root, True, True)[0]

        self.repo.write("tests/verification/known.linux.list", "v3\tvaries\n")
        self.assertIn("--checksum", sync(), "pass_all: no earlier merge")
        self.repo.pass_merge()
        self.repo.write("tests/verification/known.linux.list", "v3\tvaries: now\n")
        self.assertNotIn("--checksum", sync())
        self.assertIn("--checksum", sync(force=True))

    def test_a_pass_outside_a_merge_does_not_stand_for_one(self):
        # pass_all records `check full`'s passes: none stands for a merge.
        self.assertEqual(self.repo.plan_reasons(MERGE)["build"], "no pass of an earlier merge")

    def test_force_still_runs_every_step_of_the_merge(self):
        self.repo.pass_merge()
        self.repo.write("tools/legacy_tests/known_failures.txt", "compile_tests:c/u1  D-052\n")
        got = self.repo.plan_reasons(MERGE, force=True)
        self.assertEqual(set(got), {s.name for s in steps.STEPS} - steps.MERGE_REPLACES - {"update-lists", "wording"})
        self.assertNotIn(check.REJUDGE_REASON, got.values())

    def test_a_str_step_runs_in_gits_bash(self):
        # `pwd -W` is Git's bash's alone: WSL's bash.exe, which Windows finds first in System32, has no -W.
        if os.name != "nt":
            self.skipTest("Windows only")
        out = subprocess.run([check.bash(), "-c", "pwd -W"], capture_output=True, text=True)
        self.assertEqual(out.returncode, 0, out.stderr)


class Rejudge(unittest.TestCase):
    """A whole suite whose inputs changed only in the known lists is judged again, not run (OpenSpec change
    check-rejudge, design D2)."""

    def setUp(self):
        self.repo = FakeRepo()
        self.repo.pass_merge(failed=("upstream-whole",))

    def tearDown(self):
        self.repo.close()

    def named(self, name: str) -> dict[str, check.Planned]:
        plan = check.make_plan(self.repo.root, FULL, False, check.with_needs([name]), asked=(name,))
        return {p.step.name: p for p in plan.run}

    def test_a_known_list_row_after_a_failure_plans_a_re_judge(self):
        self.repo.write("tools/legacy_tests/known_failures.txt", "compile_tests:c/u1  D-052\n")
        got = self.named("upstream-whole")
        self.assertEqual(list(got), ["upstream-whole"])
        p = got["upstream-whole"]
        self.assertTrue(p.rejudge)
        cmd, note = check.command_of(p, self.repo.root, False, True)
        self.assertIn("--rejudge", cmd)
        self.assertNotIn("--coverage", cmd)
        self.assertEqual(note, "re-judged: no program ran")

    def test_a_list_row_and_a_crate_change_plan_a_run(self):
        self.repo.write("tools/legacy_tests/known_failures.txt", "compile_tests:c/u1  D-052\n")
        self.repo.write("crates/sema/src/lib.rs", "fn b() {}\n")
        got = self.named("upstream-whole")
        self.assertFalse(got["upstream-whole"].rejudge)

    def test_a_re_judge_is_recorded_as_one(self):
        self.repo.write("tools/legacy_tests/known_failures.txt", "compile_tests:c/u1  D-052\n")
        p = self.named("upstream-whole")["upstream-whole"]
        p.step = steps.Step(p.step.name, p.step.layer, p.step.command, p.step.inputs, rejudge=(sys.executable, "-c", "pass"))
        r = check.run_step(p, self.repo.root, False, True)
        self.assertEqual((r["result"], r["rejudged"], r["run_fingerprint"]), ("pass", True, p.inputs.run))
        # It now stands: nothing changed since.
        self.assertFalse(check.can_rejudge(p.step, check.read_record(self.repo.root, "upstream-whole"), p.inputs))

    def test_the_judge_lists_name_every_list_the_runner_reads(self):
        sys.path.insert(0, str(REPO / "tools" / "legacy_tests"))
        import run_legacy_tests as rlt

        args = mock.Mock(
            known_failures=REPO / "tools" / "legacy_tests" / "known_failures.txt",
            corpus_root=REPO / "tests" / "corpus",
            platform="linux",
        )
        _, files = rlt.known_lists(args, verification=True)
        rels = sorted(Path(os.path.relpath(f, REPO)).as_posix() for f in files)
        self.assertEqual(len(rels), 5)
        self.assertEqual(check.matching(steps.JUDGE_LISTS, rels), rels)


class Named(unittest.TestCase):
    """A step asked for by name (`check slice`): its needs first, its fingerprint the same as in its layer."""

    def setUp(self):
        self.repo = FakeRepo()

    def tearDown(self):
        self.repo.close()

    def named(self, name: str) -> check.Plan:
        return check.make_plan(self.repo.root, FULL, False, check.with_needs([name]), asked=(name,))

    def test_the_fingerprint_is_the_layers(self):
        named = next(p for p in self.named("slice").run if p.step.name == "slice")
        layer = next(p for p in check.make_plan(self.repo.root, QUICK).run if p.step.name == "slice")
        self.assertEqual(named.inputs.fingerprint, layer.inputs.fingerprint)

    def test_its_needs_run_first_unless_they_stand(self):
        self.assertEqual(check.with_needs(["slice"]), ["build", "slice"])
        self.assertEqual([p.step.name for p in self.named("slice").run], ["build", "slice"])
        self.repo.pass_all()
        self.assertEqual([p.step.name for p in self.named("slice").run], ["slice"])
        self.repo.write("crates/sema/src/lib.rs", "fn b() {}\n")
        self.assertEqual([p.step.name for p in self.named("slice").run], ["build", "slice"])


class Execute(unittest.TestCase):
    def test_a_step_whose_need_failed_is_not_run_nor_recorded(self):
        repo = FakeRepo()
        try:
            plan = check.make_plan(repo.root, QUICK)
            plan.run = [p for p in plan.run if p.step.name in ("build", "slice")]
            for p in plan.run:
                code = "raise SystemExit(1)" if p.step.name == "build" else "pass"
                p.step = steps.Step(
                    p.step.name, QUICK, (sys.executable, "-c", code), p.step.inputs, needs=p.step.needs
                )
            self.assertEqual(check.execute(repo.root, plan, False), 1)
            self.assertEqual(check.read_record(repo.root, "build")["result"], "fail")
            self.assertIsNone(check.read_record(repo.root, "slice"))
        finally:
            repo.close()

    def test_a_reader_of_a_failed_need_runs_and_of_a_skipped_one_does_not(self):
        # corpus-list reads the whole suite's results: it runs after a whole suite with a failure, not after a
        # whole suite that did not run (its build failed).
        repo = FakeRepo()
        try:
            for failing, want in (("corpus-whole", "pass"), ("build", None)):
                shutil.rmtree(repo.root / "target", ignore_errors=True)
                plan = check.make_plan(repo.root, FULL, True, check.with_needs(["corpus-list"]))
                for p in plan.run:
                    code = "raise SystemExit(1)" if p.step.name == failing else "pass"
                    p.step = steps.Step(
                        p.step.name,
                        p.step.layer,
                        (sys.executable, "-c", code),
                        p.step.inputs,
                        needs=p.step.needs,
                        after_failure=p.step.after_failure,
                    )
                self.assertEqual(check.execute(repo.root, plan, True), 1)
                got = check.read_record(repo.root, "corpus-list")
                self.assertEqual(got and got["result"], want, failing)
        finally:
            repo.close()

    def test_a_bash_line_without_gits_bash_fails_and_says_so(self):
        repo = FakeRepo()
        try:
            plan = check.make_plan(repo.root, FULL, True, ["linux-sync"])
            with mock.patch.object(check, "bash", lambda: None):
                r = check.run_step(plan.run[0], repo.root, True, True)
            self.assertEqual((r["result"], r["note"]), ("fail", "Git's bash not found"))
        finally:
            repo.close()

    def test_the_sync_checks_by_content_only_when_forced(self):
        # linux-sync's usual form lets rsync compare sizes and times (one copy per tree, design D2 of
        # linux-sync-per-tree); --force and the merge, which forces every step, ask for the exact copy.
        repo = FakeRepo()
        try:
            plan = check.make_plan(repo.root, FULL, True, ["linux-sync"])
            usual, note = check.command_of(plan.run[0], repo.root, False, True)
            forced, forced_note = check.command_of(plan.run[0], repo.root, True, True)
            self.assertNotIn("--checksum", usual)
            self.assertEqual(note, "")
            self.assertEqual(forced, usual + " --checksum")
            self.assertEqual(forced_note, "forced: the exact form")
        finally:
            repo.close()

    def test_the_linux_steps_run_in_the_trees_own_copy(self):
        # No step names a copy itself: each runs through tools/linux/run.sh, which takes the copy from the tree it is
        # run from (copy.sh), so a worktree's steps never run in the checkout's copy.
        for s in steps.STEPS:
            if s.name.startswith("linux-") and s.name != "linux-sync":
                self.assertTrue(s.command.startswith(steps.IN_COPY), s.name)
                self.assertNotIn("~/qb64rust", s.command, s.name)
                for script in steps.LINUX_SCRIPTS:
                    self.assertTrue(check.matching(s.inputs, [script]) == [script], (s.name, script))

    def test_the_copy_is_named_from_the_trees_folder(self):
        sh = check.bash()
        if sh is None:
            self.skipTest("Git's bash not found")
        rule = REPO / "tools/linux/copy.sh"
        for tree, want in (("/x/QB64Rust", "qb64rust"), ("/x/QB64Rust-full", "qb64rust-full"), ("/x/Q/", "q")):
            out = subprocess.run(
                [sh, "-c", f"HOME=/h; source '{rule.as_posix()}' && linux_copy '{tree}'"], capture_output=True, text=True
            )
            self.assertEqual(out.stdout.strip(), f"/h/{want}", (tree, out.stderr))

    def test_each_run_adds_a_line_to_the_history(self):
        repo = FakeRepo()
        try:
            repo.pass_all()
            repo.write("tools/check/check.py", "# changed\n")
            plan = check.make_plan(repo.root, QUICK)
            name = plan.run[0].step.name
            self.assertEqual(name, "tools-check")
            plan.run[0].step = steps.Step(name, QUICK, (sys.executable, "-c", "pass"), plan.run[0].step.inputs)
            self.assertEqual(check.execute(repo.root, plan, False, "quick"), 0)
            self.assertEqual(check.main(["quick"], root=repo.root), 0)  # nothing to run: a line all the same
            first, second = check.read_history(repo.root)
            self.assertEqual((first["what"], list(first["ran"]), first["ran"][name][0]), ("quick", [name], "pass"))
            self.assertIn("fmt", first["standing"])
            self.assertEqual(second["ran"], {})
            self.assertEqual(check.history(repo.root, 14), 0)
        finally:
            repo.close()


class VmSteps(unittest.TestCase):
    """steps.py's marks for the suites VM (OpenSpec change vm-suites, design D8, D9)."""

    def test_each_vm_step_is_a_list_and_a_runner_step_names_its_results_file(self):
        vm = [s for s in steps.STEPS if s.vm]
        self.assertEqual(
            {s.name for s in vm},
            {
                "slice", "upstream", "verification", "callsite", "cli", "qbasic", "addprefix", "format",
                "format-corpus", "differential", "nofold", "coverage", "corpus-whole", "upstream-whole",
                "verification-whole",
            },
        )
        for s in vm:
            self.assertIsInstance(s.command, tuple, s.name)
            if steps.RUNNER not in s.command:
                continue
            folder = s.command[s.command.index("--results") + 1] if "--results" in s.command else steps.RESULTS
            named = [o for o in s.outputs if o.endswith(".json")]
            self.assertEqual(len(named), 1, s.name)
            self.assertTrue(named[0].startswith(folder + "/results-"), (s.name, named))
            partial = "--list" in s.command or "--glob" in s.command
            self.assertEqual(named[0].endswith("/results-partial.json"), partial, s.name)
        self.assertEqual(steps.BY_NAME["coverage"].outputs, ("target/coverage/runs/results-partial.json",))

    def test_the_whole_suites_are_lists_that_clear_their_counts(self):
        for name, suite, _ in steps.WHOLE:
            s = steps.BY_NAME[f"{name}-whole"]
            counts = f"{steps.COUNTS}/{name}.txt"
            self.assertIsInstance(s.command, tuple)
            self.assertEqual(s.clears, (counts,))
            self.assertIn(counts, s.outputs)
            self.assertIn(f"{steps.RESULTS}/results-{suite}.json", s.outputs)
            self.assertEqual(s.command[s.command.index("--coverage") + 1], counts)
        self.assertEqual(steps.BY_NAME["verification-whole"].vm_first, steps.V37_LIBS)
        self.assertEqual(steps.BY_NAME["verification"].vm_first, steps.V37_LIBS)

    def test_a_cleared_record_goes_before_a_run_on_the_host(self):
        repo = FakeRepo()
        try:
            counts = repo.root / steps.COUNTS / "corpus.txt"
            repo.write(f"{steps.COUNTS}/corpus.txt", "old\n")
            plan = check.make_plan(repo.root, FULL, True, ["corpus-whole"])
            p = plan.run[0]
            p.step = dataclasses.replace(p.step, command=(sys.executable, "-c", "pass"))
            check.run_step(p, repo.root, True, True)
            self.assertFalse(counts.exists())
            self.assertTrue(counts.parent.is_dir())
        finally:
            repo.close()


# Stands in for tools/winvm/suites.ps1 and step.ps1: each call is a line of the log FAKE_VM_LOG, and a step runs the
# job's command on the host.
FAKE_VM = """
import json, os, subprocess, sys, time
from pathlib import Path
which, args = Path(sys.argv[1]).stem, sys.argv[2:]
with open(os.environ["FAKE_VM_LOG"], "a", encoding="utf-8") as f:
    f.write(" ".join([which] + [a for a in args if a.startswith("-")]) + "\\n")
if which == "step" and os.environ.get("FAKE_VM_HANG"):
    time.sleep(float(os.environ["FAKE_VM_HANG"]))
if which == "step":
    job = json.loads(Path(args[args.index("-Job") + 1]).read_text(encoding="utf-8"))
    sys.exit(subprocess.run(job["command"]).returncode)
sys.exit(int(os.environ.get("FAKE_VM_EXIT_" + which.upper(), "0")))
"""


class VmPlace(unittest.TestCase):
    """`--where`, the suites VM's session of a run, and the place in a result (spec testing/check-workflow; design
    D6, D9, D10, D11)."""

    def setUp(self):
        self.repo = FakeRepo()
        base = Path(self.repo.tmp.name)
        self.fake = base / "fake_vm.py"
        self.fake.write_text(FAKE_VM, encoding="utf-8")
        self.calls = base / "calls.txt"
        patches = (
            mock.patch.dict(os.environ, {"FAKE_VM_LOG": str(self.calls)}),
            mock.patch.object(check, "powershell", lambda root, script, *a: [sys.executable, str(self.fake), script, *a]),
            mock.patch.object(check, "toolchain_path", lambda root: ""),
        )
        for p in patches:
            p.start()
            self.addCleanup(p.stop)

    def tearDown(self):
        self.repo.close()

    def log(self) -> list[str]:
        return self.calls.read_text(encoding="utf-8").splitlines() if self.calls.is_file() else []

    def plan_of(self, names, where=check.VM, codes=None) -> check.Plan:
        plan = check.make_plan(self.repo.root, FULL, True, names, where=where)
        for p in plan.run:
            code = (codes or {}).get(p.step.name, "pass")
            p.step = dataclasses.replace(p.step, command=(sys.executable, "-c", code))
        return plan

    def test_three_steps_restore_the_vm_once_and_it_is_off_after_a_failing_last_one(self):
        plan = self.plan_of(
            ["build", "slice", "verification", "differential"], codes={"verification": "raise SystemExit(1)"}
        )
        # verification last of the three in the order: the longest by its last time.
        check.write_record(self.repo.root, "verification", {"result": "pass", "seconds": 999, "started": 0})
        check.write_record(self.repo.root, "slice", {"result": "pass", "seconds": 5, "started": 0})
        check.write_record(self.repo.root, "differential", {"result": "pass", "seconds": 6, "started": 0})
        with mock.patch.object(check, "order", lambda run, root: sorted(run, key=lambda p: p.step.name == "verification")):
            self.assertEqual(check.execute(self.repo.root, plan, True), 1)
        self.assertEqual(self.log(), ["suites -Start -Tree", "step -Tree -Job", "step -Tree -Job", "step -Tree -Job", "suites -Stop"])
        got = {n: check.read_record(self.repo.root, n) for n in ("build", "slice", "verification", "differential")}
        self.assertEqual({n: (r["result"], r["where"]) for n, r in got.items()}, {
            "build": ("pass", "host"),
            "slice": ("pass", "vm"),
            "verification": ("fail", "vm"),
            "differential": ("pass", "vm"),
        })

    def test_a_failed_start_fails_the_vm_steps_and_turns_the_vm_off(self):
        plan = self.plan_of(["build", "slice", "differential"])
        with mock.patch.dict(os.environ, {"FAKE_VM_EXIT_SUITES": "1"}):
            self.assertEqual(check.execute(self.repo.root, plan, True), 1)
        self.assertEqual(self.log(), ["suites -Start -Tree", "suites -Stop"])
        self.assertEqual(check.read_record(self.repo.root, "build")["result"], "pass")
        for name in ("slice", "differential"):
            self.assertIn("the suites VM cannot be used", check.read_record(self.repo.root, name)["note"])

    def test_without_hyper_v_a_full_run_falls_back_to_the_host_saying_so_once(self):
        why = "Hyper-V cannot be used: the module Hyper-V was not loaded"
        out = io.StringIO()
        with mock.patch.object(check, "dirty", lambda root: []), mock.patch.object(
            check, "no_clone", lambda root: None
        ), mock.patch.object(check, "vm_refusal", lambda root: why), mock.patch.object(
            check, "execute", lambda root, plan, force, what="": self.planned.append(plan) or 0
        ), contextlib.redirect_stdout(out):
            self.planned = []
            self.assertEqual(check.main(["full", "--wait"], root=self.repo.root), 0)
        self.assertEqual(out.getvalue().count("the suites VM cannot be used"), 1)
        self.assertIn(why, out.getvalue())
        self.assertTrue(all(p.where == check.HOST for p in self.planned[0].run))
        self.assertIsNone(self.planned[0].vm_refusal)

    def test_the_vm_asked_for_but_missing_fails_its_steps_and_the_host_steps_run(self):
        why = "there is no VM named qb64rust-suites: make it with tools\\winvm\\create.ps1 -Shape Suites"
        out = io.StringIO()
        with mock.patch.object(check, "dirty", lambda root: []), mock.patch.object(
            check, "no_clone", lambda root: None
        ), mock.patch.object(check, "vm_refusal", lambda root: why), contextlib.redirect_stdout(out):
            self.assertEqual(check.main(["full", "--where", "vm", "--plan"], root=self.repo.root), 0)
        self.assertIn("the steps that run programs fail", out.getvalue())
        plan = self.plan_of(["build", "slice"])
        plan.vm_refusal = why
        self.assertEqual(check.execute(self.repo.root, plan, True), 1)
        self.assertEqual(self.log(), [], "the VM is never started")
        self.assertEqual(check.read_record(self.repo.root, "build")["result"], "pass")
        slice_ = check.read_record(self.repo.root, "slice")
        self.assertEqual((slice_["result"], slice_["where"]), ("fail", "vm"))
        self.assertIn("create.ps1", slice_["note"])

    def test_a_host_pass_does_not_stand_for_a_vm_run(self):
        self.repo.pass_all()
        host = check.make_plan(self.repo.root, FULL)
        self.assertEqual(host.run, [])
        vm = check.make_plan(self.repo.root, FULL, where=check.VM)
        planned = {p.step.name: p.where for p in vm.run}
        self.assertEqual(planned, {s.name: check.VM for s in steps.STEPS if s.vm and s.layer in (QUICK, FULL)})
        self.assertIn("build", {s.name for s, _ in vm.standing})

    def test_the_status_of_a_step_says_its_pass_was_in_the_vm(self):
        plan = self.plan_of(["build", "slice"])
        check.execute(self.repo.root, plan, True)
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            check.status(self.repo.root)
        line = next(ln for ln in out.getvalue().splitlines() if " slice " in ln)
        self.assertIn("pass in the VM", line)
        build = next(ln for ln in out.getvalue().splitlines() if " build " in ln)
        self.assertNotIn("in the VM", build)

    def test_quick_plans_as_before_and_asks_nothing_of_the_vm(self):
        def no_vm(root):
            raise AssertionError("quick must not look for the VM")

        before = {p.step.name: p.inputs.fingerprint for p in check.make_plan(self.repo.root, QUICK).run}
        with mock.patch.object(check, "vm_refusal", no_vm), contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(check.main(["quick", "--plan"], root=self.repo.root), 0)
        self.assertEqual(check.choose_place(self.repo.root, QUICK, None), (check.HOST, None))
        hosted = check.make_plan(self.repo.root, QUICK, where=check.HOST)
        self.assertEqual({p.step.name: p.inputs.fingerprint for p in hosted.run}, before)
        self.assertTrue(all(p.where == check.HOST for p in hosted.run))

    def test_quick_where_vm_puts_only_the_slice_in_the_vm(self):
        plan = check.make_plan(self.repo.root, QUICK, where=check.VM)
        got = {p.step.name: p.where for p in plan.run}
        self.assertEqual(got.pop("slice"), check.VM)
        self.assertTrue({"fmt", "build", "clippy", "nextest", "lists", "doc", "tools", "tools-check"} <= set(got))
        self.assertEqual(set(got.values()), {check.HOST})

    def test_the_clones_toolchain_on_the_host_is_one_line_of_the_report(self):
        for second, want in (("clone: the toolchain of the QB64pe clone next to the repository", 1), ("installed: x", 0)):
            plan = self.plan_of(["build", "slice", "upstream"], where=check.HOST)
            out = io.StringIO()
            with mock.patch.object(check, "toolchain_path", lambda root: f"somewhere\n{second}\n"), contextlib.redirect_stdout(out):
                self.assertEqual(check.execute(self.repo.root, plan, True), 0)
            self.assertEqual(out.getvalue().count("qb64rust toolchain install"), want, second)

    def test_the_cli_tests_in_the_vm_run_the_binary_built_on_the_host(self):
        plan = check.make_plan(self.repo.root, FULL, True, ["cli"], where=check.VM)
        p = plan.run[0]
        binary = "target/debug/deps/it-0123.exe"
        with mock.patch.object(check, "cli_binary", lambda root, out: binary), mock.patch.object(
            check, "vm_limit", lambda root, name: 1800
        ):
            vm = check.SuitesVM(self.repo.root)
            with open(self.repo.root / "log.txt", "wb") as out:
                check.run_in_vm(p, self.repo.root, list(p.step.command), out, vm)
            vm.stop()
        job = json.loads((check.state(self.repo.root) / "vm" / "cli.json").read_text(encoding="utf-8"))
        self.assertEqual((job["command"], job["cwd"]), ([binary, "--ignored"], "crates/driver"))
        self.assertIn(binary, job["copy"])
        self.assertIn(check.CLI_COMPILER, job["copy"])

    def test_a_vm_step_keeps_no_build_cache_and_gets_its_outputs_and_first_command(self):
        plan = check.make_plan(self.repo.root, FULL, True, ["verification"], where=check.VM)
        p = plan.run[0]
        vm = check.SuitesVM(self.repo.root)
        with open(self.repo.root / "log.txt", "wb") as out:
            check.run_in_vm(p, self.repo.root, list(p.step.command), out, vm)
        vm.stop()
        job = json.loads((check.state(self.repo.root) / "vm" / "verification.json").read_text(encoding="utf-8"))
        self.assertEqual(job["env"], {"QB64RUST_NO_BUILD_CACHE": "1"})
        self.assertIn("--pass-cache", job["command"])
        self.assertEqual(job["outputs"], list(p.step.outputs))
        self.assertEqual(job["first"], list(steps.V37_LIBS))
        self.assertGreaterEqual(job["limit"], check.VM_LIMIT_FLOOR)

    def test_a_narrowed_list_goes_into_the_vm_with_the_step(self):
        plan = check.make_plan(self.repo.root, FULL, True, ["slice"], where=check.VM)
        p = plan.run[0]
        p.only = ["g/p1"]
        cmd, _ = check.command_of(p, self.repo.root, False, True)
        vm = check.SuitesVM(self.repo.root)
        with open(self.repo.root / "log.txt", "wb") as out:
            check.run_in_vm(p, self.repo.root, cmd, out, vm)
        vm.stop()
        job = json.loads((check.state(self.repo.root) / "vm" / "slice.json").read_text(encoding="utf-8"))
        self.assertIn("target/check/lists/slice.list", job["copy"])

    def test_a_whole_suite_that_brings_nothing_back_leaves_no_old_counts_or_results(self):
        counts = f"{steps.COUNTS}/corpus.txt"
        results = f"{steps.RESULTS}/results-corpus.json"
        for rel in (counts, results):
            self.repo.write(rel, "the last merge's\n")
        plan = self.plan_of(["corpus-whole"])
        with mock.patch.dict(os.environ, {"FAKE_VM_EXIT_SUITES": "1"}):
            self.assertEqual(check.execute(self.repo.root, plan, True), 1)
        self.assertEqual(check.read_record(self.repo.root, "corpus-whole")["where"], check.VM)
        self.assertFalse((self.repo.root / counts).exists())
        self.assertFalse((self.repo.root / results).exists())

    def test_a_quick_failure_does_not_cut_the_next_limit(self):
        check.write_record(self.repo.root, "slice", {"result": "pass", "seconds": 2000, "started": 0})
        self.assertEqual(check.vm_limit(self.repo.root, "slice"), 6000)
        plan = self.plan_of(["slice"], codes={"slice": "raise SystemExit(1)"})
        self.assertEqual(check.execute(self.repo.root, plan, True), 1)
        record = check.read_record(self.repo.root, "slice")
        self.assertEqual((record["result"], record["passed_seconds"]), ("fail", 2000))
        self.assertEqual(check.vm_limit(self.repo.root, "slice"), 6000)
        # A record of before the field: its time counts only when it passed.
        check.write_record(self.repo.root, "slice", {"result": "fail", "seconds": 5000, "started": 0})
        self.assertEqual(check.vm_limit(self.repo.root, "slice"), check.VM_LIMIT_FLOOR)

    def test_the_limit_follows_the_last_whole_pass_in_the_vm(self):
        def run(where, only=None, code="pass", seconds=None):
            plan = self.plan_of(["slice"], where=where, codes={"slice": code})
            plan.run[0].only = only
            # The step's start at 0 and every later reading at `seconds`, so that its time is that.
            clock = itertools.chain([0.0], itertools.repeat(float(seconds or 0)))
            with mock.patch.object(check.time, "time", lambda: next(clock)):
                check.run_step(plan.run[0], self.repo.root, True, True, check.SuitesVM(self.repo.root))
            return check.read_record(self.repo.root, "slice")

        # A whole pass records its own time, in the VM also as the VM's.
        r = run(check.VM, seconds=2000)
        self.assertEqual((r["passed_seconds"], r["vm_passed_seconds"]), (2000, 2000))
        # A pass on the host, quicker with its build cache, does not set the VM's limit.
        r = run(check.HOST, seconds=100)
        self.assertEqual((r["passed_seconds"], r["vm_passed_seconds"]), (100, 2000))
        self.assertEqual(check.vm_limit(self.repo.root, "slice"), 6000)
        # A narrowed pass and a failure keep the earlier times.
        for r in (run(check.VM, only=["g/p1"], seconds=5), run(check.VM, code="raise SystemExit(1)", seconds=2)):
            self.assertEqual((r["passed_seconds"], r["vm_passed_seconds"]), (100, 2000))
        # Never passed in the VM: the last whole pass anywhere.
        check.write_record(self.repo.root, "slice", {"result": "pass", "seconds": 1500, "passed_seconds": 1500})
        self.assertEqual(check.vm_limit(self.repo.root, "slice"), 4500)

    def test_a_hung_step_ps1_is_stopped_and_fails_the_step(self):
        plan = self.plan_of(["slice"])
        with mock.patch.dict(os.environ, {"FAKE_VM_HANG": "30"}), mock.patch.object(
            check, "vm_limit", lambda root, name: 0
        ), mock.patch.object(check, "VM_STEP_SLACK", 2):
            self.assertEqual(check.execute(self.repo.root, plan, True), 1)
        record = check.read_record(self.repo.root, "slice")
        self.assertIn("hung", record["note"])
        self.assertEqual(self.log()[-1], "suites -Stop")

    def test_the_vm_fingerprint_takes_only_the_winvm_files_of_a_step(self):
        def fingerprint():
            return {p.step.name: p.inputs.fingerprint for p in check.make_plan(
                self.repo.root, FULL, True, ["build", "slice"], where=check.VM
            ).run}["slice"]

        others = ("README.md", "create.ps1", "run.ps1", "test_outputs.py")
        for rel in ("step.ps1",) + others:
            self.repo.write(f"tools/winvm/{rel}", "1\n")
        before = fingerprint()
        for rel in others:
            self.repo.write(f"tools/winvm/{rel}", "2\n")
        self.assertEqual(fingerprint(), before)
        self.repo.write("tools/winvm/step.ps1", "2\n")
        self.assertNotEqual(fingerprint(), before)


class Targets(unittest.TestCase):
    def test_a_program_by_name(self):
        repo = FakeRepo()
        try:
            self.assertEqual(check.find_program(repo.root, "v1"), ("verification", ["v1"]))
            self.assertEqual(check.find_program(repo.root, "p1"), ("corpus", ["--list", "g/p1"]))
            self.assertEqual(check.find_program(repo.root, "u1"), ("compile", ["--list", "c/u1"]))
            self.assertIsNone(check.find_program(repo.root, "nothing"))
        finally:
            repo.close()

    def test_a_program_runs_after_what_its_suites_step_needs(self):
        # A verification program links the v37 libraries, which a fresh worktree does not have.
        self.assertEqual(check.program_needs(list(steps.runner("verification", "v1"))), ["build", "v37-libs"])
        self.assertEqual(check.program_needs(list(steps.runner("corpus", "--list", "g/p1"))), ["build"])

    def test_without_nextest_the_tests_run_under_cargo_test_and_it_says_so(self):
        repo = FakeRepo()
        try:
            plan = check.make_plan(repo.root, QUICK)
            p = next(p for p in plan.run if p.step.name == "nextest")
            cmd, note = check.command_of(p, repo.root, False, nextest=False)
            self.assertEqual(cmd, list(steps.FALLBACK["nextest"]))
            self.assertIn(check.INSTALL["nextest"], note)
            self.assertEqual(check.command_of(p, repo.root, False, nextest=True)[0], list(p.step.command))
        finally:
            repo.close()

    def test_a_crate_folder_runs_that_crate(self):
        repo = FakeRepo()
        try:
            cmd, _ = check.target_command(repo.root, "crates/sema", False)
            self.assertIn("qb64rust-sema", cmd)
            self.assertIn("not a crate folder", check.target_command(repo.root, "crates/none", False))
        finally:
            repo.close()

    def test_folders_and_lists_name_steps(self):
        self.assertEqual(check.step_of_target("tests/corpus/slice.list"), "slice")
        self.assertEqual(check.step_of_target("verification"), "verification")
        self.assertEqual(check.step_of_target("tests\\upstream\\"), "upstream")
        self.assertEqual(check.step_of_target("clippy"), "clippy")
        self.assertIsNone(check.step_of_target("crates/sema"))


if __name__ == "__main__":
    os.environ.pop("QB64RUST_TOOLCHAIN_ROOT", None)
    unittest.main()
