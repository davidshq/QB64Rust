$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x53_screenclick_one_arg)
' _SCREENCLICK with one argument.
k = 0
IF k = 1 THEN
    _SCREENCLICK 10
END IF
PRINT "ok"
SYSTEM
