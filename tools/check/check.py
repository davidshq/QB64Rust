#!/usr/bin/env python3
"""The one entry point for testing (spec testing/check-workflow; tools/check/README.md).

    python tools/check/check.py quick          # fmt, lints, the Rust tests, the tool tests, the slice list
    python tools/check/check.py full           # every suite; in a worktree of a commit (`git worktree add`)
    python tools/check/check.py merge          # the merge into main: every step again, the whole suites, the
                                               # lists, tables and records regenerated; in a worktree too
    python tools/check/check.py status         # each step: standing, to run, or failed, with its last run
    python tools/check/check.py history        # per day: runs, steps run and standing, wall time (--days N)
    python tools/check/check.py <target>       # a step, a crate folder, a suite folder or list, or one program
    ... --force                                # run again what stands, without the pass cache
    ... --plan                                 # print the plan and run nothing
    ... --wait                                 # stay in the foreground, however long the run

A step (tools/check/steps.py) records its result in target/check/<step>.json with the fingerprint of every file
its inputs match (SHA-256 over paths and bytes) and of the inputs outside the repository. A passing result stands
while the fingerprint is unchanged, whatever the commit or branch; a failed one never stands. A plan longer than a
minute (by the steps' last times) goes to the background, its report in target/check/run.log. Nothing here
refuses a command or holds a session: `check` runs tests, it gates nothing.

A change a step can be narrowed for runs only its programs (`narrow`): a suite's new or changed programs and the
ones added to its list, or, for an edit of libqb whose edit files all name their built-ins (`builtins:`), the
programs whose coverage records hold those built-ins (target/coverage/merge/*.txt, written at a merge). In the
quick layer the full layer's program suites run when, and only when, a change narrows to their programs.
"""

from __future__ import annotations

import argparse
import bisect
import hashlib
import json
import os
import re
import stat
import subprocess
import sys
import time
from dataclasses import dataclass, field
from functools import lru_cache
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from steps import (  # noqa: E402
    BY_NAME,
    COUNTS,
    EXE,
    FALLBACK,
    FULL,
    JUDGE_LISTS,
    MERGE,
    MERGE_REPLACES,
    QUICK,
    RUNNER,
    STEPS,
    Step,
)

ROOT = Path(__file__).resolve().parents[2]
# A plan expected to take longer than this many seconds goes to the background.
BACKGROUND_SECONDS = 60
# The time a step with no earlier run is taken to need, for that choice and for the order.
UNKNOWN_SECONDS = 120
# How many changed inputs a report names before "and N more".
SHOWN = 3
# The folders whose uncommitted changes make `full` and `merge` refuse to start: their results are to describe one
# commit.
FULL_CLEAN = ("crates", "libqb", "tests")
# Windows finds System32's bash.exe (WSL's) before Git's: the steps given as a str are Git's bash's (`pwd -W`).
# bin/bash.exe first: it is Git's launcher, which puts Git's usr/bin on the PATH. usr/bin/bash.exe started from a
# PATH without it, as from PowerShell, finds no `basename` or `tr` (tools/linux/copy.sh).
GIT_BASH = ("bin/bash.exe", "usr/bin/bash.exe")
EDIT_BUILTINS = re.compile(r"^builtins:[ \t]*(.+?)[ \t]*$", re.MULTILINE)

# Where a step runs (OpenSpec change vm-suites, design D9): on the host, or in the suites VM (tools/winvm/README.md,
# "The suites VM"), which only a step marked `vm` may.
HOST, VM = "host", "vm"
SUITES_PS1 = "tools/winvm/suites.ps1"
STEP_PS1 = "tools/winvm/step.ps1"
# What a step in the VM is run through: inputs of each VM step besides its own. Only the files that take part in a
# step, not create.ps1, run.ps1, the README or the tests: a new file that does must be added here.
VM_INPUTS = ("tools/winvm/suites.ps1", "tools/winvm/step.ps1", "tools/winvm/outputs.py")
# How long `suites.ps1 -Check` may take (Hyper-V's cmdlets hung), and `-Start` (a new clone copied in: about 4 min).
VM_CHECK_SECONDS = 120
VM_START_SECONDS = 1800
# A VM step's limit: three times its last time, at least this (design D6). An hour: `verification` took 26.5 min in
# the VM with a cold pass cache (task 6.3), and a pass served by the cache sets a far shorter last time.
VM_LIMIT_FLOOR = 3600
# How much longer than a step's limit step.ps1 may take as a whole: the copies in and out, the session, the cleanup.
VM_STEP_SLACK = 600
# The CLI tests' binary as check.py builds it for the VM (design D9): cargo's command and the target's name.
CLI_BUILD = ("cargo", "test", "-p", "qb64rust-driver", "--locked", "--test", "it", "--no-run", "--message-format=json")
CLI_TARGET = "it"
CLI_DIR = "crates/driver"
# The debug build that the CLI tests' binary runs (CARGO_BIN_EXE_qb64rust), copied into the VM beside it.
CLI_COMPILER = "target/debug/qb64rust.exe"


def state(root: Path) -> Path:
    return root / "target" / "check"


# --- The files and their hashes -------------------------------------------------------------------------------


def repo_files(root: Path) -> list[str]:
    """Every file git tracks or would track (untracked but not ignored), repo-relative with `/`. A tracked file
    deleted from the working tree is among them; hashing leaves it out (`Hashes.of`)."""
    out = subprocess.run(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--deduplicate"],
        cwd=root,
        capture_output=True,
        check=True,
    ).stdout
    return sorted({p for p in out.decode("utf-8", "surrogateescape").split("\0") if p})


def glob_regex(pattern: str) -> re.Pattern[str]:
    """A step's glob as a regular expression: `**` crosses folders, `*` and `?` do not."""
    out, i = [], 0
    while i < len(pattern):
        if pattern.startswith("**/", i):
            out.append("(?:.*/)?")
            i += 3
        elif pattern.startswith("**", i):
            out.append(".*")
            i += 2
        elif pattern[i] == "*":
            out.append("[^/]*")
            i += 1
        elif pattern[i] == "?":
            out.append("[^/]")
            i += 1
        else:
            out.append(re.escape(pattern[i]))
            i += 1
    return re.compile("".join(out) + r"\Z")


def literal_prefix(pattern: str) -> str:
    return re.split(r"[*?]", pattern, maxsplit=1)[0]


def matching(globs: tuple[str, ...], files: list[str]) -> list[str]:
    """The files (sorted) the globs take, in order: a `!glob` takes back what an earlier one took. Each glob looks
    only at the files that start with its literal prefix, found by bisection."""
    taken: set[str] = set()
    for g in globs:
        negative = g.startswith("!")
        pattern = g.lstrip("!")
        prefix, rx = literal_prefix(pattern), glob_regex(pattern)
        lo = bisect.bisect_left(files, prefix)
        hi = bisect.bisect_left(files, prefix + "\U0010ffff")
        hit = {f for f in files[lo:hi] if rx.match(f)}
        taken = taken - hit if negative else taken | hit
    return sorted(taken)


