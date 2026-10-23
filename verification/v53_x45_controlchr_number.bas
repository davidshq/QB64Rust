$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x45_controlchr_number)
' _CONTROLCHR 1.
k = 0
IF k = 1 THEN
    _CONTROLCHR 1
END IF
PRINT "ok"
SYSTEM
