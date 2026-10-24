$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x54_screenclick_string)
' _SCREENCLICK with a string.
k = 0
IF k = 1 THEN
    _SCREENCLICK "a", 1
END IF
PRINT "ok"
SYSTEM
