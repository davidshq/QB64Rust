#!/usr/bin/env python3
"""Tests of tools/check: the globs, the fingerprints, what stands, and the cost of a check (spec
testing/check-workflow, "The cost of a check is held by tests").

    python -m unittest discover -s tools/check -p "test_*.py"

The cost tests plan in a fake repository (a git repository in a temporary folder holding one file for each kind of
input), so they assert the plan of the real steps (steps.py) without running anything.
"""

from __future__ import annotations

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
        self.assertEqual(self.repo.planned(), {"tools": None})


class Full(unittest.TestCase):
    def test_full_refuses_uncommitted_changes(self):
        repo = FakeRepo()
        try:
            self.assertEqual(check.main(["full"], root=repo.root), 2)
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
            plan.run[0].step = steps.Step("tools", QUICK, (sys.executable, "-c", "pass"), plan.run[0].step.inputs)
            self.assertEqual(check.execute(repo.root, plan, False, "quick"), 0)
            self.assertEqual(check.main(["quick"], root=repo.root), 0)  # nothing to run: a line all the same
            first, second = check.read_history(repo.root)
            self.assertEqual((first["what"], list(first["ran"]), first["ran"]["tools"][0]), ("quick", ["tools"], "pass"))
            self.assertIn("fmt", first["standing"])
            self.assertEqual(second["ran"], {})
            self.assertEqual(check.history(repo.root, 14), 0)
        finally:
            repo.close()


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
