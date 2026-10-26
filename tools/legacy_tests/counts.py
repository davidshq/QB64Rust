#!/usr/bin/env python3
"""Writes the count blocks of the test READMEs from the lists, the files and the runner's results, so
that a change's close copies no number by hand (study/archive/31 item 2).

    python tools/legacy_tests/counts.py            # rewrite every block
    python tools/legacy_tests/counts.py --check    # compare, write nothing; exit code 1 when one differs

A block is the text between `<!-- counts:NAME -->` and `<!-- /counts:NAME -->` in a README; everything
outside the markers is left as it is. Two kinds:

  membership  what the lists and the folders hold now: always written and checked
  results     what the suite's last full run gave (target/legacy-tests/results-<suite>.json): written
              only when that run is of the release build at target/release/qb64rust.exe and of this
              runner, with folding on and no cache that skips a program; otherwise the block is left
              as it is (it names its own run's date and build) and the reason is printed. A pass list
              is judged in it as --check-list does, never from a compile alone.

No path is written: counts, dates and the first 12 characters of the compiler's SHA-256.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import run_legacy_tests as rlt

REPO = rlt.REPO
CORPUS = REPO / "tests" / "corpus"
UPSTREAM = REPO / "tests" / "upstream"
DIFFERENTIAL = REPO / "tests" / "differential"
VERIFICATION = REPO / "tests" / "verification"
VERIFICATION_ROOT = REPO / "verification"
FORMAT = REPO / "tests" / "format"
RESULTS = REPO / "target" / "legacy-tests"
COMPILER = REPO / "target" / "release" / "qb64rust.exe"


def list_names(path: Path) -> list[str]:
    """The <group>/<name> lines of a list, comments and blank lines left out."""
    out = []
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.split("#", 1)[0].strip()
        if line:
            out.append(line)
    return out


def table(header: list[str], rows: list[list]) -> list[str]:
    return [
        "| " + " | ".join(header) + " |",
        "|" + "---|" * len(header),
        *("| " + " | ".join(str(c) for c in row) + " |" for row in rows),
    ]


def corpus_membership() -> list[str]:
    slice_list = list_names(CORPUS / "slice.list")
    folders = sorted(p.name for p in CORPUS.iterdir() if p.is_dir() and any(p.rglob("*.bas")))
    rows = []
    for name in folders:
        bas = list(CORPUS.joinpath(name).rglob("*.bas"))

        def count(ext: str, bas=bas) -> int:
            return sum(1 for b in bas if b.with_suffix(ext).is_file())

        listed = sum(1 for n in slice_list if n.split("/", 1)[0] == name)
        rows.append(
            [
                f"`{name}`",
                len(bas),
                count(".output"),
                count(".err"),
                count(".norun"),
                count(".screen"),
                listed,
            ]
        )
    return [
        f"`slice.list` names **{len(slice_list)}** tests (a program, or a script of a screen program: "
        "`<name>.<script>.keys`). By folder:",
        "",
        *table(
            [
                "Folder",
                "Programs",
                "`.output`",
                "`.err`",
                "`.norun`",
                "`.screen`",
                "In `slice.list`",
            ],
            rows,
        ),
    ]


def upstream_membership() -> list[str]:
    programs = len(list((UPSTREAM / "compile_tests").rglob("*.bas")))
    passing = len(list_names(UPSTREAM / "pass.list"))
    deferred = len(list_names(UPSTREAM / "deferred.list"))
    counted = programs - deferred
    return [
        (
            f"**Upstream progress: {passing} of {counted}**: the programs of `pass.list` of the suite's "
            f"{programs} programs less the {deferred} of `deferred.list`."
        ),
    ]


def differential_membership() -> list[str]:
    programs = len(list(DIFFERENTIAL.rglob("*.bas")))
    passing = len(list_names(DIFFERENTIAL / "pass.list"))
    return [f"`pass.list` names **{passing}** of the {programs} programs."]


def verification_membership() -> list[str]:
    programs = len(list(VERIFICATION_ROOT.glob("*.bas")))
    passing = len(list_names(VERIFICATION / "pass.list"))
    known = rlt.load_known_list(VERIFICATION / "known.list")
    reasons: dict[str, int] = {}
    for reason in known.values():
        word = reason.split(": ", 1)[0]
        reasons[word] = reasons.get(word, 0) + 1
    lines = [
        (
            f"Of the {programs} programs of `verification\\`, `pass.list` names **{passing}** and "
            f"`known.list` {len(known)}."
        )
    ]
    if reasons:
        rows = [[f"`{word}`", n] for word, n in sorted(reasons.items())]
        lines += ["", *table(["Reason in `known.list`", "Programs"], rows)]
    return lines


def format_membership() -> list[str]:
    """The recordings of the format oracle by set, and how many tests/format/known_differences.list names."""
    sets = sorted(p.name for p in FORMAT.iterdir() if p.is_dir()) if FORMAT.is_dir() else []
    rows = []
    for name in sets:
        recorded = sum(
            1 for p in (FORMAT / name).rglob("*") if p.is_file() and p.suffix.lower() == ".bas"
        )
        rows.append([f"`{name}`", recorded])
    total = sum(n for _, n in rows)
    listed = FORMAT / "known_differences.list"
    known = len(list_names(listed)) if listed.is_file() else 0
    return [
        f"**{total}** programs recorded; `known_differences.list` names {known}. By set:",
        "",
        *table(["Set", "Recorded"], rows),
    ]


def full_run(suite: str) -> tuple[dict | None, str]:
    """The suite's last full summary when it may be written as current, else (None, why not)."""
    path = RESULTS / f"results-{suite}.json"
    if not path.is_file():
        return None, f"no full run ({path.name})"
    data = json.loads(path.read_text(encoding="utf-8"))
    run = data.get("run")
    if not isinstance(run, dict) or run.get("version") != rlt.RUN_RECORD_VERSION:
        return None, f"{path.name} has no run record of this runner"
    if not COMPILER.is_file() or run.get("compiler_sha256") != rlt.file_sha256(COMPILER):
        return None, f"{path.name} is not of the release build at target/release/qb64rust.exe"
    if run.get("runner_sha256") != rlt.file_sha256(Path(rlt.__file__).resolve()):
        return None, f"{path.name} is of another version of the runner"
    if (
        run.get("selection")
        or run.get("no_fold")
        or run.get("pass_cache")
        or run.get("record")
        or run.get("cpp_opt")
    ):
        return None, f"{path.name} is not a plain full run"
    return data, ""


