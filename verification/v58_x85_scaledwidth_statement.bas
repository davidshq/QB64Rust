$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x85_scaledwidth_statement)
' _SCALEDWIDTH as a statement.
k = 0
IF k = 1 THEN
    _SCALEDWIDTH
END IF
PRINT "ok"
SYSTEM
