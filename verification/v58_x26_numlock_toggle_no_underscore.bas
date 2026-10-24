$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x26_numlock_toggle_no_underscore)
' _NUMLOCK TOGGLE without the underscore.
k = 0
IF k = 1 THEN
    _NUMLOCK TOGGLE
END IF
PRINT "ok"
SYSTEM
