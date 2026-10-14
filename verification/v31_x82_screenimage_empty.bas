$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (82_screenimage_empty)
DIM l AS LONG, img AS LONG
l = _SCREENIMAGE()
_DEST _CONSOLE
PRINT "ok"
SYSTEM
