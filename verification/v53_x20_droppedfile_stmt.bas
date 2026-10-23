$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x20_droppedfile_stmt)
' _DROPPEDFILE as a statement.
k = 0
IF k = 1 THEN
    _DROPPEDFILE
END IF
PRINT "ok"
SYSTEM
