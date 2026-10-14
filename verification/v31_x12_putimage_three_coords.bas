$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (12_putimage_three_coords)
DIM l AS LONG, img AS LONG
_PUTIMAGE (1, 2, 3), img
_DEST _CONSOLE
PRINT "ok"
SYSTEM
