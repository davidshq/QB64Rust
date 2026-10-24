$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x58_notifypopup_number)
' _NOTIFYPOPUP with a number.
k = 0
IF k = 1 THEN
    _NOTIFYPOPUP 1
END IF
PRINT "ok"
SYSTEM
