"""The steps of `check` (tools/check/README.md): each one's command, the files it reads and the steps it needs first.

This list is the specification of what a result rests on (design D3 of the OpenSpec change testing-reset): a step's
fingerprint is taken over the files its `inputs` globs match, and a passing result stands while that fingerprint is
unchanged. A glob left out here makes a stale pass stand, so test_check.py holds every glob to at least one file,
and each runner suite's step to every file a `--list-only` run of it names.

Globs are repo-relative with `/`: `*` stays within a folder, `**` crosses folders, and a glob starting with `!`
takes back files an earlier glob matched. `external` names inputs outside the repository (check.py,
`EXTERNALS`): the C++ toolchain and the reference clone.
"""

from __future__ import annotations

from dataclasses import dataclass, field

QUICK = "quick"
FULL = "full"
# The merge into main: every step but those its whole suites stand for (MERGE_REPLACES), always run again, and the
# steps of its own, which regenerate files (the lists, the wording table, the coverage records, the count blocks)
# and hold the pass lists and the built-ins' list to the whole suites' results.
MERGE = "merge"

RUNNER = "tools/legacy_tests/run_legacy_tests.py"
EXE = "target/release/qb64rust.exe"
# Every runner suite builds with the build cache and remembers passes; the pass cache's key is the program's build
# inputs (design D1), so a hit neither builds nor runs.
CACHES = ("--jobs", "8", "--build-cache", "target/build-cache", "--pass-cache", "target/pass-cache-2")

# What `cargo` builds the compiler and its tests from.
RUST = (
    "crates/**",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "clippy.toml",
    "rustfmt.toml",
    "!crates/**/*.md",
)
# What the release build is made from: not the crates' integration tests, which only cargo's tests build.
BUILT = RUST + ("!crates/*/tests/**",)
# What the `lists` step (crates/driver/tests/it/inputs.rs) is built from: the release build's files and the parts of
# the driver's test binary that inputs.rs is compiled with.
LISTS_RUST = BUILT + (
    "crates/driver/tests/it/main.rs",
    "crates/driver/tests/it/inputs.rs",
    "crates/driver/tests/it/support/**",
)
# libqb's copy and its edits: what every built program links.
LIBQB = ("libqb/**", "tools/libqb/edits/**")
# The runner and what it runs programs with.
RUNNER_FILES = ("tools/legacy_tests/*.py", "tools/legacy_tests/*.txt", "tools/screen/**")
# The files the Rust tests read outside crates/ (the integration tests' reads, crates/*/tests), but not the
# recordings of the program suites, which only `lists` (crates/driver/tests/it/inputs.rs) reads: a new recording
# plans `lists` and its suite's programs, not the workspace's tests (spec testing/check-workflow).
RUST_TEST_READS = (
    "tests/frontend/**",
    "tests/format/**",
    "tests/differential/**",
    "tests/snippets/**",
    "tests/unit_tests/**",
    "tests/programs/**",
    "tests/callsite/**",
    "tests/upstream/root/**",
    "tests/upstream/compile_tests/**/*.bas",
    "tests/corpus/**/*.bas",
    "tests/corpus/*.list",
    "tests/*.list",
    "verification/v15_builtin_names.txt",
    "verification/v19_proc_names.txt",
    "verification/v52_builtin_name_suffixes.txt",
)


@dataclass(frozen=True)
class Step:
    name: str
    layer: str
    # The command as arguments; a str is run by bash (the WSL steps).
    command: tuple[str, ...] | str
    inputs: tuple[str, ...]
    needs: tuple[str, ...] = ()
    external: tuple[str, ...] = ()
    # A runner suite whose programs a change can be narrowed to (check.py, `narrow`): the runner's suite and the
    # list it runs.
    suite: str | None = None
    program_list: str | None = None
    # Lists whose rows each name one program first (`<name><TAB><reason>`): a changed row narrows to its program.
    side_lists: tuple[str, ...] = ()
    # Run with nothing beside it (its window takes the screen).
    alone: bool = False
    # A writer of the merge: it writes files the others read or build in, so `check merge` runs the writers as a
    # plan of their own and plans the rest after them (check.py, `main`).
    first: bool = False
    # Run after its needs whatever their result (not when one was skipped): it reads what they wrote, not their
    # verdict (a whole suite with one failure still has results to hold the pass lists to).
    after_failure: bool = False
    env: dict[str, str] = field(default_factory=dict)
    # The command under `--force` (and so at the merge, which forces every step), when it differs: the exact form
    # of a step whose usual form trusts something a forced run does not (linux-sync's modification times).
    forced: str | None = None
    # The command that judges the step's stored results again under today's known lists, without a run (a whole
    # suite's `--rejudge`; check.py, `can_rejudge`; OpenSpec change check-rejudge, design D2).
    rejudge: tuple[str, ...] | str | None = None


