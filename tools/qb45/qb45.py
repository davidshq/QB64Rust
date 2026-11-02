#!/usr/bin/env python3
"""Builds and runs a program with QuickBASIC 4.5 under DOSBox: the QB 4.5 oracle (DECISIONS.md 2026-10-26).

Usage: qb45.py [--qb45 DIR] [--dosbox EXE] [--timeout S] [--record] PROGRAM.bas...

DIR is the folder of QB 4.5's unpacked files (BC.EXE, LINK.EXE, BCOM45.LIB; default: the environment
variable QB64RUST_QB45_ROOT, else ../../qb64contain/qb45/install from the repo's root). It is outside this
repo; README.md says where it lies and how it was made. Each program is
compiled with `BC /O /E /X /D` (stand-alone, ON ERROR and RESUME NEXT allowed, and the run-time checks
QB 4.5's editor always makes: without /D an index out of bounds raises nothing), linked and run in a fresh
folder mounted as D:, with DOSBox's window given SDL's dummy video driver so nothing shows.

QB 4.5's PRINT writes to the screen, which DOS cannot redirect, so a program writes what it measures to
the file OUT.TXT (OPEN "OUT.TXT" FOR OUTPUT AS #1: PRINT #1, ...). The result is the compiler's messages
when it reported an error, then OUT.TXT, then a line if the run timed out (an untrapped run-time error
waits for a key, so it ends at the timeout). With --record it is written to <name>.qb45.txt beside the
program; otherwise it is printed.
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
# Where QB 4.5 lies on the development machine (README.md): <qb64contain>\qb45\install, with <qb64contain> two
# folders above the repo's root, beside the folder that holds the repo (not that folder itself).
DEFAULT_QB45 = Path(__file__).resolve().parents[2] / ".." / ".." / "qb64contain" / "qb45" / "install"
TIMEOUT = 60

# The banner BC prints before its messages, and its memory report: neither depends on the program.
BANNER = (
    "Microsoft (R) QuickBASIC Compiler",
    "(C) Copyright Microsoft",
    "All rights reserved.",
    "Simultaneously published",
)
LINK_BANNER = ("Microsoft (R) Overlay Linker", "Copyright (C) Microsoft")


def dos_text(text: str) -> bytes:
    """A program's source as QB 4.5 reads it: CRLF line ends, code page 437."""
    lines = text.replace("\r\n", "\n").replace("\r", "\n").split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    return "".join(line + "\r\n" for line in lines).encode("cp437")


def conf_text(qb45: Path, work: Path) -> str:
    """The DOSBox configuration: QB 4.5 on C:, the program's folder on D:, build, link, run, exit."""
    return "\n".join(
        [
            "[sdl]",
            "fullscreen=false",
            "output=surface",
            "",
            "[cpu]",
            "cycles=max",
            "",
            "[autoexec]",
            f'mount c "{qb45}"',
            f'mount d "{work}"',
            "path c:\\",
            "set LIB=c:\\",
            "d:",
            "bc P.BAS /O /E /X /D; > P.CMP",
            "if exist P.OBJ link P.OBJ; > P.LNK",
            "if exist P.EXE P.EXE",
            "exit",
            "",
        ]
    )


def compile_messages(log: str) -> list[str]:
    """BC's messages without the banner and the memory report; empty when it found no error. The bell
    BC writes after its ^ is dropped, as is any other control character."""
    text = "".join(c for c in log.replace("\r\n", "\n") if c in "\n\t" or c >= " ")
    lines = [line.rstrip() for line in text.split("\n")]
    kept = [
        line
        for line in lines
        if line.strip()
        and not line.startswith(BANNER)
        and not line.endswith(("Bytes Available", "Bytes Free"))
    ]
    if all(line.strip().startswith("0 ") for line in kept):
        return []
    return kept


def result_text(
    compile_log: str, out: str | None, timed_out: bool, timeout: int, link_log: str | None = None
) -> str:
    """The result of one program: the compiler's messages if any, the linker's when no executable was
    made (link_log), OUT.TXT, and a timeout line."""
    parts = []
    messages = compile_messages(compile_log)
    if messages:
        parts.append("compile:\n" + "\n".join(messages) + "\n")
    if link_log is not None:
        lines = [line.rstrip() for line in link_log.replace("\r\n", "\n").split("\n")]
        kept = [
            line for line in lines if line.strip() and not line.lstrip().startswith(LINK_BANNER)
        ]
        parts.append("link (no executable):\n" + "".join(line + "\n" for line in kept))
    if out is not None:
        parts.append(out.replace("\r\n", "\n"))
    elif not messages:
        parts.append("(no OUT.TXT)\n")
    if timed_out:
        parts.append(
            f"(timed out after {timeout} s: an untrapped run-time error waits for a key)\n"
        )
    return "".join(parts)


def run(bas: Path, qb45: Path, dosbox: Path, timeout: int) -> str:
    """Builds and runs bas with QB 4.5 in a fresh folder; returns its result text."""
    with tempfile.TemporaryDirectory(prefix="qb45-") as tmp:
        work = Path(tmp)
        (work / "P.BAS").write_bytes(dos_text(bas.read_text(encoding="utf-8")))
        conf = work / "run.conf"
        conf.write_text(conf_text(qb45, work), encoding="ascii")
        env = dict(os.environ, SDL_VIDEODRIVER="dummy")
        timed_out = False
        try:
            subprocess.run(
                [str(dosbox), "-conf", str(conf), "-noconsole"],
                cwd=work,
                env=env,
                timeout=timeout,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=False,
            )
        except subprocess.TimeoutExpired:
            timed_out = True
        log = work / "P.CMP"
        link = work / "P.LNK"
        out = work / "OUT.TXT"
        return result_text(
            log.read_text(encoding="cp437") if log.is_file() else "",
            out.read_text(encoding="cp437") if out.is_file() else None,
            timed_out,
            timeout,
            link.read_text(encoding="cp437")
            if link.is_file() and not (work / "P.EXE").is_file()
            else None,
        )


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("programs", nargs="+", type=Path, help="the .bas files")
    ap.add_argument(
        "--qb45",
        type=Path,
        default=os.environ.get("QB64RUST_QB45_ROOT") or DEFAULT_QB45,
        help="QB 4.5's folder",
    )
    ap.add_argument("--dosbox", type=Path, default=None, help="DOSBox.exe")
    ap.add_argument("--timeout", type=int, default=TIMEOUT, help="seconds per program")
    ap.add_argument(
        "--record", action="store_true", help="write <name>.qb45.txt beside each program"
    )
    args = ap.parse_args(argv)
    if args.qb45 is None or not (Path(args.qb45) / "BC.EXE").is_file():
        msg = f"no QB 4.5 in {args.qb45}: give --qb45 or QB64RUST_QB45_ROOT, a folder with BC.EXE"
        print(msg, file=sys.stderr)
        return 2
    dosbox = args.dosbox or (DEFAULT_DOSBOX if DEFAULT_DOSBOX.is_file() else shutil.which("dosbox"))
    if dosbox is None or not Path(dosbox).is_file():
        print("no DOSBox: give --dosbox", file=sys.stderr)
        return 2
    for bas in args.programs:
        text = run(bas, Path(args.qb45).resolve(), Path(dosbox), args.timeout)
        if args.record:
            bas.with_name(bas.stem + ".qb45.txt").write_text(text, encoding="utf-8", newline="\n")
            print(f"{bas.name}: recorded")
        else:
            print(f"== {bas.name}\n{text}", end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
