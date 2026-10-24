$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x109_mksmbf_const)
' MKSMBF$ in a CONST (accepted?).
k = 0
IF k = 1 THEN
    CONST c$ = MKSMBF$(1.5)
END IF
PRINT "ok"
SYSTEM
