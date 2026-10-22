$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x33_openfiledialog_number)
' _OPENFILEDIALOG$ of a number.
k = 0
IF k = 1 THEN
    f$ = _OPENFILEDIALOG$(1)
END IF
PRINT "ok"
SYSTEM
