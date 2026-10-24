$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x28_cinp_two_args)
' _CINP with two arguments.
k = 0
IF k = 1 THEN
    x = _CINP(1, 2)
END IF
PRINT "ok"
SYSTEM
