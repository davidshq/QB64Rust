$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x01_console_bare)
' the statement _CONSOLE with no word.
k = 0
IF k = 1 THEN
    _CONSOLE
END IF
PRINT "ok"
SYSTEM
