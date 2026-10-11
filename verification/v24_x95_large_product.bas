$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM of two dimensions of 2^32 + 1 each, 1 byte each.
ON ERROR GOTO h
DIM big AS _INTEGER64
big = 4294967296
PRINT "DIM a(big, big):";
DIM a(big, big) AS _BYTE
PRINT " then UBOUND(a):";
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
