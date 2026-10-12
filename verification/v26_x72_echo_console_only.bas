$CONSOLE:ONLY
' Verification (step 11, group 3): _ECHO in a program without a window, and with the window's handle as
' destination.
_ECHO "echo 1"
_DEST 0
_ECHO "echo 2"
PRINT "window"
_DEST _CONSOLE
PRINT "console"; _DEST
SYSTEM
