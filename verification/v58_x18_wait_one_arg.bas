$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x18_wait_one_arg)
' WAIT with one argument.
k = 0
IF k = 1 THEN
    WAIT 1
END IF
PRINT "ok"
SYSTEM
