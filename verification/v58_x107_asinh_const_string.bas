$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x107_asinh_const_string)
' _ASINH of a string in a CONST.
k = 0
IF k = 1 THEN
    CONST c = _ASINH("a")
END IF
PRINT "ok"
SYSTEM
