$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x12_keyclear_fn)
' _KEYCLEAR as a function.
k = 0
IF k = 1 THEN
    x = _KEYCLEAR
END IF
PRINT "ok"
SYSTEM
