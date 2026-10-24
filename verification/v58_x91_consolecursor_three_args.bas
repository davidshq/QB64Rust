$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x91_consolecursor_three_args)
' _CONSOLECURSOR with three arguments.
k = 0
IF k = 1 THEN
    _CONSOLECURSOR _SHOW, 50, 1
END IF
PRINT "ok"
SYSTEM
