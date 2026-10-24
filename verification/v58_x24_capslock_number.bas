$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x24_capslock_number)
' _CAPSLOCK with a number.
k = 0
IF k = 1 THEN
    _CAPSLOCK 1
END IF
PRINT "ok"
SYSTEM
