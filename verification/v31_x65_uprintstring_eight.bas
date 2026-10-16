$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (65_uprintstring_eight)
DIM l AS LONG, img AS LONG
_UPRINTSTRING (1, 2), "a", 1, 2, 3, 4, 5
_DEST _CONSOLE
PRINT "ok"
SYSTEM
