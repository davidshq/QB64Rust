$CONSOLE
$SCREENHIDE
' Verification (step 11): which PRINT reaches standard output, and the values of _DEST, _SOURCE and _CONSOLE.
PRINT "window 1"
d& = _DEST
s& = _SOURCE
_DEST _CONSOLE
PRINT "console 1"
PRINT "dest before:"; d&; "source before:"; s&
PRINT "dest now:"; _DEST; "source now:"; _SOURCE; "_CONSOLE:"; _CONSOLE
_DEST d&
PRINT "window 2"
_DEST _CONSOLE
PRINT "console 2"
_DEST 0
PRINT "window 3"
_DEST _CONSOLE
PRINT "console 3"
_SOURCE _CONSOLE
PRINT "source after _SOURCE _CONSOLE:"; _SOURCE
SYSTEM
