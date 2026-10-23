$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x05_resize_on_on)
' _RESIZE ON, ON.
k = 0
IF k = 1 THEN
    _RESIZE ON, ON
END IF
PRINT "ok"
SYSTEM
