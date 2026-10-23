$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x07_bound_var_expr)
n = 3
TYPE t1
    v(1 TO n + 1) AS LONG
END TYPE
PRINT "ok"
SYSTEM
