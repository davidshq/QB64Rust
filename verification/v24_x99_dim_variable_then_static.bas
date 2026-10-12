$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM a(n) with a variable bound, then DIM a(5) with constant
' bounds, then REDIM.
ON ERROR GOTO h
n = 3
DIM a(n) AS LONG
PRINT "second DIM:";
DIM a(5) AS LONG
PRINT UBOUND(a)
PRINT "REDIM a(7):";
REDIM a(7) AS LONG
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
