$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (14_mousebutton_two)
DIM l AS LONG, img AS LONG
PRINT _MOUSEBUTTON(1, 2)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
