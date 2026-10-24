$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x21_lpos_bare)
' LPOS without an argument.
k = 0
IF k = 1 THEN
    x = LPOS
END IF
PRINT "ok"
SYSTEM
