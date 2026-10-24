$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x66_environcount_arg)
' _ENVIRONCOUNT with an argument.
k = 0
IF k = 1 THEN
    x = _ENVIRONCOUNT(1)
END IF
PRINT "ok"
SYSTEM
