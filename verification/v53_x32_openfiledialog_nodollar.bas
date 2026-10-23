$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x32_openfiledialog_nodollar)
' _OPENFILEDIALOG without $.
k = 0
IF k = 1 THEN
    f$ = _OPENFILEDIALOG("t")
END IF
PRINT "ok"
SYSTEM
