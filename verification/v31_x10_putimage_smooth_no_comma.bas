$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (10_putimage_smooth_no_comma)
DIM l AS LONG, img AS LONG
_PUTIMAGE (0, 0)-(9, 9), img _SMOOTH
_DEST _CONSOLE
PRINT "ok"
SYSTEM
