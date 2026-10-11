$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM _PRESERVE of an unmarked member array with its declared
' bounds.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v(0) = 1: t.v(3) = 4
REDIM _PRESERVE t.v(0 TO 3)
PRINT t.v(0); t.v(3); UBOUND(t.v)
SYSTEM
