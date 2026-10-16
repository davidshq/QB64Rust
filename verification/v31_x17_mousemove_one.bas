$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (17_mousemove_one)
DIM l AS LONG, img AS LONG
_MOUSEMOVE 10
_DEST _CONSOLE
PRINT "ok"
SYSTEM
