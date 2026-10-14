$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (23_ucharpos_five)
DIM l AS LONG, img AS LONG
DIM p(10) AS LONG: n = _UCHARPOS("abc", p(), 8, 0, 1)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
