$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a dynamic array made by REDIM, erased, then DIM with constant
' bounds in another statement.
ON ERROR GOTO h
REDIM d(5) AS LONG
d(1) = 4
ERASE d
PRINT "DIM d(3) after ERASE:";
DIM d(3) AS LONG
PRINT UBOUND(d); d(1)
PRINT "REDIM d(7):";
REDIM d(7) AS LONG
PRINT UBOUND(d)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
