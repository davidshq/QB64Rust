$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x57_compiledate_nodollar)
' _COMPILEDATE without $.
k = 0
IF k = 1 THEN
    d$ = _COMPILEDATE
END IF
PRINT "ok"
SYSTEM
