$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array marked _STATIC without $UNSTABLE:TYPEFIELDS.
TYPE t1
    v(0 TO 3) _STATIC AS LONG
END TYPE
DIM t AS t1
t.v(1) = 5: PRINT t.v(1)
SYSTEM
