$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x36_savefiledialog_five)
' _SAVEFILEDIALOG$ with five arguments.
k = 0
IF k = 1 THEN
    f$ = _SAVEFILEDIALOG$("a", "b", "c", "d", "e")
END IF
PRINT "ok"
SYSTEM