def runner(suite: str, *extra: str) -> tuple[str, ...]:
    return ("python", RUNNER, "--suite", suite, "--qb64", EXE, *extra)


# The coverage records of the merge's whole suites, one file each, which check.py's libqb narrowing reads.
COUNTS = "target/coverage/merge"
# (step, runner suite, pass list): the merge's whole suites, and the list each stands for.
WHOLE = (
    ("corpus", "corpus", "tests/corpus/slice.list"),
    ("upstream", "compile", "tests/upstream/pass.list"),
    ("verification", "verification", "tests/verification/pass.list"),
)
# The steps the merge leaves out: its whole suites run every program of their lists, and write the results that
# `--check-list` reads, which a run of a list would replace.
MERGE_REPLACES = frozenset({"slice", "upstream", "verification", "linux-slice"})


def whole(name: str, suite: str) -> str:
    # The programs add to the counts file: it is removed first, so it holds this run's alone.
    counts = f"{COUNTS}/{name}.txt"
    return (
        f"rm -f {counts} && mkdir -p {COUNTS} && python {RUNNER} --suite {suite} --qb64 {EXE} --jobs 8 "
        f"--build-cache target/build-cache --coverage {counts}"
    )


# The Linux steps run in WSL, in this tree's Linux copy (tools/linux/run.sh; one copy per source tree, named by
# tools/linux/copy.sh, OpenSpec change linux-sync-per-tree, design D1). `$(pwd -W)` is Git bash's Windows path
# of the tree, which `wsl --cd` takes; the command after run.sh is one single-quoted argument.
LINUX_SYNC = 'wsl --cd "$(pwd -W)" -e bash tools/linux/sync.sh'
IN_COPY = 'wsl --cd "$(pwd -W)" -e bash tools/linux/run.sh '
# What the Linux steps are run through: a change to the rule runs them again.
LINUX_SCRIPTS = ("tools/linux/copy.sh", "tools/linux/run.sh")


def linux_whole(suite: str) -> str:
    # Each builds first, so that none runs an older build beside another's: cargo builds nothing when it is current.
    # The sync copies no git-ignored file, so the v37 programs' libraries are built in the copy.
    libs = "python3 verification/v37_lib/build_libs.py && " if suite == "verification" else ""
    return (
        f"{IN_COPY}'cargo build --release --locked && {libs}{linux_runner(suite)} --jobs 8 "
        "--build-cache target/build-cache'"
    )


def linux_runner(suite: str) -> str:
    # The compile suite reads the reference clone beside the Windows tree: there is none beside the copy.
    root = ' --qb-root "$QB64RUST_TREE/../QB64pe"' if suite == "compile" else ""
    return f"python3 tools/legacy_tests/run_legacy_tests.py --suite {suite} --qb64 target/release/qb64rust{root}"


# The lists that only judge a run, never change what a program does: a step whose inputs differ from its last run
# only in these has its stored results judged again (`Step.rejudge`; OpenSpec change check-rejudge, design D2).
# They must name every file the runner's `known_lists` reads (test_check.py holds them to it).
JUDGE_LISTS = (
    "tools/legacy_tests/known_failures.txt",
    "tests/*/known.list",
    "tests/*/known.linux.list",
)


