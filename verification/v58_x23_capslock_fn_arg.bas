$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x23_capslock_fn_arg)
' _CAPSLOCK as a function with an argument.
k = 0
IF k = 1 THEN
    x = _CAPSLOCK(1)
END IF
PRINT "ok"
SYSTEM
