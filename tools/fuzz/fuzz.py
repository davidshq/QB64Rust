#!/usr/bin/env python3
"""Run a fuzz target of the front end for a set time (OpenSpec change fuzz-front-end, design D5; tools/fuzz/README.md).

    python tools/fuzz/fuzz.py <parse|compile|server|all> <minutes> [--jobs N] [--sanitizer none|address]
    python tools/fuzz/fuzz.py --clean <target>

On Windows it syncs this tree's Linux copy (tools/linux/sync.sh) and runs itself again there, in WSL; a failing input
is copied back to the same path under this tree's fuzz/artifacts/. On Linux (WSL or GitHub's runner) it writes the
keyword dictionary, gathers the seeds, runs `cargo fuzz run` with the limits of spec testing/fuzzing, minimizes a
failing input with `cargo fuzz tmin`, and prints one summary line per target. `all` runs the three targets one after
another, each for the minutes given.

Exit status: 0 when no target found anything, 1 when one did, 2 when the run could not be made.
`--clean <target>` shrinks the kept corpus of a target (`cargo fuzz cmin`).
"""

from __future__ import annotations

import argparse
import os
import re
import shlex
import shutil
import signal
import subprocess
import sys
import time
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TARGETS = ("parse", "compile", "server")
# The limits of spec testing/fuzzing, "Time and memory limits".
MAX_LEN = 16 * 1024
TIMEOUT_S = 10
RSS_LIMIT_MB = 2048
# qb64rust_base::STACK_SIZE, which the compiler gives its own thread (design D4).
STACK_BYTES = 64 * 1024 * 1024
TMIN_SECONDS = 5 * 60
# Repository-relative paths, the same in the Windows tree and in its Linux copy.
DICT = "fuzz/target/keywords.dict"
SEEDS = "fuzz/target/seeds"


def nightly(root: Path = ROOT) -> str:
    """The channel of fuzz/rust-toolchain.toml."""
    with open(root / "fuzz" / "rust-toolchain.toml", "rb") as f:
        return tomllib.load(f)["toolchain"]["channel"]


def libfuzzer_flags(seconds: int, jobs: int, dictionary: str = DICT) -> list[str]:
    """libFuzzer's flags for one run. More than one job runs in fork mode; `-jobs` would start another job after a
    failure and go on. Fork mode goes on after a timeout or a memory failure unless told not to (measured,
    2026-11-12: the first campaign's `compile` ran its 30 minutes and wrote three timeouts), so it is told to stop
    at the first failure of any kind."""
    flags = [
        f"-max_total_time={seconds}",
        f"-max_len={MAX_LEN}",
        f"-timeout={TIMEOUT_S}",
        f"-rss_limit_mb={RSS_LIMIT_MB}",
        f"-dict={dictionary}",
        "-print_final_stats=1",
    ]
    if jobs > 1:
        flags += [f"-fork={jobs}", "-ignore_crashes=0", "-ignore_timeouts=0", "-ignore_ooms=0"]
    return flags


def run_command(channel: str, target: str, seconds: int, jobs: int, sanitizer: str) -> list[str]:
    """`cargo fuzz run` for one target: the kept corpus first (new inputs go there), then the seeds."""
    return [
        "cargo",
        f"+{channel}",
        "fuzz",
        "run",
        "--sanitizer",
        sanitizer,
        target,
        f"fuzz/corpus/{target}",
        SEEDS,
        "--",
        *libfuzzer_flags(seconds, jobs),
    ]


def build_command(channel: str, sanitizer: str) -> list[str]:
    return ["cargo", f"+{channel}", "fuzz", "build", "--sanitizer", sanitizer]


def tmin_command(channel: str, target: str, artifact: str, sanitizer: str) -> list[str]:
    return [
        "cargo",
        f"+{channel}",
        "fuzz",
        "tmin",
        "--sanitizer",
        sanitizer,
        target,
        artifact,
        "--",
        f"-max_total_time={TMIN_SECONDS}",
        f"-timeout={TIMEOUT_S}",
        f"-rss_limit_mb={RSS_LIMIT_MB}",
    ]


def cmin_command(channel: str, target: str, sanitizer: str) -> list[str]:
    return [
        "cargo",
        f"+{channel}",
        "fuzz",
        "cmin",
        "--sanitizer",
        sanitizer,
        target,
        f"fuzz/corpus/{target}",
        "--",
        f"-max_len={MAX_LEN}",
        f"-timeout={TIMEOUT_S}",
        f"-rss_limit_mb={RSS_LIMIT_MB}",
    ]


def dictionary_command() -> list[str]:
    """The ignored test of the driver that writes the dictionary to QB64RUST_FUZZ_DICT (design D5, step 1)."""
    return [
        "cargo",
        "test",
        "--locked",
        "-p",
        "qb64rust-driver",
        "--test",
        "it",
        "--",
        "--ignored",
        "--exact",
        "fuzz_regressions::write_fuzz_dictionary",
    ]


