$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x103_screenprint_fn)
' _SCREENPRINT as a function.
k = 0
IF k = 1 THEN
    x = _SCREENPRINT("a")
END IF
PRINT "ok"
SYSTEM
