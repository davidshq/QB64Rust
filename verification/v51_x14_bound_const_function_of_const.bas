$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x14_bound_const_function_of_const)
CONST top = 3
TYPE t1
    v(ABS(top)) AS LONG
END TYPE
DIM t AS t1
PRINT UBOUND(t.v)
SYSTEM