def inputs_tried(log: str) -> int | None:
    """The number of inputs a run tried, from libFuzzer's output: its final stats with one job, the last status
    line (`#N: cov: ...`) in fork mode, or its `Done N runs` line."""
    found = re.findall(r"^stat::number_of_executed_units:\s*(\d+)", log, re.MULTILINE)
    if found:
        return int(found[-1])
    found = re.findall(r"^#(\d+):? +(?:cov|DONE|pulse|NEW|REDUCE|INITED)", log, re.MULTILINE)
    if found:
        return int(found[-1])
    found = re.findall(r"^Done (\d+) runs", log, re.MULTILINE)
    return int(found[-1]) if found else None


def summary_line(target: str, seconds: float, inputs: int | None, findings: list[str]) -> str:
    """The line spec testing/fuzzing asks for: the target, the time run, the inputs tried, the failing inputs."""
    tried = "an unknown number of inputs" if inputs is None else f"{inputs} inputs"
    head = f"fuzz {target}: {seconds / 60:.1f} min, {tried}"
    if not findings:
        return f"{head}, no failure"
    return f"{head}, {len(findings)} failing: {' '.join(findings)}"


def seed_files(root: Path = ROOT) -> list[Path]:
    """The seeds (spec testing/fuzzing, "Seeds"): the corpus programs of at most MAX_LEN bytes, and every regression
    input."""
    corpus = [
        p
        for p in (root / "tests" / "corpus").rglob("*")
        if p.suffix.lower() == ".bas" and p.is_file()
    ]
    seeds = [p for p in corpus if p.stat().st_size <= MAX_LEN]
    regressions = root / "tests" / "fuzz" / "regressions"
    seeds += [p for p in regressions.glob("*") if p.is_file() and p.name != "README.md"]
    return sorted(seeds)


def fill_seeds(root: Path = ROOT) -> int:
    """Copies the seeds into SEEDS, which is emptied first, each under its path with `/` turned to `__`."""
    dest = root / SEEDS
    shutil.rmtree(dest, ignore_errors=True)
    dest.mkdir(parents=True)
    seeds = seed_files(root)
    for p in seeds:
        shutil.copyfile(p, dest / "__".join(p.relative_to(root).parts))
    return len(seeds)


def new_artifacts(root: Path, target: str, since: float) -> list[Path]:
    """The failing inputs a run wrote: the files of fuzz/artifacts/<target>/ written since `since`, but not those of
    tmin."""
    folder = root / "fuzz" / "artifacts" / target
    if not folder.is_dir():
        return []
    return sorted(
        p
        for p in folder.iterdir()
        if p.is_file() and p.stat().st_mtime >= since and not p.name.startswith("minimized-from-")
    )


def smallest(root: Path, target: str, artifact: Path, since: float) -> Path:
    """The smallest input tmin left for `artifact`: its newest `minimized-from-*` file, or the artifact itself when
    tmin could not shrink it."""
    folder = root / "fuzz" / "artifacts" / target
    tried = [p for p in folder.glob("minimized-from-*") if p.stat().st_mtime >= since]
    return min(tried, key=lambda p: (p.stat().st_size, -p.stat().st_mtime), default=artifact)


def wsl_command(tree: str, argv: list[str]) -> list[str]:
    """This script run again in the Linux copy of the Windows tree `tree`, with `--copy-to` naming the tree as WSL
    sees it, so that a failing input comes back to it."""
    args = " ".join(shlex.quote(a) for a in argv)
    script = (
        'tree="$PWD"; source tools/linux/copy.sh; cd "$(linux_copy "$tree")" && '
        f'python3 tools/fuzz/fuzz.py {args} --copy-to "$tree"'
    )
    return ["wsl", "--cd", tree, "-e", "bash", "-lc", script]


def run(cmd: list[str], **kwargs) -> subprocess.CompletedProcess:
    """A command whose exit status the caller reads."""
    return subprocess.run(cmd, check=False, **kwargs)


def run_bounded(cmd: list[str], seconds: float, **kwargs) -> int | None:
    """A command stopped after `seconds`, with every process it started (Linux only); `None` when it was stopped.
    `cargo fuzz tmin` needs it: its `-max_total_time` bounds each run of libFuzzer it starts, not the whole
    minimization (measured, 2026-11-12: one timeout input still minimizing after 15 minutes)."""
    with subprocess.Popen(cmd, start_new_session=True, **kwargs) as p:
        try:
            return p.wait(timeout=seconds)
        except subprocess.TimeoutExpired:
            os.killpg(p.pid, signal.SIGKILL)
            p.wait()
            return None


def on_windows(argv: list[str]) -> int:
    tree = str(ROOT)
    if run(["wsl", "--cd", tree, "-e", "bash", "tools/linux/sync.sh"]).returncode != 0:
        print("fuzz.py: the sync of the Linux copy failed", file=sys.stderr)
        return 2
    return run(wsl_command(tree, argv)).returncode


def write_dictionary(root: Path) -> bool:
    path = root / DICT
    path.parent.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, QB64RUST_FUZZ_DICT=str(path))
    r = run(
        dictionary_command(), cwd=root, env=env, capture_output=True, text=True, errors="replace"
    )
    if r.returncode != 0 or not path.is_file():
        sys.stderr.write(r.stdout + r.stderr)
        print("fuzz.py: the dictionary could not be written", file=sys.stderr)
        return False
    return True


