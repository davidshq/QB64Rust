#!/usr/bin/env python3
"""Builds and runs a program with QuickBASIC 4.5 or BASIC PDS 7.1 under DOSBox: the QB 4.5 oracle
(DECISIONS.md 2026-10-26), and the PDS 7.1 one (2026-11-05).

Usage: qb45.py [--compiler qb45|pds71] [--qb45 DIR] [--pds71 DIR] [--dosbox EXE] [--timeout S] [--record]
       PROGRAM.bas...

--compiler picks the compiler; the default is pds71 when --pds71 is given, else qb45. A folder option of
the other compiler is refused. Each compiler has a folder of its unpacked files, outside this repo
(README.md says where they lie and how they were made):
- QB 4.5: BC.EXE, LINK.EXE, BCOM45.LIB. --qb45, else the environment variable QB64RUST_QB45_ROOT, else
  ../../qb64contain/qb45/install from the repo's root.
- PDS 7.1: BC.EXE, LINK.EXE and BCL71ENR.LIB (made by pds71_install.py). --pds71, else
  QB64RUST_PDS71_ROOT, else ../pds71/install from the repo's root.

Each program is compiled with `BC /O /E /X /D` (stand-alone, ON ERROR and RESUME NEXT allowed, and the
run-time checks the QB 4.5 and PDS 7.1 editors always make: without /D an index out of bounds raises
nothing), linked and run in a fresh folder mounted as D:, with DOSBox's window given SDL's dummy video
driver so nothing shows.

PRINT writes to the screen, which DOS cannot redirect, so a program writes what it measures to the file
OUT.TXT (OPEN "OUT.TXT" FOR OUTPUT AS #1: PRINT #1, ...). The result is the compiler's messages when it
reported an error, then OUT.TXT, then a line if the run timed out (an untrapped run-time error waits for a
key, so it ends at the timeout). With --record it is written beside the program, to <name>.qb45.txt or
<name>.pds71.txt; otherwise it is printed.
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path

DEFAULT_DOSBOX = Path(r"C:\Program Files (x86)\DOSBox-0.74-3\DOSBox.exe")
# Where QB 4.5 lies on the development machine (README.md): <qb64contain>\qb45\install, with <qb64contain> two
# folders above the repo's root, beside the folder that holds the repo (not that folder itself).
DEFAULT_QB45 = (
    Path(__file__).resolve().parents[2] / ".." / ".." / "qb64contain" / "qb45" / "install"
)
# Where PDS 7.1 lies (README.md): <repo's parent>\pds71\install, beside the repo.
DEFAULT_PDS71 = Path(__file__).resolve().parents[2] / ".." / "pds71" / "install"
TIMEOUT = 60


@dataclass(frozen=True)
class Compiler:
    """One of the old compilers the oracle runs."""

    name: str  # the --compiler value, and the name of its folder option
    label: str  # for messages
    env: str  # the environment variable that gives its folder
    default: Path  # its folder when neither the option nor the variable gives one
    needs: tuple[str, ...]  # files its folder must hold
    suffix: str  # of the record written beside the program


COMPILERS = {
    "qb45": Compiler(
        "qb45", "QB 4.5", "QB64RUST_QB45_ROOT", DEFAULT_QB45, ("BC.EXE",), ".qb45.txt"
    ),
    "pds71": Compiler(
        "pds71",
        "PDS 7.1",
        "QB64RUST_PDS71_ROOT",
        DEFAULT_PDS71,
        ("BC.EXE", "BCL71ENR.LIB"),
        ".pds71.txt",
    ),
}

# The banner BC prints before its messages, and its memory report: neither depends on the program.
# QB 4.5's lines, then PDS 7.1's.
BANNER = (
    "Microsoft (R) QuickBASIC Compiler",
    "(C) Copyright Microsoft",
    "All rights reserved.",
    "Simultaneously published",
    "Microsoft (R) BASIC Compiler",
    "Copyright (C) Microsoft",
)
LINK_BANNER = (
    "Microsoft (R) Overlay Linker",
    "Microsoft (R) Segmented-Executable Linker",
    "Copyright (C) Microsoft",
)
# BC's last line, after any messages: "    0 Severe  Error(s)". Without it BC did not finish.
BC_SUMMARY = re.compile(r"Severe\s+Error\(s\)")


@dataclass
class Run:
    """One build and run under DOSBox."""

    text: str  # the result text; empty when BC did not finish
    built: bool  # BC finished: its summary line is in P.CMP
    timed_out: bool
    silent: bool  # an executable was made and the run ended in time, but there is no OUT.TXT
    compile_log: str = ""  # P.CMP as BC wrote it ("" when there is none)


def bc_finished(compile_log: str) -> bool:
    """Whether BC ran to its end. Seen once on 2026-11-02: a run gave neither BC's messages nor
    OUT.TXT, and the same program ran in full the next time. P.CMP was missing, empty or only BC's
    banner."""
    return BC_SUMMARY.search(compile_log) is not None


def dos_text(text: str) -> bytes:
    """A program's source as QB 4.5 reads it: CRLF line ends, code page 437."""
    lines = text.replace("\r\n", "\n").replace("\r", "\n").split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    return "".join(line + "\r\n" for line in lines).encode("cp437")


