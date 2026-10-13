$CONSOLE
$SCREENHIDE
' Verification (step 11): a setting inside $IF _CONSOLE_ = 0 below the line of $CONSOLE. In the first round the
' flag is not settled yet; is the OPTION _EXPLICIT met there?
$IF _CONSOLE_ = 0 THEN
    OPTION _EXPLICIT
$END IF
_DEST _CONSOLE
$IF _EXPLICIT_ THEN
    PRINT "explicit 1"
$ELSE
    PRINT "explicit 0"
$END IF
SYSTEM
