$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x47_defaultcolor_string)
' _DEFAULTCOLOR of a string.
k = 0
IF k = 1 THEN
    x = _DEFAULTCOLOR("a")
END IF
PRINT "ok"
SYSTEM
