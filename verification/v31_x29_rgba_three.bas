$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (29_rgba_three)
DIM l AS LONG, img AS LONG
PRINT _RGBA(1, 2, 3)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
