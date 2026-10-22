$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x13_bound_const_in_sub_above)
s
SUB s
    CONST top = 3
END SUB
TYPE t1
    v(top) AS LONG
END TYPE
PRINT "ok"
SYSTEM
