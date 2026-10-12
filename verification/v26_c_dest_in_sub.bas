$CONSOLE
$SCREENHIDE
' Verification (step 11): _DEST _CONSOLE in a procedure stays in force after it returns, also in a FUNCTION.
PRINT "window"
toConsole
PRINT "console after the sub"
_DEST 0
PRINT "window again"
x = f
PRINT "console after the function"; x
SYSTEM

SUB toConsole
    _DEST _CONSOLE
    PRINT "console in the sub"
END SUB

FUNCTION f
    _DEST _CONSOLE
    f = 7
END FUNCTION
