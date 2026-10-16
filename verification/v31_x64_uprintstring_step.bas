$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (64_uprintstring_step)
DIM l AS LONG, img AS LONG
_UPRINTSTRING STEP(1, 2), "a"
_DEST _CONSOLE
PRINT "ok"
SYSTEM
