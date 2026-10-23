$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x60_readbit_string)
' _READBIT of a string.
k = 0
IF k = 1 THEN
    x = _READBIT("a", 1)
END IF
PRINT "ok"
SYSTEM
