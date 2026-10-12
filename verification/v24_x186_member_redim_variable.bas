$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of an unmarked member array with a variable bound.
ON ERROR GOTO h
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    tail AS LONG
END TYPE
DIM t AS t1, n AS LONG
t.tail = 8: t.v(1) = 2
n = 6
PRINT "REDIM t.v(n):";
REDIM t.v(n)
PRINT LBOUND(t.v); UBOUND(t.v); LEN(t); t.v(1); t.tail

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
