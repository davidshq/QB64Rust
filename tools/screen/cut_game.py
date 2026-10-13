#!/usr/bin/env python3
"""Writes the drawing programs cut from Civil War Strategy into tests/corpus/screen/.

    python tools/screen/cut_game.py

Each program is a few lines written here (a comment, DEFINT A-Z as in the game, the shared
variables its procedures use, the calls, SYSTEM) followed by procedures of
tests/programs/civil-war-strategy/CWSTRAT.BAS, copied byte for byte with LF line ends. So what is
drawn is the game's own code, and a program can be made again when the copy of the game changes.
Where a procedure of the game cannot be cut whole (the map, the reports: they read the whole state
of a game), lines of it stand among the written ones: a (first line, last line, start of the first
line) entry copies that range of the game byte for byte, and the script stops when the first line
no longer starts as said. A program that loads pictures gets copies of them in <name>.files/,
which the corpus runner puts beside the program when it runs (tools/legacy_tests).
The recordings (.output, .screen) are made by the corpus runner's --record, not by this script.
"""

from __future__ import annotations

import shutil
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent.parent
GAME = REPO / "tests" / "programs" / "civil-war-strategy" / "CWSTRAT.BAS"
OUT = REPO / "tests" / "corpus" / "screen"

# name: (what it covers, the lines before the procedures, the procedures taken from the game).
# A line before the procedures is text written here, or a range of the game's lines.
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
    "g07_mountains": (
        (
            "BLOAD of a picture into an array in conventional memory (DEF SEG, VARSEG, VARPTR) and "
            "graphics PUT with PSET: the game's mountains, loaded as its main module loads them and "
            "drawn as the first lines of SUB usa draw them"
        ),
        [
            "DIM SHARED mtn(1 TO 1564)",
            (139, 141, "\tDEF SEG = VARSEG(mtn(1))"),
            (4145, 4160, "SCREEN 12"),
        ],
        [],
    ),
    "g08_icon": (
        (
            "BLOAD, PUT and a GET out of the picture just drawn: the game's icon and the capital "
            "marker its main module takes from it, then the marker put in a row as SUB showcity "
            "puts it on a city"
        ),
        [
            "DIM SHARED graphic(1 TO 1564), Ncap(60)",
            (169, 175, "SCREEN 12"),
            "FOR i = 0 TO 5: PUT (300 + 20 * i, 200 + 5 * i), Ncap, PSET: NEXT i",
        ],
        [],
    ),
    "g09_faces_forts": (
        (
            "BLOAD with a file name in a string, eight pictures one after the other into the same two "
            "arrays: the faces SUB battle loads and the forts SUB menu loads, each with the game's own "
            "three lines, put in two rows"
        ),
        [
            "DIM SHARED graft(1 TO 1564)",
            "DIM SHARED graphic(1 TO 1564)",
            "SCREEN 12",
            "FOR k = 1 TO 5",
            'IF k < 4 THEN t$ = "fort" + LTRIM$(STR$(k - 1)) + ".vga"',
            "IF k < 4 THEN",
            (4817, 4819, "\t\t\t\t\tDEF SEG = VARSEG(graft(1))"),
            "PUT (100 * k - 80, 150), graft, PSET",
            "END IF",
            't$ = "face" + LTRIM$(STR$(k)) + ".vga"',
            (1008, 1010, "\t\tDEF SEG = VARSEG(graphic(1))"),
            "PUT (100 * k - 80, 20), graphic, PSET",
            "NEXT k",
        ],
        [],
    ),
    "g10_snapshot": (
        (
            "graphics GET and PUT through a SUB on a shared array: the game's snapshot takes the screen "
            "under a place and puts it back, here over a drawing that was changed in between, and at "
            "a second place"
        ),
        [
            "DIM SHARED image(300)",
            "SCREEN 12",
            "LINE (80, 80)-(120, 120), 9, BF",
            "CIRCLE (100, 100), 8, 14",
            "CALL snapshot(100, 100, 0)",
            "LINE (60, 60)-(140, 140), 4, BF",
            "CALL snapshot(100, 100, 1)",
            "CALL snapshot(300, 200, 1)",
        ],
        ["snapshot"],
    ),
    "g11_city_report": (
        (
            "TAB in PRINT in SCREEN 12, after LOCATE and with numbers: the game's city report, the "
            "lines of SUB report from its label cityrep on, over values written here"
        ),
        [
            "DIM SHARED month$(12), city$(40), cityp(40), cityv(40), force$(2), control(2), income(2), cash(2)",
            "SCREEN 12",
            'month = 4: year = 1861: month$(4) = "April"',
            'force$(0) = "NEUTRAL": force$(1) = "Union": force$(2) = "Rebel"',
            "control(1) = 14: control(2) = 12: income(1) = 420: income(2) = 310: cash(1) = 1250: cash(2) = 980",
            "FOR i = 1 TO 40",
            'city$(i) = "City" + LTRIM$(STR$(i)): cityp(i) = i MOD 3: cityv(i) = 5 * i',
            "NEXT i",
            (3553, 3571, "CLS : LOCATE 1, 1: COLOR 14"),
        ],
        [],
    ),
}
KEYS = {"g06_message_box": "text x\n"}
# The pictures a program loads, by their names in the game's folder; copied into <name>.files/.
FILES = {
    "g07_mountains": ["MTN.VGA"],
    "g08_icon": ["CWSICON.VGA"],
    "g09_faces_forts": [f"FACE{k}.VGA" for k in range(1, 6)] + [f"FORT{k}.VGA" for k in range(3)],
}


def procedure(lines: list[bytes], name: str) -> list[bytes]:
    """The lines of SUB name ... END SUB in the game."""
    start = [
        i for i, line in enumerate(lines) if line.split(b"(")[0].split() == [b"SUB", name.encode()]
    ]
    if len(start) != 1:
        raise SystemExit(f"SUB {name}: found {len(start)} times in {GAME.name}")
    end = next(i for i in range(start[0], len(lines)) if lines[i].strip() == b"END SUB")
    return lines[start[0] : end + 1]


def head_lines(lines: list[bytes], head: list) -> list[bytes]:
    """The lines before the procedures: written ones as they are, a (first, last, start) entry
    as lines first to last of the game (counted from 1), whose first must start with `start`."""
    out: list[bytes] = []
    for item in head:
        if isinstance(item, str):
            out.append(item.encode())
            continue
        first, last, start = item
        if not lines[first - 1].startswith(start.encode()):
            raise SystemExit(f"{GAME.name} line {first} no longer starts with {start!r}")
        out += lines[first - 1 : last]
    return out


def main() -> int:
    lines = GAME.read_bytes().replace(b"\r\n", b"\n").split(b"\n")
    for name, (covers, head, procedures) in CUTS.items():
        out = [
            f"' Cut from Civil War Strategy (tools/screen/cut_game.py): {covers}.".encode(),
            b"DEFINT A-Z",
            *head_lines(lines, head),
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
        if name in FILES:
            folder = OUT / f"{name}.files"
            folder.mkdir(exist_ok=True)
            for picture in FILES[name]:
                shutil.copyfile(GAME.parent / picture, folder / picture)
        print(f"{name}.bas")
    return 0


if __name__ == "__main__":
    sys.exit(main())
