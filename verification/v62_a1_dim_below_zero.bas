$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 3, DIM of a dynamic array with a count below zero, under a
' handler that prints ERR and resumes next.
ON ERROR GOTO h
n = -3
PRINT "1. DIM d(n), n = -3:";
DIM d(n) AS LONG
PRINT " then UBOUND(d):";
PRINT UBOUND(d)
PRINT "2. end"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
