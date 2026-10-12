$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SWAP of two elements of a member array.
TYPE t1
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1
t.v(0) = 1: t.v(1) = 2
SWAP t.v(0), t.v(1)
PRINT t.v(0); t.v(1)
SYSTEM
