$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x04_bound_const_in_sub)
TYPE t1
    v(top) AS LONG
END TYPE
DIM t AS t1
PRINT UBOUND(t.v)
SYSTEM
SUB s
    CONST top = 3
END SUB
