$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM a() followed by DIM a(3) with constant bounds, and by REDIM
' of it.
ON ERROR GOTO h
DIM a() AS LONG
DIM a(3) AS LONG
a(1) = 5: PRINT "after DIM a(): DIM a(3): a(1), UBOUND"; a(1); UBOUND(a)
PRINT "REDIM a(8):";
REDIM a(8) AS LONG
PRINT UBOUND(a); a(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
