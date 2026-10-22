$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x11_keyclear_two)
' _KEYCLEAR 1, 2.
k = 0
IF k = 1 THEN
    _KEYCLEAR 1, 2
END IF
PRINT "ok"
SYSTEM
