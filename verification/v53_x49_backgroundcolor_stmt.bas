$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x49_backgroundcolor_stmt)
' _BACKGROUNDCOLOR as a statement.
k = 0
IF k = 1 THEN
    _BACKGROUNDCOLOR 1
END IF
PRINT "ok"
SYSTEM
