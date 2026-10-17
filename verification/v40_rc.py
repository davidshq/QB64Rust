"""Records the resource files the old compiler writes for $VERSIONINFO (m3-versioninfo, task 1.1).

For each program named (default: every verification/v40_*.bas), runs `qb64pe -z -w -q` on it from
verification/ and copies what it wrote into the clone's internal/temp for the resource compiler:
icon.rc, manifest.h and the manifest (<program>.exe.manifest), into verification/v40_rc/ as
<name>.icon.rc, <name>.manifest.h and <name>.manifest; its console output (with the warnings -w
prints) goes to <name>.z.txt. A file it did not write is not recorded. The folder's full path is
written as <verification>, as the test runner does.

Usage: python verification/v40_rc.py [name ...]
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
QB = REPO.parent / "QB64pe"
TEMP = QB / "internal" / "temp"
OUT = HERE / "v40_rc"

sys.path.insert(0, str(REPO / "tools" / "legacy_tests"))
from clone_lock import lock_clone  # pylint: disable=wrong-import-position,import-error


def scrub(text: str) -> str:
    for form in (str(HERE), str(HERE).replace("\\", "/")):
        text = text.replace(form, "<verification>").replace(form.lower(), "<verification>")
    return text


def record(bas: Path) -> None:
    for p in [TEMP / "icon.rc", TEMP / "manifest.h", *TEMP.glob("*.manifest")]:
        p.unlink(missing_ok=True)
    run = subprocess.run(
        [str(QB / "qb64pe.exe"), "-z", "-w", "-q", str(bas)],
        cwd=HERE,
        capture_output=True,
        text=True,
        errors="replace",
        timeout=600,
        check=False,
    )
    name = bas.stem
    (OUT / f"{name}.z.txt").write_text(
        scrub(run.stdout + run.stderr) + f"exit={run.returncode}\n", newline="\n"
    )
    found = {"icon.rc": TEMP / "icon.rc", "manifest.h": TEMP / "manifest.h"}
    manifests = sorted(TEMP.glob("*.manifest"))
    if manifests:
        found["manifest"] = manifests[0]
    for suffix, p in found.items():
        if p.is_file():
            (OUT / f"{name}.{suffix}").write_bytes(p.read_bytes())
    print(name, ", ".join(s for s, p in found.items() if p.is_file()) or "(no resource)")


def main() -> int:
    names = sys.argv[1:]
    programs = (
        [HERE / f"{n}.bas" for n in names] if names else sorted(HERE.glob("v40_*.bas"))
    )
    OUT.mkdir(exist_ok=True)
    _lock = lock_clone(wait=True)  # held until the process ends
    for bas in programs:
        record(bas)
    return 0


if __name__ == "__main__":
    sys.exit(main())
