' Verification (step 11): $CONSOLE and $SCREENHIDE inside a SUB.
_DEST _CONSOLE
PRINT "ok"
s
SYSTEM

SUB s
    $CONSOLE
    $SCREENHIDE
    PRINT "in the sub"
END SUB
