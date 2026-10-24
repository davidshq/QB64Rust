$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x47_smooth_arg)
' _SMOOTH with an argument.
k = 0
IF k = 1 THEN
    x = _SMOOTH(1)
END IF
PRINT "ok"
SYSTEM
