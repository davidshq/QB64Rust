$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x32_consoletitle_number)
' _CONSOLETITLE with a number.
k = 0
IF k = 1 THEN
    _CONSOLETITLE 1
END IF
PRINT "ok"
SYSTEM
