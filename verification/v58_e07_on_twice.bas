$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON twice (accepted?).
$ERRORLOCATION:ON
PRINT "a"
$ERRORLOCATION:ON
PRINT "ok"
SYSTEM
