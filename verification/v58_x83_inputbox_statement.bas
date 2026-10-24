$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x83_inputbox_statement)
' _INPUTBOX$ as a statement.
k = 0
IF k = 1 THEN
    _INPUTBOX$ "t"
END IF
PRINT "ok"
SYSTEM
