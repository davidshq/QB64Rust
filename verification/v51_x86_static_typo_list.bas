$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x86_static_typo_list)
s
PRINT "ok"
SYSTEM
SUB s
    STATIC AS nosuch a, b
END SUB
