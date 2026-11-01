#!/usr/bin/env python3
"""The one entry point for testing (spec testing/check-workflow; tools/check/README.md).

    python tools/check/check.py quick          # fmt, lints, the Rust tests, the tool tests, the slice list
    python tools/check/check.py full           # every suite; in a worktree of a commit (`git worktree add`)
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
import shutil
import stat
import subprocess
import sys
import time
from dataclasses import dataclass, field
from functools import lru_cache
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from steps import BY_NAME, FALLBACK, FULL, QUICK, RUNNER, STEPS, Step  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
# A plan expected to take longer than this many seconds goes to the background.
BACKGROUND_SECONDS = 60
# The time a step with no earlier run is taken to need, for that choice and for the order.
UNKNOWN_SECONDS = 120
# How many changed inputs a report names before "and N more".
SHOWN = 3
# The folders whose uncommitted changes make `full` refuse to start: its results are to describe one commit.
FULL_CLEAN = ("crates", "libqb", "tests")
COUNTS = "target/coverage/merge"
EDIT_BUILTINS = re.compile(r"^builtins:[ \t]*(.+?)[ \t]*$", re.MULTILINE)


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
    times; the reference clone by its commit and its uncommitted changes."""
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
    raise ValueError(f"no external input {name!r}")


# --- Fingerprints, records and the plan -----------------------------------------------------------------------


@dataclass
class Inputs:
    fingerprint: str
    files: dict[str, str]
    lists: dict[str, list[str]] = field(default_factory=dict)


def list_entries(root: Path, rel: str) -> list[str]:
    p = root / rel
    if not p.is_file():
        return []
    return [
        line.strip()
        for line in p.read_text(encoding="utf-8", errors="replace").splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]


def inputs_of(step: Step, root: Path, files: list[str], hashes: Hashes, done: dict[str, Inputs]) -> Inputs:
    """A step's fingerprint: its command, every input file by path and hash, its external inputs and the
    fingerprints of the steps it needs."""
    h = hashlib.sha256()
    h.update(repr((step.name, step.command, sorted(step.env.items()))).encode())
    taken = {f: d for f in matching(step.inputs, files) if (d := hashes.of(f)) is not None}
    for f, digest in taken.items():
        h.update(f"{f}\0{digest}\n".encode("utf-8", "surrogateescape"))
    for name in step.external:
        h.update(external(name, root).encode("utf-8", "surrogateescape"))
    for need in step.needs:
        if need in done:
            h.update(done[need].fingerprint.encode())
    lists = {p: list_entries(root, p) for p in (step.program_list, *step.side_lists) if p}
    return Inputs(h.hexdigest(), taken, lists)


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


@dataclass
class Plan:
    run: list[Planned]
    standing: list[tuple[Step, dict]]
    left: list[tuple[Step, str]]


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


