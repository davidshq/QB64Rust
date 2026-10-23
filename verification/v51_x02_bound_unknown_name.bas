$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x02_bound_unknown_name)
TYPE t1
    v(zz) AS LONG
END TYPE
PRINT "ok"
SYSTEM