class Hashes:
    """SHA-256 of files, remembered by size and modification time in target/check/hashes.json (as git's index
    does), so that a plan reads only the files changed since the last one."""

    def __init__(self, root: Path):
        self.root = root
        self.path = state(root) / "hashes.json"
        try:
            self.known: dict[str, list] = json.loads(self.path.read_text(encoding="utf-8"))
        except (OSError, ValueError):
            self.known = {}
        self.dirty = False
        self.base = str(root) + os.sep
        # The hashes of this run: each file is looked at once, however many steps read it.
        self.now: dict[str, str | None] = {}

    def of(self, rel: str) -> str | None:
        """The file's hash, or None when it is not there (a tracked file deleted, or a folder)."""
        if rel in self.now:
            return self.now[rel]
        path = self.base + rel
        try:
            st = os.stat(path)
        except OSError:
            self.now[rel] = None
            return None
        if not stat.S_ISREG(st.st_mode):
            self.now[rel] = None
            return None
        known = self.known.get(rel)
        if known and known[0] == st.st_size and known[1] == st.st_mtime_ns:
            digest = known[2]
        else:
            with open(path, "rb") as f:
                digest = hashlib.sha256(f.read()).hexdigest()
            self.known[rel] = [st.st_size, st.st_mtime_ns, digest]
            self.dirty = True
        self.now[rel] = digest
        return digest

    def save(self) -> None:
        if self.dirty:
            self.path.parent.mkdir(parents=True, exist_ok=True)
            tmp = self.path.with_suffix(".tmp")
            tmp.write_text(json.dumps(self.known), encoding="utf-8")
            tmp.replace(self.path)


def toolchain_dir(root: Path) -> Path | None:
    """The C++ toolchain the driver finds without options (crates/README.md): the environment's, the installed
    one, else the reference clone's."""
    env = os.environ.get("QB64RUST_TOOLCHAIN_ROOT")
    if env:
        return Path(env)
    home = os.environ.get("QB64RUST_TOOLCHAIN_HOME") or (
        os.path.join(os.environ["LOCALAPPDATA"], "qb64rust", "toolchain") if os.environ.get("LOCALAPPDATA") else ""
    )
    if home and Path(home).is_dir():
        return Path(home)
    clone = root.parent / "QB64pe" / "internal" / "c" / "c_compiler"
    return clone if clone.is_dir() else None


@lru_cache(maxsize=None)
def external(name: str, root: Path) -> str:
    """The fingerprint of an input outside the repository: the toolchain by its executables' names, sizes and
    times; the reference clone by its commit and its uncommitted changes; the community clones by their commits."""
    if name == "toolchain":
        d = toolchain_dir(root)
        if d is None:
            return "toolchain: none"
        stamps = []
        for p in sorted((d / "bin").glob("*")) if (d / "bin").is_dir() else []:
            if p.is_file():
                st = p.stat()
                stamps.append(f"{p.name} {st.st_size} {st.st_mtime_ns}")
        return f"toolchain: {d.as_posix()}\n" + "\n".join(stamps)
    if name == "clone":
        clone = root.parent / "QB64pe"
        r = subprocess.run(["git", "-C", str(clone), "rev-parse", "HEAD"], capture_output=True, text=True)
        s = subprocess.run(["git", "-C", str(clone), "status", "--porcelain"], capture_output=True, text=True)
        return f"clone: {r.stdout.strip() or 'none'}\n{s.stdout}"
    if name == "community":
        # The commit each community clone is at, from the heads.txt that `community.py fetch` writes
        # (tools/community/community.py): a clone fetched to another commit runs the step again.
        env = os.environ.get("QB64RUST_COMMUNITY")
        heads = (Path(env) if env else root.parent / "community") / "heads.txt"
        text = heads.read_text(encoding="utf-8", errors="replace") if heads.is_file() else "none\n"
        return f"community:\n{text}"
    raise ValueError(f"no external input {name!r}")


# --- Fingerprints, records and the plan -----------------------------------------------------------------------


@dataclass
class Inputs:
    fingerprint: str
    files: dict[str, str]
    lists: dict[str, list[str]] = field(default_factory=dict)
    # The fingerprint without the lists that only judge a run (JUDGE_LISTS) and with the needs' run fingerprints:
    # equal to a record's while only those lists changed, so that the step's stored results can be judged again.
    run: str = ""


def list_entries(root: Path, rel: str) -> list[str]:
    p = root / rel
    if not p.is_file():
        return []
    return [
        line.strip()
        for line in p.read_text(encoding="utf-8", errors="replace").splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]


def place_of(step: Step, where: str) -> str:
    """Where a step of a run asking for `where` runs: in the VM only when it may (design D9)."""
    return VM if step.vm and where == VM else HOST


def inputs_of(
    step: Step, root: Path, files: list[str], hashes: Hashes, done: dict[str, Inputs], place: str = HOST
) -> Inputs:
    """A step's fingerprint: its command, every input file by path and hash, its external inputs, the fingerprints
    of the steps it needs, and its place when that is the VM (design D10: a pass in one place never stands for the
    other; the host adds nothing, so that the host's records from before the VM still stand)."""
    h = hashlib.sha256()
    run = hashlib.sha256()
    for x in (h, run):
        x.update(repr((step.name, step.command, sorted(step.env.items()))).encode())
        if place == VM:
            x.update(b"where: vm\n")
    globs = step.inputs + (VM_INPUTS if place == VM else ())
    taken = {f: d for f in matching(globs, files) if (d := hashes.of(f)) is not None}
    judge = set(matching(JUDGE_LISTS, list(taken)))
    for f, digest in taken.items():
        line = f"{f}\0{digest}\n".encode("utf-8", "surrogateescape")
        h.update(line)
        if f not in judge:
            run.update(line)
    for name in step.external:
        ext = external(name, root).encode("utf-8", "surrogateescape")
        h.update(ext)
        run.update(ext)
    for need in step.needs:
        if need in done:
            h.update(done[need].fingerprint.encode())
            run.update(done[need].run.encode())
    lists = {p: list_entries(root, p) for p in (step.program_list, *step.side_lists) if p}
    return Inputs(h.hexdigest(), taken, lists, run.hexdigest())


def record_path(root: Path, name: str) -> Path:
    return state(root) / f"{name}.json"


