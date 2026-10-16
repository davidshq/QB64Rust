$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (20_ucharpos_long_scalar)
DIM l AS LONG, img AS LONG
n = _UCHARPOS("abc", v&)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
