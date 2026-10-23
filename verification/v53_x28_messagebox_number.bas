$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x28_messagebox_number)
' _MESSAGEBOX with a number for the icon.
k = 0
IF k = 1 THEN
    _MESSAGEBOX "a", "b", 1
END IF
PRINT "ok"
SYSTEM
