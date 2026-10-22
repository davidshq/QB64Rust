$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x22_member_unsigned_nosuch)
TYPE t1
    x AS _UNSIGNED nosuch
END TYPE
PRINT "ok"
SYSTEM
