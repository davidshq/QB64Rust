$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x33_consolecursor_bare)
' _CONSOLECURSOR without an argument.
k = 0
IF k = 1 THEN
    _CONSOLECURSOR
END IF
PRINT "ok"
SYSTEM
