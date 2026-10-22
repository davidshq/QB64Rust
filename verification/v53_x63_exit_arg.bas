$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x63_exit_arg)
' _EXIT with an argument.
k = 0
IF k = 1 THEN
    x = _EXIT(1)
END IF
PRINT "ok"
SYSTEM
