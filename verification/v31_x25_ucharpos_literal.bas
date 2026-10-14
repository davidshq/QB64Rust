$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (25_ucharpos_literal)
DIM l AS LONG, img AS LONG
n = _UCHARPOS("abc", 5)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
