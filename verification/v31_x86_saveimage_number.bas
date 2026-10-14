$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (86_saveimage_number)
DIM l AS LONG, img AS LONG
_SAVEIMAGE 5
_DEST _CONSOLE
PRINT "ok"
SYSTEM
