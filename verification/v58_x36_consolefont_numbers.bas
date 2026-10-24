$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x36_consolefont_numbers)
' _CONSOLEFONT with a number for the name.
k = 0
IF k = 1 THEN
    _CONSOLEFONT 1, 2
END IF
PRINT "ok"
SYSTEM
