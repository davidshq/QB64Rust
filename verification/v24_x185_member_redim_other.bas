$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of an unmarked member array with other bounds.
ON ERROR GOTO h
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    tail AS LONG
END TYPE
DIM t AS t1, i AS LONG
t.id = 7: t.tail = 8
FOR i = 0 TO 3: t.v(i) = i + 1: NEXT
PRINT "REDIM t.v(0 TO 9):";
REDIM t.v(0 TO 9)
PRINT LBOUND(t.v); UBOUND(t.v); LEN(t); t.id; t.tail
PRINT "t.v(9) = 5:";
t.v(9) = 5
PRINT t.v(9); t.tail

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
