$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON a() never given bounds, then a(1) = 1.
ON ERROR GOTO h
COMMON a() AS LONG
PRINT "a(1) = 1:";
a(1) = 1
PRINT " UBOUND(a):";
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