def results_block(suite: str, root: Path, pass_list: Path) -> tuple[list[str] | None, str]:
    data, why = full_run(suite)
    if data is None:
        return None, why
    run = data["run"]
    summary = next(iter(data["summary"].values()), {"PASS": 0, "FAIL": 0, "KFAIL": 0})
    total = sum(summary.values())
    wanted = rlt.load_list(pass_list, root)
    problems, failures, passed = rlt.check_list(suite, wanted, root, data, COMPILER)
    if problems:
        return None, "; ".join(problems)
    day = time.strftime("%Y-%m-%d", time.localtime(run["ended"]))
    minutes = (run["ended"] - run["started"]) / 60
    known, failed = summary["KFAIL"], summary["FAIL"]
    lines = [
        (
            f"Last full run: {day}, compiler build `{run['compiler_sha256'][:12]}`, folding on, "
            f"`--jobs {run['jobs']}`{', build cache' if run['build_cache'] else ''}, {minutes:.0f} min. "
            f"Of {total} programs: **{summary['PASS']} pass**, {failed} "
            f"{'does' if failed == 1 else 'do'} not (a wrong result, "
            "or rejected or with only marked errors where the old compiler accepts the program), "
            f"{known} known failure{'' if known == 1 else 's'} "
            f"(`target\\legacy-tests\\results-{suite}.json`)."
        ),
        f"`{pass_list.name}` judged in it (`--check-list`): **{passed} of {len(wanted)}** pass.",
    ]
    lines += [f"- {key}: {reason}" for key, reason in failures]
    return lines, ""


def compile_root() -> Path:
    return (REPO.parent / "QB64pe" / "tests" / "compile_tests").resolve()


# (README, block name, kind, function giving the lines or (None, why not)).
BLOCKS = [
    (CORPUS / "README.md", "corpus-membership", "membership", lambda: (corpus_membership(), "")),
    (
        CORPUS / "README.md",
        "corpus-results",
        "results",
        lambda: results_block("corpus", CORPUS.resolve(), CORPUS / "slice.list"),
    ),
    (
        UPSTREAM / "README.md",
        "upstream-membership",
        "membership",
        lambda: (upstream_membership(), ""),
    ),
    (
        UPSTREAM / "README.md",
        "upstream-results",
        "results",
        lambda: results_block("compile", compile_root(), UPSTREAM / "pass.list"),
    ),
    (
        DIFFERENTIAL / "README.md",
        "differential-membership",
        "membership",
        lambda: (differential_membership(), ""),
    ),
    (
        VERIFICATION / "README.md",
        "verification-membership",
        "membership",
        lambda: (verification_membership(), ""),
    ),
    (FORMAT / "README.md", "format-membership", "membership", lambda: (format_membership(), "")),
    (
        VERIFICATION / "README.md",
        "verification-results",
        "results",
        lambda: results_block(
            "verification", VERIFICATION_ROOT.resolve(), VERIFICATION / "pass.list"
        ),
    ),
]


def replace_block(text: str, name: str, lines: list[str]) -> str:
    """The text with the block `name` holding `lines`, in the text's own line ends (a checkout may
    have CR LF); ValueError when its markers are not there exactly once."""
    begin, end = f"<!-- counts:{name} -->", f"<!-- /counts:{name} -->"
    pattern = re.compile(re.escape(begin) + r"\r?\n.*?" + re.escape(end), re.DOTALL)
    if len(pattern.findall(text)) != 1:
        raise ValueError(f"the markers of block {name} are not there exactly once")
    nl = "\r\n" if "\r\n" in text else "\n"
    body = nl.join([begin, *lines, end])
    return pattern.sub(lambda _: body, text)


def read(path: Path) -> str:
    with path.open(encoding="utf-8", newline="") as f:
        return f.read()


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--check", action="store_true", help="compare the blocks, write nothing")
    args = ap.parse_args()
    stale = []
    texts: dict[Path, str] = {}
    for readme, name, kind, make in BLOCKS:
        text = texts.setdefault(readme, read(readme))
        lines, why = make()
        if lines is None:
            print(f"{name}: left as it is: {why}")
            continue
        try:
            new = replace_block(text, name, lines)
        except ValueError as e:
            print(f"{readme.relative_to(REPO).as_posix()}: {e}", file=sys.stderr)
            return 2
        if new != text:
            stale.append(name)
        texts[readme] = new
    if args.check:
        for name in stale:
            print(f"{name}: differs from what the lists and results give")
        print(f"{len(BLOCKS)} blocks, {len(stale)} differ")
        return 1 if stale else 0
    for readme, text in texts.items():
        if text != read(readme):
            readme.write_text(text, encoding="utf-8", newline="")
    print(f"{len(BLOCKS)} blocks, {len(stale)} rewritten")
    return 0


if __name__ == "__main__":
    sys.exit(main())
