$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (45_mapunicode_comma)
DIM l AS LONG, img AS LONG
_MAPUNICODE 1, 2
_DEST _CONSOLE
PRINT "ok"
SYSTEM
