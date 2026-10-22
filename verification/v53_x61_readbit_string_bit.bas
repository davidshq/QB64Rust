$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x61_readbit_string_bit)
' _READBIT with a string bit number.
k = 0
IF k = 1 THEN
    x = _READBIT(1, "a")
END IF
PRINT "ok"
SYSTEM
