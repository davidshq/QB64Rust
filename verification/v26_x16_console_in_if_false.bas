$IF 0 THEN
    $CONSOLE
$END IF
$SCREENHIDE
' Verification (step 11): $CONSOLE only in a $IF branch that is not taken; _DEST _CONSOLE all the same.
_DEST _CONSOLE
PRINT "printed"
SYSTEM
