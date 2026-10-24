$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x27_scrolllock_two)
' _SCROLLLOCK ON, 1.
k = 0
IF k = 1 THEN
    _SCROLLLOCK ON, 1
END IF
PRINT "ok"
SYSTEM
