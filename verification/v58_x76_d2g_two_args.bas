$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x76_d2g_two_args)
' _D2G with two arguments.
k = 0
IF k = 1 THEN
    x = _D2G(1, 2)
END IF
PRINT "ok"
SYSTEM
