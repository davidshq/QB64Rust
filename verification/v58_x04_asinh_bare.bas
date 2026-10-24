$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x04_asinh_bare)
' _ASINH without an argument.
k = 0
IF k = 1 THEN
    x = _ASINH
END IF
PRINT "ok"
SYSTEM
