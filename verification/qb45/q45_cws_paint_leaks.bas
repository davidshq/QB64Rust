' QB 4.5 and PDS 7.1: two shapes of Civil War Strategy (W.R. Hutsell, PDS 7.1) that the game fills
' with PAINT. In the QB64pe and QB64Rust builds both fills leak over the whole map, so the game's
' port changes them (the game's docs\devnotes.md, branch qb64rust).
'   1. The railroad's small train: DRAW at scale 6, then PAINT inside its outline.
'   2. The cannon's wheel: two ellipses and an arc drawn with CIRCLE, then PAINT between them.
' For each: where the graphics cursor ends, whether the paint reached three far points of the
' screen, and the shape's pixels, one character per pixel (the hex digit of its color, "." for 0).
OPEN "OUT.TXT" FOR OUTPUT AS #1
SCREEN 12

PRINT #1, "train"
PSET (550, 20), 0
DRAW "C15S6R9D4R6U3R3D3R7U5H3U2R9D3G2D6F1D3F5L10D1G1L4H2L7G2L3H2L3U8L2U5R1BF4"
x = POINT(0): y = POINT(1)
PRINT #1, "cursor"; x; y
PAINT (x, y), 9, 15
PRINT #1, "far points"; POINT(10, 400); POINT(300, 300); POINT(630, 470)
FOR r = 0 TO 50
a$ = ""
FOR c = 540 TO 600
p = POINT(c, r)
IF p = 0 THEN a$ = a$ + "." ELSE a$ = a$ + HEX$(p)
NEXT c
PRINT #1, a$
NEXT r

CLS
PRINT #1, "cannon"
PSET (580, 380), 0
DRAW "S4BR15"
x = POINT(0): y = POINT(1): CIRCLE (x, y), 25, 1, , , 1.4
CIRCLE (x + 5, y), 29, 1, 4, 1.5, 1.4
CIRCLE (x, y), 29, 1, , , 1.4
DRAW "BR20"
x = POINT(0): y = POINT(1): PAINT (x, y), 3, 1
PRINT #1, "first paint at"; x; y
DRAW "BR3BU7"
x = POINT(0): y = POINT(1): PAINT (x, y), 5, 1
PRINT #1, "second paint at"; x; y
PRINT #1, "far points"; POINT(10, 400); POINT(300, 300); POINT(630, 470)
FOR r = 345 TO 415
a$ = ""
FOR c = 570 TO 625
p = POINT(c, r)
IF p = 0 THEN a$ = a$ + "." ELSE a$ = a$ + HEX$(p)
NEXT c
PRINT #1, a$
NEXT r
CLOSE #1
SYSTEM