def raise_stack() -> None:
    """The fuzzer runs the target on its main thread: give it the stack the compiler gives itself (design D4). The
    children of cargo and of fork mode inherit it."""
    import resource

    soft, hard = resource.getrlimit(resource.RLIMIT_STACK)
    want = STACK_BYTES if hard == resource.RLIM_INFINITY else min(STACK_BYTES, hard)
    if soft != resource.RLIM_INFINITY and soft < want:
        resource.setrlimit(resource.RLIMIT_STACK, (want, hard))


def fuzz_one(
    root: Path,
    channel: str,
    target: str,
    seconds: int,
    jobs: int,
    sanitizer: str,
    copy_to: Path | None,
) -> bool:
    """Runs one target; True when it found nothing."""
    (root / "fuzz" / "corpus" / target).mkdir(parents=True, exist_ok=True)
    log_path = root / "fuzz" / "target" / f"{target}.log"
    print(
        f"fuzz {target}: {seconds / 60:g} min, {jobs} job(s), log in {log_path.relative_to(root).as_posix()}",
        flush=True,
    )
    start = time.time()
    with open(log_path, "w", encoding="utf-8", errors="replace") as log:
        r = run(
            run_command(channel, target, seconds, jobs, sanitizer),
            cwd=root,
            stdout=log,
            stderr=subprocess.STDOUT,
        )
    elapsed = time.time() - start
    text = log_path.read_text(encoding="utf-8", errors="replace")
    found = new_artifacts(root, target, start)
    if r.returncode != 0 and not found:
        sys.stdout.write("\n".join(text.splitlines()[-30:]) + "\n")
        print(
            f"fuzz {target}: cargo fuzz ended with exit {r.returncode} and wrote no failing input",
            file=sys.stderr,
        )
        found_paths: list[str] = ["(none written; see the log)"]
    else:
        found_paths = []
        for artifact in found:
            tmin_start = time.time()
            rel = artifact.relative_to(root).as_posix()
            print(f"fuzz {target}: minimizing {rel} (at most {TMIN_SECONDS // 60} min)", flush=True)
            with open(
                root / "fuzz" / "target" / f"{target}-tmin.log",
                "w",
                encoding="utf-8",
                errors="replace",
            ) as log:
                run_bounded(
                    tmin_command(channel, target, rel, sanitizer),
                    TMIN_SECONDS,
                    cwd=root,
                    stdout=log,
                    stderr=subprocess.STDOUT,
                )
            small = smallest(root, target, artifact, tmin_start)
            found_paths.append(small.relative_to(root).as_posix())
            if copy_to is not None:
                for p in {artifact, small}:
                    dest = copy_to / p.relative_to(root)
                    dest.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(p, dest)
    print(summary_line(target, elapsed, inputs_tried(text), found_paths), flush=True)
    return not found_paths


def main(argv: list[str] | None = None) -> int:
    argv = sys.argv[1:] if argv is None else argv
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("target", nargs="?", choices=(*TARGETS, "all"))
    parser.add_argument("minutes", nargs="?", type=float)
    parser.add_argument("--jobs", type=int, default=2)
    parser.add_argument("--sanitizer", choices=("none", "address"), default="none")
    parser.add_argument(
        "--clean", choices=TARGETS, metavar="TARGET", help="shrink the kept corpus of a target"
    )
    parser.add_argument("--copy-to", type=Path, help=argparse.SUPPRESS)
    args = parser.parse_args(argv)
    if args.clean is None and (args.target is None or args.minutes is None):
        parser.error("give a target and a number of minutes, or --clean <target>")
    if args.jobs < 1 or (args.minutes is not None and args.minutes <= 0):
        parser.error("--jobs and the minutes must be positive")
    if os.name == "nt":
        return on_windows(argv)

    channel = nightly()
    if run(["cargo", f"+{channel}", "fuzz", "--version"], cwd=ROOT, capture_output=True).returncode:
        print(f"fuzz.py: no cargo-fuzz for {channel}; see tools/fuzz/README.md", file=sys.stderr)
        return 2
    raise_stack()
    if args.clean:
        (ROOT / "fuzz" / "corpus" / args.clean).mkdir(parents=True, exist_ok=True)
        return run(cmin_command(channel, args.clean, args.sanitizer), cwd=ROOT).returncode
    if not write_dictionary(ROOT):
        return 2
    # Built before the clock starts, so that the minutes given are fuzzing.
    if run(build_command(channel, args.sanitizer), cwd=ROOT).returncode != 0:
        print("fuzz.py: the fuzz targets did not build", file=sys.stderr)
        return 2
    print(f"fuzz: {fill_seeds(ROOT)} seeds", flush=True)
    seconds = max(1, round(args.minutes * 60))
    targets = TARGETS if args.target == "all" else (args.target,)
    for target in targets:
        # A run ends at the first failure (spec testing/fuzzing, "Starting a run"), `all` too.
        if not fuzz_one(ROOT, channel, target, seconds, args.jobs, args.sanitizer, args.copy_to):
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
