$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x88_uptime_statement)
' _UPTIME as a statement.
k = 0
IF k = 1 THEN
    _UPTIME
END IF
PRINT "ok"
SYSTEM
