$CONSOLE
$SCREENHIDE
' Verification (m2-parser-breadth, M5): B and BF as variable names next to the LINE box options.
' Drawn on an image; POINT reads it back; output goes to the console.
img& = _NEWIMAGE(20, 20, 256)
_DEST img&: _SOURCE img&
B = 7: BF = 9
CLS: LINE (0, 0)-(9, 9), 4, B
r1 = POINT(5, 5): r2 = POINT(0, 5)
CLS: LINE (0, 0)-(9, 9), 4, BF
r3 = POINT(5, 5)
CLS: LINE (0, 0)-(9, 9), B
r4 = POINT(5, 5): r5 = POINT(0, 5)
CLS: LINE (0, 0)-(9, 9), B, B
r6 = POINT(5, 5): r7 = POINT(0, 5)
CLS: LINE (0, 0)-(9, 9), BF, BF
r8 = POINT(5, 5)
CLS: LINE (0, 0)-(9, 9), , BF
r9 = POINT(5, 5)
CLS: LINE -STEP(3, 3), 2
r10 = POINT(1, 1)
_DEST _CONSOLE
PRINT "4,B box: inside"; r1; "edge"; r2
PRINT "4,BF: inside"; r3
PRINT "B (var) colour: diagonal"; r4; "edge"; r5
PRINT "B,B: inside"; r6; "edge"; r7
PRINT "BF,BF: inside"; r8
PRINT ",BF: inside"; r9
PRINT "-STEP: point"; r10
SYSTEM
