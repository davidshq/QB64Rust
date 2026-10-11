$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM f(m) with m = -3, a count below zero.
ON ERROR GOTO h
DIM m AS LONG
m = -3
PRINT "DIM f(m):";
DIM f(m) AS LONG
PRINT " then UBOUND(f):";
PRINT UBOUND(f)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
