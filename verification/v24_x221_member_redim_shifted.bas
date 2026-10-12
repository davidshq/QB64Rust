$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of a member array 0 TO 3 as 1 TO 4.
ON ERROR GOTO h
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v(3) = 4
PRINT "REDIM t.v(1 TO 4):";
REDIM t.v(1 TO 4)
PRINT LBOUND(t.v); UBOUND(t.v); t.v(3)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
