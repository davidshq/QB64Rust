$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (104_newimage_one)
DIM l AS LONG, img AS LONG
l = _NEWIMAGE(1)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
