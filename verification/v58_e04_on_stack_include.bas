$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON and the recursion in a SUB of an included file (v58_inc\e_sub.bi):
' the message names the line in that file and the file.
$ERRORLOCATION:ON
PRINT "start"
deepinc 1
PRINT "not reached"
SYSTEM

'$INCLUDE:'v58_inc/e_sub.bi'
