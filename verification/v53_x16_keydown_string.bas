$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x16_keydown_string)
' _KEYDOWN of a string.
k = 0
IF k = 1 THEN
    x = _KEYDOWN("a")
END IF
PRINT "ok"
SYSTEM
