$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): OPTION BASE 1 below the TYPE block and above the DIM of the
' variable.
TYPE t1
    v(3) AS LONG
END TYPE
OPTION BASE 1
DIM t AS t1
PRINT LBOUND(t.v); UBOUND(t.v); LEN(t)
SYSTEM
