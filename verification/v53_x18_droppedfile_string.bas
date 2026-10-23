$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x18_droppedfile_string)
' _DROPPEDFILE of a string.
k = 0
IF k = 1 THEN
    f$ = _DROPPEDFILE("a")
END IF
PRINT "ok"
SYSTEM
