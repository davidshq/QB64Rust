$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (06_loadimage_string_mode)
DIM l AS LONG, img AS LONG
l = _LOADIMAGE("a", "x")
_DEST _CONSOLE
PRINT "ok"
SYSTEM
