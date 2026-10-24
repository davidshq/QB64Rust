$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON and the critical error in an included file's main-module code
' (v58_inc\e_div.bi): the line in that file and the file's name as written.
$ERRORLOCATION:ON
PRINT "start"
'$INCLUDE:'v58_inc/e_div.bi'
PRINT "not reached"
SYSTEM
