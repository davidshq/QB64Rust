$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (36_rgb_string)
DIM l AS LONG, img AS LONG
PRINT _RGB("a", 2, 3)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
