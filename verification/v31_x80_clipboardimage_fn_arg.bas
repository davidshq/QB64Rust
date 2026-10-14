$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (80_clipboardimage_fn_arg)
DIM l AS LONG, img AS LONG
l = _CLIPBOARDIMAGE(1)
_DEST _CONSOLE
PRINT "ok"
SYSTEM
