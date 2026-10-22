$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x40_screenmove_one)
' _SCREENMOVE with one coordinate.
k = 0
IF k = 1 THEN
    _SCREENMOVE 10
END IF
PRINT "ok"
SYSTEM
