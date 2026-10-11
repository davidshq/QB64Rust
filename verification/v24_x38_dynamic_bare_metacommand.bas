$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): $DYNAMIC as a statement, not in a comment.
ON ERROR GOTO h
$DYNAMIC
DIM a(5) AS LONG
REDIM a(8) AS LONG
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
