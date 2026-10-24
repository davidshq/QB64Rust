$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x15_out_one_arg)
' OUT with one argument.
k = 0
IF k = 1 THEN
    OUT 1
END IF
PRINT "ok"
SYSTEM
