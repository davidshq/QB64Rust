$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x05_setbit_one_arg)
' _SETBIT with one argument.
k = 0
IF k = 1 THEN
    x = _SETBIT(1)
END IF
PRINT "ok"
SYSTEM
