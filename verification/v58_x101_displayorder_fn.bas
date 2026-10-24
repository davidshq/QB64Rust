$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x101_displayorder_fn)
' _DISPLAYORDER as a function.
k = 0
IF k = 1 THEN
    x = _DISPLAYORDER(1)
END IF
PRINT "ok"
SYSTEM
