$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (02_copyimage_bare)
DIM l AS LONG, img AS LONG
l = _COPYIMAGE
_DEST _CONSOLE
PRINT "ok"
SYSTEM
