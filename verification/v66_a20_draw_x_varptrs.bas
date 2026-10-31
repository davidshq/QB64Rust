$CONSOLE
$SCREENHIDE
' Verification (SOMEDAY.md, the m3-sound row on VARPTR$ in PLAY and DRAW): DRAW's X command with VARPTR$ of a string variable.
SCREEN 13
d$ = "R10"
PSET (5, 5), 15
DRAW "X" + VARPTR$(d$)
p1 = POINT(10, 5): p2 = POINT(15, 5): p3 = POINT(16, 5)
_DEST _CONSOLE
PRINT "after X:"; p1; p2; p3
SYSTEM
