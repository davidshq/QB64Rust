$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x56_screenprint_bare)
' _SCREENPRINT without an argument.
k = 0
IF k = 1 THEN
    _SCREENPRINT
END IF
PRINT "ok"
SYSTEM
