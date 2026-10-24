$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x01_asinh_statement)
' _ASINH as a statement.
k = 0
IF k = 1 THEN
    _ASINH 1
END IF
PRINT "ok"
SYSTEM
