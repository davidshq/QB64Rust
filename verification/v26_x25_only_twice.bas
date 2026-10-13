$CONSOLE:ONLY
' Verification (step 11): $CONSOLE:ONLY a second time after _DEST 0.
PRINT "console 1"
_DEST 0
PRINT "window"
$CONSOLE:ONLY
PRINT "console 2"
SYSTEM
