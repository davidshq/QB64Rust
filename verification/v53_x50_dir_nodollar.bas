$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x50_dir_nodollar)
' _DIR without $.
k = 0
IF k = 1 THEN
    d$ = _DIR("fonts")
END IF
PRINT "ok"
SYSTEM
