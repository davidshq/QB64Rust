$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x23_finishdrop_arg)
' _FINISHDROP 1.
k = 0
IF k = 1 THEN
    _FINISHDROP 1
END IF
PRINT "ok"
SYSTEM
