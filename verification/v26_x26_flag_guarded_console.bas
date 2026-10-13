' Verification (step 11): the only $CONSOLE stands in the branch its own flag switches off.
$IF _CONSOLE_ = 0 THEN
    $CONSOLE
    $SCREENHIDE
$END IF
_DEST _CONSOLE
$IF _CONSOLE_ = 1 THEN
    PRINT "flag 1"
$ELSE
    PRINT "flag not 1"
$END IF
PRINT "console is there"
SYSTEM
