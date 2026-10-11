$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array of _BIT.
TYPE t1
    b(7) AS _BIT
END TYPE
DIM t AS t1
t.b(1) = 1
PRINT t.b(1)
SYSTEM
