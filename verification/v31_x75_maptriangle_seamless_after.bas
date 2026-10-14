$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (75_maptriangle_seamless_after)
DIM l AS LONG, img AS LONG
_MAPTRIANGLE _SEAMLESS _CLOCKWISE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
