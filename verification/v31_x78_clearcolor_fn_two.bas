$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (78_clearcolor_fn_two)
DIM l AS LONG, img AS LONG
PRINT _CLEARCOLOR(1, 2)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
