$CONSOLE:ONLY
' Verification (language-reference, second review): does the old compiler accept this? (x07_windowhasfocus_arg)
' _WINDOWHASFOCUS with an argument.
k = 0
IF k = 1 THEN
    x = _WINDOWHASFOCUS(1)
END IF
PRINT "ok"
SYSTEM
