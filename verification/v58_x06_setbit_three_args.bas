$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x06_setbit_three_args)
' _SETBIT with three arguments.
k = 0
IF k = 1 THEN
    x = _SETBIT(1, 2, 3)
END IF
PRINT "ok"
SYSTEM