# `nextest` and `lists` split the workspace's tests by whether a test is inputs.rs's (the folded driver binary's
# module `inputs`); without cargo-nextest, `nextest` falls back to `cargo test` (FALLBACK), inputs.rs's tests too.
NOT_INPUTS = "not (package(qb64rust-driver) & test(/^inputs::/))"

STEPS: tuple[Step, ...] = (
    Step("fmt", QUICK, ("cargo", "fmt", "--all", "--check"), RUST),
    Step("build", QUICK, ("cargo", "build", "--release"), BUILT),
    Step("clippy", QUICK, ("cargo", "clippy", "--all-targets"), RUST),
    Step(
        "nextest",
        QUICK,
        ("cargo", "nextest", "run", "--workspace", "--locked", "--no-fail-fast", "-E", NOT_INPUTS),
        RUST + RUST_TEST_READS + LIBQB + (".config/nextest.toml",),
    ),
    Step(
        "lists",
        QUICK,
        # Under `cargo test`, not nextest: inputs.rs computes the outcome of every input once per process and its
        # tests share it, so nextest's process per test took 85 s against 33 s (2026-10-31).
        ("cargo", "test", "-p", "qb64rust-driver", "--locked", "--test", "it", "inputs::"),
        # The crates' sources, not their tests: a test file other than inputs.rs (and the binary's `main.rs` and
        # `support`, which it is built with) changes nothing inputs.rs computes. It reads the reference clone's
        # qbasic test cases and sources too.
        LISTS_RUST + ("tests/**", "verification/**", "!verification/*.exe"),
        external=("clone",),
    ),
    Step("doc", QUICK, ("cargo", "test", "--workspace", "--locked", "--doc"), RUST),
    Step(
        "tools",
        QUICK,
        ("python", "tools/check/tool_tests.py"),
        ("tools/**/*.py", "tools/**/*.txt", "tools/**/*.json"),
    ),
    Step(
        "libqb-copy",
        QUICK,
        ("python", "tools/libqb/copy_libqb.py", "--check"),
        LIBQB + ("tools/libqb/*.py",),
        external=("clone",),
    ),
    Step(
        "slice",
        QUICK,
        runner("corpus", *CACHES, "--pass-list", "--list", "tests/corpus/slice.list"),
        BUILT + LIBQB + RUNNER_FILES + ("tests/corpus/**", "tests/upstream/root/**"),
        needs=("build",),
        external=("toolchain",),
        suite="corpus",
        program_list="tests/corpus/slice.list",
    ),
    Step(
        "upstream",
        FULL,
        runner("compile", *CACHES, "--pass-list", "--list", "tests/upstream/pass.list"),
        BUILT + LIBQB + RUNNER_FILES + ("tests/upstream/**",),
        needs=("build",),
        external=("toolchain", "clone"),
        suite="compile",
        program_list="tests/upstream/pass.list",
    ),
    # The libraries the v37 programs link, built into verification/ with the reference clone's C compiler. A source
    # edit builds them again, and `verification` after it, which takes this step's fingerprint: the libraries are
    # git-ignored, so no fingerprint holds them, and libraries deleted by hand are built again by
    # `just check v37-libs`. `clone` sees the clone's commit, not its compiler (ignored there); `toolchain` the
    # programs link with. In the quick layer, which runs the verification programs a change reaches.
    Step(
        "v37-libs",
        QUICK,
        ("python", "verification/v37_lib/build_libs.py"),
        ("verification/v37_lib/**",),
        external=("clone", "toolchain"),
    ),
    Step(
        "verification",
        FULL,
        runner("verification", *CACHES, "--pass-list", "--list", "tests/verification/pass.list"),
        BUILT + LIBQB + RUNNER_FILES + ("verification/**", "!verification/*.exe", "tests/verification/**"),
        needs=("build", "v37-libs"),
        external=("toolchain",),
        suite="verification",
        program_list="tests/verification/pass.list",
        side_lists=("tests/verification/known.list", "tests/verification/known.linux.list"),
    ),
    Step(
        "callsite",
        FULL,
        ("python", "tools/callsite/callsite.py", "--jobs", "8", "--wait"),
        BUILT + ("tools/callsite/**", "tests/callsite/**", "tests/upstream/root/**"),
        needs=("build",),
        external=("clone",),
    ),
    Step(
        "cli",
        FULL,
        ("cargo", "test", "-p", "qb64rust-driver", "--locked", "--test", "it", "--", "--ignored"),
        RUST + LIBQB + ("verification/**", "!verification/*.exe", "tests/corpus/slice/**", "tests/upstream/root/**"),
        external=("toolchain", "clone"),
        alone=True,
    ),
    Step(
        "qbasic",
        FULL,
        runner("qbasic", "--jobs", "8", "--wait"),
        BUILT + LIBQB + RUNNER_FILES,
        needs=("build",),
        external=("toolchain", "clone"),
    ),
    Step(
        "addprefix",
        FULL,
        runner("addprefix", "--wait"),
        BUILT + LIBQB + RUNNER_FILES,
        needs=("build",),
        external=("toolchain", "clone"),
    ),
    Step("format", FULL, runner("format"), BUILT + RUNNER_FILES, needs=("build",), external=("clone",)),
    Step(
        "format-corpus",
        FULL,
        runner("format-corpus", "--jobs", "8"),
        BUILT + RUNNER_FILES + ("tests/format/**", "tools/format/**"),
        needs=("build",),
    ),
    Step(
        "differential",
        FULL,
        runner(
            "corpus",
            "--corpus-root",
            "tests/differential",
            *CACHES,
            "--results",
            "target/legacy-tests-differential",
            "--pass-list",
            "--list",
            "tests/differential/pass.list",
        ),
        BUILT + LIBQB + RUNNER_FILES + ("tests/differential/**", "tests/upstream/root/**"),
        needs=("build",),
        external=("toolchain",),
    ),
    Step(
        "nofold",
        FULL,
        runner(
            "corpus",
            *CACHES[:4],
            "--pass-cache",
            "target/pass-cache-nofold",
            "--results",
            "target/legacy-tests-nofold",
            "--wait",
            "--pass-list",
            "--list",
            "tests/corpus/slice.list",
        ),
        BUILT + LIBQB + RUNNER_FILES + ("tests/corpus/**", "tests/upstream/root/**"),
        needs=("build",),
        external=("toolchain",),
        env={"QB64RUST_NO_FOLD": "1"},
    ),
    Step(
        "coverage",
        FULL,
        ("python", "tools/coverage/coverage.py", "--check"),
        BUILT + LIBQB + RUNNER_FILES + ("tools/coverage/**", "tests/corpus/screen/**", "tests/programs/**"),
        needs=("build",),
        external=("toolchain",),
    ),
    # The copy gets the files git reports changed since the last sync; under --force (the merge) every file,
    # compared by content (design D2).
    Step(
        "linux-sync",
        FULL,
        LINUX_SYNC,
        RUST + LIBQB + RUNNER_FILES + ("tools/linux/**", "tests/**", "verification/**", "!verification/*.exe"),
        forced=LINUX_SYNC + " --checksum",
    ),
    Step(
        "linux-tier1",
        FULL,
        IN_COPY + "'cargo test --workspace --locked --no-fail-fast'",
        RUST + RUST_TEST_READS + LIBQB + LINUX_SCRIPTS,
        needs=("linux-sync",),
    ),
    Step(
        "linux-slice",
        FULL,
        IN_COPY + "'cargo build --release --locked && python3 "
        "tools/legacy_tests/run_legacy_tests.py --suite corpus --qb64 target/release/qb64rust --jobs 8 "
        "--build-cache target/build-cache --pass-cache target/pass-cache-2 --list tests/corpus/slice.list'",
        BUILT + LIBQB + RUNNER_FILES + ("tools/linux/**", "tests/corpus/**", "tests/upstream/root/**"),
        needs=("linux-sync",),
    ),
    Step(
        "sandbox",
        FULL,
        ("python", "tools/sandbox/run.py"),
        # It runs in the test VM (tools/winvm), whose scripts are inputs too. Not alone: the VM takes no screen, and
        # the family passed there with the verification step loading the host to 88-100% (OpenSpec change
        # windows-vm, task 4.2). `--where sandbox`, run by hand, still wants the machine to itself.
        BUILT + LIBQB + ("tools/sandbox/**", "tools/winvm/**", "tests/sandbox/**", "tests/never_run.list"),
        needs=("build",),
        external=("toolchain",),
    ),
)

