$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x35_consolefont_one_arg)
' _CONSOLEFONT with one argument.
k = 0
IF k = 1 THEN
    _CONSOLEFONT "a"
END IF
PRINT "ok"
SYSTEM
