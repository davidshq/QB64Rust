$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (26_ucharpos_long64_array)
DIM l AS LONG, img AS LONG
DIM y(10) AS _INTEGER64: n = _UCHARPOS("abc", y())
_DEST _CONSOLE
PRINT "ok"
SYSTEM
