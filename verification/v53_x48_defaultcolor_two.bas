$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x48_defaultcolor_two)
' _DEFAULTCOLOR with two arguments.
k = 0
IF k = 1 THEN
    x = _DEFAULTCOLOR(1, 2)
END IF
PRINT "ok"
SYSTEM
