$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 2, LINE INPUT at the end of the input (stdin the null device)
' under a handler: what it does, and the value the variable keeps.
ON ERROR GOTO h
DIM s AS STRING
s = "old"
LINE INPUT "line? "; s
PRINT "<"; s; ">"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
