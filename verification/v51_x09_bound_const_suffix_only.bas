$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x09_bound_const_suffix_only)
CONST top = 3
TYPE t1
    v(top%) AS LONG
END TYPE
PRINT "ok"
SYSTEM
