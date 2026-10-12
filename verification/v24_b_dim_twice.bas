$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): two DIM statements of one dynamic array.
ON ERROR GOTO h
DIM n AS LONG
n = 4
DIM d(n) AS LONG
d(1) = 5
PRINT "second DIM d(n + 1):";
DIM d(n + 1) AS LONG
PRINT UBOUND(d); d(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
