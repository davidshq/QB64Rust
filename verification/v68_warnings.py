"""Records the warnings qb64pe -w gives for the v68 family (OpenSpec change `lints`, task 5.1).

For each program named (default: every verification/v68_*.bas), runs `qb64pe -z -w -q` on it from
verification/ and writes its console output, then "exit=<code>", to verification/v68_warnings/<name>.txt.
`-z` writes the C++ only: nothing is built or run. The folder's full path is written as <verification>, as
the test runner does. The CLI test `qbpe_warnings_as_recorded` (crates/driver/tests/it/cli.rs) compares
`qb64rust -w -z` with these files.

Usage: python verification/v68_warnings.py [name ...]
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
QB = REPO.parent / "QB64pe"
OUT = HERE / "v68_warnings"

sys.path.insert(0, str(REPO / "tools" / "legacy_tests"))
from clone_lock import lock_clone  # pylint: disable=wrong-import-position,import-error


def scrub(text: str) -> str:
    for form in (str(HERE), str(HERE).replace("\\", "/")):
        text = text.replace(form, "<verification>").replace(form.lower(), "<verification>")
    return text


def record(bas: Path) -> None:
    run = subprocess.run(
        [str(QB / "qb64pe.exe"), "-z", "-w", "-q", bas.name],
        cwd=HERE,
        capture_output=True,
        text=True,
        errors="replace",
        timeout=600,
        check=False,
    )
    text = scrub(run.stdout + run.stderr) + f"exit={run.returncode}\n"
    (OUT / f"{bas.stem}.txt").write_text(text, newline="\n")
    print(f"== {bas.stem}\n{text}", end="")


def main() -> int:
    names = sys.argv[1:]
    programs = [HERE / f"{n}.bas" for n in names] if names else sorted(HERE.glob("v68_*.bas"))
    OUT.mkdir(exist_ok=True)
    _lock = lock_clone(wait=True)  # held until the process ends
    for bas in programs:
        record(bas)
    return 0


if __name__ == "__main__":
    sys.exit(main())
