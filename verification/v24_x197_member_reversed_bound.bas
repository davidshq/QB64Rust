$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array declared with bounds in the wrong order.
TYPE t1
    v(5 TO 1) AS LONG
END TYPE
DIM t AS t1
PRINT UBOUND(t.v)
SYSTEM
