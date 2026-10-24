$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x90_displayorder_trailing_comma)
' _DISPLAYORDER with a trailing comma.
k = 0
IF k = 1 THEN
    _DISPLAYORDER _SOFTWARE,
END IF
PRINT "ok"
SYSTEM
