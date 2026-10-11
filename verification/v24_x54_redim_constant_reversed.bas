$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM with constant bounds in the wrong order.
ON ERROR GOTO h
REDIM a(5 TO 1) AS LONG
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
