$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x55_shellhide_stmt)
' _SHELLHIDE as a statement.
k = 0
IF k = 1 THEN
    _SHELLHIDE "cmd /c exit 0"
END IF
PRINT "ok"
SYSTEM
