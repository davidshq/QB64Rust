"""Builds the libraries of the verification programs v37_* from the C sources beside this script.

On Windows it uses the C compiler bundled with the reference clone (../../../QB64pe/internal/c/c_compiler/bin,
or --qb-root); on Linux gcc, g++ and ar on PATH (OpenSpec change m7-linux, task 3.5). Writes into
verification/, beside the programs (git-ignored), because the old compiler finds a library without a header
only as "./name" beside the source (or by a path from its own folder):
  v37c.a    the C library v37c.c, for DECLARE LIBRARY "./v37c"
  v37s.a    the same, for DECLARE STATIC LIBRARY "./v37s"
  v37o.o    the object file v37o.c, for DECLARE LIBRARY "./v37o"
  v37cpp.a  the C++ library v37cpp.cpp (mangled names)
  v37dll.dll  the DLL v37dll.c, for DECLARE DYNAMIC LIBRARY "./v37dll"
On Linux each is named as the old compiler's Linux build looks for it (lib<name>.a, lib<name>.o) and the DLL
is the shared object libv37dll.so; the programs are the same.

Usage: python verification/v37_lib/build_libs.py [--qb-root <QB64pe clone>]
"""

import argparse
import subprocess
import sys
from pathlib import Path

LINUX = sys.platform.startswith("linux")


def run(cmd: list[str], cwd: Path) -> None:
    print(" ".join(cmd))
    subprocess.run(cmd, cwd=cwd, check=True)


def file_names() -> dict[str, str]:
    """The file each library is written to on this host, by its Windows name."""
    names = ("v37c.a", "v37s.a", "v37o.o", "v37cpp.a", "v37dll.dll")
    if not LINUX:
        return {n: n for n in names}
    linux = {n: f"lib{n}" for n in names}
    linux["v37dll.dll"] = "libv37dll.so"
    return linux


def main() -> int:
    here = Path(__file__).resolve().parent
    out = here.parent
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--qb-root", type=Path, default=here.parents[2] / "QB64pe")
    args = ap.parse_args()
    if LINUX:
        gcc, gxx, ar = "gcc", "g++", "ar"
        pic = ["-fPIC"]
    else:
        bin_dir = args.qb_root / "internal" / "c" / "c_compiler" / "bin"
        gcc, gxx, ar = (str(bin_dir / f"{t}.exe") for t in ("gcc", "g++", "ar"))
        pic = []
        if not Path(gcc).is_file():
            print(f"no C compiler at {bin_dir}", file=sys.stderr)
            return 1
    name = file_names()

    def o(n: str) -> str:
        return str(out / n)

    run([gcc, "-O2", *pic, "-c", "v37c.c", "-o", o("v37c.o")], here)
    for lib in ("v37c.a", "v37s.a"):
        (out / name[lib]).unlink(missing_ok=True)
        run([ar, "rcs", o(name[lib]), o("v37c.o")], here)
    (out / "v37c.o").unlink()
    run([gcc, "-O2", *pic, "-c", "v37o.c", "-o", o(name["v37o.o"])], here)
    run([gxx, "-O2", *pic, "-c", "v37cpp.cpp", "-o", o("v37cpp.o")], here)
    (out / name["v37cpp.a"]).unlink(missing_ok=True)
    run([ar, "rcs", o(name["v37cpp.a"]), o("v37cpp.o")], here)
    (out / "v37cpp.o").unlink()
    run([gcc, "-O2", *pic, "-shared", "v37dll.c", "-o", o(name["v37dll.dll"])], here)
    return 0


if __name__ == "__main__":
    sys.exit(main())
