$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON SHARED a() AS LONG, DIM a(3) AS LONG, then REDIM a(9):
' the array is dynamic.
ON ERROR GOTO h
COMMON SHARED a() AS LONG
DIM a(3) AS LONG
a(1) = 5
PRINT "REDIM a(9):";
REDIM a(9) AS LONG
PRINT UBOUND(a); a(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
