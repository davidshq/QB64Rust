$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x43_screenmove_string)
' _SCREENMOVE of strings.
k = 0
IF k = 1 THEN
    _SCREENMOVE "a", "b"
END IF
PRINT "ok"
SYSTEM
