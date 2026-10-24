$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x65_compilerversion_arg)
' _COMPILERVERSION$ with an argument.
k = 0
IF k = 1 THEN
    x$ = _COMPILERVERSION$(1)
END IF
PRINT "ok"
SYSTEM
