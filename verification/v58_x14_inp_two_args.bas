$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x14_inp_two_args)
' INP with two arguments.
k = 0
IF k = 1 THEN
    x = INP(1, 2)
END IF
PRINT "ok"
SYSTEM
