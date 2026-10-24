$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON and a critical error in the main module (integer division by
' zero, which libqb reports without the event path): the message names the line of the division.
$ERRORLOCATION:ON
PRINT "start"
z% = 0
q% = 7 \ z%
PRINT "not reached"
SYSTEM
