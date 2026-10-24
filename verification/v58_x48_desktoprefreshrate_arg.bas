$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x48_desktoprefreshrate_arg)
' _DESKTOPREFRESHRATE with an argument.
k = 0
IF k = 1 THEN
    x = _DESKTOPREFRESHRATE(1)
END IF
PRINT "ok"
SYSTEM
