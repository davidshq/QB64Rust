$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM a() with nothing between the parentheses: an element before
' any bounds are given, then REDIM.
ON ERROR GOTO h
DIM a() AS LONG
PRINT "a(1) = 5 before any bounds:";
a(1) = 5
PRINT
PRINT "UBOUND(a):";
PRINT UBOUND(a)
REDIM a(3) AS LONG
a(1) = 5: PRINT "after REDIM a(3): a(1), UBOUND"; a(1); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
