$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (69_loadfont_number_name)
DIM l AS LONG, img AS LONG
l = _LOADFONT(5, 16)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
