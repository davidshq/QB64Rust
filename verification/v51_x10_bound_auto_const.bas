$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x10_bound_auto_const)
TYPE t1
    v(_PI) AS LONG
END TYPE
DIM t AS t1
PRINT UBOUND(t.v)
SYSTEM
