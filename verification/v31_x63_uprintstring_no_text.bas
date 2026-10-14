$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (63_uprintstring_no_text)
DIM l AS LONG, img AS LONG
_UPRINTSTRING (1, 2)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
