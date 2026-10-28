$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 3, REDIM with a count below zero (one dimension of two, and
' lower TO upper), under a handler that prints ERR and resumes next.
ON ERROR GOTO h
REDIM r(4, 3) AS LONG
r(1, 1) = 7
m = 0
PRINT "1. REDIM r(2 TO m, 3), m = 0:";
REDIM r(2 TO m, 3) AS LONG
PRINT " then UBOUND(r):";
PRINT UBOUND(r)
PRINT "2. end"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
