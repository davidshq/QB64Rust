$CONSOLE
$SCREENHIDE
' Verification (m2-parser-breadth, M5): template words in lower case (bf, step), and STEP in the first point.
img& = _NEWIMAGE(20, 20, 256)
_DEST img&: _SOURCE img&
line (0, 0)-step(9, 9), 4, bf
r = POINT(5, 5)
PSET STEP(1, 1), 3
_DEST _CONSOLE
PRINT r
SYSTEM
