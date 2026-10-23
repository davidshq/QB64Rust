$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x19_droppedfile_two)
' _DROPPEDFILE$ with two arguments.
k = 0
IF k = 1 THEN
    f$ = _DROPPEDFILE$(1, 2)
END IF
PRINT "ok"
SYSTEM