def make_plan(
    root: Path, layer: str, force: bool = False, names: list[str] | None = None, asked: tuple[str, ...] = ()
) -> Plan:
    """Which steps of the layer (or the named ones) run, which stand, and which the quick layer leaves to the full
    one, with why. `names` must hold the steps the named ones need (with_needs): a step's fingerprint takes theirs,
    so that it is the same whether the step was named or run in its layer. A step in `asked` runs even when it
    stands; the others by the usual rule."""
    external.cache_clear()
    files = repo_files(root)
    hashes = Hashes(root)
    done: dict[str, Inputs] = {}
    plan = Plan([], [], [])
    wanted = (
        [BY_NAME[n] for n in names]
        if names
        else [s for s in STEPS if s.layer == QUICK or layer == FULL or s.suite in ("compile", "verification")]
    )
    for step in wanted:
        inputs = inputs_of(step, root, files, hashes, done)
        done[step.name] = inputs
        record = read_record(root, step.name)
        if force:
            plan.run.append(Planned(step, inputs, "--force"))
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
    alone after the others, the sandbox family last of all."""

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


def run_step(p: Planned, root: Path, force: bool, nextest: bool) -> dict:
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
        with log.open("wb") as out:
            argv = ["bash", "-c", cmd] if isinstance(cmd, str) else cmd
            out.write(f"$ {cmd if isinstance(cmd, str) else ' '.join(cmd)}\n".encode())
            out.flush()
            rc = subprocess.run(argv, cwd=root, stdout=out, stderr=subprocess.STDOUT, env=env).returncode
    record = {
        "fingerprint": p.inputs.fingerprint,
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
    write_record(root, p.step.name, record)
    return record


def when(t: float) -> str:
    return time.strftime("%Y-%m-%d %H:%M", time.localtime(t))


def print_plan(plan: Plan) -> None:
    for step, record in plan.standing:
        print(f"  standing  {step.name:<14} passed {when(record['started'])}, {record['seconds']:g}s")
    for step, why in plan.left:
        print(f"  left      {step.name:<14} for `check full` ({why})")
    for p in plan.run:
        extra = f" [{len(p.only)} program(s)]" if p.only else ""
        print(f"  to run    {p.step.name:<14} {p.reason}{extra}")
    if not plan.run:
        print("nothing to run")


def execute(root: Path, plan: Plan, force: bool, what: str = "") -> int:
    nextest = have_nextest()
    failed = 0
    failed_names: set[str] = set()
    ran: dict[str, list] = {}
    started = time.time()
    for p in order(plan.run, root):
        broken = [n for n in p.step.needs if n in failed_names]
        if broken:
            # Run on an older build, its result would say nothing of this one: not run, not recorded.
            print(f"  SKIP    {p.step.name:<14} not run: {', '.join(broken)} failed", flush=True)
            failed += 1
            failed_names.add(p.step.name)
            ran[p.step.name] = ["skip", 0]
            continue
        print(f"run       {p.step.name} ...", flush=True)
        r = run_step(p, root, force, nextest)
        ran[p.step.name] = [r["result"], r["seconds"]]
        tag = "PASS" if r["result"] == "pass" else "FAIL"
        note = f"  ({r['note']})" if r["note"] else ""
        print(f"  {tag}    {p.step.name:<14} {r['seconds']:g}s  {r['log']}{note}", flush=True)
        if r["result"] != "pass":
            failed += 1
            failed_names.add(p.step.name)
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
        kw["creationflags"] = subprocess.DETACHED_PROCESS | subprocess.CREATE_NEW_PROCESS_GROUP
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
    plan = make_plan(root, FULL)
    for step in STEPS:
        r = read_record(root, step.name)
        standing = any(s.name == step.name for s, _ in plan.standing)
        if r is None:
            print(f"  never run {step.name}")
            continue
        state_word = "standing" if standing else ("FAILED" if r.get("result") != "pass" else "to run")
        print(f"  {state_word:<9} {step.name:<14} {r['result']} {when(r['started'])}, {r['seconds']:g}s  {r['log']}")
    lock = state(root) / "run.log"
    if lock.is_file():
        print(f"last background report: {lock.relative_to(root).as_posix()}, {when(lock.stat().st_mtime)}")
    return 0


def main(argv: list[str] | None = None, root: Path = ROOT) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument(
        "what", help="quick, full, status, history, or a step, crate folder, suite folder, list or program"
    )
    ap.add_argument("--force", action="store_true", help="run again what stands, without the pass cache")
    ap.add_argument("--plan", action="store_true", help="print the plan, run nothing")
    ap.add_argument("--wait", action="store_true", help="run in the foreground however long it takes")
    ap.add_argument("--days", type=int, default=14, help="history: how many of the last days with runs to show")
    a = ap.parse_args(argv)
    if a.what == "status":
        return status(root)
    if a.what == "history":
        return history(root, a.days)
    if a.what in (QUICK, FULL):
        if a.what == FULL and not a.plan:
            changes = dirty(root)
            if changes:
                print(
                    f"check full refused: uncommitted changes under {', '.join(FULL_CLEAN)} "
                    f"({describe(changes)}). Its results are to describe one commit: commit, then run it in a "
                    "worktree of the commit (`git worktree add ../qb64rust-full HEAD`, then `just full` there)."
                )
                return 2
        plan = make_plan(root, a.what, a.force)
    else:
        name = step_of_target(a.what)
        if name is None:
            got = target_command(root, a.what, a.force)
            if isinstance(got, str):
                print(got)
                return 2
            cmd, _ = got
            if RUNNER in cmd:
                # A program runs on the release build: `build` first, unless it stands.
                first = make_plan(root, FULL, a.force, with_needs(["build"]))
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
        plan = make_plan(root, FULL, a.force, with_needs([name]), asked=(name,))
    print_plan(plan)
    if a.plan:
        return 0
    if not plan.run:
        log_history(root, a.what, {}, plan, 0)
        return 0
    if not a.wait and estimate(root, plan) > BACKGROUND_SECONDS:
        return to_background(root, [x for x in argv if x != "--wait"])
    return execute(root, plan, a.force, a.what)


if __name__ == "__main__":
    sys.exit(main())
