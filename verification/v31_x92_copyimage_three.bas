$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (92_copyimage_three)
DIM l AS LONG, img AS LONG
l = _COPYIMAGE(1, 2, 3)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
