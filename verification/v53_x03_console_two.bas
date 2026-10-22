$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x03_console_two)
' _CONSOLE ON, OFF.
k = 0
IF k = 1 THEN
    _CONSOLE ON, OFF
END IF
PRINT "ok"
SYSTEM
