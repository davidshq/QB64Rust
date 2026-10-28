$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 12, ERROR 11 under a handler.
ON ERROR GOTO h
PRINT "before"
ERROR 11
PRINT "after"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
