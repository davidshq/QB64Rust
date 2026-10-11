$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of a member array with lower TO a variable that equals the
' declared bound.
ON ERROR GOTO h
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1, n AS LONG
t.v(3) = 4
n = 3
PRINT "REDIM t.v(0 TO n):";
REDIM t.v(0 TO n)
PRINT t.v(3); UBOUND(t.v)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
