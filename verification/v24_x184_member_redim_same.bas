$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of an unmarked member array with its declared bounds.
ON ERROR GOTO h
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1, i AS LONG
t.id = 7
FOR i = 0 TO 3: t.v(i) = i + 1: NEXT
PRINT "REDIM t.v(0 TO 3):";
REDIM t.v(0 TO 3)
PRINT t.v(0); t.v(1); t.v(2); t.v(3); t.id; LBOUND(t.v); UBOUND(t.v); LEN(t)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
