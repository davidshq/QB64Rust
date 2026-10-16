$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (107_mouseinput_statement)
DIM l AS LONG, img AS LONG
_MOUSEINPUT
_DEST _CONSOLE
PRINT "ok"
SYSTEM
