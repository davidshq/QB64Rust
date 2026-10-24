$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x67_uptime_arg)
' _UPTIME with an argument.
k = 0
IF k = 1 THEN
    x = _UPTIME(1)
END IF
PRINT "ok"
SYSTEM
