$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x95_compilerversion_statement)
' _COMPILERVERSION$ as a statement.
k = 0
IF k = 1 THEN
    _COMPILERVERSION$
END IF
PRINT "ok"
SYSTEM
