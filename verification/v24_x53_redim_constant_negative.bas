$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM with a constant bound that gives no element.
ON ERROR GOTO h
REDIM a(-2) AS LONG
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
