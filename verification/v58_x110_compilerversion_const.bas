$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x110_compilerversion_const)
' _COMPILERVERSION$ in a CONST (accepted?).
k = 0
IF k = 1 THEN
    CONST c$ = _COMPILERVERSION$
END IF
PRINT "ok"
SYSTEM
