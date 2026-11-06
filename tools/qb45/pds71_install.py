#!/usr/bin/env python3
"""Makes the PDS 7.1 folder that qb45.py --compiler pds71 uses, from the files of PDS 7.1's disks.

Usage: pds71_install.py [--dosbox EXE] DISKS OUT

DISKS is a folder holding the files of PDS 7.1's five 360K disks, one subfolder per disk (README.md).
Every file is expanded into OUT; then BCL71ENR.LIB is built there with PDS's own LIB.EXE
under DOSBox. Nothing of PDS goes into this repo.

- The disks' files are compressed, in the QuickBASIC form of Microsoft's SZDD format: the header "SZ "
  and 5 more bytes, the expanded size, then LZSS data whose window starts 18 bytes before its end (the
  standard SZDD form starts 16 before). The last letter of each name was replaced by "$" and is not in the
  header, so the names come from the disks' PACKING.LST. A name not listed there had a two-letter
  extension: its "$" is dropped (.FR$ is .FOR). 42 files on the disks have a "$" name but are not
  compressed (the .OB$ files among them); they are named the same way and copied as they are.
- A file on more than one disk (NMAKE.EXE) is taken from the last disk, in the subfolders' name order.
- SETUP.EXE would build BCL71ENR.LIB, the library LINK takes for `BC /O` with emulator math and near
  strings in real mode. SETUP is interactive, so this script does what it does: LIB joins the parts that
  SETUP's table selects for those choices (PARTS below).
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

DEFAULT_DOSBOX = Path(r"C:\Program Files (x86)\DOSBox-0.74-3\DOSBox.exe")
MAGIC_QB = b"SZ \x88\xf0\x27\x33\xd1"  # the form on PDS 7.1's disks
MAGIC_SZDD = b"SZDD\x88\xf0\x27\x33"  # the standard form, for completeness

# The parts of BCL71ENR.LIB. SETUP.EXE gives each part library a code of the options it serves: r real
# mode, p protected mode, e emulator math, a alternate math, n near strings, f far strings, o the
# stand-alone library (BC /O), t the run-time module; a letter left out means any. These are the parts
# whose codes allow r, e, n and o. B71ROON 2ron, B71OON 2on, B71ROOJ 2ro, B71ROEN 2ren, B71ROBN 2rn,
# B71OEJ 2e, B71OBN 2n, B71ROEJ 2re, B71ROBJ 2r, EMR 2re. Two are inferred, not read: B71OBJ shows no
# code in SETUP's strings (taken as "2", any choice, which a dump of strings of two or more characters
# leaves out), and BLIBFP's code is cut off there (BLIBFA, the alternate math library, is 2a; BLIBFP is
# taken as its emulator partner). Linking the t parts instead gives a program that wants the run-time
# module; mixing the two links but hangs at start (measured 2026-11-05).
PARTS = (
    "B71ROON", "B71OON", "B71ROOJ", "B71ROEN", "B71ROBN", "B71OEJ", "B71OBN",
    "B71ROEJ", "B71OBJ", "B71ROBJ", "EMR", "BLIBFP",
)  # fmt: skip


def expand(data: bytes, start: int, size: int, back: int) -> bytes:
    """LZSS as Microsoft's COMPRESS writes it: a flag byte for each 8 items, a set bit a literal byte,
    a clear bit a 12-bit window offset and a 4-bit length (+3). The 4096-byte window starts as blanks,
    with its write position `back` bytes before its end."""
    window = bytearray(b" " * 4096)
    pos = 4096 - back
    out = bytearray()
    i = start
    while i < len(data) and len(out) < size:
        flags = data[i]
        i += 1
        for bit in range(8):
            if i >= len(data) or len(out) >= size:
                break
            if flags & (1 << bit):
                c = data[i]
                i += 1
                out.append(c)
                window[pos] = c
                pos = (pos + 1) & 4095
            else:
                if i + 1 >= len(data):
                    break
                lo, hi = data[i], data[i + 1]
                i += 2
                offset = lo | ((hi & 0xF0) << 4)
                for k in range((hi & 0x0F) + 3):
                    c = window[(offset + k) & 4095]
                    out.append(c)
                    window[pos] = c
                    pos = (pos + 1) & 4095
    return bytes(out[:size])


def expand_file(data: bytes) -> bytes | None:
    """A compressed file's contents, or None if it is not compressed."""
    if data.startswith(MAGIC_QB):
        return expand(data, 12, int.from_bytes(data[8:12], "little"), 18)
    if data.startswith(MAGIC_SZDD):
        return expand(data, 14, int.from_bytes(data[10:14], "little"), 16)
    return None


def packing_names(text: str) -> dict[str, str]:
    """Compressed name ("BC.EX$") -> real name ("BC.EXE"), from PACKING.LST: each line that starts
    with a file name."""
    names = {}
    for line in text.splitlines():
        word = line.split("\t")[0].split(" ")[0].strip().upper()
        stem, dot, ext = word.partition(".")
        if dot and stem.isalnum() and ext.isalnum() and len(ext) == 3:
            names[word[:-1] + "$"] = word
    return names


