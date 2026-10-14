$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (07_loadimage_four)
DIM l AS LONG, img AS LONG
l = _LOADIMAGE("a", 32, "", 1)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
