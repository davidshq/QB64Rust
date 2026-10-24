$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x55_screenclick_four_args)
' _SCREENCLICK with four arguments.
k = 0
IF k = 1 THEN
    _SCREENCLICK 1, 2, 3, 4
END IF
PRINT "ok"
SYSTEM
