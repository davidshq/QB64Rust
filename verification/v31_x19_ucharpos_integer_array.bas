$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (19_ucharpos_integer_array)
DIM l AS LONG, img AS LONG
DIM q%(10): n = _UCHARPOS("abc", q%())
_DEST _CONSOLE
PRINT "ok"
SYSTEM
