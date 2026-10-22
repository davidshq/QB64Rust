$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x01_bound_var_below)
TYPE t1
    v(n) AS LONG
END TYPE
DIM n AS LONG
n = 3
PRINT "ok"
SYSTEM
