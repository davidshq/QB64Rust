$CONSOLE:ONLY
' Verification (step 11): _DEST 0 and _DEST _CONSOLE under $CONSOLE:ONLY, and the values of _DEST and _CONSOLE.
PRINT "console 1"; _DEST; _CONSOLE; _SOURCE
_DEST 0
PRINT "window"
_DEST _CONSOLE
PRINT "console 2"
SYSTEM
