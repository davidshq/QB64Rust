$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x64_compiledate_stmt)
' _COMPILEDATE$ as a statement.
k = 0
IF k = 1 THEN
    _COMPILEDATE$
END IF
PRINT "ok"
SYSTEM
