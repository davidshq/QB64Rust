$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a member array declared below OPTION BASE 1.
OPTION BASE 1
TYPE t1
    v(3) AS LONG
    w(0 TO 2) AS LONG
END TYPE
DIM t AS t1
PRINT LBOUND(t.v); UBOUND(t.v); LBOUND(t.w); LEN(t)
SYSTEM
