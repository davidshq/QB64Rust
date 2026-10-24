$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x114_input_three_args)
' INPUT$ with three arguments.
k = 0
IF k = 1 THEN
    s$ = INPUT$(1, 2, 3)
END IF
PRINT "ok"
SYSTEM
