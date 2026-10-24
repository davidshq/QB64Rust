$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x03_asinh_string)
' _ASINH of a string.
k = 0
IF k = 1 THEN
    x = _ASINH("a")
END IF
PRINT "ok"
SYSTEM
