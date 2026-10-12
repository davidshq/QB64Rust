$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM of 2^61 - 1 elements of 8 bytes.
ON ERROR GOTO h
DIM big AS _INTEGER64
big = 2305843009213693951
PRINT "DIM a(big):";
DIM a(big) AS _INTEGER64
PRINT " then UBOUND(a):";
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