def conf_text(qb45: Path, work: Path) -> str:
    """The DOSBox configuration: the compiler's folder on C:, the program's folder on D:, build, link,
    run, exit. The same for QB 4.5 and PDS 7.1: each LINK finds its default library through LIB."""
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


def run(bas: Path, qb45: Path, dosbox: Path, timeout: int) -> Run:
    """Builds and runs bas with the compiler in folder qb45 (QB 4.5 or PDS 7.1) in a fresh folder."""
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
        compile_log = log.read_text(encoding="cp437") if log.is_file() else ""
        out_text = out.read_text(encoding="cp437") if out.is_file() else None
        exe = (work / "P.EXE").is_file()
        if not bc_finished(compile_log):
            return Run("", False, timed_out, False, compile_log)
        text = result_text(
            compile_log,
            out_text,
            timed_out,
            timeout,
            link.read_text(encoding="cp437") if link.is_file() and not exe else None,
        )
        silent = exe and out_text is None and not timed_out
        return Run(text, True, timed_out, silent, compile_log)


def no_build_message(result: Run, timeout: int) -> str:
    """Why a run gave no result: BC did not finish, and whether DOSBox ended or hung."""
    if result.timed_out:
        return f"BC did not finish: DOSBox hung, timed out after {timeout} s"
    return "BC did not finish: DOSBox ended first"


def unfinished_log(compile_log: str) -> str:
    """What BC wrote before it stopped, for stderr: its messages without the banner, or a line
    saying there were none."""
    messages = compile_messages(compile_log)
    if messages:
        return "".join(f"  {line}\n" for line in messages)
    if compile_log.strip():
        return "  (no messages from BC)\n"
    return "  (no P.CMP, or an empty one)\n"


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("programs", nargs="+", type=Path, help="the .bas files")
    ap.add_argument(
        "--compiler",
        choices=sorted(COMPILERS),
        default=None,
        help="the old compiler to run (default: pds71 when --pds71 is given, else qb45)",
    )
    ap.add_argument("--qb45", type=Path, default=None, help="QB 4.5's folder")
    ap.add_argument("--pds71", type=Path, default=None, help="PDS 7.1's folder")
    ap.add_argument("--dosbox", type=Path, default=None, help="DOSBox.exe")
    ap.add_argument("--timeout", type=int, default=TIMEOUT, help="seconds per program")
    ap.add_argument(
        "--record",
        action="store_true",
        help="write <name>.qb45.txt (or .pds71.txt) beside each program",
    )
    args = ap.parse_args(argv)
    # A folder given for one compiler picks it, so that --pds71 alone never records a QB 4.5 run.
    given = [name for name in COMPILERS if getattr(args, name) is not None]
    if args.compiler is None:
        if len(given) > 1:
            options = " and ".join(f"--{name}" for name in given)
            print(f"give only one of {options}, or --compiler", file=sys.stderr)
            return 2
        args.compiler = given[0] if given else "qb45"
    if any(name != args.compiler for name in given):
        others = ", ".join(f"--{name}" for name in given if name != args.compiler)
        print(f"{others} does not go with --compiler {args.compiler}", file=sys.stderr)
        return 2
    compiler = COMPILERS[args.compiler]
    root = Path(getattr(args, compiler.name) or os.environ.get(compiler.env) or compiler.default)
    missing = [name for name in compiler.needs if not (root / name).is_file()]
    if missing:
        msg = (
            f"no {compiler.label} in {root} ({', '.join(missing)} missing): give --{compiler.name}"
            f" or {compiler.env}, a folder with {', '.join(compiler.needs)}"
        )
        print(msg, file=sys.stderr)
        return 2
    args.qb45 = root
    dosbox = args.dosbox or (DEFAULT_DOSBOX if DEFAULT_DOSBOX.is_file() else shutil.which("dosbox"))
    if dosbox is None or not Path(dosbox).is_file():
        print("no DOSBox: give --dosbox", file=sys.stderr)
        return 2
    status = 0
    for bas in args.programs:
        result = run(bas, Path(args.qb45).resolve(), Path(dosbox), args.timeout)
        if not result.built:
            why = no_build_message(result, args.timeout)
            print(f"{bas.name}: {why}; running it once more", file=sys.stderr)
            result = run(bas, Path(args.qb45).resolve(), Path(dosbox), args.timeout)
        if not result.built:
            why = no_build_message(result, args.timeout)
            print(f"{bas.name}: {why}, twice; nothing recorded. P.CMP:", file=sys.stderr)
            print(unfinished_log(result.compile_log), end="", file=sys.stderr)
            status = 1
            continue
        if result.silent:
            # Every q45_ program opens OUT.TXT first: likely DOSBox ending early, not the program.
            warning = "warning: the program was built and ran, but wrote no OUT.TXT"
            print(f"{bas.name}: {warning}", file=sys.stderr)
        text = result.text
        if args.record:
            record = bas.with_name(bas.stem + compiler.suffix)
            record.write_text(text, encoding="utf-8", newline="\n")
            print(f"{bas.name}: recorded")
        else:
            print(f"== {bas.name}\n{text}", end="")
    return status


if __name__ == "__main__":
    sys.exit(main())
