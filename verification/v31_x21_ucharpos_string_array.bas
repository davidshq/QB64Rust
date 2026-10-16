$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (21_ucharpos_string_array)
DIM l AS LONG, img AS LONG
DIM t$(10): n = _UCHARPOS("abc", t$())
_DEST _CONSOLE
PRINT "ok"
SYSTEM
