$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x03_bound_const_below)
TYPE t1
    v(top) AS LONG
END TYPE
CONST top = 3
DIM t AS t1
PRINT UBOUND(t.v)
SYSTEM
