$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM _PRESERVE r(n) with n = -2 of an allocated array.
ON ERROR GOTO h
DIM n AS LONG
REDIM r(3) AS LONG
r(1) = 5
n = -2
PRINT "REDIM _PRESERVE r(n):";
REDIM _PRESERVE r(n) AS LONG
PRINT " then UBOUND(r), r(1):";
PRINT UBOUND(r); r(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