# Extensions PACKING.LST does not list whose last letter is not simply dropped.
ODD_EXTENSIONS = {"FR": "FOR"}  # the FORTRAN samples, MXADSTF.FOR and MXSHKF.FOR


def real_name(name: str, names: dict[str, str]) -> str:
    """A file's name in the PDS folder: from PACKING.LST, else the "$" of a two-letter extension
    dropped. Some files on the disks keep a "$" name though they are not compressed (DATIM.BI$)."""
    upper = name.upper()
    if upper in names:
        return names[upper]
    if not upper.endswith("$"):
        return upper
    stem, _, ext = upper.rstrip("$").partition(".")
    return f"{stem}.{ODD_EXTENSIONS.get(ext, ext)}"


def lib_response() -> str:
    """LIB's response file: create BCL71ENR.LIB from the parts, no listing. A line ends with & to go
    on; DOS lines stay short."""
    parts = [f"+{p}.LIB" for p in PARTS]
    half = len(parts) // 2
    return (
        "BCL71ENR.LIB\r\ny\r\n"
        + "".join(parts[:half])
        + "&\r\n"
        + "".join(parts[half:])
        + "\r\nNUL;\r\n"
    )


def lib_conf(out: Path, work: Path) -> str:
    """DOSBox: PDS on C:, the response file's folder on D:, run LIB, exit."""
    return "\n".join(
        [
            "[sdl]",
            "output=surface",
            "",
            "[cpu]",
            "cycles=max",
            "",
            "[autoexec]",
            f'mount c "{out}"',
            f'mount d "{work}"',
            "c:",
            "lib @D:MKLIB.RSP > D:MKLIB.LOG",
            "exit",
            "",
        ]
    )


def install(disks: Path, out: Path) -> int:
    """Expands every disk's files into out; returns how many were expanded."""
    folders = sorted(p for p in disks.iterdir() if p.is_dir())
    names: dict[str, str] = {}
    for folder in folders:
        lst = folder / "PACKING.LST"
        if lst.is_file():
            names.update(packing_names(lst.read_text(encoding="cp437")))
    out.mkdir(parents=True, exist_ok=True)
    count = 0
    for folder in folders:
        for p in sorted(folder.iterdir()):
            if not p.is_file():
                continue
            body = expand_file(p.read_bytes())
            if body is None:
                shutil.copy2(p, out / real_name(p.name, names))
                continue
            (out / real_name(p.name, names)).write_bytes(body)
            count += 1
    return count


LIB_TIMEOUT = 120


def build_library(out: Path, dosbox: Path) -> str:
    """Builds BCL71ENR.LIB in out with LIB.EXE under DOSBox; returns LIB's log, with a line when
    DOSBox hung."""
    with tempfile.TemporaryDirectory(prefix="pds71-lib-") as tmp:
        work = Path(tmp)
        (work / "MKLIB.RSP").write_bytes(lib_response().encode("ascii"))
        conf = work / "lib.conf"
        conf.write_text(lib_conf(out, work), encoding="ascii")
        env = dict(os.environ, SDL_VIDEODRIVER="dummy")
        hung = ""
        try:
            # cwd: with -noconsole, DOSBox writes stdout.txt and stderr.txt into its current folder.
            subprocess.run(
                [str(dosbox), "-conf", str(conf), "-noconsole"],
                cwd=work,
                env=env,
                timeout=LIB_TIMEOUT,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=False,
            )
        except subprocess.TimeoutExpired:
            hung = f"(DOSBox hung: stopped after {LIB_TIMEOUT} s)\n"
        log = work / "MKLIB.LOG"
        return hung + (log.read_text(encoding="cp437") if log.is_file() else "(no log)")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("disks", type=Path, help="a folder with one subfolder per disk")
    ap.add_argument("out", type=Path, help="the PDS 7.1 folder to make")
    ap.add_argument("--dosbox", type=Path, default=None, help="DOSBox.exe")
    args = ap.parse_args(argv)
    dosbox = args.dosbox or (DEFAULT_DOSBOX if DEFAULT_DOSBOX.is_file() else shutil.which("dosbox"))
    if dosbox is None or not Path(dosbox).is_file():
        print("no DOSBox: give --dosbox", file=sys.stderr)
        return 2
    out = args.out.resolve()
    count = install(args.disks, out)
    print(f"expanded {count} files into {out}")
    missing = [p for p in PARTS if not (out / f"{p}.LIB").is_file()]
    if missing or not (out / "LIB.EXE").is_file():
        print(f"not on the disks: {', '.join(missing) or 'LIB.EXE'}", file=sys.stderr)
        return 1
    (out / "BCL71ENR.LIB").unlink(missing_ok=True)
    log = build_library(out, Path(dosbox))
    if not (out / "BCL71ENR.LIB").is_file():
        print("LIB made no BCL71ENR.LIB. Its log:\n" + log, file=sys.stderr)
        return 1
    print("built BCL71ENR.LIB")
    return 0


if __name__ == "__main__":
    sys.exit(main())
