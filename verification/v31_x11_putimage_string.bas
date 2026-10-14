$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (11_putimage_string)
DIM l AS LONG, img AS LONG
_PUTIMAGE ("a", 2), img
_DEST _CONSOLE
PRINT "ok"
SYSTEM
