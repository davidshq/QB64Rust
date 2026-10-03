"""Run qb64fresh-fmt over QB64pe's test programs and check that it keeps the program text that must not change:
string literals, comments (ignoring the space after the apostrophe), and the sequence of non-whitespace characters
ignoring case (a rough lossless check). The formatter's trailing 'Summary' block on stdout is cut off first.

Environment: QBF_BIN = directory holding qb64fresh-fmt.exe (required); QB64PE = QB64pe checkout with tests/
(default: ../../../QB64pe relative to this script)."""
import os, pathlib, re, subprocess, collections

HERE = pathlib.Path(__file__).resolve().parent
FMT = pathlib.Path(os.environ["QBF_BIN"]) / "qb64fresh-fmt.exe"
T = pathlib.Path(os.environ.get("QB64PE", HERE / ".." / ".." / ".." / "QB64pe")) / "tests"

STR = re.compile(r'"[^"\n]*"?')
SUMMARY = re.compile(r"\r?\n---\r?\nSummary:")


def strings(t):
    return collections.Counter(STR.findall(t))


def comments(t):
    out = []
    for line in t.splitlines():
        inq = False
        for i, ch in enumerate(line):
            if ch == '"':
                inq = not inq
            elif ch == "'" and not inq:
                out.append(re.sub(r"^'\s*", "'", line[i:].strip()))
                break
    return collections.Counter(out)


def squash(t):
    return re.sub(r"\s+", "", STR.sub('""', t)).upper()


res = collections.Counter()
bad = []
for f in sorted(list(T.glob("compile_tests/**/*.bas")) + list(T.glob("qbasic_testcases/**/*.bas"))):
    src = f.read_bytes().decode("latin-1")
    try:
        p = subprocess.run([str(FMT), "--stdout", str(f)], capture_output=True, timeout=60)
    except subprocess.TimeoutExpired:
        res["timeout"] += 1
        bad.append((f, "timeout"))
        continue
    if p.returncode != 0:
        res["fmt error"] += 1
        bad.append((f, p.stderr.decode("latin-1")[:120].replace("\n", " ")))
        continue
    out = SUMMARY.split(p.stdout.decode("latin-1"))[0]
    probs = []
    if strings(src) != strings(out):
        probs.append("strings")
    if comments(src) != comments(out):
        probs.append("comments")
    if squash(src) != squash(out):
        probs.append("tokens")
    if probs:
        res["changed: " + "+".join(probs)] += 1
        bad.append((f, ",".join(probs)))
    else:
        res["ok"] += 1
for k, v in sorted(res.items()):
    print(f"{v:5}  {k}")
print("examples:")
for f, why in bad[:15]:
    print("  ", f.relative_to(T), "-", why)
