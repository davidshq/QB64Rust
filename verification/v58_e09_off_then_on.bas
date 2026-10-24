$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:OFF on line 3, :ON on line 5 (rejected? at which line?).
$ERRORLOCATION:OFF
PRINT "a"
$ERRORLOCATION:ON
PRINT "ok"
SYSTEM
