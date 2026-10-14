$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (60_printstring_dash)
DIM l AS LONG, img AS LONG
_PRINTSTRING (1, 2)-(3, 4), "a"
_DEST _CONSOLE
PRINT "ok"
SYSTEM
