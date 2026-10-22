$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x51_dir_number)
' _DIR$ of a number.
k = 0
IF k = 1 THEN
    d$ = _DIR$(1)
END IF
PRINT "ok"
SYSTEM
