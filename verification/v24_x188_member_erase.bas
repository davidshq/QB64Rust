$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): ERASE of an unmarked member array.
ON ERROR GOTO h
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    tail AS LONG
END TYPE
DIM t AS t1, i AS LONG
t.id = 7: t.tail = 8
FOR i = 0 TO 3: t.v(i) = i + 1: NEXT
PRINT "ERASE t.v:";
ERASE t.v
PRINT t.v(0); t.v(3); LBOUND(t.v); UBOUND(t.v); t.id; t.tail; LEN(t)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
