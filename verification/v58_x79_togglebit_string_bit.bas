$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x79_togglebit_string_bit)
' _TOGGLEBIT with a string bit number.
k = 0
IF k = 1 THEN
    x = _TOGGLEBIT(1, "a")
END IF
PRINT "ok"
SYSTEM
