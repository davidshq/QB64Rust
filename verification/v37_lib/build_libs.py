"""Builds the libraries of the verification programs v37_* from the C sources beside this script.

Uses the C compiler bundled with the reference clone (../../../QB64pe/internal/c/c_compiler/bin, or
--qb-root). Writes into verification/, beside the programs (git-ignored), because the old compiler finds
a library without a header only as "./name" beside the source (or by a path from its own folder):
  v37c.a    the C library v37c.c, for DECLARE LIBRARY "./v37c"
  v37s.a    the same, for DECLARE STATIC LIBRARY "./v37s"
  v37o.o    the object file v37o.c, for DECLARE LIBRARY "./v37o"
  v37cpp.a  the C++ library v37cpp.cpp (mangled names)
  v37dll.dll  the DLL v37dll.c, for DECLARE DYNAMIC LIBRARY "./v37dll"

Usage: python verification/v37_lib/build_libs.py [--qb-root <QB64pe clone>]
"""

import argparse
import subprocess
import sys
from pathlib import Path


def run(cmd: list[str], cwd: Path) -> None:
    print(" ".join(cmd))
    subprocess.run(cmd, cwd=cwd, check=True)


def main() -> int:
    here = Path(__file__).resolve().parent
    out = here.parent
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--qb-root", type=Path, default=here.parents[2] / "QB64pe")
    args = ap.parse_args()
    bin_dir = args.qb_root / "internal" / "c" / "c_compiler" / "bin"
    gcc, gxx, ar = (str(bin_dir / f"{t}.exe") for t in ("gcc", "g++", "ar"))
    if not Path(gcc).is_file():
        print(f"no C compiler at {bin_dir}", file=sys.stderr)
        return 1

    def o(name: str) -> str:
        return str(out / name)

    run([gcc, "-O2", "-c", "v37c.c", "-o", o("v37c.o")], here)
    for lib in ("v37c.a", "v37s.a"):
        (out / lib).unlink(missing_ok=True)
        run([ar, "rcs", o(lib), o("v37c.o")], here)
    (out / "v37c.o").unlink()
    run([gcc, "-O2", "-c", "v37o.c", "-o", o("v37o.o")], here)
    run([gxx, "-O2", "-c", "v37cpp.cpp", "-o", o("v37cpp.o")], here)
    (out / "v37cpp.a").unlink(missing_ok=True)
    run([ar, "rcs", o("v37cpp.a"), o("v37cpp.o")], here)
    (out / "v37cpp.o").unlink()
    run([gcc, "-O2", "-shared", "v37dll.c", "-o", o("v37dll.dll")], here)
    return 0


if __name__ == "__main__":
    sys.exit(main())
