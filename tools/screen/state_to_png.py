#!/usr/bin/env python3
"""Shows a recorded screen state: a graphics page as a PNG file, a text page as its rows.

    python tools/screen/state_to_png.py <file.screen> [--page N | --image | --source-image]
                                        [--state N | --all] [-o <out.png>]

A file may hold several states, one after the other (the `state` steps of a keys file, then the
state at exit): --state N takes the N-th (from 1), the default the last; --all writes each, the
N-th to <out>-N.png. Without --page the displayed page is taken; --image takes the destination
when it is an image that is no page, --source-image the source. A text page is printed: its
characters as CP437 shows them (the bytes below 32 as its symbols), without colours; one the
console cannot show as "?".
For a look at what a recording holds or at how two differ; no test uses it. Standard library only.
"""

from __future__ import annotations

import argparse
import re
import struct
import sys
import zlib
from pathlib import Path

# A byte of a text row that is not printable ASCII, a backslash or a quote: \xNN.
ESCAPE = re.compile(r"\\x([0-9A-F]{2})")
# What CP437 shows for the bytes 0 to 31 (and 127, in cell): printed as they are, they would
# move the cursor.
SYMBOLS = " ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼"


def cell(byte: int) -> str:
    """The character of a text cell."""
    if byte < 32:
        return SYMBOLS[byte]
    return "⌂" if byte == 127 else bytes([byte]).decode("cp437")


def runs(words: list[str]) -> list[int]:
    """The values of a run-length line: "v" or "v*n", v in hex."""
    out: list[int] = []
    for word in words:
        value, _, count = word.partition("*")
        out += [int(value, 16)] * int(count or 1)
    return out


def read_state(text: str) -> tuple[dict[str, str], dict[str, dict]]:
    """The second line's fields, and each record ("page 0", "image write", "image read") with its
    header fields, palette and rows (pixel rows as lists of values, text rows as strings)."""
    lines = text.replace("\r\n", "\n").split("\n")
    if lines[0] != "screen-state 1":
        raise SystemExit(f"not a screen state of version 1: {lines[0]!r}")
    top = lines[1].split()
    places = dict(zip(top[2::2], top[3::2]))
    records: dict[str, dict] = {}
    rec: dict = {}
    for line in lines[2:]:
        kind, _, rest = line.partition(" ")
        if kind in ("page", "image"):
            words = rest.split()
            at = words.index("mode")
            names = [f"page {words[0]}"] if kind == "page" else [f"image {w}" for w in words[:at]]
            size = words[at + 3].split("x")
            rec = {
                "mode": words[at + 1],
                "text": words[at + 2] == "text",
                "width": int(size[0]),
                "height": int(size[1]),
                "bits": int(words[at + 5]),
                "palette": [],
                "rows": [],
            }
            for name in names:
                records[name] = rec
        elif kind == "palette":
            rec["palette"] = runs(rest.split())
        elif kind == "p":
            rec["rows"].append(runs(rest.split()))
        elif kind == "t":
            rec["rows"].append(ESCAPE.sub(lambda m: cell(int(m.group(1), 16)), rest[1:-1]))
        elif kind == "=":
            rec["rows"] += [rec["rows"][-1]] * int(rest)
    return places, records


def png(width: int, height: int, rgb_rows: list[bytes]) -> bytes:
    def chunk(tag: bytes, data: bytes) -> bytes:
        return (
            struct.pack(">I", len(data))
            + tag
            + data
            + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
        )

    raw = b"".join(b"\0" + row for row in rgb_rows)
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(raw))
        + chunk(b"IEND", b"")
    )


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("state", type=Path)
    which = ap.add_mutually_exclusive_group()
    which.add_argument("--page", type=int)
    which.add_argument("--image", action="store_true")
    which.add_argument("--source-image", action="store_true")
    pick = ap.add_mutually_exclusive_group()
    pick.add_argument("--state", type=int, dest="number")
    pick.add_argument("--all", action="store_true")
    ap.add_argument("-o", "--out", type=Path)
    args = ap.parse_args()
    states = split_states(args.state.read_text(encoding="latin-1"))
    print(f"{args.state.name}: {len(states)} state(s)", file=sys.stderr)
    out = args.out or args.state.with_suffix(".png")
    if args.all:
        failed = 0
        for n, text in enumerate(states, 1):
            failed |= show(args, text, out.with_name(f"{out.stem}-{n}{out.suffix}"), f"state {n}")
        return failed
    n = args.number if args.number is not None else len(states)
    if not 1 <= n <= len(states):
        print(f"no state {n} in {args.state.name}", file=sys.stderr)
        return 1
    return show(args, states[n - 1], out, args.state.name)


def split_states(text: str) -> list[str]:
    """The states of a file, each from its line "screen-state 1" to its line "end"."""
    lines = text.replace("\r\n", "\n").split("\n")
    starts = [i for i, line in enumerate(lines) if line.startswith("screen-state ")] or [0]
    return ["\n".join(lines[a:b]) for a, b in zip(starts, starts[1:] + [len(lines)])]


def show(args: argparse.Namespace, text: str, out: Path, what: str) -> int:
    """Prints a text page or writes a graphics page of one state."""
    places, records = read_state(text)
    if args.source_image:
        name = "image read"
    elif args.image:
        name = "image write"
    else:
        name = f"page {args.page if args.page is not None else places['display']}"
    if name not in records:
        print(f"no {name} in {what}; it has: {', '.join(records)}", file=sys.stderr)
        return 1
    rec = records[name]
    print(f"{name}: mode {rec['mode']}, {rec['width']}x{rec['height']}, {rec['bits']} bits")
    if rec["text"]:
        sys.stdout.reconfigure(errors="replace")
        for row in rec["rows"]:
            print(row.rstrip())
        return 0
    palette = rec["palette"]
    rows = [
        b"".join((v if rec["bits"] == 32 else palette[v]).to_bytes(4, "big")[1:] for v in row)
        for row in rec["rows"]
    ]
    out.write_bytes(png(rec["width"], rec["height"], rows))
    print(f"written: {out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
