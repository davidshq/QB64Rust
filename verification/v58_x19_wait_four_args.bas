$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x19_wait_four_args)
' WAIT with four arguments.
k = 0
IF k = 1 THEN
    WAIT 1, 2, 3, 4
END IF
PRINT "ok"
SYSTEM
