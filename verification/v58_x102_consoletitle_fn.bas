$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x102_consoletitle_fn)
' _CONSOLETITLE as a function.
k = 0
IF k = 1 THEN
    x = _CONSOLETITLE("a")
END IF
PRINT "ok"
SYSTEM
