$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x06_bound_len_literal)
TYPE t1
    v(LEN("abc")) AS LONG
END TYPE
DIM t AS t1
PRINT UBOUND(t.v)
SYSTEM
