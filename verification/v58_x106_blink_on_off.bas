$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x106_blink_on_off)
' _BLINK ON, OFF.
k = 0
IF k = 1 THEN
    _BLINK ON, OFF
END IF
PRINT "ok"
SYSTEM
