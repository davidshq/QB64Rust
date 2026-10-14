$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (51_setalpha_to_without_first)
DIM l AS LONG, img AS LONG
_SETALPHA 128 TO 9
_DEST _CONSOLE
PRINT "ok"
SYSTEM
