$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x87_cinp_statement)
' _CINP as a statement.
k = 0
IF k = 1 THEN
    _CINP
END IF
PRINT "ok"
SYSTEM
