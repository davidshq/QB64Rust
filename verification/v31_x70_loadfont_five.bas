$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (70_loadfont_five)
DIM l AS LONG, img AS LONG
l = _LOADFONT("cour.ttf", 16, "", 1, 2)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
