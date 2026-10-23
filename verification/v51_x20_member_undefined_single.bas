$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x20_member_undefined_single)
TYPE t1
    x AS nosuch
END TYPE
PRINT "ok"
SYSTEM
