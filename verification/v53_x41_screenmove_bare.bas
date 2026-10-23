$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x41_screenmove_bare)
' _SCREENMOVE with nothing.
k = 0
IF k = 1 THEN
    _SCREENMOVE
END IF
PRINT "ok"
SYSTEM
