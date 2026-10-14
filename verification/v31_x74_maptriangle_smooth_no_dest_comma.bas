$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (74_maptriangle_smooth_no_dest_comma)
DIM l AS LONG, img AS LONG
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50) _SMOOTH
_DEST _CONSOLE
PRINT "ok"
SYSTEM
