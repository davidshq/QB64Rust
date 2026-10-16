$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (44_depthbuffer_number)
DIM l AS LONG, img AS LONG
_DEPTHBUFFER 1
_DEST _CONSOLE
PRINT "ok"
SYSTEM
