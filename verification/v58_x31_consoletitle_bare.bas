$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x31_consoletitle_bare)
' _CONSOLETITLE without an argument.
k = 0
IF k = 1 THEN
    _CONSOLETITLE
END IF
PRINT "ok"
SYSTEM
