$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x34_openfiledialog_stmt)
' _OPENFILEDIALOG$ as a statement.
k = 0
IF k = 1 THEN
    _OPENFILEDIALOG$ "t"
END IF
PRINT "ok"
SYSTEM
