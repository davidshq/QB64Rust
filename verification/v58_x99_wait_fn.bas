$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x99_wait_fn)
' WAIT as a function.
k = 0
IF k = 1 THEN
    x = WAIT(1, 2)
END IF
PRINT "ok"
SYSTEM
