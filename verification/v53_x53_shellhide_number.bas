$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x53_shellhide_number)
' _SHELLHIDE of a number.
k = 0
IF k = 1 THEN
    x = _SHELLHIDE(1)
END IF
PRINT "ok"
SYSTEM
