$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x39_screenmove_middle_xy)
' _SCREENMOVE _MIDDLE with coordinates after it.
k = 0
IF k = 1 THEN
    _SCREENMOVE _MIDDLE, 10, 20
END IF
PRINT "ok"
SYSTEM
