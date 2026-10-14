$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (09_putimage_too_many)
DIM l AS LONG, img AS LONG
_PUTIMAGE , , , , , 1
_DEST _CONSOLE
PRINT "ok"
SYSTEM
