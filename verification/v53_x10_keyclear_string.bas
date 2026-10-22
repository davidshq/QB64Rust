$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x10_keyclear_string)
' _KEYCLEAR "a".
k = 0
IF k = 1 THEN
    _KEYCLEAR "a"
END IF
PRINT "ok"
SYSTEM
