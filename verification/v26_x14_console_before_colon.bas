$CONSOLE: x = 1
$SCREENHIDE: y = 2
' Verification (step 11): a colon and a statement after $CONSOLE and after $SCREENHIDE.
_DEST _CONSOLE
PRINT "ok"; x; y
SYSTEM
