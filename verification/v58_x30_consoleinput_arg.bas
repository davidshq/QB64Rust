$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x30_consoleinput_arg)
' _CONSOLEINPUT with an argument.
k = 0
IF k = 1 THEN
    x = _CONSOLEINPUT(1)
END IF
PRINT "ok"
SYSTEM
