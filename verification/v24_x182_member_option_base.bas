$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): member arrays declared below OPTION BASE 1, and the literal 0.
OPTION BASE 1
TYPE t1
    v(3) AS LONG
    z(0) AS LONG
    w(0 TO 2) AS LONG
END TYPE
DIM t AS t1
PRINT LBOUND(t.v); UBOUND(t.v); LBOUND(t.z); UBOUND(t.z); LBOUND(t.w); LEN(t)
SYSTEM
