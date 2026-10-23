$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x25_totaldropped_stmt)
' _TOTALDROPPEDFILES as a statement.
k = 0
IF k = 1 THEN
    _TOTALDROPPEDFILES
END IF
PRINT "ok"
SYSTEM
