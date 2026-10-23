$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x14_keyhit_stmt)
' _KEYHIT as a statement.
k = 0
IF k = 1 THEN
    _KEYHIT
END IF
PRINT "ok"
SYSTEM
