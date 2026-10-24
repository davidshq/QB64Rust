$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x37_scaledwidth_arg)
' _SCALEDWIDTH with an argument.
k = 0
IF k = 1 THEN
    x = _SCALEDWIDTH(1)
END IF
PRINT "ok"
SYSTEM
