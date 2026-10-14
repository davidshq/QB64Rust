$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (68_loadfont_one)
DIM l AS LONG, img AS LONG
l = _LOADFONT("cour.ttf")
_DEST _CONSOLE
PRINT "ok"
SYSTEM
