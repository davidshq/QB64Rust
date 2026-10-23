$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x26_member_as_list_nosuch)
TYPE t1
    AS nosuch x, y
END TYPE
PRINT "ok"
SYSTEM
