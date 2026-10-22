$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x42_screenmove_fn)
' _SCREENMOVE as a function.
k = 0
IF k = 1 THEN
    x = _SCREENMOVE
END IF
PRINT "ok"
SYSTEM
