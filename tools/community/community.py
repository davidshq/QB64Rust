#!/usr/bin/env python3
"""Community programs: fetch pinned clones of other people's repositories and check their programs.

    python tools/community/community.py fetch [--only <name>]...
    python tools/community/community.py audit [--only <name>]... [--since <commit>]
    python tools/community/community.py check [--only <name>]... [--update-list] [--jobs N] [--from-results <file>]
    python tools/community/community.py heads                    # write and print <root>/heads.txt

OpenSpec change community-programs (tests/community/README.md). The repositories are listed with their pinned
commits in tests/community/repos.toml. They are untrusted: this tool never runs anything in them. `fetch` is the
only part that uses the network. `check` runs `qb64rust -z` (the front end and the C++ writer, no C++ build) on
every .bas file and holds the result to tests/community/pass.list.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "tests" / "community" / "repos.toml"
PASS_LIST = ROOT / "tests" / "community" / "pass.list"
RESULTS = ROOT / "target" / "community" / "results.json"
EXE = ROOT / "target" / "release" / ("qb64rust.exe" if os.name == "nt" else "qb64rust")
MIN_GIT = (2, 45, 1)  # the first release with the fix for CVE-2024-32002
TIMEOUT = 120  # seconds for one program's check
ERROR_LIMIT = 200  # characters of a recorded first error
BIG_FILE = 10 * 1024 * 1024
HEADS = "heads.txt"
LIST_SHOWN = 20  # names printed for new passes before "and N more"

CONTROL = re.compile(r"[\x00-\x1f\x7f]")
WIN_PATH = re.compile(r"[A-Za-z]:[\\/][^:'\"`<>|*?\r\n]*")
POSIX_PATH = re.compile(r"(^|[\s'\"(=])/[^\s:'\"`)]*[^\s:'\"`)/]")
NAME = re.compile(r"^[a-z0-9][a-z0-9._-]*$")
SHA1 = re.compile(r"^[0-9a-f]{40}$")


class ToolError(Exception):
    """A failure to report in one line, without a traceback."""


# --- The manifest ------------------------------------------------------------------------------------------------


@dataclass
class Repo:
    name: str
    url: str
    commit: str
    license: str = ""
    submodules: dict[str, tuple[str, str]] = field(default_factory=dict)
    include_root: str = ""


def safe_relative(path: str) -> bool:
    parts = path.replace("\\", "/").split("/")
    return bool(path) and "\\" not in path and not path.startswith("/") and ":" not in path and all(
        p not in ("", ".", "..") and p.lower() != ".git" for p in parts
    )


def load_manifest(path: Path = MANIFEST) -> list[Repo]:
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    repos: list[Repo] = []
    for entry in data.get("repo", []):
        name = entry.get("name", "")
        if not NAME.match(name):
            raise ToolError(f"manifest: bad repository name {name!r}")
        if any(r.name == name for r in repos):
            raise ToolError(f"manifest: {name} is listed twice")
        if not SHA1.match(entry.get("commit", "")):
            raise ToolError(f"manifest: {name}: commit must be a full lowercase SHA-1")
        subs: dict[str, tuple[str, str]] = {}
        for sub_path, sub in entry.get("submodules", {}).items():
            if not safe_relative(sub_path):
                raise ToolError(f"manifest: {name}: unsafe submodule path {sub_path!r}")
            if not SHA1.match(sub.get("commit", "")) or not sub.get("url"):
                raise ToolError(f"manifest: {name}: submodule {sub_path} needs a url and a full commit")
            subs[sub_path] = (sub["url"], sub["commit"])
        repos.append(
            Repo(
                name,
                entry.get("url", ""),
                entry["commit"],
                entry.get("license", ""),
                subs,
                entry.get("include_root", ""),
            )
        )
    for r in repos:
        if not r.url:
            raise ToolError(f"manifest: {r.name} has no url")
        if r.include_root and not any(o.name == r.include_root for o in repos):
            raise ToolError(f"manifest: {r.name}: include_root {r.include_root!r} is no repository")
    return repos


def select(repos: list[Repo], only: list[str] | None) -> list[Repo]:
    if not only:
        return repos
    known = {r.name for r in repos}
    for n in only:
        if n not in known:
            raise ToolError(f"no repository named {n!r} in the manifest")
    return [r for r in repos if r.name in only]


def community_root(arg: str | None) -> Path:
    if arg:
        return Path(arg)
    env = os.environ.get("QB64RUST_COMMUNITY")
    return Path(env) if env else ROOT.parent / "community"


# --- Text that leaves this tool ------------------------------------------------------------------------------------


def clean(text: str) -> str:
    """No control characters: a name or an error from a clone may hold escape sequences."""
    return CONTROL.sub(" ", text)


def sanitize(text: str, clone: Path | None, limit: int = ERROR_LIMIT) -> str:
    """One recorded error: control characters gone, the clone's own path removed, every other absolute path shown
    as <path>, cut to `limit` characters."""
    text = clean(text)
    if clone is not None:
        for form in {str(clone), clone.as_posix(), str(clone.resolve()), clone.resolve().as_posix()}:
            text = re.sub(re.escape(form) + r"[\\/]?", "", text, flags=re.IGNORECASE)
    text = WIN_PATH.sub("<path>", text)
    text = POSIX_PATH.sub(r"\1<path>", text)
    return text.strip()[:limit]


# --- git, hardened -------------------------------------------------------------------------------------------------


def git_version(text: str | None = None) -> tuple[int, ...]:
    if text is None:
        try:
            text = subprocess.run(["git", "--version"], capture_output=True, text=True, check=True).stdout
        except (OSError, subprocess.CalledProcessError) as e:
            raise ToolError(f"cannot run git: {e}") from e
    m = re.search(r"(\d+)\.(\d+)\.(\d+)", text)
    if not m:
        raise ToolError(f"cannot read git's version from {text.strip()!r}")
    return tuple(int(g) for g in m.groups())


def require_git(text: str | None = None) -> None:
    v = git_version(text)
    if v < MIN_GIT:
        need = ".".join(str(n) for n in MIN_GIT)
        raise ToolError(f"git {need} or newer is needed (CVE-2024-32002); found {'.'.join(str(n) for n in v)}")


class Git:
    """Every git command of the fetch: symlinks, `file` remotes, malformed objects, hooks, LFS and prompts all
    closed (design D8). `allow_file` is for the tests, whose remotes are local folders."""

    def __init__(self, hooks_dir: Path, allow_file: bool = False):
        hooks_dir.mkdir(parents=True, exist_ok=True)
        self.options = [
            "-c", "core.symlinks=false",
            "-c", f"protocol.file.allow={'always' if allow_file else 'never'}",
            "-c", "transfer.fsckObjects=true",
            "-c", f"core.hooksPath={hooks_dir.resolve().as_posix()}",
            "-c", "submodule.recurse=false",
            "-c", "advice.detachedHead=false",
        ]  # fmt: skip
        self.env = dict(os.environ, GIT_LFS_SKIP_SMUDGE="1", GIT_TERMINAL_PROMPT="0")

    def command(self, *args: str, cwd: Path | None = None) -> list[str]:
        return ["git", *self.options, *(["-C", str(cwd)] if cwd else []), *args]

    def run(self, *args: str, cwd: Path | None = None, binary: bool = False):
        r = subprocess.run(self.command(*args, cwd=cwd), capture_output=True, env=self.env)
        if r.returncode != 0:
            err = r.stderr.decode("utf-8", "replace").strip().splitlines()
            raise ToolError(f"git {args[0]} failed: {sanitize(err[-1] if err else 'no message', None, 300)}")
        return r.stdout if binary else r.stdout.decode("utf-8", "replace")

    def ok(self, *args: str, cwd: Path | None = None) -> bool:
        return subprocess.run(self.command(*args, cwd=cwd), capture_output=True, env=self.env).returncode == 0


def head_of(git: Git, folder: Path) -> str | None:
    if not (folder / ".git").exists():
        return None
    try:
        return git.run("rev-parse", "HEAD", cwd=folder).strip()
    except (ToolError, OSError):
        return None


# --- fetch ---------------------------------------------------------------------------------------------------------


def checkout_pin(git: Git, folder: Path, url: str, commit: str) -> str:
    """Clone `url` into `folder` if it is no clone yet, and check out `commit`. Returns what was done."""
    if head_of(git, folder) == commit:
        return "unchanged"
    action = "moved"
    if not (folder / ".git").exists():
        if folder.exists() and any(folder.iterdir()):
            raise ToolError(f"{folder.name} exists and is not a clone; remove it")
        folder.parent.mkdir(parents=True, exist_ok=True)
        git.run("clone", "--no-checkout", "--filter=blob:none", url, str(folder))
        action = "cloned"
    else:
        git.run("remote", "set-url", "origin", url, cwd=folder)
    if not git.ok("cat-file", "-e", f"{commit}^{{commit}}", cwd=folder):
        git.run("fetch", "--filter=blob:none", "origin", cwd=folder)
        if not git.ok("cat-file", "-e", f"{commit}^{{commit}}", cwd=folder):
            git.run("fetch", "--filter=blob:none", "origin", commit, cwd=folder)
    git.run("checkout", "--force", "--detach", commit, cwd=folder)
    return action


def fetch_repo(git: Git, repo: Repo, root: Path) -> str:
    dest = root / repo.name
    try:
        action = checkout_pin(git, dest, repo.url, repo.commit)
        # Submodules come from the manifest's URLs and commits. The repository's .gitmodules is never read.
        for sub_path, (url, commit) in repo.submodules.items():
            sub = dest / sub_path
            sub.mkdir(parents=True, exist_ok=True)
            sub_action = checkout_pin(git, sub, url, commit)
            if sub_action != "unchanged" and action == "unchanged":
                action = "moved"
    except ToolError as e:
        raise ToolError(f"{repo.name}: {e}") from e
    return action


def write_heads(git: Git, repos: list[Repo], root: Path) -> str:
    """<root>/heads.txt: `<name> <HEAD>` for each clone and `<name>/<path> <HEAD>` for each submodule. The copy of
    the clones in the suites VM carries no .git folder, so this is what says which commits it holds."""
    lines = []
    for r in repos:
        h = head_of(git, root / r.name)
        if h is None:
            continue
        lines.append(f"{r.name} {h}")
        for sub_path in r.submodules:
            sh = head_of(git, root / r.name / sub_path)
            if sh:
                lines.append(f"{r.name}/{sub_path} {sh}")
    text = "\n".join(lines) + ("\n" if lines else "")
    root.mkdir(parents=True, exist_ok=True)
    (root / HEADS).write_text(text, encoding="utf-8", newline="\n")
    return text


def read_heads(root: Path) -> dict[str, str]:
    p = root / HEADS
    if not p.is_file():
        return {}
    out = {}
    for line in p.read_text(encoding="utf-8").splitlines():
        parts = line.split()
        if len(parts) == 2:
            out[parts[0]] = parts[1]
    return out


def cmd_fetch(args, repos: list[Repo]) -> int:
    require_git()
    root = community_root(args.root)
    root.mkdir(parents=True, exist_ok=True)
    git = Git(root / ".no-hooks", allow_file=getattr(args, "allow_file", False))
    failed = []
    for r in select(repos, args.only):
        try:
            action = fetch_repo(git, r, root)
            print(f"{r.name}: {action}")
        except ToolError as e:
            print(f"FAILED {e}")
            failed.append(r.name)
        sys.stdout.flush()
    write_heads(git, repos, root)
    if failed:
        print(f"{len(failed)} repositories failed: {', '.join(failed)}")
        return 1
    return 0


# --- the clones' state ---------------------------------------------------------------------------------------------


def clone_problem(git: Git, repo: Repo, root: Path, heads: dict[str, str]) -> str | None:
    """Why a clone is not at its pin, or None. With a .git folder git says; the VM's copy has none, so it is
    judged by heads.txt."""
    dest = root / repo.name
    if not dest.is_dir():
        return "is missing"
    pins = {repo.name: repo.commit, **{f"{repo.name}/{p}": c for p, (_u, c) in repo.submodules.items()}}
    for key, pin in pins.items():
        folder = root / key
        have = head_of(git, folder) if (folder / ".git").exists() else heads.get(key)
        if have is None:
            return f"has no record of its commit ({key})"
        if have != pin:
            return f"is at {have[:10]}, not its pin {pin[:10]} ({key})"
    return None


def require_clones(git: Git, repos: list[Repo], root: Path) -> None:
    heads = read_heads(root)
    for r in repos:
        why = clone_problem(git, r, root, heads)
        if why:
            raise ToolError(
                f"clone {r.name} {why}; run: python tools/community/community.py fetch --only {r.name}"
            )


# --- check ---------------------------------------------------------------------------------------------------------


def list_programs(clone: Path) -> tuple[list[str], int]:
    """The .bas files below a clone (any case of the extension), as sorted paths with `/`; and the number left
    out because their names hold control characters."""
    found, skipped = [], 0
    for base, dirs, files in os.walk(clone):
        dirs[:] = [d for d in dirs if d.lower() != ".git"]
        for f in files:
            if f.lower().endswith(".bas"):
                rel = (Path(base) / f).relative_to(clone).as_posix()
                if CONTROL.search(rel):
                    skipped += 1
                else:
                    found.append(rel)
    return sorted(found), skipped


def first_error(rc: int, out: str, err: str) -> str | None:
    lines = [ln for ln in (err + "\n" + out).splitlines() if ln.strip()]
    for ln in lines:
        if ": error:" in ln or ln.startswith("error:"):
            return ln
    if rc != 0:
        return lines[0] if lines else f"exit code {rc}"
    return None


def read_head_of_file(path: Path, limit: int = 1 << 20) -> str:
    try:
        with open(path, "rb") as f:
            return f.read(limit).decode("utf-8", "replace")
    except OSError:
        return ""


def check_program(qb: list[str], clone: Path, rel: str, include: Path | None, scratch: Path, timeout: float) -> dict:
    """Check one program. It runs from the scratch folder, with the program's absolute path: `$INCLUDE` is
    relative to the including file, and nothing of the clone is the working directory."""
    started = time.monotonic()
    out_path, err_path = scratch / "stdout.txt", scratch / "stderr.txt"
    cmd = [*qb, "-z", str(clone / rel), "-o", str(scratch / "out.exe")]
    if include is not None:
        cmd += ["--include-root", str(include)]
    error: str | None
    with open(out_path, "wb") as so, open(err_path, "wb") as se:
        try:
            r = subprocess.run(cmd, stdout=so, stderr=se, stdin=subprocess.DEVNULL, cwd=scratch, timeout=timeout)
            rc = r.returncode
            error = first_error(rc, read_head_of_file(out_path), read_head_of_file(err_path))
        except subprocess.TimeoutExpired:
            rc, error = -1, "timed out"
        except OSError as e:
            rc, error = -1, f"cannot start the compiler: {e}"
    passed = rc == 0 and error is None
    shutil.rmtree(scratch, ignore_errors=True)
    return {
        "program": rel,
        "passed": passed,
        "error": None if passed else sanitize(error or f"exit code {rc}", clone),
        "seconds": round(time.monotonic() - started, 2),
    }


def run_checks(qb: list[str], repos: list[Repo], root: Path, jobs: int, timeout: float) -> list[dict]:
    work = []  # (repo, rel)
    skipped = 0
    for r in repos:
        progs, skip = list_programs(root / r.name)
        skipped += skip
        work += [(r, p) for p in progs]
    if skipped:
        print(f"left out {skipped} programs whose names hold control characters")
    results: list[dict | None] = [None] * len(work)
    scratch_root = Path(tempfile.mkdtemp(prefix="community-"))
    done = 0

    def one(i: int) -> None:
        r, rel = work[i]
        include = root / r.include_root if r.include_root else None
        scratch = scratch_root / str(i)
        scratch.mkdir()
        res = check_program(qb, root / r.name, rel, include, scratch, timeout)
        res["program"] = f"{r.name}/{rel}"
        results[i] = res

    try:
        with ThreadPoolExecutor(max_workers=max(1, jobs)) as pool:
            futures = [pool.submit(one, i) for i in range(len(work))]
            for f in futures:
                f.result()
                done += 1
                if done % 250 == 0:
                    print(f"  {done}/{len(work)} checked")
                    sys.stdout.flush()
    finally:
        shutil.rmtree(scratch_root, ignore_errors=True)
    return [x for x in results if x is not None]


def read_list(path: Path) -> list[str]:
    if not path.is_file():
        return []
    return [ln.strip() for ln in path.read_text(encoding="utf-8").splitlines() if ln.strip()]


def write_list(path: Path, lines: set[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("".join(f"{ln}\n" for ln in sorted(lines)), encoding="utf-8", newline="\n")


@dataclass
class Verdict:
    regressions: list[dict]
    missing: list[str]
    new_passes: list[str]
    passed: int
    failed: int

    @property
    def ok(self) -> bool:
        return not self.regressions and not self.missing


def judge(results: list[dict], listed: list[str], names: set[str]) -> Verdict:
    by_program = {r["program"]: r for r in results}
    in_scope = [p for p in listed if p.split("/", 1)[0] in names]
    regressions = [by_program[p] for p in in_scope if p in by_program and not by_program[p]["passed"]]
    missing = [p for p in in_scope if p not in by_program]
    listed_set = set(listed)
    new = sorted(r["program"] for r in results if r["passed"] and r["program"] not in listed_set)
    passed = sum(1 for r in results if r["passed"])
    return Verdict(regressions, missing, new, passed, len(results) - passed)


def report(v: Verdict, total_listed: int) -> None:
    for r in v.regressions:
        print(f"FAIL {clean(r['program'])}: {r['error']}")
    for p in v.missing:
        print(f"MISSING {clean(p)} (on the pass list, not in the clone)")
    if v.new_passes:
        print(f"NEW {len(v.new_passes)} programs pass and are not on the pass list (check --update-list adds them):")
        for p in v.new_passes[:LIST_SHOWN]:
            print(f"  {clean(p)}")
        if len(v.new_passes) > LIST_SHOWN:
            print(f"  and {len(v.new_passes) - LIST_SHOWN} more")
    print(
        f"{v.passed + v.failed} programs checked: {v.passed} pass, {v.failed} fail; "
        f"pass list {total_listed}: {len(v.regressions)} regressions, {len(v.missing)} missing, "
        f"{len(v.new_passes)} new passes"
    )


def cmd_check(args, repos: list[Repo]) -> int:
    chosen = select(repos, args.only)
    results_path = Path(args.results)
    list_path = Path(args.list)
    if args.from_results:
        data = json.loads(Path(args.from_results).read_text(encoding="utf-8"))
        results = data["programs"]
    else:
        # No git needed: the VM's copy has no .git folders, and heads.txt says which commits it holds.
        root = community_root(args.root)
        require_clones(Git(root / ".no-hooks"), chosen, root)
        qb = args.qb64rust if isinstance(args.qb64rust, list) else [str(args.qb64rust)]
        started = time.monotonic()
        results = run_checks(qb, chosen, root, args.jobs, args.timeout)
        results_path.parent.mkdir(parents=True, exist_ok=True)
        doc = {
            "commits": {r.name: r.commit for r in chosen},
            "seconds": round(time.monotonic() - started, 1),
            "programs": results,
        }
        results_path.write_text(json.dumps(doc, indent=1), encoding="utf-8", newline="\n")
    listed = read_list(list_path)
    v = judge(results, listed, {r.name for r in chosen})
    if args.update_list and v.new_passes:
        write_list(list_path, set(listed) | set(v.new_passes))
        print(f"added {len(v.new_passes)} programs to {list_path.name}")
        v.new_passes = []
        listed = read_list(list_path)
    report(v, len(listed))
    return 0 if v.ok else 1


# --- audit ---------------------------------------------------------------------------------------------------------

RISKY_EXT = {
    ".exe", ".dll", ".com", ".bat", ".cmd", ".ps1", ".vbs", ".vbe", ".wsf", ".hta", ".msi", ".msp", ".scr",
    ".lnk", ".reg", ".sys", ".jar", ".so", ".dylib", ".lib", ".o", ".obj", ".a", ".sh", ".py", ".pl", ".rb",
    ".js", ".zip", ".7z", ".rar", ".tar", ".gz", ".tgz", ".bz2", ".xz", ".cab", ".iso", ".img", ".vhd", ".dmg",
    ".deb", ".rpm", ".apk",
}  # fmt: skip
MAGIC = (b"MZ", b"\x7fELF", b"PK\x03\x04", b"7z\xbc\xaf", b"Rar!", b"\xcf\xfa\xed\xfe", b"\xca\xfe\xba\xbe", b"#!")
RESERVED = re.compile(r"^(con|prn|aux|nul|com[1-9]|lpt[1-9])(\..*)?$", re.IGNORECASE)
RISKY_FOLDERS = {".vscode", ".devcontainer"}


def odd_windows_name(path: str) -> bool:
    for part in path.split("/"):
        if part == ".." or RESERVED.match(part) or part.endswith((".", " ")) or any(c in part for c in ':*?"<>|\\'):
            return True
    return False


def audit_repo(git: Git, repo: Repo, root: Path, since: str | None = None, big: int = BIG_FILE) -> list[tuple[str, str]]:
    dest = root / repo.name
    have = head_of(git, dest)
    if have != repo.commit:
        raise ToolError(f"clone {repo.name} is not at its pin; run: python tools/community/community.py fetch --only {repo.name}")
    changed: set[str] | None = None
    if since:
        raw = git.run("diff", "--name-only", "-z", "--diff-filter=ACMRT", "--no-renames", since, "HEAD", cwd=dest)
        changed = {p for p in raw.split("\0") if p}
    listing = git.run("ls-tree", "-r", "-z", "--long", "HEAD", cwd=dest)
    items: set[tuple[str, str]] = set()
    folders: dict[str, int] = {}
    for entry in listing.split("\0"):
        if not entry or "\t" not in entry:
            continue
        meta, path = entry.split("\t", 1)
        if changed is not None and path not in changed:
            continue
        mode, kind, sha, size = (meta.split() + ["", "", "", ""])[:4]
        shown = clean(path)
        if mode == "160000":
            url = repo.submodules.get(path, ("NOT IN THE MANIFEST", ""))[0]
            items.add(("submodule", f"{shown} -> {url}"))
            continue
        if mode == "120000":
            target = clean(git.run("cat-file", "blob", sha, cwd=dest).strip())[:100]
            items.add(("symlink", f"{shown} -> {target}"))
        ext = os.path.splitext(path)[1].lower()
        file_on_disk = dest / path
        if ext in RISKY_EXT:
            items.add(("program, script, library or archive", shown))
        elif mode != "120000" and file_on_disk.is_file():
            try:
                with open(file_on_disk, "rb") as f:
                    first = f.read(8)
            except OSError:
                first = b""
            if first.startswith(MAGIC):
                items.add(("program, script, library or archive (by content)", shown))
        if size.isdigit() and int(size) > big:
            items.add(("large file", f"{shown} ({int(size)} bytes)"))
        if odd_windows_name(path):
            items.add(("name Windows treats specially", shown))
        parts = path.split("/")
        for i, part in enumerate(parts[:-1]):
            if part.lower() in RISKY_FOLDERS:
                folder = clean("/".join(parts[: i + 1]))
                folders[folder] = folders.get(folder, 0) + 1
                break
    for folder, n in folders.items():
        items.add(("folder that can run commands when trusted", f"{folder} ({n} files)"))
    return sorted(items)


def cmd_audit(args, repos: list[Repo]) -> int:
    require_git()
    root = community_root(args.root)
    git = Git(root / ".no-hooks")
    status = 0
    for r in select(repos, args.only):
        try:
            found = audit_repo(git, r, root, args.since)
        except ToolError as e:
            print(f"== {r.name}: {e}")
            status = 1
            continue
        scope = f" since {args.since[:10]}" if args.since else ""
        print(f"== {r.name} ({r.commit[:10]}){scope}, license {r.license or 'unknown'}")
        if not found:
            print("   nothing to list")
        for category, detail in found:
            print(f"   {category}: {detail}")
    return status


def cmd_heads(args, repos: list[Repo]) -> int:
    root = community_root(args.root)
    sys.stdout.write(write_heads(Git(root / ".no-hooks"), repos, root))
    return 0


# --- command line --------------------------------------------------------------------------------------------------


def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--root", help="the folder of the clones (default: QB64RUST_COMMUNITY, or ../community)")
    p.add_argument("--manifest", default=str(MANIFEST), help="the manifest (default: tests/community/repos.toml)")
    sub = p.add_subparsers(dest="command", required=True)
    f = sub.add_parser("fetch", help="clone or move each repository to its pinned commit (uses the network)")
    f.add_argument("--only", action="append", help="a repository's name (repeatable)")
    a = sub.add_parser("audit", help="list the files of a clone that a person should read before trusting a pin")
    a.add_argument("--only", action="append")
    a.add_argument("--since", help="only what changed since this commit")
    c = sub.add_parser("check", help="run the front end over every .bas file and hold the result to the pass list")
    c.add_argument("--only", action="append")
    c.add_argument("--update-list", action="store_true", help="add the programs that pass and are not listed")
    c.add_argument("--jobs", type=int, default=8)
    c.add_argument("--timeout", type=float, default=TIMEOUT)
    c.add_argument("--qb64rust", default=str(EXE))
    c.add_argument("--list", default=str(PASS_LIST))
    c.add_argument("--results", default=str(RESULTS))
    c.add_argument("--from-results", help="judge a results.json that an earlier run wrote; run nothing")
    sub.add_parser("heads", help="write and print heads.txt, the commit each clone is at")
    return p


COMMANDS = {"fetch": cmd_fetch, "audit": cmd_audit, "check": cmd_check, "heads": cmd_heads}


def main(argv: list[str] | None = None) -> int:
    for stream in (sys.stdout, sys.stderr):
        stream.reconfigure(errors="replace")
    args = parser().parse_args(argv)
    try:
        repos = load_manifest(Path(args.manifest))
        return COMMANDS[args.command](args, repos)
    except ToolError as e:
        print(f"error: {e}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
