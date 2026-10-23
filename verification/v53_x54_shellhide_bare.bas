$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x54_shellhide_bare)
' _SHELLHIDE with no argument.
k = 0
IF k = 1 THEN
    x = _SHELLHIDE
END IF
PRINT "ok"
SYSTEM
