$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x08_bound_var_suffix)
n% = 3
TYPE t1
    v(n%) AS LONG
END TYPE
PRINT "ok"
SYSTEM
