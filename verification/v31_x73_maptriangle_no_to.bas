$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (73_maptriangle_no_to)
DIM l AS LONG, img AS LONG
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img
_DEST _CONSOLE
PRINT "ok"
SYSTEM