def read_record(root: Path, name: str) -> dict | None:
    try:
        return json.loads(record_path(root, name).read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return None


def write_record(root: Path, name: str, record: dict) -> None:
    p = record_path(root, name)
    p.parent.mkdir(parents=True, exist_ok=True)
    tmp = p.with_suffix(".tmp")
    tmp.write_text(json.dumps(record), encoding="utf-8")
    tmp.replace(p)


def changed_files(old: dict[str, str], new: dict[str, str]) -> list[str]:
    """The inputs added, changed or removed since `old`."""
    return sorted(f for f in set(old) | set(new) if old.get(f) != new.get(f))


def program_of(step: Step, path: str, root: Path) -> str | None:
    """The program of the step's suite that a file belongs to, as its list names it, or None when the file is
    not one program's (a list, the runner, a support folder or file of several)."""
    parts = path.split("/")
    if step.suite == "verification":
        if len(parts) == 2 and parts[0] == "verification":
            # A file beside no program of its name is a support item (v16_inc_a.bi for every v16 program,
            # cp437_all_bytes.bin for all: the runner's verification_support), or a program's deleted file.
            stem = parts[1].split(".")[0]
            return stem if (root / "verification" / f"{stem}.bas").is_file() else None
        if len(parts) == 4 and parts[:2] == ["tests", "verification"] and parts[2] in ("normalize", "divergences", "fixed"):
            return parts[3].split(".")[0]
        return None
    if step.suite == "corpus" and parts[:2] == ["tests", "corpus"] and len(parts) >= 4:
        # <group>/<name>.<anything>, or a file of <group>/<name>.files/.
        for i in range(3, len(parts)):
            if parts[i - 1].endswith(".files"):
                return "/".join(parts[2 : i - 1] + [parts[i - 1][: -len(".files")]])
        return "/".join(parts[2:-1] + [parts[-1].split(".")[0]])
    if step.suite == "compile" and parts[:3] == ["tests", "upstream", "compile_tests"] and len(parts) == 5:
        # Every file of a category's folder is an input of each of its programs (the runner's data_files).
        return parts[3] + "/*"
    if step.suite == "compile" and parts[:2] == ["tests", "upstream"] and len(parts) == 5:
        if parts[2] in ("external", "divergences"):
            return f"{parts[3]}/{parts[4].split('.')[0]}"
    return None


def program_files_only(step: Step, changed: list[str], root: Path) -> set[str] | None:
    """The programs a change of the step's inputs comes to when every changed file is one program's or the
    step's list; None when one is not."""
    progs = set()
    for f in changed:
        if f == step.program_list or f in step.side_lists:
            continue
        p = program_of(step, f, root)
        if p is None:
            return None
        progs.add(p)
    return progs


def libqb_builtins(root: Path, changed: list[str]) -> set[str] | None:
    """The built-ins a libqb edit changes, when the change is libqb's alone and every edit file it changes or
    adds names them (`builtins:`); None otherwise."""
    if not changed or not all(f.startswith(("libqb/", "tools/libqb/edits/")) for f in changed):
        return None
    edits = [f for f in changed if f.startswith("tools/libqb/edits/")]
    if not edits:
        return None
    names: set[str] = set()
    for f in edits:
        p = root / f
        if not p.is_file():
            return None
        m = EDIT_BUILTINS.search(p.read_text(encoding="utf-8", errors="replace"))
        if not m:
            return None
        names.update(m.group(1).split())
    return names


def programs_holding(root: Path, builtins: set[str]) -> set[str] | None:
    """The file names (`v01_numeric.bas`) of the programs whose coverage records hold any of the built-ins,
    from the counts of the last merge; None when there are none to read."""
    counts = sorted((root / COUNTS).glob("*.txt"))
    if not counts:
        return None
    sys.path.insert(0, str(ROOT / "tools" / "coverage"))
    from builtin_runs import Builtins  # pylint: disable=import-outside-toplevel

    b = Builtins()
    for p in counts:
        b.add(p)
    out: set[str] = set()
    for name in builtins:
        out |= b.held.get(name.upper(), set())
    return out


def select(step: Step, root: Path, programs: set[str] | None, held: set[str] | None) -> list[str]:
    """The entries of the step's list that the narrowed change reaches: the programs by name (a `<category>/*`
    for every program of a category), or by file name from the coverage records."""
    entries = list_entries(root, step.program_list) if step.program_list else []
    out = []
    for e in entries:
        stem = e.split("/")[-1]
        group = "/".join(e.split("/")[:-1])
        if programs is not None and (
            e in programs
            or f"{group}/*" in programs
            # A script's test (<name>.<script>) is its program's.
            or ("/".join(filter(None, [group, stem.split(".")[0]])) in programs)
        ):
            out.append(e)
        elif held is not None and f"{stem.split('.')[0]}.bas" in held:
            out.append(e)
    return out


@dataclass
class Planned:
    step: Step
    inputs: Inputs
    reason: str
    # The list entries of a narrowed run, or None for the whole step.
    only: list[str] | None = None
    # "coverage" when this run, or one since the step last ran whole, was narrowed by the coverage records: a guess
    # (records of the last merge, without the programs added since), which stands for `quick`, never for `full`.
    narrowed: str | None = None
    # The step's stored results judged again (its `rejudge` command), not a run: only the known lists changed.
    rejudge: bool = False
    # Planned by `check merge`: its record says so, and only such a pass stands for a later merge.
    merge: bool = False
    # Run in its usual form under the merge, not its `forced` one: the Linux sync after an earlier merge's passed in
    # this tree, whose copy was then checked by content and has followed git since (check-rejudge, design D6).
    usual: bool = False
    # HOST or VM (design D9).
    where: str = HOST


@dataclass
class Plan:
    run: list[Planned]
    standing: list[tuple[Step, dict]]
    left: list[tuple[Step, str]]
    # Why the suites VM cannot be used, when the run asked for it with `--where vm`: its VM steps then fail with
    # this, and the host's run (design D9).
    vm_refusal: str | None = None


def describe(changed: list[str]) -> str:
    more = f" and {len(changed) - SHOWN} more" if len(changed) > SHOWN else ""
    return ", ".join(changed[:SHOWN]) + more


def with_needs(names: list[str]) -> list[str]:
    """The named steps and every step they need, in the steps' order (a step's needs come before it)."""
    wanted = set()
    todo = list(names)
    while todo:
        n = todo.pop()
        if n not in wanted:
            wanted.add(n)
            todo.extend(BY_NAME[n].needs)
    return [s.name for s in STEPS if s.name in wanted]


REJUDGE_REASON = "re-judge: only the known lists changed"


def can_rejudge(step: Step, record: dict | None, inputs: Inputs) -> bool:
    """The step's stored results can be judged again instead of run (OpenSpec change check-rejudge, design D2): it
    has a re-judge, and since its last run, passed or failed, only the lists that judge it changed. With nothing
    changed it runs: a failure may have been the machine's, and a refused re-judge needs a run."""
    return (
        step.rejudge is not None
        and bool(record)
        and record.get("run_fingerprint") == inputs.run
        and record.get("fingerprint") != inputs.fingerprint
    )


def merge_reason(record: dict | None, inputs: Inputs) -> str:
    if not record or not record.get("merge"):
        return "no pass of an earlier merge"
    if record.get("result") != "pass":
        return "failed last time"
    return f"changed: {describe(changed_files(record.get('files', {}), inputs.files)) or 'its command or a step it needs'}"


def make_plan(
    root: Path,
    layer: str,
    force: bool = False,
    names: list[str] | None = None,
    asked: tuple[str, ...] = (),
    merge: bool = False,
    where: str = HOST,
) -> Plan:
    """Which steps of the layer (or the named ones) run, which stand, and which the quick layer leaves to the full
    one, with why. `names` must hold the steps the named ones need (with_needs): a step's fingerprint takes theirs,
    so that it is the same whether the step was named or run in its layer. A step in `asked` runs even when it
    stands; the others by the usual rule. `merge` (and the merge layer): a step stands only on a pass of an earlier
    merge with the same fingerprint (design D3 of check-rejudge). `where`: VM puts the steps that may run in the
    suites VM there (`place_of`), which their fingerprints hold."""
    external.cache_clear()
    files = repo_files(root)
    hashes = Hashes(root)
    done: dict[str, Inputs] = {}
    plan = Plan([], [], [])
    if names:
        wanted = [BY_NAME[n] for n in names]
    elif layer == MERGE:
        # A result of the merge describes the merge's commit alone: only an earlier merge's pass stands. The writers
        # (`first`) are a plan of their own, run before this one is made (`main`), so that every fingerprint here
        # is of the files they wrote.
        wanted = [s for s in STEPS if s.name not in MERGE_REPLACES and not s.first]
        merge = True
    else:
        wanted = [
            s
            for s in STEPS
            if s.layer == QUICK or (s.layer == FULL and (layer == FULL or s.suite in ("compile", "verification")))
        ]
    for step in wanted:
        inputs = inputs_of(step, root, files, hashes, done, place_of(step, where))
        done[step.name] = inputs
        record = read_record(root, step.name)
        if force:
            plan.run.append(Planned(step, inputs, "--force", merge=merge))
            continue
        if merge:
            if (
                record
                and record.get("merge")
                and record.get("result") == "pass"
                and record.get("fingerprint") == inputs.fingerprint
            ):
                plan.standing.append((step, record))
            elif record and record.get("merge") and can_rejudge(step, record, inputs):
                plan.run.append(Planned(step, inputs, REJUDGE_REASON, rejudge=True, merge=True))
            else:
                usual = bool(
                    step.forced is not None and record and record.get("merge") and record.get("result") == "pass"
                )
                reason = merge_reason(record, inputs) + ("; its usual form: an earlier merge's passed" if usual else "")
                plan.run.append(Planned(step, inputs, reason, merge=True, usual=usual))
            continue
        if can_rejudge(step, record, inputs):
            plan.run.append(Planned(step, inputs, REJUDGE_REASON, rejudge=True))
            continue
        if step.name in asked:
            plan.run.append(Planned(step, inputs, "asked for"))
            continue
        guessed = bool(record and record.get("narrowed") == "coverage")
        if guessed and layer == FULL and record.get("result") == "pass":
            plan.run.append(Planned(step, inputs, "its last pass ran only the programs of the coverage records"))
            continue
        if record and record.get("result") == "pass" and record.get("fingerprint") == inputs.fingerprint:
            plan.standing.append((step, record))
            continue
        quick_only = layer == QUICK and step.layer == FULL and not names
        if not record:
            if quick_only:
                plan.left.append((step, "no earlier run"))
            else:
                plan.run.append(Planned(step, inputs, "no earlier run"))
            continue
        if record.get("result") != "pass":
            if quick_only:
                plan.left.append((step, "failed last time"))
            else:
                plan.run.append(Planned(step, inputs, "failed last time"))
            continue
        changed = changed_files(record.get("files", {}), inputs.files)
        reason = f"changed: {describe(changed)}" if changed else "changed: its command, a step it needs or an input outside the repository"
        only, how = (
            narrowed(step, root, changed, record, inputs) if changed and step.suite and layer == QUICK else (None, None)
        )
        # A narrowed run on top of a guessed one is still a guess: the mark goes when the step next runs whole.
        mark = "coverage" if only is not None and (how == "coverage" or guessed) else None
        if quick_only and only is None:
            plan.left.append((step, reason))
        elif only == []:
            # Nothing of the list is reached: the result stands for the new fingerprint.
            plan.run.append(Planned(step, inputs, reason + " (no program of its list reached)", [], mark))
        else:
            plan.run.append(Planned(step, inputs, reason, only, mark))
    for p in plan.run:
        p.where = place_of(p.step, where)
    hashes.save()
    return plan


def narrowed(
    step: Step, root: Path, changed: list[str], record: dict, inputs: Inputs
) -> tuple[list[str] | None, str | None]:
    """The list entries a change narrows the step to and how ("files" or "coverage"), or (None, None) when it
    cannot be narrowed: the programs whose files changed, the entries added to its list, and the programs of the
    rows of its side lists that changed; for a libqb edit naming its built-ins, the programs the coverage records
    of the last merge show running them."""
    old = record.get("lists", {})
    added = set(inputs.lists.get(step.program_list, [])) - set(old.get(step.program_list, []))
    for side in step.side_lists:
        rows = set(inputs.lists.get(side, [])) ^ set(old.get(side, []))
        added |= {row.split("\t")[0].strip() for row in rows}
    programs = program_files_only(step, changed, root)
    if programs is not None:
        return sorted(select(step, root, programs | added, None)), "files"
    own_lists = {step.program_list, *step.side_lists}
    builtins = libqb_builtins(root, [f for f in changed if program_of(step, f, root) is None and f not in own_lists])
    if builtins:
        held = programs_holding(root, builtins)
        if held is not None:
            extra = program_files_only(step, [f for f in changed if program_of(step, f, root) is not None], root)
            return sorted(select(step, root, (extra or set()) | added, held)), "coverage"
    return None, None


def order(planned: list[Planned], root: Path) -> list[Planned]:
    """Failures first, then the longest by their last time, each after the steps it needs; a step that runs
    alone after the others, the sandbox family last of all. (The merge's writers are a plan of their own, `main`.)"""

    def last(p: Planned) -> tuple[int, float]:
        r = read_record(root, p.step.name) or {}
        return (0 if r.get("result") == "fail" else 1, -float(r.get("seconds", UNKNOWN_SECONDS)))

    pending = sorted(planned, key=lambda p: (p.step.name == "sandbox", p.step.alone, last(p)))
    names = {p.step.name for p in planned}
    out: list[Planned] = []
    while pending:
        for p in pending:
            if all(n not in names or n in {q.step.name for q in out} for n in p.step.needs):
                out.append(p)
                pending.remove(p)
                break
        else:
            out += pending
            break
    return out


# --- Running ----------------------------------------------------------------------------------------------------


def have_nextest() -> bool:
    try:
        return subprocess.run(["cargo", "nextest", "--version"], capture_output=True).returncode == 0
    except OSError:
        return False


INSTALL = {
    "nextest": "cargo install cargo-nextest --locked",
    "just": "cargo install just --locked",
}


def command_of(p: Planned, root: Path, force: bool, nextest: bool) -> tuple[list[str] | str, str]:
    """The command of a planned step and a note on how it differs from the step's own."""
    step, note = p.step, ""
    cmd = step.command
    if p.rejudge and step.rejudge is not None:
        return step.rejudge, "re-judged: no program ran"
    if force and step.forced is not None and not p.usual:
        cmd, note = step.forced, "forced: the exact form"
    if step.name in FALLBACK and not nextest:
        cmd = FALLBACK[step.name]
        note = f"cargo-nextest not found, ran `cargo test` (install: {INSTALL['nextest']})"
    if isinstance(cmd, str):
        return cmd, note
    args = list(cmd)
    if force and "--pass-cache" in args:
        i = args.index("--pass-cache")
        del args[i : i + 2]
    if p.only is not None and "--list" in args:
        lst = state(root) / "lists" / f"{step.name}.list"
        lst.parent.mkdir(parents=True, exist_ok=True)
        lst.write_text("".join(f"{e}\n" for e in p.only), encoding="utf-8")
        args[args.index("--list") + 1] = lst.relative_to(root).as_posix()
        note = (note + "; " if note else "") + f"{len(p.only)} program(s) of {step.program_list}"
    return args, note


@lru_cache(maxsize=None)
def bash() -> str | None:
    """Git's bash on Windows (found from `git --exec-path`, <git>/mingw64/libexec/git-core), else None: the PATH's
    `bash` there may be WSL's. `bash` elsewhere."""
    if os.name != "nt":
        return "bash"
    try:
        exec_path = subprocess.run(["git", "--exec-path"], capture_output=True, text=True).stdout.strip()
    except OSError:
        exec_path = ""
    parents = Path(exec_path).parents if exec_path else []
    if len(parents) > 2:
        top = parents[2]
        for rel in GIT_BASH:
            if (top / rel).is_file():
                return str(top / rel)
    return None


# --- The suites VM (OpenSpec change vm-suites, design D6, D8, D9, D11) -------------------------------------------


def powershell(root: Path, script: str, *args: str) -> list[str]:
    """A script of tools/winvm with `args`, under Windows PowerShell, as tools/sandbox/run.py starts run.ps1."""
    return ["powershell.exe", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", str(root / script), *args]


def last_line(text: str) -> str:
    lines = [ln.strip() for ln in text.splitlines() if ln.strip()]
    return lines[-1] if lines else ""


def vm_refusal(root: Path) -> str | None:
    """Why the suites VM cannot be used here, or None: not Windows, a CI run, or `suites.ps1 -Check`'s refusal (no
    Hyper-V, no VM, the VM running, ...), which leaves the VM as it is."""
    if os.name != "nt":
        return "not Windows"
    if os.environ.get("CI") or os.environ.get("GITHUB_ACTIONS"):
        return "a CI run"
    try:
        r = subprocess.run(
            powershell(root, SUITES_PS1, "-Check"), cwd=root, capture_output=True, text=True, timeout=VM_CHECK_SECONDS
        )
    except (OSError, subprocess.TimeoutExpired) as e:
        return f"{SUITES_PS1} -Check did not run: {e}"
    if r.returncode == 0:
        return None
    why = last_line(r.stdout + r.stderr).removeprefix("suites-vm: refused: ").rstrip(".")
    return why or f"{SUITES_PS1} -Check: exit {r.returncode}"


class SuitesVM:
    """The suites VM for one `check` run (design D6): restored and filled (`suites.ps1 -Start`) before the first VM
    step, turned off (`-Stop`) after the last one and at the end of the run whatever happened. A refusal known when
    the run was planned, or a failed start, fails every VM step with its reason."""

    def __init__(self, root: Path, refusal: str | None = None, community: bool = False):
        self.root = root
        self.refusal = refusal
        # The run has the step `community`: `-Start` also copies the community clones in (design D7).
        self.community = community
        self.started = False
        self.tried = False

    def start(self) -> str | None:
        """None when the VM is ready, else why it is not."""
        if self.refusal or self.started:
            return self.refusal
        self.tried = True
        print(f"suites VM: starting ({SUITES_PS1} -Start)", flush=True)
        try:
            r = subprocess.run(
                powershell(self.root, SUITES_PS1, "-Start", "-Tree", str(self.root), *(["-Community"] if self.community else [])),
                cwd=self.root,
                capture_output=True,
                text=True,
                timeout=VM_START_SECONDS,
            )
            out, code = r.stdout + r.stderr, r.returncode
        except (OSError, subprocess.TimeoutExpired) as e:
            out, code = str(e), 1
        for line in out.splitlines():
            if line.strip():
                print(f"  {line.rstrip()}", flush=True)
        if code != 0:
            self.refusal = (last_line(out).removeprefix("suites-vm: refused: ") or f"-Start: exit {code}")
            self.stop()
            return self.refusal
        self.started = True
        return None

    def stop(self) -> None:
        if not self.tried:
            return
        self.tried = self.started = False
        try:
            r = subprocess.run(
                powershell(self.root, SUITES_PS1, "-Stop"), cwd=self.root, capture_output=True, text=True,
                timeout=VM_CHECK_SECONDS,
            )
            said = last_line(r.stdout + r.stderr).removeprefix("suites-vm: ")
            print(f"suites VM: {said or 'turned off'}", flush=True)
        except (OSError, subprocess.TimeoutExpired) as e:
            print(f"suites VM: {SUITES_PS1} -Stop did not run ({e}); the VM may still be on", flush=True)


def cli_binary(root: Path, out) -> str | None:
    """The CLI tests' binary, built on the host (`cargo test --no-run`, its output to the step's log): its path
    relative to the tree, from cargo's messages, or None when it was not built."""
    r = subprocess.run(CLI_BUILD, cwd=root, stdout=subprocess.PIPE, stderr=out)
    found = None
    for line in r.stdout.decode("utf-8", "replace").splitlines():
        try:
            msg = json.loads(line)
        except ValueError:
            continue
        if msg.get("reason") == "compiler-artifact" and msg.get("target", {}).get("name") == CLI_TARGET:
            found = msg.get("executable") or found
    if r.returncode != 0 or not found:
        return None
    return Path(os.path.relpath(found, root)).as_posix()


# Set in every VM step (design D4): the runner then ignores --build-cache, also where a tool starts it
# (coverage.py), and the pass cache serves what passed while the rest builds cold.
NO_BUILD_CACHE = {"QB64RUST_NO_BUILD_CACHE": "1"}
# Also set in every VM step: the runner skips network programs on a Windows host, where Windows Defender Firewall
# asks the user about a program that opens a port (CLAUDE.md working rule 6, 2026-11-05). The VM asks no one.
ALLOW_NETWORK = {"QB64RUST_ALLOW_NETWORK": "1"}
# And the clipboard programs, which the runner skips on a Windows host so that they do not overwrite the user's
# clipboard (DECISIONS.md 2026-11-07). No one uses the VM's clipboard.
ALLOW_CLIPBOARD = {"QB64RUST_ALLOW_CLIPBOARD": "1"}


def passed_seconds(record: dict | None) -> float:
    """The time of a step's last whole pass, from its record: `passed_seconds`, or for a record written before that
    field, its `seconds` when it passed. 0 when unknown."""
    if not record:
        return 0.0
    if "passed_seconds" in record:
        return float(record["passed_seconds"] or 0)
    return float(record.get("seconds", 0)) if record.get("result") == "pass" else 0.0


def vm_limit(root: Path, name: str) -> int:
    """Three times the step's last whole pass in the VM, else its last whole pass anywhere, at least VM_LIMIT_FLOOR
    (design D6). The VM's time first: the VM keeps no build cache, so a pass on the host can be far quicker."""
    record = read_record(root, name) or {}
    seconds = float(record.get("vm_passed_seconds") or 0) or passed_seconds(record)
    return max(VM_LIMIT_FLOOR, int(3 * seconds))


def run_in_vm(p: Planned, root: Path, cmd: list[str], out, vm: SuitesVM | None) -> tuple[int, str]:
    """A step in the suites VM through step.ps1 (design D6), which prints the guest's log as it grows: the exit code
    and the note of the record."""
    why = vm.start() if vm else "no VM session in this run"
    if why:
        out.write(f"the suites VM cannot be used: {why}\n".encode())
        return 1, f"the suites VM cannot be used: {why}"
    copy = [EXE] if (root / EXE).is_file() else []
    cwd = ""
    if p.step.name == "cli":
        out.write(f"$ {' '.join(CLI_BUILD)}\n".encode())
        out.flush()
        binary = cli_binary(root, out)
        if binary is None:
            return 1, "the CLI tests' binary was not built"
        cmd, cwd = [binary, "--ignored"], CLI_DIR
        copy += [binary, CLI_COMPILER]
    # A file under target\ that the command names, such as a narrowed run's list (target/check/lists): git ignores
    # target\, so the tree's copy in the VM has none.
    copy += [a for a in cmd if a.startswith("target/") and a not in copy and (root / a).is_file()]
    job = {
        "step": p.step.name,
        "command": list(cmd),
        "first": list(p.step.vm_first) if p.step.vm_first else None,
        "cwd": cwd,
        "env": {**p.step.env, **NO_BUILD_CACHE, **ALLOW_NETWORK, **ALLOW_CLIPBOARD},
        "copy": copy,
        "outputs": list(p.step.outputs),
        "clears": list(p.step.clears),
        "limit": vm_limit(root, p.step.name),
    }
    path = state(root) / "vm" / f"{p.step.name}.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(job, indent=1), encoding="utf-8")
    out.write(f"$ (in the suites VM, {cwd or '.'}) {' '.join(job['command'])}\n".encode())
    out.flush()
    try:
        rc = subprocess.run(
            powershell(root, STEP_PS1, "-Tree", str(root), "-Job", str(path)),
            cwd=root,
            stdout=out,
            stderr=subprocess.STDOUT,
            timeout=job["limit"] + VM_STEP_SLACK,
        ).returncode
    except subprocess.TimeoutExpired:
        # step.ps1 stops the step at its limit itself; this is for Hyper-V's cmdlets hanging around it.
        out.write(f"\nstep.ps1 did not end within {job['limit'] + VM_STEP_SLACK} s; it was stopped\n".encode())
        return 1, f"in the suites VM; step.ps1 hung past {job['limit'] + VM_STEP_SLACK} s"
    return rc, "in the suites VM"


def clear(root: Path, files: tuple[str, ...]) -> None:
    for rel in files:
        p = root / rel
        p.unlink(missing_ok=True)
        p.parent.mkdir(parents=True, exist_ok=True)


def toolchain_path(root: Path) -> str:
    """What `qb64rust toolchain path` prints on the host, or "" when the release build is not there."""
    exe = root / EXE
    if not exe.is_file():
        return ""
    try:
        return subprocess.run([str(exe), "toolchain", "path"], capture_output=True, text=True, timeout=60).stdout
    except (OSError, subprocess.TimeoutExpired):
        return ""


def toolchain_note(root: Path) -> str | None:
    """The report's line when the host's builds use the QB64pe clone's toolchain, qb64rust's fallback, rather than an
    installed one (design D11): nothing noticed that until 2026-11-03."""
    lines = toolchain_path(root).splitlines()
    if len(lines) > 1 and lines[1].startswith("clone:"):
        return (
            "the programs built on the host use the QB64pe clone's C++ toolchain, qb64rust's fallback "
            "(`qb64rust toolchain path`); install one with `qb64rust toolchain install`"
        )
    return None


def run_step(p: Planned, root: Path, force: bool, nextest: bool, vm: SuitesVM | None = None) -> dict:
    logs = state(root) / "logs"
    logs.mkdir(parents=True, exist_ok=True)
    log = logs / f"{p.step.name}.log"
    started = time.time()
    if p.only == []:
        log.write_text("nothing of the list reached by the change\n", encoding="utf-8")
        rc, note = 0, "no program reached"
    else:
        cmd, note = command_of(p, root, force, nextest)
        env = {**os.environ, **p.step.env}
        shell = bash() if isinstance(cmd, str) else ""
        # A re-judge reads the stored results on the host, wherever they were made.
        in_vm = p.where == VM and not p.rejudge
        with log.open("wb") as out:
            if in_vm and isinstance(cmd, list):
                # The host's copies go first: a VM step that brings nothing back must not leave the last run's
                # counts or results for the steps after it (`builtins`, the `-list` steps) to read.
                clear(root, p.step.clears + tuple(o for o in p.step.outputs if o.endswith(".json")))
                rc, where_note = run_in_vm(p, root, cmd, out, vm)
                note = f"{note}; {where_note}" if note else where_note
            else:
                if not p.rejudge:
                    clear(root, p.step.clears)
                out.write(f"$ {cmd if isinstance(cmd, str) else ' '.join(cmd)}\n".encode())
                out.flush()
                if shell is None:
                    out.write(b"Git's bash not found (from `git --exec-path`): the step is a bash line\n")
                    rc, note = 1, "Git's bash not found"
                else:
                    argv = [shell, "-c", cmd] if isinstance(cmd, str) else cmd
                    rc = subprocess.run(argv, cwd=root, stdout=out, stderr=subprocess.STDOUT, env=env).returncode
    record = {
        "fingerprint": p.inputs.fingerprint,
        "run_fingerprint": p.inputs.run,
        "merge": p.merge,
        "rejudged": p.rejudge,
        "where": p.where,
        "result": "pass" if rc == 0 else "fail",
        "started": started,
        "seconds": round(time.time() - started, 1),
        "log": log.relative_to(root).as_posix(),
        "note": note,
        "files": p.inputs.files,
        "lists": p.inputs.lists,
    }
    if p.narrowed:
        record["narrowed"] = p.narrowed
    # The time of the step's last whole pass, anywhere and in the VM, which a VM step's limit is taken from
    # (`vm_limit`). A failure or a narrowed run keeps the earlier ones: a step that failed in a second must not cut
    # the next run's limit.
    previous = read_record(root, p.step.name)
    whole_pass = rc == 0 and p.only is None and not p.rejudge
    record["passed_seconds"] = record["seconds"] if whole_pass else passed_seconds(previous)
    in_vm_time = record["seconds"] if whole_pass and p.where == VM else (previous or {}).get("vm_passed_seconds")
    if in_vm_time:
        record["vm_passed_seconds"] = in_vm_time
    write_record(root, p.step.name, record)
    return record


def when(t: float) -> str:
    return time.strftime("%Y-%m-%d %H:%M", time.localtime(t))


def in_vm_word(where: str | None) -> str:
    return " in the VM" if where == VM else ""


def print_plan(plan: Plan) -> None:
    for step, record in plan.standing:
        print(
            f"  standing  {step.name:<18} passed{in_vm_word(record.get('where'))} {when(record['started'])}, "
            f"{record['seconds']:g}s"
        )
    for step, why in plan.left:
        print(f"  left      {step.name:<18} for `check full` ({why})")
    for p in plan.run:
        extra = f" [{len(p.only)} program(s)]" if p.only else ""
        print(f"  to run    {p.step.name:<18} {p.reason}{extra}{in_vm_word(p.where)}")
    if not plan.run:
        print("nothing to run")


def uses_vm(p: Planned) -> bool:
    return p.where == VM and not p.rejudge and p.only != []


def execute(root: Path, plan: Plan, force: bool, what: str = "") -> int:
    nextest = have_nextest()
    failed = 0
    failed_names: set[str] = set()
    skipped: set[str] = set()
    ran: dict[str, list] = {}
    started = time.time()
    steps_in_order = order(plan.run, root)
    in_vm = [p for p in steps_in_order if uses_vm(p)]
    vm = SuitesVM(root, plan.vm_refusal, any(p.step.name == "community" for p in in_vm)) if in_vm else None
    told_toolchain = False
    try:
        for p in steps_in_order:
            # A step that reads what its needs wrote runs after a failed one, not after one that did not run.
            broken = [n for n in p.step.needs if n in (skipped if p.step.after_failure else failed_names)]
            if broken:
                # Run on an older build, its result would say nothing of this one: not run, not recorded.
                why = ", ".join(f"{n} {'did not run' if n in skipped else 'failed'}" for n in broken)
                print(f"  SKIP    {p.step.name:<18} not run: {why}", flush=True)
                failed += 1
                failed_names.add(p.step.name)
                skipped.add(p.step.name)
                ran[p.step.name] = ["skip", 0]
            else:
                builds_here = p.where == HOST and "toolchain" in p.step.external and p.only != [] and not p.rejudge
                if builds_here and not told_toolchain:
                    told_toolchain = True
                    note = toolchain_note(root)
                    if note:
                        print(f"note: {note}", flush=True)
                print(f"run       {p.step.name}{in_vm_word(p.where)} ...", flush=True)
                r = run_step(p, root, force, nextest, vm)
                ran[p.step.name] = [r["result"], r["seconds"]]
                tag = "PASS" if r["result"] == "pass" else "FAIL"
                note = f"  ({r['note']})" if r["note"] else ""
                print(f"  {tag}    {p.step.name:<18} {r['seconds']:g}s  {r['log']}{note}", flush=True)
                if r["result"] != "pass":
                    failed += 1
                    failed_names.add(p.step.name)
            # The VM is off before the steps after the last VM step: the sandbox family's VM may start then.
            if vm and p is in_vm[-1]:
                vm.stop()
    finally:
        if vm:
            vm.stop()
    print(f"{len(plan.run) - failed} passed, {failed} failed, {len(plan.standing)} standing")
    log_history(root, what, ran, plan, time.time() - started)
    return 1 if failed else 0


# --- The history of runs ----------------------------------------------------------------------------------------


def history_path(root: Path) -> Path:
    return state(root) / "history.jsonl"


def log_history(root: Path, what: str, ran: dict[str, list], plan: Plan | None, seconds: float) -> None:
    """One line per `check` run in target/check/history.jsonl: what was asked, each step run with its result and
    seconds, the steps that stood and those left to `full`, the wall time. `check history` sums it per day, the
    measure of the time testing takes (the change testing-reset; the record of tools\\testtime, archived, did this
    from the session logs)."""
    line = {
        "date": time.strftime("%Y-%m-%d %H:%M:%S"),
        "what": what,
        "seconds": round(seconds, 1),
        "ran": ran,
        "standing": [s.name for s, _ in plan.standing] if plan else [],
        "left": [s.name for s, _ in plan.left] if plan else [],
    }
    try:
        p = history_path(root)
        p.parent.mkdir(parents=True, exist_ok=True)
        with p.open("a", encoding="utf-8") as f:
            f.write(json.dumps(line) + "\n")
    except OSError:
        pass


def read_history(root: Path) -> list[dict]:
    try:
        lines = history_path(root).read_text(encoding="utf-8").splitlines()
    except OSError:
        return []
    out = []
    for line in lines:
        try:
            out.append(json.loads(line))
        except ValueError:
            pass
    return out


def history(root: Path, days: int) -> int:
    """Per day: the runs, the steps run and standing, the failures and the wall time; then the steps that took the
    most time over those days."""
    rows = read_history(root)
    if not rows:
        print("no runs recorded yet (target/check/history.jsonl)")
        return 0
    by_day: dict[str, list[dict]] = {}
    for r in rows:
        by_day.setdefault(r["date"][:10], []).append(r)
    shown = sorted(by_day)[-days:]
    print(f"  {'day':<10} {'runs':>4} {'ran':>4} {'stood':>5} {'failed':>6} {'wall':>8}")
    per_step: dict[str, list[float]] = {}
    for day in shown:
        runs = by_day[day]
        ran = sum(len(r["ran"]) for r in runs)
        stood = sum(len(r["standing"]) for r in runs)
        bad = sum(1 for r in runs for res, _ in r["ran"].values() if res != "pass")
        wall = sum(r["seconds"] for r in runs)
        print(f"  {day:<10} {len(runs):>4} {ran:>4} {stood:>5} {bad:>6} {wall / 60:>7.1f}m")
        for r in runs:
            for name, (_, s) in r["ran"].items():
                t = per_step.setdefault(name, [0, 0.0])
                t[0] += 1
                t[1] += s
    print("  the steps that took the most time:")
    for name, (n, s) in sorted(per_step.items(), key=lambda kv: -kv[1][1])[:8]:
        print(f"    {name:<16} {n:>4} runs {s / 60:>7.1f}m")
    return 0


def estimate(root: Path, plan: Plan) -> float:
    return sum(
        float((read_record(root, p.step.name) or {}).get("seconds", UNKNOWN_SECONDS)) for p in plan.run if p.only != []
    )


def to_background(root: Path, argv: list[str]) -> int:
    log = state(root) / "run.log"
    log.parent.mkdir(parents=True, exist_ok=True)
    out = log.open("wb")
    kw: dict = {"cwd": root, "stdout": out, "stderr": subprocess.STDOUT, "stdin": subprocess.DEVNULL}
    if os.name == "nt":
        # A console of its own that is never shown, which every program the run starts shares. With no console at
        # all (DETACHED_PROCESS), Windows opened a new window for each console program a step started.
        kw["creationflags"] = subprocess.CREATE_NO_WINDOW | subprocess.CREATE_NEW_PROCESS_GROUP
    else:
        kw["start_new_session"] = True
    subprocess.Popen([sys.executable, str(Path(__file__).resolve()), *argv, "--wait"], **kw)
    print(f"running in the background; report: {log.relative_to(root).as_posix()} (`check status` for each step)")
    return 0


def dirty(root: Path) -> list[str]:
    out = subprocess.run(
        ["git", "status", "--porcelain", "--", *FULL_CLEAN], cwd=root, capture_output=True, text=True
    ).stdout
    return [line[3:] for line in out.splitlines() if line.strip()]


# --- Targets ----------------------------------------------------------------------------------------------------

SUITE_FOLDERS = {
    "verification": "verification",
    "tests/verification": "verification",
    "tests/corpus": "slice",
    "tests/upstream": "upstream",
    "tests/callsite": "callsite",
    "tests/sandbox": "sandbox",
    "tests/differential": "differential",
    "tests/format": "format-corpus",
    "tools": "tools",
}


def crate_package(root: Path, folder: Path) -> str | None:
    toml = root / folder / "Cargo.toml"
    if not toml.is_file():
        return None
    m = re.search(r'^name\s*=\s*"([^"]+)"', toml.read_text(encoding="utf-8"), re.MULTILINE)
    return m.group(1) if m else None


def find_program(root: Path, name: str) -> tuple[str, list[str]] | None:
    """The runner's suite and selection for a program named alone: a verification program, a corpus program or
    script, or a compile-suite program."""
    base = name.split(".")[0]
    if (root / "verification" / f"{base}.bas").is_file():
        return "verification", [name]
    for bas in sorted((root / "tests" / "corpus").rglob(f"{base}.bas")):
        group = bas.parent.relative_to(root / "tests" / "corpus").as_posix()
        return "corpus", ["--list", f"{group}/{name}"]
    for bas in sorted((root / "tests" / "upstream" / "compile_tests").glob(f"*/{base}.bas")):
        return "compile", ["--list", f"{bas.parent.name}/{name}"]
    return None


def target_command(root: Path, target: str, force: bool) -> tuple[list[str], Path | None] | str:
    """The command for `check <target>` (a step name is run as a step instead), or the reason there is none."""
    t = target.replace("\\", "/").rstrip("/")
    nextest = have_nextest()
    if t.startswith("crates/") and t.count("/") == 1:
        pkg = crate_package(root, Path(t))
        if not pkg:
            return f"{t} is not a crate folder"
        if nextest:
            return ["cargo", "nextest", "run", "-p", pkg, "--no-fail-fast"], None
        return ["cargo", "test", "-p", pkg, "--no-fail-fast"], None
    if t.startswith("tools/") and (root / t).is_dir():
        return ["python", "-m", "unittest", "discover", "-s", t, "-p", "test_*.py"], None
    found = find_program(root, t)
    if found is None:
        return f"no step, crate, suite folder, list or program named {target}"
    suite, selection = found
    tmp = None
    if selection[0] == "--list":
        tmp = state(root) / "lists" / "target.list"
        tmp.parent.mkdir(parents=True, exist_ok=True)
        tmp.write_text(selection[1] + "\n", encoding="utf-8")
        selection = ["--list", tmp.relative_to(root).as_posix()]
    from steps import CACHES, runner  # pylint: disable=import-outside-toplevel

    caches = list(CACHES)
    if force:
        caches = caches[:4]
    return list(runner(suite, *caches, *selection)), tmp


def no_clone(root: Path) -> str | None:
    """Why `full` and `merge` cannot run here, or None: their steps read the reference clone beside the checkout,
    and a worktree made elsewhere has none (it would fail step by step, an hour into the run)."""
    clone = root.parent / "QB64pe"
    if (clone / "internal" / "c").is_dir():
        return None
    return (
        f"no reference clone at {clone.as_posix()}: a worktree goes beside the checkout, so that ../QB64pe and "
        "../../qb64contain are the same folders (CLAUDE.md, \"Where things live\")."
    )


def program_needs(cmd: list[str]) -> list[str]:
    """The steps a runner command of `check <program>` needs first: those of its suite's step (the release build,
    and for a verification program the v37 libraries)."""
    suite = cmd[cmd.index("--suite") + 1]
    own = next((s for s in STEPS if s.suite == suite), None)
    return list(own.needs) if own else ["build"]


def step_of_target(target: str) -> str | None:
    t = target.replace("\\", "/").rstrip("/")
    if t in BY_NAME:
        return t
    if t in SUITE_FOLDERS:
        return SUITE_FOLDERS[t]
    for s in STEPS:
        if s.program_list == t or (isinstance(s.command, tuple) and t.endswith(".list") and t in s.command):
            return s.name
    return None


# --- The command line -------------------------------------------------------------------------------------------


def status(root: Path) -> int:
    # Each step judged in the place its last run was in.
    plans = {HOST: make_plan(root, FULL), VM: make_plan(root, FULL, where=VM)}
    for step in STEPS:
        r = read_record(root, step.name)
        if r is None:
            print(f"  never run {step.name}")
            continue
        where = r.get("where", HOST)
        standing = any(s.name == step.name for s, _ in plans.get(where, plans[HOST]).standing)
        state_word = "standing" if standing else ("FAILED" if r.get("result") != "pass" else "to run")
        print(
            f"  {state_word:<9} {step.name:<18} {r['result']}{in_vm_word(where)} {when(r['started'])}, "
            f"{r['seconds']:g}s  {r['log']}"
        )
    lock = state(root) / "run.log"
    if lock.is_file():
        print(f"last background report: {lock.relative_to(root).as_posix()}, {when(lock.stat().st_mtime)}")
    return 0


def main(argv: list[str] | None = None, root: Path = ROOT) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument(
        "what", help="quick, full, merge, status, history, or a step, crate folder, suite folder, list or program"
    )
    ap.add_argument("--force", action="store_true", help="run again what stands, without the pass cache")
    ap.add_argument("--plan", action="store_true", help="print the plan, run nothing")
    ap.add_argument("--wait", action="store_true", help="run in the foreground however long it takes")
    ap.add_argument("--days", type=int, default=14, help="history: how many of the last days with runs to show")
    ap.add_argument(
        "--where",
        choices=(HOST, VM),
        help="where the steps that run programs run: the host, or the suites VM (default: the VM for full and "
        "merge, the host otherwise)",
    )
    ap.add_argument(
        "--uncommitted",
        action="store_true",
        help="full: run on uncommitted changes rather than refuse (do not edit crates/, libqb/ or tests/ meanwhile)",
    )
    a = ap.parse_args(argv)
    if a.uncommitted and a.what != FULL:
        why = ": a merge's results are to describe the merge commit" if a.what == MERGE else ""
        print(f"--uncommitted applies to `check {FULL}` only{why}")
        return 2
    if a.what == MERGE:
        # Without the pass cache too, as the plan runs every step.
        a.force = True
    if a.what == "status":
        return status(root)
    if a.what == "history":
        return history(root, a.days)
    if a.what in (QUICK, FULL, MERGE):
        if a.what != QUICK and not a.plan:
            changes = dirty(root)
            if changes and a.uncommitted:
                # The user's choice, 2026-11-03: a commit before every full run costs a staging round. The line goes
                # into the report; the results stand by the files' contents, as always.
                print(
                    f"note: running on uncommitted changes under {', '.join(FULL_CLEAN)} ({describe(changes)}), "
                    "--uncommitted; an edit there during the run may leave a pass for files it did not test",
                    flush=True,
                )
            elif changes:
                print(
                    f"check {a.what} refused: uncommitted changes under {', '.join(FULL_CLEAN)} "
                    f"({describe(changes)}). Its results are to describe one commit: commit, then run it in a "
                    f"worktree of the commit (`git worktree add ../qb64rust-full HEAD`, then `just {a.what}` there; "
                    f"or `git checkout <commit>` in the worktree you have: what stands is not run again)."
                )
                return 2
            missing = no_clone(root)
            if missing:
                print(f"check {a.what} refused: {missing}")
                return 2
        where, refusal = choose_place(root, a.what, a.where)
        if a.what == MERGE:
            writers = make_plan(root, FULL, a.force, [s.name for s in STEPS if s.first], merge=True, where=where)
            print_plan(writers)
        plan = make_plan(root, a.what, a.force, where=where)
    else:
        name = step_of_target(a.what)
        if name is None:
            got = target_command(root, a.what, a.force)
            if isinstance(got, str):
                print(got)
                return 2
            cmd, _ = got
            if a.where == VM:
                print(f"note: a program named alone runs on the host; for the VM name its step with --where {VM}")
            if RUNNER in cmd:
                # A program runs on what its suite's step needs, unless they stand.
                first = make_plan(root, FULL, a.force, with_needs(program_needs(cmd)))
                print_plan(first)
                if first.run and execute(root, first, a.force, f"build for {a.what}") != 0:
                    return 1
            print("$ " + " ".join(cmd), flush=True)
            started = time.time()
            rc = subprocess.run(cmd, cwd=root).returncode
            took = time.time() - started
            log_history(root, a.what, {a.what: ["pass" if rc == 0 else "fail", round(took, 1)]}, None, took)
            return rc
        # Asked for by name, a step runs even when it stands (its pass cache serves what is unchanged); the steps
        # it needs run first unless they stand.
        where, refusal = choose_place(root, a.what, a.where, BY_NAME[name].vm)
        plan = make_plan(root, FULL, a.force, with_needs([name]), asked=(name,), where=where)
    plan.vm_refusal = refusal
    print_plan(plan)
    if a.plan:
        return 0
    if not plan.run:
        log_history(root, a.what, {}, plan, 0)
        return 0
    if not a.wait and estimate(root, plan) > BACKGROUND_SECONDS:
        return to_background(root, [x for x in argv if x != "--wait"])
    if a.what == MERGE:
        # The writers first, then the rest planned again: its fingerprints are of the files they wrote.
        bad = execute(root, writers, True, "merge: the writers") if writers.run else 0
        plan = make_plan(root, MERGE, a.force, where=where)
        plan.vm_refusal = refusal
        return execute(root, plan, True, a.what) or bad
    return execute(root, plan, a.force, a.what)


def choose_place(root: Path, what: str, asked: str | None, has_vm_steps: bool = True) -> tuple[str, str | None]:
    """Where the run's steps that may run in the VM go, and why the VM cannot be used when it was asked for (design
    D9): `--where` as given, else the VM for `full` and `merge` and the host otherwise. When the VM was not asked
    for but is the default and cannot be used, the host, with one line saying why."""
    where = asked or (VM if what in (FULL, MERGE) else HOST)
    if where != VM or not has_vm_steps:
        return where, None
    why = vm_refusal(root)
    if why is None:
        return VM, None
    if asked == VM:
        print(f"the suites VM cannot be used: {why}; the steps that run programs fail (`--where {VM}`)", flush=True)
        return VM, why
    print(f"the suites VM cannot be used ({why}); the steps that run programs run on the host", flush=True)
    return HOST, None


if __name__ == "__main__":
    sys.exit(main())
