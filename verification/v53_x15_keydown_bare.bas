$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x15_keydown_bare)
' _KEYDOWN with no argument.
k = 0
IF k = 1 THEN
    x = _KEYDOWN
END IF
PRINT "ok"
SYSTEM
