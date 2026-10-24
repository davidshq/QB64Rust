$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x108_setbit_const)
' _SETBIT in a CONST (accepted?).
k = 0
IF k = 1 THEN
    CONST c = _SETBIT(1, 2)
END IF
PRINT "ok"
SYSTEM
