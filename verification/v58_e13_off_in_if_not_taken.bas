$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON, then :OFF in a $IF branch that is not taken.
$ERRORLOCATION:ON
PRINT "a"
$IF 0 THEN
    $ERRORLOCATION:OFF
$END IF
PRINT "ok"
SYSTEM