_BY_NAME = {s.name: s for s in STEPS}
_WORDING = BUILT + ("tools/diagnostics/**", "tests/**", "verification/**", "!verification/*.exe")

# The merge's own steps (the merge set of the old tools/close/close.py, git history). In `check merge` none stands:
# it runs every step again. A step named alone (`check counts`) follows the rule of a named step, and the steps it
# needs may stand.
STEPS += (
    Step(
        "update-lists",
        MERGE,
        ("cargo", "test", "-p", "qb64rust-driver", "--locked", "--test", "it", "inputs::"),
        _BY_NAME["lists"].inputs,
        external=("clone",),
        first=True,
        env={"QB64RUST_UPDATE_LISTS": "1"},
    ),
    # The table is written from the rejections compiled by the release build, and the compiler holds the table:
    # it is built first here, and `build` builds the new table in.
    Step(
        "wording",
        MERGE,
        "cargo build --release && python tools/diagnostics/old_wording.py",
        _WORDING,
        first=True,
    ),
    Step("wording-check", MERGE, ("python", "tools/diagnostics/old_wording.py", "--check"), _WORDING, needs=("build",)),
    *(
        Step(
            f"{name}-whole",
            MERGE,
            whole(name, suite),
            _BY_NAME["slice" if name == "corpus" else name].inputs,
            needs=_BY_NAME["slice" if name == "corpus" else name].needs,
            external=("toolchain", "clone"),
            rejudge=runner(suite, "--rejudge"),
        )
        for name, suite, _ in WHOLE
    ),
    *(
        Step(
            f"{name}-list",
            MERGE,
            runner(suite, "--check-list", path),
            _BY_NAME["slice" if name == "corpus" else name].inputs,
            needs=(f"{name}-whole",),
            after_failure=True,
        )
        for name, suite, path in WHOLE
    ),
    Step(
        "builtins",
        MERGE,
        ("python", "tools/coverage/builtin_runs.py", "--check", *(f"{COUNTS}/{name}.txt" for name, _, _ in WHOLE)),
        BUILT + ("tools/coverage/**",),
        needs=tuple(f"{name}-whole" for name, _, _ in WHOLE),
        after_failure=True,
    ),
    # A str: bash runs the release build by its path from the repository's top.
    Step(
        "demand-corpus",
        MERGE,
        f"{EXE} --report unsupported tests/corpus tests/programs",
        BUILT + ("tests/corpus/**", "tests/programs/**"),
        needs=("build",),
    ),
    Step(
        "demand-upstream",
        MERGE,
        f"{EXE} --report unsupported --include-root tests/upstream/root tests/upstream/compile_tests",
        BUILT + ("tests/upstream/**",),
        needs=("build",),
    ),
    # After the whole suites, whose results it reads; `--strict` fails a block left as it is (no results, or of
    # another build), which at a merge means a whole suite did not run as it should.
    Step(
        "counts",
        MERGE,
        ("python", "tools/legacy_tests/counts.py", "--strict"),
        ("tools/legacy_tests/counts.py", "tests/**"),
        needs=tuple(f"{name}-whole" for name, _, _ in WHOLE),
        after_failure=True,
    ),
    *(
        Step(
            f"linux-{name}",
            MERGE,
            linux_whole(suite),
            _BY_NAME["linux-slice"].inputs + _BY_NAME["slice" if name == "corpus" else name].inputs,
            needs=("linux-sync",),
            rejudge=f"{IN_COPY}'{linux_runner(suite)} --rejudge'",
        )
        for name, suite, _ in WHOLE
    ),
)

BY_NAME = {s.name: s for s in STEPS}

# Without cargo-nextest: the same tests under `cargo test` (check.py says so when it uses one).
FALLBACK: dict[str, tuple[str, ...]] = {
    "nextest": ("cargo", "test", "--workspace", "--locked", "--no-fail-fast"),
}
