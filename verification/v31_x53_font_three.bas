$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (53_font_three)
DIM l AS LONG, img AS LONG
_FONT 1, 2, 3
_DEST _CONSOLE
PRINT "ok"
SYSTEM
