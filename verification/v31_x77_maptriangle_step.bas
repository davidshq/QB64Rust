$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (77_maptriangle_step)
DIM l AS LONG, img AS LONG
_MAPTRIANGLE STEP(0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
