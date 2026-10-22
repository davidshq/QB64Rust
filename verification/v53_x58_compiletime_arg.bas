$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x58_compiletime_arg)
' _COMPILETIME$ with an argument.
k = 0
IF k = 1 THEN
    d$ = _COMPILETIME$(1)
END IF
PRINT "ok"
SYSTEM
