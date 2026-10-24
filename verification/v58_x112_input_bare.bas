$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x112_input_bare)
' INPUT$ without an argument.
k = 0
IF k = 1 THEN
    s$ = INPUT$
END IF
PRINT "ok"
SYSTEM
