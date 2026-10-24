$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON and the critical error on the second physical line of a
' joined line (the old compiler names the line where the statement starts).
$ERRORLOCATION:ON
PRINT "start"
z% = 0
PRINT "a"; _
    7 \ z%
PRINT "not reached"
SYSTEM
