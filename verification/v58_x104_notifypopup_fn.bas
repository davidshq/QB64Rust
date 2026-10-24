$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x104_notifypopup_fn)
' _NOTIFYPOPUP as a function.
k = 0
IF k = 1 THEN
    x = _NOTIFYPOPUP("a")
END IF
PRINT "ok"
SYSTEM
