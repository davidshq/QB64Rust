$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x09_resizewidth_stmt)
' _RESIZEWIDTH as a statement.
k = 0
IF k = 1 THEN
    _RESIZEWIDTH
END IF
PRINT "ok"
SYSTEM
