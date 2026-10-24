$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x20_wait_bare)
' WAIT without an argument.
k = 0
IF k = 1 THEN
    WAIT
END IF
PRINT "ok"
SYSTEM
