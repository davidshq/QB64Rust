$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 2, INPUT of two variables at the end of the input (stdin the null
' device) under a handler: the values they keep (INPUT clears them before it reads).
ON ERROR GOTO h
DIM a AS LONG, b AS STRING
a = 7: b = "old"
INPUT a, b
PRINT "<"; a; b; ">"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
