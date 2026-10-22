$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x27_member_integer_star)
TYPE t1
    x AS INTEGER * 2
END TYPE
PRINT "ok"
SYSTEM
