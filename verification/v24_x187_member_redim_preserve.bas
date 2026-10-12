$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM _PRESERVE of an unmarked member array with its declared
' bounds and with others.
ON ERROR GOTO h
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    tail AS LONG
END TYPE
DIM t AS t1, i AS LONG
t.tail = 8
FOR i = 0 TO 3: t.v(i) = i + 1: NEXT
PRINT "REDIM _PRESERVE t.v(0 TO 3):";
REDIM _PRESERVE t.v(0 TO 3)
PRINT t.v(0); t.v(3); UBOUND(t.v)
PRINT "REDIM _PRESERVE t.v(0 TO 5):";
REDIM _PRESERVE t.v(0 TO 5)
PRINT t.v(0); t.v(3); UBOUND(t.v); LEN(t); t.tail

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
