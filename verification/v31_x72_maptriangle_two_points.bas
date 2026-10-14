$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (72_maptriangle_two_points)
DIM l AS LONG, img AS LONG
_MAPTRIANGLE (0, 0)-(9, 0), img TO (10, 10)-(50, 10)-(10, 50)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
