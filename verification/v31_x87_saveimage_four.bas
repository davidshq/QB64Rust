$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (87_saveimage_four)
DIM l AS LONG, img AS LONG
_SAVEIMAGE "a", 1, "b", 2
_DEST _CONSOLE
PRINT "ok"
SYSTEM
