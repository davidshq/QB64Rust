$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x59_notifypopup_four_args)
' _NOTIFYPOPUP with four arguments.
k = 0
IF k = 1 THEN
    _NOTIFYPOPUP "a", "b", "c", "d"
END IF
PRINT "ok"
SYSTEM
