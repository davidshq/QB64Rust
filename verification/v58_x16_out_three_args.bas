$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x16_out_three_args)
' OUT with three arguments.
k = 0
IF k = 1 THEN
    OUT 1, 2, 3
END IF
PRINT "ok"
SYSTEM
