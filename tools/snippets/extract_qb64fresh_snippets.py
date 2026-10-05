"""Extracts the BASIC snippets of QB64Fresh's inline tests into tests/snippets/qb64fresh/ and labels each with the
old compiler's verdict (design D5 of the OpenSpec change m2-upstream-tests; CLAUDE.md rule 5).

Only the BASIC text is taken: every raw string r#"..."# in QB64Fresh's tests/integration_tests.rs that is assigned
to `source` (or `source2`) or passed to `compile_to_c`/`compile_to_c_debug`. Raw strings inside assertions about
the generated C (`code.contains(r#"..."#)`) are not BASIC and are skipped. QB64Fresh's assertions are not taken:
the verdict is qb64pe.exe's.

Each snippet: the common indentation removed, a leading empty line and a trailing blank line dropped, line ends
CR LF (as QB64pe sources are), a final CR LF, bytes as in the Rust source (UTF-8; the snippets are ASCII where
it matters). A snippet whose text equals an earlier one is dropped. File names are
<module>__<test>.bas, with __2, __3... for the second and later snippet of one test.

Each snippet is compiled with `qb64pe.exe -z -q -w` (C++ generation only) in a scratch folder outside the repo; a
non-zero exit writes <name>.err with the compiler's output, the scratch folder's path replaced by <SCRATCH>.
SOURCE.md records QB64Fresh's commit and the counts. A second run with the same QB64Fresh revision and qb64pe.exe
changes no file.

Usage: python tools/snippets/extract_qb64fresh_snippets.py <QB64Fresh dir> <qb64pe.exe>
"""

import argparse
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
import textwrap

REPO = pathlib.Path(__file__).resolve().parents[2]
DEST = REPO / "tests" / "snippets" / "qb64fresh"
RAW = re.compile(r'r#"(.*?)"#', re.S)
BASIC_CONTEXT = re.compile(r"(let\s+source\w*\s*=|compile_to_c\w*\()\s*$")
MOD = re.compile(r"^mod\s+(\w+)", re.M)
FN = re.compile(r"^\s*fn\s+(\w+)", re.M)


def last(pattern, text):
    found = pattern.findall(text)
    return found[-1] if found else "top"


def normalise(raw):
    lines = raw.split("\n")
    if lines and lines[0].strip() == "":
        lines = lines[1:]
    if lines and lines[-1].strip() == "":
        lines = lines[:-1]
    text = textwrap.dedent("\n".join(line.rstrip("\r") for line in lines))
    return "\r\n".join(text.split("\n")) + "\r\n"


def snippets(rs_text):
    """(name, text) for every BASIC snippet, in file order, before deduplication."""
    out = []
    per_test = {}
    for m in RAW.finditer(rs_text):
        before = rs_text[: m.start()]
        if not BASIC_CONTEXT.search(before[-200:]):
            continue
        base = f"{last(MOD, before)}__{last(FN, before)}"
        n = per_test.get(base, 0) + 1
        per_test[base] = n
        name = base if n == 1 else f"{base}__{n}"
        out.append((name, normalise(m.group(1))))
    return out


def label(qb64pe, text, scratch):
    """The compiler's output if it rejects the snippet, else None."""
    work = pathlib.Path(scratch)
    for f in work.iterdir():
        if f.is_dir():
            shutil.rmtree(f)
        else:
            f.unlink()
    (work / "snippet.bas").write_bytes(text.encode("utf-8"))
    env = dict(os.environ, QB64PE_NOPROMPT="y")
    p = subprocess.run([qb64pe, "-z", "-q", "-w", "snippet.bas"], cwd=work, env=env, capture_output=True,
                       timeout=120)
    if p.returncode == 0:
        return None
    out = p.stdout
    for form in {str(work), str(work.resolve()), str(work).replace("\\", "/"), str(work.resolve()).replace("\\", "/")}:
        out = out.replace(form.encode(), b"<SCRATCH>")
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("qb64fresh")
    ap.add_argument("qb64pe")
    a = ap.parse_args()

    fresh = pathlib.Path(a.qb64fresh)
    rs = fresh / "tests" / "integration_tests.rs"
    commit = subprocess.run(["git", "-C", str(fresh), "rev-parse", "--short=10", "HEAD"], check=True,
                            capture_output=True, text=True).stdout.strip()
    date = subprocess.run(["git", "-C", str(fresh), "log", "-1", "--format=%cs", "HEAD"], check=True,
                          capture_output=True, text=True).stdout.strip()
    qb64pe = str(pathlib.Path(a.qb64pe).resolve())

    rs_text = rs.read_text(encoding="utf-8")
    total = len(RAW.findall(rs_text))
    found = snippets(rs_text)
    seen = set()
    kept = []
    for name, text in found:
        if text in seen:
            continue
        seen.add(text)
        kept.append((name, text))

    if DEST.exists():
        for f in DEST.iterdir():
            if f.suffix in (".bas", ".err"):
                f.unlink()
    DEST.mkdir(parents=True, exist_ok=True)
    rejected = 0
    with tempfile.TemporaryDirectory(prefix="qb64rust-snippets-") as scratch:
        for i, (name, text) in enumerate(kept, 1):
            (DEST / f"{name}.bas").write_bytes(text.encode("utf-8"))
            err = label(qb64pe, text, scratch)
            if err is not None:
                (DEST / f"{name}.err").write_bytes(err)
                rejected += 1
            print(f"\r{i}/{len(kept)}", end="", file=sys.stderr)
    print(file=sys.stderr)

    (DEST / "SOURCE.md").write_text(
        f"""# Source of `tests/snippets/qb64fresh/`

The BASIC snippets of QB64Fresh's inline tests (`tests/integration_tests.rs` of QB64Fresh, the user's own MIT
code; commit `{commit}`, {date}). Only the BASIC text is taken, never the assertions (`CLAUDE.md` rule 5): each
snippet's verdict is that of `qb64pe.exe` (QB64pe at the commit in `tests/upstream/SOURCE.md`). A snippet it
rejects has an `.err` file with its output; one it accepts has none. Nothing is run.

Made by `tools/snippets/extract_qb64fresh_snippets.py` (rules in its header). Do not edit the files by hand.

| | Snippets |
|---|---|
| Raw strings with BASIC text (assigned to `source` or passed to `compile_to_c`) | {len(found)} |
| After dropping duplicates | {len(kept)} |
| Rejected by `qb64pe.exe` (`.err`) | {rejected} |

The {total - len(found)} other raw strings in the file are C text in assertions about QB64Fresh's generated code and are not taken.
""",
        encoding="utf-8",
        newline="\n",
    )
    print(f"{len(found)} snippets, {len(kept)} after deduplication, {rejected} rejected by qb64pe")


if __name__ == "__main__":
    main()
