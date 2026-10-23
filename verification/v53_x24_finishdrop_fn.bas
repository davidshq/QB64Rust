$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x24_finishdrop_fn)
' _FINISHDROP as a function.
k = 0
IF k = 1 THEN
    x = _FINISHDROP
END IF
PRINT "ok"
SYSTEM
