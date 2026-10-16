$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (22_ucharpos_array_name_bare)
DIM l AS LONG, img AS LONG
DIM p(10) AS LONG: n = _UCHARPOS("abc", p)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
