$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of a member array with AS and its own type, upper bound
' alone.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v(3) = 4
REDIM t.v(3) AS LONG
PRINT t.v(3); UBOUND(t.v)
SYSTEM
