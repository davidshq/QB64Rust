$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x07_resize_middle)
' _RESIZE ON, _MIDDLE.
k = 0
IF k = 1 THEN
    _RESIZE ON, _MIDDLE
END IF
PRINT "ok"
SYSTEM
