$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (106_setalpha_string)
DIM l AS LONG, img AS LONG
_SETALPHA "a"
_DEST _CONSOLE
PRINT "ok"
SYSTEM
