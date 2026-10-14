$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (76_maptriangle_3d_source)
DIM l AS LONG, img AS LONG
_MAPTRIANGLE (0, 0, 1)-(9, 0, 1)-(0, 9, 1), img TO (10, 10)-(50, 10)-(10, 50)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
