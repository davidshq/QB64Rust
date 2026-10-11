#!/usr/bin/env python3
"""Writes the drawing programs cut from Civil War Strategy into tests/corpus/screen/.

    python tools/screen/cut_game.py

Each program is a few lines written here (a comment, DEFINT A-Z as in the game, the shared
variables its procedures use, the calls, SYSTEM) followed by procedures of
tests/programs/civil-war-strategy/CWSTRAT.BAS, copied byte for byte with LF line ends. So what is
drawn is the game's own code, and a program can be made again when the copy of the game changes.
The recordings (.output, .screen) are made by the corpus runner's --record, not by this script.
"""

from __future__ import annotations

import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent.parent
GAME = REPO / "tests" / "programs" / "civil-war-strategy" / "CWSTRAT.BAS"
OUT = REPO / "tests" / "corpus" / "screen"

# name: (what it covers, the lines before the procedures, the procedures taken from the game)
CUTS = {
    "g01_flags": (
        (
            "LINE with B and BF, PSET, SELECT CASE, text in SCREEN 12: both flags, and the row of flags "
            "of the game's end screen"
        ),
        [
            "SCREEN 12",
            "CALL flags(1, 0, 0)",
            "CALL flags(2, -100, 300)",
            "FOR i = -565 TO 50 STEP 50: CALL flags(1, i, 100): NEXT i",
        ],
        ["flags"],
    ),
    "g02_cannon": (
        (
            "DRAW strings, POINT(0) and POINT(1), CIRCLE with arcs and an aspect, PAINT with a border, "
            "VIEW and CLS 1"
        ),
        ["SCREEN 12", "CALL cannon"],
        ["clrrite", "cannon"],
    ),
    "g03_capitol": (
        (
            "the end screen without its music (noise is 0): CIRCLE arc, DRAW with a scale and in loops, "
            "GOTO, LOCATE and PRINT over a filled box"
        ),
        ["CALL capitol"],
        ["capitol", "flags"],
    ),
    "g04_ironclad": (
        "LINE -(x, y) chains, PAINT with and without a border colour",
        ["CALL ironclad"],
        ["ironclad"],
    ),
    "g05_schooner": (
        "LINE with the colour left out, PAINT without a colour",
        ["CALL schooner"],
        ["schooner"],
    ),
    "g06_message_box": (
        (
            "graphics GET and PUT, VIEW, CLS 1: a message box drawn and taken away again. TICK waits "
            "for a key, which the keys file gives"
        ),
        [
            "DIM SHARED mtx$(21), tlx, tly, size",
            "SCREEN 12",
            "LINE (100, 100)-(540, 300), 2, BF",
            "CIRCLE (320, 200), 90, 14",
            'CALL image2("A message over the drawing", 4)',
        ],
        ["image2", "mxw", "TICK"],
    ),
}
KEYS = {"g06_message_box": "text x\n"}


def procedure(lines: list[bytes], name: str) -> list[bytes]:
    """The lines of SUB name ... END SUB in the game."""
    start = [
        i for i, line in enumerate(lines) if line.split(b"(")[0].split() == [b"SUB", name.encode()]
    ]
    if len(start) != 1:
        raise SystemExit(f"SUB {name}: found {len(start)} times in {GAME.name}")
    end = next(i for i in range(start[0], len(lines)) if lines[i].strip() == b"END SUB")
    return lines[start[0] : end + 1]


def main() -> int:
    lines = GAME.read_bytes().replace(b"\r\n", b"\n").split(b"\n")
    for name, (covers, head, procedures) in CUTS.items():
        out = [
            f"' Cut from Civil War Strategy (tools/screen/cut_game.py): {covers}.".encode(),
            b"DEFINT A-Z",
            *(line.encode() for line in head),
            b"SYSTEM",
        ]
        for p in procedures:
            out += [b"", *procedure(lines, p)]
        (OUT / f"{name}.bas").write_bytes(b"\n".join(out) + b"\n")
        screen = OUT / f"{name}.screen"
        if not screen.exists():
            screen.write_bytes(b"")  # marks a screen program before its first recording
        if name in KEYS:
            (OUT / f"{name}.keys").write_bytes(KEYS[name].encode())
        print(f"{name}.bas")
    return 0


if __name__ == "__main__":
    sys.exit(main())
