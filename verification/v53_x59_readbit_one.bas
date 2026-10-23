$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x59_readbit_one)
' _READBIT with one argument.
k = 0
IF k = 1 THEN
    x = _READBIT(1)
END IF
PRINT "ok"
SYSTEM
