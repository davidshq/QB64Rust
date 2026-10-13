$CONSOLE
$SCREENHIDE
' Verification (step 11): _ECHO writes to the console whatever the destination is, and leaves the destination.
_ECHO "echo 1"
PRINT "window"
PRINT "dest:"; _DEST
_ECHO "echo 2: " + STR$(_DEST)
_DEST _CONSOLE
_ECHO "echo 3"
PRINT "console"
SYSTEM
