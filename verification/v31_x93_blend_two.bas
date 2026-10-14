$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (93_blend_two)
DIM l AS LONG, img AS LONG
_BLEND 1, 2
_DEST _CONSOLE
PRINT "ok"
SYSTEM
