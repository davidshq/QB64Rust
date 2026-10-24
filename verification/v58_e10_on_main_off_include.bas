$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON in the main file and :OFF in an included file (v58_inc\e_off.bi).
$ERRORLOCATION:ON
PRINT "a"
'$INCLUDE:'v58_inc/e_off.bi'
PRINT "ok"
SYSTEM
