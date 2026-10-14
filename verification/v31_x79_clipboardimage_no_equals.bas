$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): does the old compiler accept this? (79_clipboardimage_no_equals)
DIM l AS LONG, img AS LONG
_CLIPBOARDIMAGE l
_DEST _CONSOLE
PRINT "ok"
SYSTEM
