$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x52_dir_bare)
' _DIR$ with no argument.
k = 0
IF k = 1 THEN
    d$ = _DIR$
END IF
PRINT "ok"
SYSTEM
