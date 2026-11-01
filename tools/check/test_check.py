#!/usr/bin/env python3
"""Tests of tools/check: the globs, the fingerprints, what stands, and the cost of a check (spec
testing/check-workflow, "The cost of a check is held by tests").

    python -m unittest discover -s tools/check -p "test_*.py"

The cost tests plan in a fake repository (a git repository in a temporary folder holding one file for each kind of
input), so they assert the plan of the real steps (steps.py) without running anything.
"""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import check  # noqa: E402
import steps  # noqa: E402
from steps import FULL, QUICK  # noqa: E402

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
        self.assertEqual(set(self.repo.planned(FULL, force=True)), {s.name for s in steps.STEPS})

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
