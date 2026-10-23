$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x38_selectfolderdialog_bare)
' _SELECTFOLDERDIALOG$ with no argument.
k = 0
IF k = 1 THEN
    f$ = _SELECTFOLDERDIALOG$
END IF
PRINT "ok"
SYSTEM
