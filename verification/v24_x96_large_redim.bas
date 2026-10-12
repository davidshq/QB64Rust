$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of an allocated array to 2^40 elements of 8 bytes.
ON ERROR GOTO h
DIM big AS _INTEGER64
REDIM r(3) AS _INTEGER64
r(1) = 5
big = 1099511627776
PRINT "REDIM r(big):";
REDIM r(big) AS _INTEGER64
PRINT " then UBOUND(r), r(1):";
PRINT UBOUND(r); r(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
