$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (46_mapunicode_one)
DIM l AS LONG, img AS LONG
_MAPUNICODE 1
_DEST _CONSOLE
PRINT "ok"
SYSTEM
