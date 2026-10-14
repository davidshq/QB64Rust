$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (18_mousemove_string)
DIM l AS LONG, img AS LONG
_MOUSEMOVE "a", 1
_DEST _CONSOLE
PRINT "ok"
SYSTEM
