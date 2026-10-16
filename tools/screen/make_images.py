"""Writes the image files the screen programs of m3-images-fonts-mouse load (design D5): small patterns, simple
enough to see in a state's picture, made by this project so that no image from elsewhere goes into the repository.

    python tools/screen/make_images.py            # into tests/corpus/screen/i02_load_images.files/
    python tools/screen/make_images.py <folder>   # into another folder

Standard library only. The files are committed; run the script again only when it changes.

- `pattern8.bmp`: 16 x 12 pixels, 8 bits a pixel with a palette of 16 colours; the pixel (x, y) has the index
  (x + 2 * y) mod 16, the colour of index i is red 16 * i, green 255 - 16 * i, blue 128.
- `pattern24.bmp`: 16 x 12 pixels, 24 bits; red 16 * x, green 20 * y, blue 255 - 16 * x.
- `alpha.png`: 16 x 12 pixels, RGBA; four bands of four columns with alpha 0, 85, 170 and 255; red 255 in the top
  half, blue 255 in the bottom half, green 15 * x.
"""

import os
import struct
import sys
import zlib

W, H = 16, 12


def bmp(bits, rows, palette=None):
    """A bottom-up BMP with a BITMAPINFOHEADER. `rows` are lists of pixel bytes, top row first."""
    stride = (len(rows[0]) + 3) // 4 * 4
    data = b"".join(bytes(r) + b"\0" * (stride - len(r)) for r in reversed(rows))
    colours = b"".join(bytes((b, g, r, 0)) for r, g, b in (palette or []))
    offset = 14 + 40 + len(colours)
    info = struct.pack(
        "<IiiHHIIiiII", 40, W, H, 1, bits, 0, len(data), 2835, 2835, len(palette or []), 0
    )
    head = struct.pack("<2sIHHI", b"BM", offset + len(data), 0, 0, offset)
    return head + info + colours + data


def png(rows):
    """An RGBA PNG; `rows` are lists of (r, g, b, a), top row first."""

    def chunk(kind, body):
        return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body))

    raw = b"".join(b"\0" + b"".join(bytes(p) for p in r) for r in rows)
    ihdr = struct.pack(">IIBBBBB", W, H, 8, 6, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr) + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b"")


def files():
    palette = [(16 * i, 255 - 16 * i, 128) for i in range(16)]
    p8 = bmp(8, [[(x + 2 * y) % 16 for x in range(W)] for y in range(H)], palette)
    p24 = bmp(24, [[c for x in range(W) for c in (255 - 16 * x, 20 * y, 16 * x)] for y in range(H)])
    alpha = png(
        [
            [(255 if y < H // 2 else 0, 15 * x, 0 if y < H // 2 else 255, (0, 85, 170, 255)[x // 4]) for x in range(W)]
            for y in range(H)
        ]
    )
    return {"pattern8.bmp": p8, "pattern24.bmp": p24, "alpha.png": alpha}


def main():
    root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(root, "tests", "corpus", "screen", "i02_load_images.files")
    os.makedirs(out, exist_ok=True)
    for name, data in files().items():
        with open(os.path.join(out, name), "wb") as f:
            f.write(data)
        print(name, len(data), "bytes")


if __name__ == "__main__":
    main()
