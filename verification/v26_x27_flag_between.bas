$CONSOLE:ONLY
' Verification (step 11): _CONSOLE_ between $CONSOLE:ONLY and $CONSOLE, in a PRINT.
$IF _CONSOLE_ = 2 THEN
    PRINT "between: 2"
$ELSEIF _CONSOLE_ = 1 THEN
    PRINT "between: 1"
$ELSE
    PRINT "between: 0"
$END IF
$CONSOLE
SYSTEM
