$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x44_const_undefined_in_sub)
s
PRINT "ok"
SYSTEM
SUB s
    CONST c = 20 OR UNDEFINED
END SUB
