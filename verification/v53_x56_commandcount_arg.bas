$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x56_commandcount_arg)
' _COMMANDCOUNT with an argument.
k = 0
IF k = 1 THEN
    x = _COMMANDCOUNT(1)
END IF
PRINT "ok"
SYSTEM
