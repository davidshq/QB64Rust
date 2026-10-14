$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (97_autodisplay_fn_arg)
DIM l AS LONG, img AS LONG
PRINT _AUTODISPLAY(1)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
