$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON on line 3, :OFF on line 5 (rejected? at which line?).
$ERRORLOCATION:ON
PRINT "a"
$ERRORLOCATION:OFF
PRINT "ok"
SYSTEM
