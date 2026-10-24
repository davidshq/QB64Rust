$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x78_setbit_float_bit)
' _SETBIT with a float bit number (accepted?).
k = 0
IF k = 1 THEN
    x = _SETBIT(1, 1.5)
END IF
PRINT "ok"
SYSTEM
