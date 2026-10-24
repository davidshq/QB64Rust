$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:OFF in the main file and :ON in an included file (v58_inc\e_on.bi).
$ERRORLOCATION:OFF
PRINT "a"
'$INCLUDE:'v58_inc/e_on.bi'
PRINT "ok"
SYSTEM
