$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x11_bound_const_function)
TYPE t1
    v(ABS(-3)) AS LONG
END TYPE
DIM t AS t1
PRINT UBOUND(t.v)
SYSTEM
