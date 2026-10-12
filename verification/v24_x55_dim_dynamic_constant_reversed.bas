$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM below $DYNAMIC with constant bounds in the wrong order.
ON ERROR GOTO h
'$DYNAMIC
DIM a(5 TO 1) AS LONG
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
