$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x02_asinh_two_args)
' _ASINH with two arguments.
k = 0
IF k = 1 THEN
    x = _ASINH(1, 2)
END IF
PRINT "ok"
SYSTEM
