$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x111_environcount_const)
' _ENVIRONCOUNT in a CONST (accepted?).
k = 0
IF k = 1 THEN
    CONST c = _ENVIRONCOUNT
END IF
PRINT "ok"
SYSTEM
