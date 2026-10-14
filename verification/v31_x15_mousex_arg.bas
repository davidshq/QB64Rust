$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (15_mousex_arg)
DIM l AS LONG, img AS LONG
PRINT _MOUSEX(1)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
