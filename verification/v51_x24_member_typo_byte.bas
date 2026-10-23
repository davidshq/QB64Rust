$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x24_member_typo_byte)
TYPE t1
    x AS _Bytef
END TYPE
PRINT "ok"
SYSTEM
