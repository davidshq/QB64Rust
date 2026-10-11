$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM a(5), then DIM a(n) with a variable bound.
ON ERROR GOTO h
n = 3
DIM a(5) AS LONG
PRINT "second DIM:";
DIM a(n) AS LONG
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
