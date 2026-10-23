$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x37_selectfolderdialog_three)
' _SELECTFOLDERDIALOG$ with three arguments.
k = 0
IF k = 1 THEN
    f$ = _SELECTFOLDERDIALOG$("a", "b", "c")
END IF
PRINT "ok"
SYSTEM
